use crate::do_serialize;
use canadensis_can::{OutOfMemoryError, Transmitter};
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::{NodeId, Priority, SubjectId, TransferId};
use canadensis_encoding::Serialize;

/// Assembles transfers and manages transfer IDs to send messages
///
/// The subject ID is not part of this struct because it is used as a key in the map of publishers.
pub struct Publisher<I: Instant> {
    /// The ID of the next transfer sent
    next_transfer_id: TransferId,
    /// Timeout for sending a transfer, measured from the time the payload is serialized
    timeout: I::Duration,
    /// Priority for transfers
    priority: Priority,
    /// ID of this node
    source: NodeId,
}

impl<I: Instant> Publisher<I> {
    /// Creates a message transmitter
    ///
    /// node: The ID of this node
    ///
    /// priority: The priority to use for messages
    pub fn new(node_id: NodeId, timeout: I::Duration, priority: Priority) -> Self {
        Publisher {
            next_transfer_id: TransferId::const_default(),
            timeout,
            priority,
            source: node_id,
        }
    }

    pub fn publish<T>(
        &mut self,
        now: I,
        subject: SubjectId,
        payload: &T,
        transmitter: &mut Transmitter<I>,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Serialize,
        I: Instant,
    {
        let deadline = self.timeout.clone() + now;
        // Part 1: Serialize
        do_serialize(payload, |payload_bytes| {
            // Part 2: Split into frames and put frames in the queue
            self.send_payload(subject, payload_bytes, deadline, transmitter)
        })
    }

    pub fn send_payload(
        &mut self,
        subject: SubjectId,
        payload: &[u8],
        deadline: I,
        transmitter: &mut Transmitter<I>,
    ) -> Result<(), OutOfMemoryError>
    where
        I: Clone,
    {
        // Assemble the transfer
        let transfer: Transfer<&[u8], I> = Transfer {
            header: Header::Message(MessageHeader {
                timestamp: deadline,
                transfer_id: self.next_transfer_id,
                priority: self.priority,
                subject,
                source: Some(self.source),
            }),
            payload,
        };
        self.next_transfer_id = self.next_transfer_id.increment();

        transmitter.push(transfer)
    }
}
