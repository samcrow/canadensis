#![no_std]
#![deny(missing_docs)]

//!
//! # Canadensis compatibility for bxCAN CAN controllers
//!
//! This library provides various utilities that make it easier to use `canadensis` with the
//! bxCAN CAN controllers found on many STM32 microcontrollers.
//!

extern crate alloc;

extern crate bxcan;
extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_filter_config;
extern crate canadensis_pnp_client;
extern crate cortex_m;
extern crate fallible_collections;
extern crate heapless;
extern crate nb;

pub mod pnp;

pub use bxcan::OverrunError;

use bxcan::filter::Mask32;
use bxcan::{Can, ExtendedId, Fifo, FilterOwner, Instance, Mailbox};
use canadensis::core::subscription::Subscription;
use canadensis::core::time::{Clock, Microseconds32};
use canadensis::core::OutOfMemoryError;
use canadensis_can::driver::{optimize_filters, ReceiveDriver, TransmitDriver};
use canadensis_can::{CanNodeId, Frame};
use core::convert::{Infallible, TryFrom};
use heapless::Deque;

/// Maximum number of loopback frames that can be stored
const LOOPBACK_CAPACITY: usize = 2;

/// A CAN driver that wraps a bxCAN device and keeps track of deadlines for queued frames
pub struct BxCanDriver<N>
where
    N: Instance,
{
    can: Can<N>,
    deadlines: DeadlineTracker,
    /// Copies of transmitted loopback frames that have not yet been received
    loopback_frames: Deque<Frame, LOOPBACK_CAPACITY>,
}

impl<N> BxCanDriver<N>
where
    N: Instance,
{
    /// Creates a CAN driver
    pub fn new(can: Can<N>) -> Self {
        BxCanDriver {
            can,
            deadlines: DeadlineTracker::new(),
            loopback_frames: Deque::new(),
        }
    }

    /// Consumes this driver and returns its CAN object
    pub fn into_can(self) -> Can<N> {
        self.can
    }

    /// Returns a reference to the CAN driver
    pub fn can(&self) -> &Can<N> {
        &self.can
    }
    /// Returns a mutable reference to the CAN driver
    pub fn can_mut(&mut self) -> &mut Can<N> {
        &mut self.can
    }

    /// Returns true if at least one loopback frame is ready to receive
    pub fn loopback_frame_waiting(&self) -> bool {
        !self.loopback_frames.is_empty()
    }

    /// Tries to transmit a frame, and assumes that the frame's deadline has not passed
    fn transmit_inner<C: Clock>(
        &mut self,
        frame: &Frame,
        deadline: Microseconds32,
    ) -> nb::Result<Option<Frame>, <Self as TransmitDriver<C>>::Error> {
        let frame = cyphal_frame_to_bxcan(frame);
        match self.can.transmit(&frame) {
            Ok(status) => {
                // Store the deadline for this frame
                let replaced_deadline = self.deadlines.replace(status.mailbox(), deadline);
                if let (Some(removed_frame), Some(removed_frame_deadline)) =
                    (status.dequeued_frame(), replaced_deadline)
                {
                    if let Ok(removed_frame) =
                        bxcan_frame_to_cyphal(removed_frame, removed_frame_deadline)
                    {
                        Ok(Some(removed_frame))
                    } else {
                        // Frame that was removed is not compatible with Cyphal, so ignore it
                        Ok(None)
                    }
                } else {
                    // No frame was removed
                    Ok(None)
                }
            }
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(infallible)) => match infallible {},
        }
    }
}
impl<N> BxCanDriver<N>
where
    N: Instance + FilterOwner,
{
    /// Tries to receive a frame from the CAN bus
    fn receive_from_bus<C: Clock>(
        &mut self,
        clock: &mut C,
    ) -> nb::Result<Frame, <Self as ReceiveDriver<C>>::Error> {
        loop {
            match self.can.receive() {
                Ok(frame) => {
                    if let Ok(frame) = bxcan_frame_to_cyphal(&frame, clock.now()) {
                        break Ok(frame);
                    }
                    // Otherwise the frame is remote or basic ID, not compatible with Cyphal.
                    // Try to receive another frame.
                }
                Err(nb::Error::WouldBlock) => break Err(nb::Error::WouldBlock),
                Err(nb::Error::Other(e)) => break Err(nb::Error::Other(e)),
            }
        }
    }
    /// Tries to receive a frame from the loopback queue
    fn receive_loopback<C: Clock>(
        &mut self,
    ) -> nb::Result<Frame, <Self as ReceiveDriver<C>>::Error> {
        match self.loopback_frames.pop_front() {
            Some(frame) => Ok(frame),
            None => Err(nb::Error::WouldBlock),
        }
    }
}

impl<C, N> TransmitDriver<C> for BxCanDriver<N>
where
    C: Clock,
    N: Instance,
{
    type Error = Infallible;

    fn try_reserve(&mut self, frames: usize) -> Result<(), OutOfMemoryError> {
        if frames == 1 {
            // There's likely space for at least one frame
            Ok(())
        } else {
            // However, there is no in-memory queue.
            Err(OutOfMemoryError)
        }
    }

    fn transmit(&mut self, frame: Frame, clock: &mut C) -> nb::Result<Option<Frame>, Self::Error> {
        let now = clock.now();
        clean_expired_frames(&mut self.deadlines, &mut self.can, now);
        // Check that the frame's deadline has not passed
        let deadline = frame.timestamp();
        if deadline >= now {
            // Deadline is now or in the future. Continue to transmit.
            let transmit_status = self.transmit_inner::<C>(&frame, deadline);
            if transmit_status.is_ok() && frame.loopback() {
                // Loopback frame successfully sent; store a copy with its timestamp set to
                // the time just before it was sent
                let mut loopback_frame = frame;
                loopback_frame.set_timestamp(now);
                // If the loopback queue is full, drop this frame
                let _ = self.loopback_frames.push_back(loopback_frame);
            }
            transmit_status
        } else {
            // Deadline passed, ignore frame
            Ok(None)
        }
    }

    fn flush(&mut self, _clock: &mut C) -> nb::Result<(), Self::Error> {
        // The hardware does this automatically
        Ok(())
    }
}

