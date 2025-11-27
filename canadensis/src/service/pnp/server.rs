use crate::service::pnp::{AllocationMessage, NewError};
use crate::{Node, StartSendError, TransferHandler};
use alloc::vec::Vec;
use canadensis_core::time::milliseconds;
use canadensis_core::transfer::MessageTransfer;
use canadensis_core::transport::{Transmitter, Transport};
use canadensis_core::Priority;
use canadensis_data_types::uavcan::node::heartbeat_1_0;
use canadensis_data_types::uavcan::pnp::node_id_allocation_data_1_0::NodeIDAllocationData as Data1;
use canadensis_data_types::uavcan::pnp::node_id_allocation_data_2_0::NodeIDAllocationData as Data2;
use canadensis_encoding::Deserialize;
use core::convert::TryInto;
use core::marker::PhantomData;
use num_traits::Bounded;

/// Defines the states of assignment
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Assignment {
    /// No assignment
    Unassigned,
    /// This is used to mark an ID as assigned without knowing the unique ID
    Reserved,
    /// Fully assigned with unique ID known
    Assigned([u8; 16]),
}

impl Assignment {
    /// Returns true if the value is [`Assigned`] or [`Reserved`] and the value matches a predicate
    pub fn is_assigned_and(self, f: impl FnOnce([u8; 16]) -> bool) -> bool {
        match self {
            Assignment::Assigned(id) => f(id),
            Assignment::Reserved => true,
            _ => false,
        }
    }

    /// Returns true if the value is [`Assigned`] or [`Reserved`]
    pub fn is_assigned(self) -> bool {
        self.is_assigned_and(|_| true)
    }
}

/// A plug-and-play allocation server
pub struct PnpServerService<N, const C: usize> {
    assignments: [Assignment; C],
    _node: PhantomData<N>,
}

impl<N: Node, const C: usize> PnpServerService<N, C> {
    /// Creates a new plug-and-play server
    ///
    /// # Panics
    ///
    /// This function will panic if the message size is larger than the MTU of the node's transmitter or if the `C` generic parameter is not equal to the maximum value of `Transport::NodeId`.
    pub fn new(node: &mut N) -> Result<Self, NewError<N>> {
        debug_assert_eq!(
            C,
            <<N::Transport as Transport>::NodeId as Bounded>::max_value()
                .into()
                .into(),
            "C must be the maximum value of the node ID type"
        );

        node.subscribe_message(
            <Data1 as AllocationMessage<N::Transport>>::SUBJECT,
            <Data1 as AllocationMessage<N::Transport>>::PAYLOAD_SIZE_MAX,
            milliseconds(1000),
        )
        .map_err(|err| NewError::Subscribe(err))?;

        node.start_publishing(
            <Data1 as AllocationMessage<N::Transport>>::SUBJECT,
            milliseconds(1000),
            Priority::Nominal.into(),
        )
        .map_err(|err| match err {
            StartSendError::Memory(_) => NewError::OutOfMemory,
            StartSendError::Duplicate => NewError::Duplicate,
            StartSendError::Transport(err) => NewError::Publish(err),
            StartSendError::AnonymousRequest => unreachable!(), // we are publishing a message, not a request
        })?;

        if <Data2 as AllocationMessage<N::Transport>>::PAYLOAD_SIZE_MAX <= node.transmitter().mtu()
        {
            node.subscribe_message(
                <Data2 as AllocationMessage<N::Transport>>::SUBJECT,
                <Data2 as AllocationMessage<N::Transport>>::PAYLOAD_SIZE_MAX,
                milliseconds(1000),
            )
            .map_err(|err| NewError::Subscribe(err))?;

            node.start_publishing(
                <Data2 as AllocationMessage<N::Transport>>::SUBJECT,
                milliseconds(1000),
                Priority::Nominal.into(),
            )
            .map_err(|err| match err {
                StartSendError::Memory(_) => NewError::OutOfMemory,
                StartSendError::Duplicate => NewError::Duplicate,
                StartSendError::Transport(err) => NewError::Publish(err),
                StartSendError::AnonymousRequest => unreachable!(), // we are publishing a message, not a request
            })?;
        }

        Ok(PnpServerService {
            assignments: [Assignment::Unassigned; C],
            _node: PhantomData,
        })
    }

