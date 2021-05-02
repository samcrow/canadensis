use crate::{do_serialize, Clock};
use canadensis_can::{Frame, Mtu, OutOfMemoryError, Transmitter};
use canadensis_core::time::Instant;
use canadensis_core::transfer::{MessageHeader, Transfer, TransferHeader, TransferKindHeader};
use canadensis_core::{NodeId, Priority, SubjectId, TransferId};
use canadensis_encoding::{Message, Serialize};
use std::marker::PhantomData;

/// A transmitter that sends anonymous messages and does not require a node ID
///
/// Anonymous nodes have some limitations:
/// * They can only send messages, not service requests or responses
/// * They cannot send multi-frame messages
pub struct AnonymousPublisher<C: Clock, T> {
    /// The priority of transfers from this transmitter
    priority: Priority,
    /// The subject to transmit on
    subject: SubjectId,
    /// The ID of the next transfer sent
    next_transfer_id: TransferId,
    /// Frame transmit timeout
    timeout: <C::Instant as Instant>::Duration,
    /// Transport MTU (used to check that transfers are single-frame)
    mtu: Mtu,
    /// Message type phantom
    message: PhantomData<*mut T>,
}

impl<C, T> AnonymousPublisher<C, T>
where
    C: Clock,
    T: Message + Serialize,
{
    /// Creates an anonymous message publisher
    pub fn new(
        subject: SubjectId,
        priority: Priority,
        timeout: <C::Instant as Instant>::Duration,
        mtu: Mtu,
    ) -> Self {
        AnonymousPublisher {
            priority,
            subject,
            next_transfer_id: TransferId::const_default(),
            timeout,
            mtu,
            message: PhantomData,
        }
    }

    /// Prepares an anonymous message for sending and pushes it into the provided transmitter
    ///
    /// This function returns an error if the message is too long to fit into one frame, or if
    /// memory allocation fails.
    pub fn send(
        &mut self,
        payload: &T,
        now: C::Instant,
        transmitter: &mut Transmitter<C::Instant>,
    ) -> Result<(), AnonymousPublishError> {
        // Check that the message fits into one frame
        let mtu_bits = self.mtu.as_bytes() * 8;
        if payload.size_bits() > mtu_bits {
            return Err(AnonymousPublishError::Length);
        }
        // Part 1: Serialize
        let deadline = self.timeout.clone() + now;
        do_serialize(payload, |payload_bytes| {
            self.send_payload(payload_bytes, deadline, transmitter)
        })?;
        Ok(())
    }

    fn send_payload<I>(
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

        transmitter.push(transfer)?;
        Ok(())
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

/// Errors that can occur when publishing an anonymous message
#[derive(Debug)]
pub enum AnonymousPublishError {
    /// The message was too long to fit into one frame
    Length,
    /// Not enough memory was available
    Memory(OutOfMemoryError),
}

impl From<OutOfMemoryError> for AnonymousPublishError {
    fn from(inner: OutOfMemoryError) -> Self {
        AnonymousPublishError::Memory(inner)
    }
}
