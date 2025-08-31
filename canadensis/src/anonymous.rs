//!
//! A publisher that can be used by anonymous nodes
//!

use core::marker::PhantomData;

use crate::serialize::do_serialize;
use crate::Clock;
use canadensis_core::time::{MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::{TransferId, Transmitter, Transport};
use canadensis_core::{nb, SubjectId};
use canadensis_encoding::{Message, Serialize};

/// A transmitter that sends anonymous messages and does not require a node ID
///
/// Anonymous nodes have some limitations:
/// * They can only send messages, not service requests or responses
/// * They cannot send multi-frame messages
pub struct AnonymousPublisher<C: Clock, M, T: Transmitter<C>> {
    /// The priority of transfers from this transmitter
    priority: <T::Transport as Transport>::Priority,
    /// The subject to transmit on
    subject: SubjectId,
    /// The ID of the next transfer sent
    next_transfer_id: <T::Transport as Transport>::TransferId,
    /// Frame transmit timeout
    timeout: MicrosecondDuration32,
    /// Message type phantom data
    _message_phantom: PhantomData<M>,
}

impl<C, M, T> AnonymousPublisher<C, M, T>
where
    C: Clock,
    M: Message + Serialize,
    T: Transmitter<C>,
{
    /// Creates an anonymous message publisher
    pub fn new(
        subject: SubjectId,
        priority: <T::Transport as Transport>::Priority,
        timeout: MicrosecondDuration32,
    ) -> Self {
        AnonymousPublisher {
            priority,
            subject,
            next_transfer_id: <T::Transport as Transport>::TransferId::default(),
            timeout,
            _message_phantom: PhantomData,
        }
    }

    /// Prepares an anonymous message for sending and pushes it into the provided transmitter
    ///
    /// This function returns an error if the message is too long to fit into one frame, or if
    /// memory allocation fails.
    pub fn send(
        &mut self,
        payload: &M,
        clock: &mut C,
        transmitter: &mut T,
        driver: &mut T::Driver,
    ) -> nb::Result<(), AnonymousPublishError<T::Error>> {
        self.send_inner(payload, false, clock, transmitter, driver)
    }

    /// Prepares an anonymous message, with the loopback flag set, for sending and pushes it into
    /// the provided transmitter
    ///
    /// This function returns an error if the message is too long to fit into one frame, or if
    /// memory allocation fails.
    pub fn send_loopback(
        &mut self,
        payload: &M,
        clock: &mut C,
        transmitter: &mut T,
        driver: &mut T::Driver,
    ) -> nb::Result<(), AnonymousPublishError<T::Error>> {
        self.send_inner(payload, true, clock, transmitter, driver)
    }

    fn send_inner(
        &mut self,
        payload: &M,
        loopback: bool,
        clock: &mut C,
        transmitter: &mut T,
        driver: &mut T::Driver,
    ) -> nb::Result<(), AnonymousPublishError<T::Error>> {
        // Check that the message fits into one frame
        let payload_size_bytes = payload.size_bits().div_ceil(8);
        if payload_size_bytes > transmitter.mtu() {
            return Err(nb::Error::Other(AnonymousPublishError::Length));
        }
        // Part 1: Serialize
        let deadline = clock.now() + self.timeout;
        do_serialize(payload, |payload_bytes| {
            self.send_payload(
                payload_bytes,
                deadline,
                loopback,
                transmitter,
                clock,
                driver,
            )
        })
        .map_err(|e| e.map(AnonymousPublishError::Transport))?;
        Ok(())
    }

    fn send_payload(
        &mut self,
        payload: &[u8],
        deadline: Microseconds32,
        loopback: bool,
        transmitter: &mut T,
        clock: &mut C,
        driver: &mut T::Driver,
    ) -> nb::Result<(), T::Error> {
        // Assemble the transfer
        let transfer = Transfer {
            header: Header::Message(MessageHeader {
                timestamp: deadline,
                transfer_id: self.next_transfer_id.clone(),
                priority: self.priority.clone(),
                subject: self.subject,
                source: None,
            }),
            loopback,
            payload,
        };
        self.next_transfer_id = self.next_transfer_id.clone().increment();

        transmitter.push(transfer, clock, driver)
    }
}

/// Errors that can occur when publishing an anonymous message
#[derive(Debug)]
pub enum AnonymousPublishError<E> {
    /// The message was too long to fit into one frame
    Length,
    /// The transport returned an error
    Transport(E),
}

impl<E> From<E> for AnonymousPublishError<E> {
    fn from(inner: E) -> Self {
        AnonymousPublishError::Transport(inner)
    }
}
