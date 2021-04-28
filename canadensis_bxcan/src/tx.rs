//! Transmit interface

use crate::{bxcan_frame_to_uavcan, uavcan_frame_to_bxcan};
use bxcan::{Instance, Mailbox, Tx};
use canadensis::time::Instant;
use canadensis::Transmitter;
use core::cmp::Ordering;
use core::convert::Infallible;

pub struct TransmitAdapter<C, I> {
    uavcan_transmitter: Transmitter<I>,
    can: Tx<C>,
    deadline_tracker: DeadlineTracker<I>,
}

impl<C, I> TransmitAdapter<C, I>
where
    C: Instance,
    I: Instant,
{
    pub fn new(can: Tx<C>, uavcan_transmitter: Transmitter<I>) -> Self {
        TransmitAdapter {
            uavcan_transmitter,
            can,
            deadline_tracker: DeadlineTracker::new(),
        }
    }

    pub fn try_send(&mut self, now: I) {
        self.clean_expired_frames(&now);
        while let Some(frame) = self.uavcan_transmitter.pop() {
            // Check that the frame's deadline has not passed
            match frame.timestamp().overflow_safe_compare(&now) {
                Ordering::Greater | Ordering::Equal => {
                    // Deadline is now or in the future. Continue to transmit.
                    let send_status = self.send_frame(frame);
                    match send_status {
                        Ok(()) => {}
                        Err(nb::Error::Other(infallible)) => match infallible {},
                        Err(nb::Error::WouldBlock) => break,
                    }
                }
                Ordering::Less => {
                    // Deadline passed, ignore frame
                    drop(frame);
                }
            }
        }
    }

    fn send_frame(&mut self, frame: canadensis::Frame<I>) -> nb::Result<(), Infallible> {
        // Convert frame to BXCAN format
        let bxcan_frame = uavcan_frame_to_bxcan(&frame);
        match self.can.transmit_and_get_mailbox(&bxcan_frame) {
            Ok((None, mailbox)) => {
                // Store the deadline for the frame just submitted
                let _ = self.deadline_tracker.replace(mailbox, frame.timestamp());
                Ok(())
            }
            Ok((Some(removed_frame), mailbox)) => {
                // Store the deadline for the frame just submitted, and get the deadline for
                // the removed frame
                let removed_frame_deadline = self
                    .deadline_tracker
                    .replace(mailbox, frame.timestamp())
                    .expect("Bug: removed a frame from the mailbox, but no deadline");
                let removed_frame = bxcan_frame_to_uavcan(&removed_frame, removed_frame_deadline)
                    .expect("Bug: Replaced frame has invalid format");
                // Put the removed frame back in the queue to be transmitted later
                // This may return an error if it runs out of memory, but there's nothing we can
                // do about that.
                let _ = self.uavcan_transmitter.return_frame(removed_frame);
                Ok(())
            }
            Err(nb::Error::WouldBlock) => {
                // No mailbox available
                Err(nb::Error::WouldBlock)
            }
            Err(nb::Error::Other(infallible)) => match infallible {},
        }
    }

    /// Aborts transmission for all frames placed in transmit mailboxes that have missed their
    /// transmit deadlines
    ///
    /// now: The current time
    fn clean_expired_frames(&mut self, now: &I) {
        for mailbox in [Mailbox::Mailbox0, Mailbox::Mailbox1, Mailbox::Mailbox2].iter() {
            if let Some(deadline) = self.deadline_tracker.get(mailbox.clone()) {
                if now.overflow_safe_compare(&deadline) == Ordering::Greater {
                    // Deadline has passed, abort transmission
                    // Ignore if the mailbox is really empty or the frame has been transmitted.
                    self.can.abort(mailbox.clone());
                }
            }
        }
    }
}

/// Keeps track of the deadline for each frame in a CAN transmit mailbox
struct DeadlineTracker<I> {
    deadlines: [Option<I>; 3],
}

impl<I> DeadlineTracker<I>
where
    I: Clone,
{
    /// Creates a deadline tracker with no deadlines
    pub fn new() -> Self {
        DeadlineTracker {
            deadlines: [None, None, None],
        }
    }
    /// Returns the deadline for a mailbox
    pub fn get(&self, mailbox: Mailbox) -> Option<I> {
        self.deadlines[mailbox as usize].clone()
    }
    /// Stores the deadline for a mailbox and returns the deadline for the previous frame in that
    /// mailbox, if any
    pub fn replace(&mut self, mailbox: Mailbox, new_deadline: I) -> Option<I> {
        let slot = &mut self.deadlines[mailbox as usize];
        slot.replace(new_deadline)
    }
}
