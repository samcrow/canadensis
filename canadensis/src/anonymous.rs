//!
//! A publisher that can be used by anonymous nodes
//!

use core::marker::PhantomData;

use crate::core_node::do_serialize;
use crate::Clock;
use canadensis_can::queue::FrameSink;
use canadensis_can::{Mtu, OutOfMemoryError, Transmitter};
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::{Priority, SubjectId, TransferId};
use canadensis_encoding::{Message, Serialize};

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
    pub fn send<Q>(
        &mut self,
        payload: &T,
        now: C::Instant,
        transmitter: &mut Transmitter<Q>,
    ) -> Result<(), AnonymousPublishError>
    where
        Q: FrameSink<C::Instant>,
    {
        // Check that the message fits into one frame
        // (subtract one byte to leave room for the tail byte)
        let mtu_bits = (self.mtu.as_bytes() - 1) * 8;
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

    fn send_payload<Q>(
        &mut self,
        payload: &[u8],
        deadline: C::Instant,
        transmitter: &mut Transmitter<Q>,
    ) -> Result<(), OutOfMemoryError>
    where
        Q: FrameSink<C::Instant>,
    {
        // Assemble the transfer
        let transfer: Transfer<&[u8], C::Instant> = Transfer {
            header: Header::Message(MessageHeader {
                timestamp: deadline,
                transfer_id: self.next_transfer_id,
                priority: self.priority,
                subject: self.subject,
                source: None,
            }),
            payload,
        };
        self.next_transfer_id = self.next_transfer_id.increment();

        transmitter.push(transfer)?;
        Ok(())
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
