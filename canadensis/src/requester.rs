use crate::serialize::do_serialize;
use canadensis_can::queue::FrameSink;
use canadensis_can::{OutOfMemoryError, Transmitter};
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, ServiceHeader, Transfer};
use canadensis_core::{NodeId, Priority, ServiceId, TransferId};
use canadensis_encoding::Serialize;

/// Assembles transfers and manages transfer IDs to send service requests
pub struct Requester<I: Instant> {
    /// The ID of this node
    this_node: NodeId,
    /// The priority of transfers from this transmitter
    priority: Priority,
    /// The timeout for sending transfers
    timeout: I::Duration,
    /// The ID of the next transfer to send, for each destination node
    next_transfer_ids: NextTransferIds,
}

impl<I: Instant> Requester<I> {
    /// Creates a service request transmitter
    ///
    /// this_node: The ID of this node
    ///
    /// priority: The priority to use for messages
    ///
    /// service: The service ID to request
    pub fn new(this_node: NodeId, timeout: I::Duration, priority: Priority) -> Self {
        Requester {
            this_node,
            priority,
            timeout,
            next_transfer_ids: NextTransferIds::new(),
        }
    }

    pub fn send<T, Q>(
        &mut self,
        now: I,
        service: ServiceId,
        payload: &T,
        destination: NodeId,
        transmitter: &mut Transmitter<Q>,
    ) -> Result<TransferId, OutOfMemoryError>
    where
        T: Serialize,
        Q: FrameSink<I>,
    {
        // Part 1: Serialize
        let deadline = self.timeout + now;
        do_serialize(payload, |payload_bytes| {
            // Part 2: Split into frames and send
            self.send_payload(payload_bytes, service, destination, deadline, transmitter)
        })
    }

    fn send_payload<Q>(
        &mut self,
        payload: &[u8],
        service: ServiceId,
        destination: NodeId,
        deadline: I,
        transmitter: &mut Transmitter<Q>,
    ) -> Result<TransferId, OutOfMemoryError>
    where
        Q: FrameSink<I>,
    {
        // Assemble the transfer
        let transfer_id = self.next_transfer_ids.get_and_increment(destination);
        let transfer: Transfer<&[u8], I> = Transfer {
            header: Header::Request(ServiceHeader {
                timestamp: deadline,
                transfer_id,
                priority: self.priority,
                service,
                source: self.this_node,
                destination,
            }),
            payload,
        };

        transmitter.push(transfer)?;
        Ok(transfer_id)
    }
}

const NUM_TRANSFER_IDS: usize = (NodeId::MAX.to_u8() as usize) + 1;

/// A map from destination node IDs to transfer IDs of the next transfer
struct NextTransferIds {
    ids: [TransferId; NUM_TRANSFER_IDS],
}

impl NextTransferIds {
    /// Creates a new transfer ID map with the default transfer ID for each node
    pub fn new() -> Self {
        NextTransferIds {
            ids: [TransferId::default(); NUM_TRANSFER_IDS],
        }
    }
    /// Returns the next transfer ID for the provided node, and increments the stored transfer
    /// ID
    pub fn get_and_increment(&mut self, destination: NodeId) -> TransferId {
        let entry = &mut self.ids[usize::from(destination)];
        let current = *entry;
        *entry = entry.increment();
        current
    }
}
