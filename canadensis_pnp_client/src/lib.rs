#![no_std]

extern crate alloc;

extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_data_types;
extern crate canadensis_encoding;
extern crate canadensis_filter_config;
extern crate crc_any;

use alloc::vec::Vec;
use canadensis::anonymous::AnonymousPublisher;
use canadensis_can::queue::{FrameQueueSource, FrameSink};
use canadensis_can::{Frame, Mtu, OutOfMemoryError, Receiver, Transmitter};
use canadensis_core::time::{milliseconds, Clock};
use canadensis_core::{NodeId, Priority, SubjectId};
use canadensis_data_types::uavcan::pnp::node_id_allocation_data_1_0::NodeIdAllocationData;
use canadensis_encoding::{Deserialize, Message, Serialize};
use canadensis_filter_config::Filter;
use core::convert::TryFrom;
use core::marker::PhantomData;
use crc_any::CRCu64;

/// A plug-and-play allocation client that can be used to find a node ID
pub struct PnpClient<C: Clock, M> {
    /// The unique ID of this node
    unique_id: [u8; 16],
    /// Publisher used to send messages
    publisher: AnonymousPublisher<C, M>,
    /// Transmitter used along with the publisher to send messages
    transmitter: Transmitter<SingleFrameQueue<C::Instant>>,
    /// Receiver used to receive messages
    receiver: Receiver<C::Instant>,
    _message: PhantomData<M>,
}

impl<C, M> PnpClient<C, M>
where
    C: Clock,
    M: AllocationMessage,
{
    pub fn new(mtu: Mtu, unique_id: [u8; 16]) -> Result<Self, OutOfMemoryError> {
        let mut receiver = Receiver::new_anonymous(mtu);
        receiver.subscribe_message(M::SUBJECT, 9, milliseconds(1000))?;

        Ok(PnpClient {
            unique_id,
            publisher: AnonymousPublisher::new(
                M::SUBJECT,
                Priority::Nominal,
                milliseconds(1000),
                mtu,
            ),
            transmitter: Transmitter::new(mtu, SingleFrameQueue::new()),
            receiver,
            _message: PhantomData,
        })
    }

    /// Creates an outgoing node ID allocation message and returns it encoded into one CAN frame
    pub fn assemble_request(&mut self, now: C::Instant) -> Frame<C::Instant> {
        let message = M::with_unique_id(&self.unique_id);
        let publish_status = self.publisher.send(&message, now, &mut self.transmitter);
        if publish_status.is_err() {
            panic!("Can't fit message into one frame");
        }
        match self.transmitter.frame_queue_mut().pop_frame() {
            Some(frame) => frame,
            None => panic!("Didn't get a frame"),
        }
    }

    /// Handles an incoming frame and checks if it provides an ID for this node
    ///
    /// This function returns the node ID if one was assigned.
    pub fn accept(&mut self, frame: Frame<C::Instant>) -> Result<Option<NodeId>, OutOfMemoryError> {
        if let Some(transfer_in) = self.receiver.accept(frame)? {
            if let Ok(message) = M::deserialize_from_bytes(&transfer_in.payload) {
                if message.matches_unique_id(&self.unique_id) {
                    if let Some(node_id) = message.node_id() {
                        return Ok(Some(node_id));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Returns the filter(s) that will accept node ID allocation frames
    pub fn frame_fiters(&self) -> Result<Vec<Filter>, OutOfMemoryError> {
        self.receiver.frame_filters()
    }
}

/// A node ID allocation message
///
/// This is currently implemented for `uavcan.pnp.NodeIdAllocationData` version 1.0. In the future,
/// it may also be implemented for version 2.0 of that data type.
pub trait AllocationMessage: Message + Serialize + Deserialize {
    /// The fixed subject ID for this message
    const SUBJECT: SubjectId;

    /// Creates a message with the provided unique ID and no allocated node ID
    ///
    /// The message must fit into one frame of the transport that is being used.
    fn with_unique_id(id: &[u8; 16]) -> Self;

    /// Determines if this message matches the provided unique ID
    fn matches_unique_id(&self, id: &[u8; 16]) -> bool;

    /// Returns the allocated node ID in this message, if one is specified
    fn node_id(&self) -> Option<NodeId>;
}

impl AllocationMessage for NodeIdAllocationData {
    const SUBJECT: SubjectId = NodeIdAllocationData::SUBJECT;

    fn with_unique_id(id: &[u8; 16]) -> Self {
        let id_hash = crc_64we_48_bits(id);
        NodeIdAllocationData {
            unique_id_hash: id_hash,
            allocated_node_id: None,
        }
    }

    fn matches_unique_id(&self, id: &[u8; 16]) -> bool {
        let id_hash = crc_64we_48_bits(id);
        self.unique_id_hash == id_hash
    }

    fn node_id(&self) -> Option<NodeId> {
        self.allocated_node_id.and_then(|id_bits| {
            // The message allows a wider range of node IDs than the UAVCAN/CAN transport allows.
            // If the ID is too large, return None.
            let short_id = u8::try_from(id_bits).ok()?;
            NodeId::try_from(short_id).ok()
        })
    }
}

/// Calculates a CRC-64WE hash of the provided ID and returns the less significant 48 bits of the
/// result
fn crc_64we_48_bits(id: &[u8; 16]) -> u64 {
    let mut crc = CRCu64::crc64we();
    crc.digest(id);
    let value = crc.get_crc();
    value & 0x0000_ffff_ffff_ffff
}

/// An outgoing frame queue that can hold only one frame
struct SingleFrameQueue<I> {
    frame: Option<Frame<I>>,
}

impl<I> SingleFrameQueue<I> {
    fn new() -> Self {
        SingleFrameQueue { frame: None }
    }
}

impl<I> FrameSink<I> for SingleFrameQueue<I> {
    fn try_reserve(&mut self, additional: usize) -> Result<(), OutOfMemoryError> {
        if self.frame.is_none() && additional == 1 {
            Ok(())
        } else {
            Err(OutOfMemoryError)
        }
    }

    fn shrink_to_fit(&mut self) {
        // Nothing to do
    }

    fn push_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        if self.frame.is_none() {
            self.frame = Some(frame);
            Ok(())
        } else {
            Err(OutOfMemoryError)
        }
    }
}

impl<I> FrameQueueSource<I> for SingleFrameQueue<I> {
    fn peek_frame(&self) -> Option<&Frame<I>> {
        self.frame.as_ref()
    }

    fn pop_frame(&mut self) -> Option<Frame<I>> {
        self.frame.take()
    }

    fn return_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        if self.frame.is_some() {
            Err(OutOfMemoryError)
        } else {
            self.frame = Some(frame);
            Ok(())
        }
    }
}
