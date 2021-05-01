use crate::do_serialize;
use canadensis_can::{OutOfMemoryError, Transmitter};
use canadensis_core::transfer::{MessageHeader, Transfer, TransferHeader, TransferKindHeader};
use canadensis_core::{NodeId, Priority, SubjectId, TransferId};
use canadensis_encoding::Serialize;

/// A transmitter that sends anonymous messages and does not require a node ID
pub struct AnonymousPublisher {
    /// The priority of transfers from this transmitter
    priority: Priority,
    /// The subject to transmit on
    subject: SubjectId,
    /// The ID of the next transfer sent
    next_transfer_id: TransferId,
}

impl AnonymousPublisher {
    /// Creates an anonymous message transmitter
    ///
    /// priority: The priority to use for messages
    ///
    /// subject: The subject ID to publish to
    pub fn new(priority: Priority, subject: SubjectId) -> Self {
        AnonymousPublisher {
            priority,
            subject,
            next_transfer_id: TransferId::const_default(),
        }
    }

    pub fn send<T, I>(
        &mut self,
        payload: &T,
        deadline: I,
        transmitter: &mut Transmitter<I>,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Serialize,
        I: Clone,
    {
        // Part 1: Serialize
        do_serialize(payload, |payload_bytes| {
            self.send_payload(payload_bytes, deadline, transmitter)
        })
    }

    pub fn send_payload<I>(
        &mut self,
        payload: &[u8],
        deadline: I,
        transmitter: &mut Transmitter<I>,
    ) -> Result<(), OutOfMemoryError>
    where
        I: Clone,
    {
        // Assemble the transfer
        let transfer: Transfer<&[u8], I> = Transfer {
            timestamp: deadline,
            header: TransferHeader {
                source: make_pseudo_id(payload),
                priority: self.priority,
                kind: TransferKindHeader::Message(MessageHeader {
                    anonymous: false,
                    subject: self.subject,
                }),
            },
            transfer_id: self.next_transfer_id,
            payload,
        };
        self.next_transfer_id = self.next_transfer_id.increment();

        transmitter.push(transfer)
    }
}

fn make_pseudo_id(payload: &[u8]) -> NodeId {
    // XOR some things. I don't know if this will actually work well.
    let mut id_bits = 37u8;
    for &byte in payload {
        id_bits ^= byte;
    }
    // Get a non-reserved ID
    loop {
        let id = NodeId::from_truncating(id_bits);
        if !id.is_diagnostic_reserved() {
            // Got a valid, non-diagnostic ID
            break id;
        }
        // This one is reserved. Try one lower.
        id_bits = id_bits.wrapping_sub(1);
    }
}