    /// Returns a handler for the server
    pub fn handler(&mut self) -> PnpServerServiceHandler<'_, N, C> {
        PnpServerServiceHandler { server: self }
    }

    /// Assigns a [`NodeId`] to a given unique ID. This will only have an effect if the [`NodeId`] has not yet been assigned or reserved.
    pub fn assign(&mut self, node_id: <N::Transport as Transport>::NodeId, unique_id: [u8; 16]) {
        let idx: usize = node_id.into().into();
        if self.assignments[idx].is_assigned() {
            return;
        }

        self.assignments[idx] = Assignment::Assigned(unique_id);
    }

    /// Get an iterator over the current assignments
    ///
    /// # Panics
    ///
    /// This function should never panic despite the use of [`unwrap`] as the values are known to be sufficiently constrained
    pub fn assignments(
        &self,
    ) -> impl Iterator<Item = (<N::Transport as Transport>::NodeId, Assignment)> + use<'_, N, C>
    {
        self.assignments.iter().copied().enumerate().map(|x| {
            (
                <usize as TryInto<u16>>::try_into(x.0)
                    .unwrap()
                    .try_into()
                    .unwrap(),
                x.1,
            )
        })
    }
}

/// Handler for the server
pub struct PnpServerServiceHandler<'a, N, const C: usize> {
    server: &'a mut PnpServerService<N, C>,
}

impl<N: Node, const C: usize> PnpServerServiceHandler<'_, N, C> {
    fn handle_allocation_message<
        N2: Node<Transport = N::Transport>,
        M: AllocationMessage<N::Transport>,
    >(
        &mut self,
        node: &mut N2,
        message: M,
    ) {
        let id = self
            .server
            .assignments
            .iter()
            .position(|x| x.is_assigned_and(|x| message.matches_unique_id(&x)));

        // if no assignment is found, find the first unassigned address and assign it
        let id = id.or_else(|| {
            self.server
                .assignments
                .iter()
                .rev()
                .position(|x| !x.is_assigned())
                .inspect(|id| self.server.assignments[*id] = Assignment::Reserved)
        });

        let id = id.map(|x| TryInto::<u16>::try_into(x).unwrap().try_into().unwrap());

        if let Some(id) = id {
            let message = message.with_node_id(id);
            node.publish(M::SUBJECT, &message).ok();
        }
    }
}

impl<N: Node, const C: usize> TransferHandler<N::Transport> for PnpServerServiceHandler<'_, N, C> {
    fn handle_message<N2: Node<Transport = N::Transport>>(
        &mut self,
        node: &mut N2,
        transfer: &MessageTransfer<Vec<u8>, N2::Transport>,
    ) -> bool {
        if transfer.header.subject == <Data1 as AllocationMessage<N::Transport>>::SUBJECT {
            if let Ok(message) = Data1::deserialize_from_bytes(&transfer.payload) {
                self.handle_allocation_message(node, message);
                return true;
            }
        } else if transfer.header.subject == <Data2 as AllocationMessage<N::Transport>>::SUBJECT {
            if let Ok(message) = Data2::deserialize_from_bytes(&transfer.payload) {
                self.handle_allocation_message(node, message);
                return true;
            }
        } else if transfer.header.subject == heartbeat_1_0::SUBJECT {
            if let Some(id) = transfer.header.source.clone() {
                if let Ok(_) = heartbeat_1_0::Heartbeat::deserialize_from_bytes(&transfer.payload) {
                    let id = Into::<u16>::into(id) as usize;
                    if !self.server.assignments[id].is_assigned() {
                        self.server.assignments[id] = Assignment::Reserved;
                    }
                    return false;
                }
            }
        }

        false
    }
}
