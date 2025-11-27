extern crate alloc;

extern crate canadensis_data_types;
extern crate heapless;

use crate::core::time::milliseconds;
use crate::core::transfer::MessageTransfer;
use crate::core::transport::Transmitter;
use crate::core::{nb, Priority};
use crate::service::pnp::{AllocationMessage, NewError};
use crate::{Node, PublishError, StartSendError, TransferHandler};
use alloc::vec::Vec;
use core::marker::PhantomData;

/// A plug-and-play allocation client that can be used to find a node ID
pub struct PnpClientService<N, M> {
    /// The unique ID of this node
    unique_id: [u8; 16],
    _node: PhantomData<N>,
    _message: PhantomData<M>,
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

        node.start_publishing(M::SUBJECT, milliseconds(1000), Priority::Nominal.into())
            .map_err(|err| match err {
                StartSendError::Memory(_) => NewError::OutOfMemory,
                StartSendError::Duplicate => NewError::Duplicate,
                StartSendError::Transport(err) => NewError::Publish(err),
                StartSendError::AnonymousRequest => unreachable!(), // we are publishing a message, not a request
            })?;

        Ok(Self {
            unique_id,
            _node: PhantomData,
            _message: PhantomData,
        })
    }

    /// Creates an outgoing node ID allocation message and gives it to the node
    pub fn send_request(
        &mut self,
        node: &mut N,
    ) -> nb::Result<(), PublishError<<N::Transmitter as Transmitter<N::Clock>>::Error>> {
        let message = M::with_unique_id(&self.unique_id);
        node.publish(M::SUBJECT, &message)
    }

    /// Returns a handler for the client
    pub fn handler(&mut self) -> PnpClientServiceHandler<'_, N, M> {
        PnpClientServiceHandler { client: self }
    }
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
        if let Ok(message) = M::deserialize_from_bytes(&transfer.payload) {
            if message.matches_unique_id(&self.client.unique_id) {
                if let Some(node_id) = message.node_id() {
                    node.set_node_id(node_id);
                    node.unsubscribe_message(M::SUBJECT);
                    node.stop_publishing(M::SUBJECT);
                    return true;
                }
            }
        }
        false
    }
}
