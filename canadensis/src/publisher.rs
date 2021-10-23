use crate::serialize::do_serialize;
use canadensis_core::time::{Clock, Instant};
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::{TransferId, Transmitter, Transport};
use canadensis_core::{nb, SubjectId};
use canadensis_encoding::{Message, Serialize};

/// Assembles transfers and manages transfer IDs to send messages
///
/// The subject ID is not part of this struct because it is used as a key in the map of publishers.
pub struct Publisher<I: Instant, T: Transmitter<I>> {
    /// The ID of the next transfer sent
    next_transfer_id: <T::Transport as Transport>::TransferId,
    /// Timeout for sending a transfer, measured from the time the payload is serialized
    timeout: I::Duration,
    /// Priority for transfers
    priority: <T::Transport as Transport>::Priority,
    /// ID of this node
    source: <T::Transport as Transport>::NodeId,
}

impl<I: Instant, T: Transmitter<I>> Publisher<I, T> {
    /// Creates a message transmitter
    ///
    /// node: The ID of this node
    ///
    /// priority: The priority to use for messages
    pub fn new(
        node_id: <T::Transport as Transport>::NodeId,
        timeout: I::Duration,
        priority: <T::Transport as Transport>::Priority,
    ) -> Self {
        Publisher {
            next_transfer_id: <T::Transport as Transport>::TransferId::default(),
            timeout,
            priority,
            source: node_id,
        }
    }

    pub fn publish<M, C>(
        &mut self,
        clock: &mut C,
        subject: SubjectId,
        payload: &M,
        transmitter: &mut T,
        driver: &mut T::Driver,
    ) -> nb::Result<(), T::Error>
    where
        M: Message + Serialize,
        I: Instant,
        C: Clock<Instant = I>,
    {
        let deadline = self.timeout + clock.now();
        // Part 1: Serialize
        do_serialize(payload, |payload_bytes| {
            // Part 2: Split into frames and put frames in the queue
            self.send_payload(subject, payload_bytes, deadline, transmitter, clock, driver)
        })
    }

    pub fn send_payload<C>(
        &mut self,
        subject: SubjectId,
        payload: &[u8],
        deadline: I,
        transmitter: &mut T,
        clock: &mut C,
        driver: &mut T::Driver,
    ) -> nb::Result<(), T::Error>
    where
        I: Clone,
        C: Clock<Instant = I>,
    {
        // Assemble the transfer
        let transfer = Transfer {
            header: Header::Message(MessageHeader {
                timestamp: deadline,
                transfer_id: self.next_transfer_id.clone(),
                priority: self.priority.clone(),
                subject,
                source: Some(self.source.clone()),
            }),
            payload,
        };
        self.next_transfer_id = self.next_transfer_id.clone().increment();

        transmitter.push(transfer, clock, driver)
    }
}
