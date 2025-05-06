extern crate alloc;

extern crate canadensis_data_types;
extern crate heapless;

use crate::core::time::milliseconds;
use crate::core::transfer::MessageTransfer;
use crate::core::transport::{Receiver, Transmitter, Transport};
use crate::core::{nb, Priority, SubjectId};
use crate::encoding::{Deserialize, Message, Serialize};
use crate::{Node, PublishToken, StartSendError, TransferHandler};
use alloc::vec::Vec;
use canadensis_data_types::uavcan::node::id_1_0::ID;
use canadensis_data_types::uavcan::pnp::{
    node_id_allocation_data_1_0, node_id_allocation_data_2_0,
};
use core::convert::TryFrom;
use core::marker::PhantomData;
use crc_any::CRCu64;
use defmt::Format;

/// A plug-and-play allocation client that can be used to find a node ID
pub struct PnpClientService<N, M> {
    /// The unique ID of this node
    unique_id: [u8; 16],
    publish_token: Option<PublishToken<M>>,
    _node: PhantomData<N>,
}

impl<N, M> PnpClientService<N, M>
where
    N: Node,
    M: AllocationMessage<N::Transport>,
{
    /// Creates a new plug-and-play client
    ///
    /// * `unique_id`: The unique ID of this node
    ///
    /// # Panics
    ///
    /// This function will panic if the message size is larger than the MTU of the node's transmitter.
    pub fn new(node: &mut N, unique_id: [u8; 16]) -> Result<Self, NewError<N>> {
        debug_assert!(
            M::PAYLOAD_SIZE_MAX <= node.transmitter().mtu(),
            "Can't fit transfer into one frame"
        );

        node.subscribe_message(M::SUBJECT, M::PAYLOAD_SIZE_MAX, milliseconds(1000))
            .map_err(|err| NewError::Subscribe(err))?;

        let token = node
            .start_publishing(M::SUBJECT, milliseconds(1000), Priority::Nominal.into())
            .map_err(|err| match err {
                StartSendError::Memory(_) => NewError::OutOfMemory,
                StartSendError::Duplicate => NewError::Duplicate,
                StartSendError::Transport(err) => NewError::Publish(err),
                StartSendError::AnonymousRequest => unreachable!(), // we are publishing a message, not a request
            })?;

        Ok(Self {
            unique_id,
            publish_token: Some(token),
            _node: PhantomData,
        })
    }

    /// Creates an outgoing node ID allocation message and gives it to the node
    pub fn send_request(&mut self, node: &mut N) -> Result<(), SendRequestError<N>> {
        let message = M::with_unique_id(&self.unique_id);
        node.publish(
            self.publish_token
                .as_ref()
                .ok_or(SendRequestError::Allocated)?,
            &message,
        )
        .map_err(|err| match err {
            nb::Error::WouldBlock => SendRequestError::WouldBlock,
            nb::Error::Other(err) => SendRequestError::Other(err),
        })
    }

    /// Returns a handler for the client
    pub fn handler(&mut self) -> PnpClientServiceHandler<'_, N, M> {
        PnpClientServiceHandler { client: self }
    }
}

/// Error type returned by [`PnpClientService::new`].
#[derive(Debug, Format)]
pub enum NewError<N: Node> {
    /// The client could not subscribe to the message subject due to a receiver error.
    Subscribe(<N::Receiver as Receiver<N::Clock>>::Error),
    /// The client could not allocate a publish token due to an out of memory error.
    OutOfMemory,
    /// The client could not allocate a publish token as the subject is already in use.
    Duplicate,
    /// The client could not allocate a publish token due to a transmitter error.
    Publish(<N::Transmitter as Transmitter<N::Clock>>::Error),
}

/// Error type returned by [`PnpClientService::send_request`].
#[derive(Debug, Format)]
pub enum SendRequestError<N: Node> {
    /// The client has allocated an address already and no longer holds a publish token.
    Allocated,
    /// The client could not send the request because it would block
    WouldBlock,
    /// The client could not send the request due to a transmitter error
    Other(<N::Transmitter as Transmitter<N::Clock>>::Error),
}

/// Handler for the client
pub struct PnpClientServiceHandler<'a, N, M> {
    client: &'a mut PnpClientService<N, M>,
}

impl<N, M> TransferHandler<N::Transport> for PnpClientServiceHandler<'_, N, M>
where
    N: Node,
    M: AllocationMessage<N::Transport>,
{
    fn handle_message<N2: Node<Transport = N::Transport>>(
        &mut self,
        node: &mut N2,
        transfer: &MessageTransfer<Vec<u8>, N2::Transport>,
    ) -> bool {
        if self.client.publish_token.is_some() {
            if let Ok(message) = M::deserialize_from_bytes(&transfer.payload) {
                if message.matches_unique_id(&self.client.unique_id) {
                    if let Some(node_id) = message.node_id() {
                        node.set_node_id(node_id);
                        node.unsubscribe_message(M::SUBJECT);
                        node.stop_publishing(self.client.publish_token.take().unwrap());
                        return true;
                    }
                }
            }
        }
        false
    }
}

/// A node ID allocation message
pub trait AllocationMessage<T: Transport>: Message + Serialize + Deserialize {
    /// The fixed subject ID for this message
    const SUBJECT: SubjectId;

    /// The maximum payload size for this message
    const PAYLOAD_SIZE_MAX: usize;

    /// Creates a message with the provided unique ID and no allocated node ID
    ///
    /// The message must fit into one frame of the transport that is being used.
    fn with_unique_id(id: &[u8; 16]) -> Self;

    /// Determines if this message matches the provided unique ID
    fn matches_unique_id(&self, id: &[u8; 16]) -> bool;

    /// Returns the allocated node ID in this message, if one is specified
    fn node_id(&self) -> Option<T::NodeId>;
}

impl<T: Transport> AllocationMessage<T> for node_id_allocation_data_1_0::NodeIDAllocationData {
    const SUBJECT: SubjectId = node_id_allocation_data_1_0::SUBJECT;
    const PAYLOAD_SIZE_MAX: usize = 9;

    fn with_unique_id(id: &[u8; 16]) -> Self {
        let id_hash = crc_64we_48_bits(id);
        Self {
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

impl<T: Transport> AllocationMessage<T> for node_id_allocation_data_2_0::NodeIDAllocationData {
    const SUBJECT: SubjectId = node_id_allocation_data_2_0::SUBJECT;
    const PAYLOAD_SIZE_MAX: usize = 18;

    fn with_unique_id(id: &[u8; 16]) -> Self {
        Self {
            unique_id: *id,
            node_id: ID { value: 0 },
        }
    }

    fn matches_unique_id(&self, id: &[u8; 16]) -> bool {
        self.unique_id == *id
    }

    fn node_id(&self) -> Option<<T as Transport>::NodeId> {
        // The message may allow a wider range of node IDs than the transport allows.
        // If the ID is too large, return None.
        T::NodeId::try_from(self.node_id.value).ok()
    }
}
