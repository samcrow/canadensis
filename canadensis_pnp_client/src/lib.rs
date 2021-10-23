//!
//! # Canadensis plug-and-play client
//!
//! This library implements the UAVCAN plug-and-play node ID allocation protocol.
//!

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

extern crate canadensis;
extern crate canadensis_data_types;
extern crate canadensis_filter_config;
extern crate crc_any;
extern crate heapless;

use canadensis::anonymous::AnonymousPublisher;
use canadensis::core::time::{milliseconds, Clock};
use canadensis::core::transport::{Receiver, Transmitter, Transport};
use canadensis::core::{Priority, SubjectId};
use canadensis::encoding::{Deserialize, Message, Serialize};
use canadensis_data_types::uavcan::pnp::node_id_allocation_data_1_0::{self, NodeIDAllocationData};
use core::convert::TryFrom;
use core::fmt::Debug;
use core::marker::PhantomData;
use crc_any::CRCu64;

/// A plug-and-play allocation client that can be used to find a node ID
pub struct PnpClient<C: Clock, M, T: Transmitter<C::Instant>, R: Receiver<C::Instant>> {
    /// The unique ID of this node
    unique_id: [u8; 16],
    /// Publisher used to send messages
    publisher: AnonymousPublisher<C, M, T>,
    /// Transmitter used along with the publisher to send messages
    transmitter: T,
    /// Receiver used to receive messages
    receiver: R,
    _message: PhantomData<M>,
}

impl<C, M, T, R, P> PnpClient<C, M, T, R>
where
    C: Clock,
    M: AllocationMessage<P>,
    T: Transmitter<C::Instant, Transport = P>,
    R: Receiver<C::Instant, Transport = P>,
    P: Transport,
    P::Error: Debug,
{
    /// Creates a new plug-and-play client
    ///
    /// * `mtu`: The maximum transmission unit size to use when sending frames
    /// * `unique_id`: The unique ID of this node
    pub fn new(transmitter: T, mut receiver: R, unique_id: [u8; 16]) -> Result<Self, P::Error> {
        receiver.subscribe_message(M::SUBJECT, 9, milliseconds(1000))?;

        Ok(PnpClient {
            unique_id,
            publisher: AnonymousPublisher::new(
                M::SUBJECT,
                Priority::Nominal.into(),
                milliseconds(1000),
            ),
            transmitter,
            receiver,
            _message: PhantomData,
        })
    }

    /// Creates an outgoing node ID allocation message and gives it to the transmitter
    pub fn send_request(&mut self, clock: &mut C) {
        let message = M::with_unique_id(&self.unique_id);
        self.publisher
            .send(&message, clock, &mut self.transmitter)
            .expect("Can't fit message into one frame");
    }

    /// Handles an incoming frame and checks if it provides an ID for this node
    ///
    /// This function returns the node ID if one was assigned.
    pub fn receive(&mut self, now: C::Instant) -> Result<Option<P::NodeId>, P::Error> {
        if let Some(transfer_in) = self.receiver.receive(now)? {
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

    /// Returns a reference to the transmitter
    pub fn transmitter(&self) -> &T {
        &self.transmitter
    }
    /// Returns a mutable reference to the transmitter
    pub fn transmitter_mut(&mut self) -> &mut T {
        &mut self.transmitter
    }
    /// Returns a reference to the receiver
    pub fn receiver(&self) -> &R {
        &self.receiver
    }
    /// Returns a mutable reference to the receiver
    pub fn receiver_mut(&mut self) -> &mut R {
        &mut self.receiver
    }
}

/// A node ID allocation message
///
/// This is currently implemented for `uavcan.pnp.NodeIdAllocationData` version 1.0. In the future,
/// it may also be implemented for version 2.0 of that data type.
pub trait AllocationMessage<T: Transport>: Message + Serialize + Deserialize {
    /// The fixed subject ID for this message
    const SUBJECT: SubjectId;

    /// Creates a message with the provided unique ID and no allocated node ID
    ///
    /// The message must fit into one frame of the transport that is being used.
    fn with_unique_id(id: &[u8; 16]) -> Self;

    /// Determines if this message matches the provided unique ID
    fn matches_unique_id(&self, id: &[u8; 16]) -> bool;

    /// Returns the allocated node ID in this message, if one is specified
    fn node_id(&self) -> Option<T::NodeId>;
}

impl<T: Transport> AllocationMessage<T> for NodeIDAllocationData {
    const SUBJECT: SubjectId = node_id_allocation_data_1_0::SUBJECT;

    fn with_unique_id(id: &[u8; 16]) -> Self {
        let id_hash = crc_64we_48_bits(id);
        NodeIDAllocationData {
            unique_id_hash: id_hash,
            allocated_node_id: heapless::Vec::new(),
        }
    }

    fn matches_unique_id(&self, id: &[u8; 16]) -> bool {
        let id_hash = crc_64we_48_bits(id);
        self.unique_id_hash == id_hash
    }

    fn node_id(&self) -> Option<T::NodeId> {
        self.allocated_node_id.iter().next().and_then(|id| {
            // The message may allow a wider range of node IDs than the transport allows.
            // If the ID is too large, return None.
            T::NodeId::try_from(id.value).ok()
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