impl<C, N> ReceiveDriver<C> for BxCanDriver<N>
where
    C: Clock,
    N: Instance + FilterOwner,
{
    /// This matches the error type defined in bxcan
    type Error = OverrunError;

    /// Tries to receive a frame from the frame receive queue or the loopback frame queue
    ///
    /// If both loopback and non-loopback frames are waiting, this function returns a non-loopback
    /// frame.
    ///
    fn receive(&mut self, clock: &mut C) -> nb::Result<Frame, Self::Error> {
        match self.receive_from_bus(clock) {
            Ok(frame) => Ok(frame),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
            // No frames waiting from the bus. Try the loopback queue.
            Err(nb::Error::WouldBlock) => self.receive_loopback::<C>(),
        }
    }

    fn apply_filters<S>(&mut self, local_node: Option<CanNodeId>, subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>,
    {
        let mut filters = self.can.modify_filters();
        let status = optimize_filters(
            local_node,
            subscriptions,
            filters.num_banks().into(),
            |optimized| {
                // Apply filters
                filters.clear();
                for (i, filter) in optimized.iter().enumerate() {
                    let id = ExtendedId::new(filter.id()).unwrap();
                    let mask = ExtendedId::new(filter.mask()).unwrap();
                    filters.enable_bank(i as u8, Fifo::Fifo0, Mask32::frames_with_ext_id(id, mask));
                }
            },
        );
        if status.is_err() {
            // Not enough memory to apply the ideal filters. Just accept all frames.
            filters
                .clear()
                .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
        }
    }

    fn apply_accept_all(&mut self) {
        self.can
            .modify_filters()
            .clear()
            .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    }
}

/// Aborts transmission for all frames placed in transmit mailboxes that have missed their
/// transmit deadlines
///
/// now: The current time
fn clean_expired_frames<C>(deadlines: &mut DeadlineTracker, can: &mut Can<C>, now: Microseconds32)
where
    C: Instance,
{
    for mailbox in [Mailbox::Mailbox0, Mailbox::Mailbox1, Mailbox::Mailbox2].iter() {
        if let Some(deadline) = deadlines.get(*mailbox) {
            if now > deadline {
                // Deadline has passed, abort transmission
                // Ignore if the mailbox is really empty or the frame has been transmitted.
                can.abort(*mailbox);
            }
        }
    }
}

/// Keeps track of the deadline for each frame in a CAN transmit mailbox
///
/// This struct does not have any public associated functions except `new()`.
#[derive(Default)]
pub struct DeadlineTracker {
    deadlines: [Option<Microseconds32>; 3],
}

impl DeadlineTracker {
    /// Creates a deadline tracker with no deadlines
    pub fn new() -> Self {
        DeadlineTracker::default()
    }
    /// Returns the deadline for a mailbox
    pub(crate) fn get(&self, mailbox: Mailbox) -> Option<Microseconds32> {
        self.deadlines[mailbox as usize]
    }
    /// Stores the deadline for a mailbox and returns the deadline for the previous frame in that
    /// mailbox, if any
    pub(crate) fn replace(
        &mut self,
        mailbox: Mailbox,
        new_deadline: Microseconds32,
    ) -> Option<Microseconds32> {
        let slot = &mut self.deadlines[mailbox as usize];
        slot.replace(new_deadline)
    }
}

/// Converts a Canadensis frame into a bxCAN frame
///
/// # Panics
///
/// This function panics if the provided frame has more than 8 bytes of data.
fn cyphal_frame_to_bxcan(frame: &Frame) -> bxcan::Frame {
    let bxcan_id = ExtendedId::new(frame.id().into()).unwrap();
    let bxcan_data = bxcan::Data::new(frame.data()).expect("Frame data more than 8 bytes");
    bxcan::Frame::new_data(bxcan_id, bxcan_data)
}

/// Converts a bxCAN frame into a Canadensis frame
///
/// This function returns an error if the frame does not have an extended ID, has an ID with an
/// invalid format, or does not have any data.
fn bxcan_frame_to_cyphal(
    frame: &bxcan::Frame,
    timestamp: Microseconds32,
) -> Result<Frame, InvalidFrameFormat> {
    let id_bits = match frame.id() {
        bxcan::Id::Extended(extended_id) => extended_id.as_raw(),
        bxcan::Id::Standard(_) => return Err(InvalidFrameFormat),
    };
    let cyphal_id = canadensis_can::CanId::try_from(id_bits).map_err(|_| InvalidFrameFormat)?;
    let cyphal_data = frame.data().ok_or(InvalidFrameFormat)?;
    Ok(canadensis_can::Frame::new(
        timestamp,
        cyphal_id,
        cyphal_data.as_ref(),
    ))
}

/// An error indicating that a frame did not have the correct format for use with Cyphal
#[derive(Debug)]
pub struct InvalidFrameFormat;
