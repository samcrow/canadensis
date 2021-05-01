use crate::do_serialize;
use canadensis_can::{OutOfMemoryError, Transmitter};
use canadensis_core::time::Instant;
use canadensis_core::transfer::{ServiceHeader, Transfer, TransferHeader, TransferKindHeader};
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
    /// The ID of the next transfer sent
    next_transfer_id: TransferId,
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
            next_transfer_id: TransferId::const_default(),
        }
    }

    pub fn send<T>(
        &mut self,
        now: I,
        service: ServiceId,
        payload: &T,
        destination: NodeId,
        transmitter: &mut Transmitter<I>,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Serialize,
    {
        // Part 1: Serialize
        let deadline = self.timeout.clone() + now;
        do_serialize(payload, |payload_bytes| {
            // Part 2: Split into frames and send
            self.send_payload(payload_bytes, service, destination, deadline, transmitter)
        })
    }

    pub fn send_payload(
        &mut self,
        payload: &[u8],
        service: ServiceId,
        destination: NodeId,
        deadline: I,
        transmitter: &mut Transmitter<I>,
    ) -> Result<(), OutOfMemoryError> {
        // Assemble the transfer
        let transfer: Transfer<&[u8], I> = Transfer {
            timestamp: deadline,
            header: TransferHeader {
                source: self.this_node,
                priority: self.priority,
                kind: TransferKindHeader::Request(ServiceHeader {
                    service,
                    destination,
                }),
            },
            transfer_id: self.next_transfer_id,
            payload,
        };
        self.next_transfer_id = self.next_transfer_id.increment();

        transmitter.push(transfer)
    }
}
