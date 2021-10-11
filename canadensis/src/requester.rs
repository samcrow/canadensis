use crate::core::transport::{NodeId, TransferId};
use crate::serialize::do_serialize;
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, ServiceHeader, Transfer};
use canadensis_core::transport::{Transmitter, Transport};
use canadensis_core::ServiceId;
use canadensis_encoding::Serialize;

/// Assembles transfers and manages transfer IDs to send service requests
pub struct Requester<I: Instant, T: Transmitter<I>> {
    /// The ID of this node
    this_node: <T::Transport as Transport>::NodeId,
    /// The priority of transfers from this transmitter
    priority: <T::Transport as Transport>::Priority,
    /// The timeout for sending transfers
    timeout: I::Duration,
    /// The ID of the next transfer to send, for each destination node
    next_transfer_ids: NextTransferIds<
        <T::Transport as Transport>::TransferId,
        <T::Transport as Transport>::NodeId,
    >,
}

impl<I: Instant, T: Transmitter<I>> Requester<I, T> {
    /// Creates a service request transmitter
    ///
    /// this_node: The ID of this node
    ///
    /// priority: The priority to use for messages
    ///
    /// service: The service ID to request
    pub fn new(
        this_node: <T::Transport as Transport>::NodeId,
        timeout: I::Duration,
        priority: <T::Transport as Transport>::Priority,
    ) -> Self {
        Requester {
            this_node,
            priority,
            timeout,
            next_transfer_ids: NextTransferIds::new(),
        }
    }

    /// Sends a service request and returns its transfer ID
    pub fn send<R>(
        &mut self,
        now: I,
        service: ServiceId,
        payload: &R,
        destination: <T::Transport as Transport>::NodeId,
        transmitter: &mut T,
    ) -> Result<<T::Transport as Transport>::TransferId, <T::Transport as Transport>::Error>
    where
        R: Serialize,
    {
        // Part 1: Serialize
        let deadline = self.timeout + now;
        do_serialize(payload, |payload_bytes| {
            // Part 2: Split into frames and send
            self.send_payload(payload_bytes, service, destination, deadline, transmitter)
        })
    }

    fn send_payload(
        &mut self,
        payload: &[u8],
        service: ServiceId,
        destination: <T::Transport as Transport>::NodeId,
        deadline: I,
        transmitter: &mut T,
    ) -> Result<<T::Transport as Transport>::TransferId, <T::Transport as Transport>::Error> {
        // Assemble the transfer
        let transfer_id = self
            .next_transfer_ids
            .get_and_increment(destination.clone());
        let transfer = Transfer {
            header: Header::Request(ServiceHeader {
                timestamp: deadline,
                transfer_id: transfer_id.clone(),
                priority: self.priority.clone(),
                service,
                source: self.this_node.clone(),
                destination,
            }),
            payload,
        };

        transmitter.push(transfer)?;
        Ok(transfer_id)
    }
}

/// A map from destination node IDs to transfer IDs of the next transfer
struct NextTransferIds<I, N: NodeId<I>> {
    // Because we can't do [I; N::MAX + 1], there's a separate field for the last transfer ID.
    ids: N::TransferIds,
}

impl<I, N> NextTransferIds<I, N>
where
    I: TransferId,
    N: NodeId<I>,
{
    /// Creates a new transfer ID map with the default transfer ID for each node
    pub fn new() -> Self {
        NextTransferIds {
            ids: N::TransferIds::default(),
        }
    }
    /// Returns the next transfer ID for the provided node, and increments the stored transfer
    /// ID
    pub fn get_and_increment(&mut self, destination: N) -> I {
        let entry = &mut self.ids.as_mut()[destination.into()];
        let current = entry.clone();
        *entry = entry.clone().increment();
        current
    }
}
