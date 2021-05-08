#![no_std]

extern crate alloc;

extern crate bxcan;
extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_filter_config;
extern crate nb;

use bxcan::filter::{BankConfig, Mask32};
use bxcan::{Can, ExtendedId, FilterOwner, Instance, Mailbox};
use canadensis::{Node, TransferHandler};
use canadensis_can::queue::FrameQueueSource;
use canadensis_can::OutOfMemoryError;
use canadensis_core::time::{Clock, Instant};
use canadensis_filter_config::optimize;
use core::cmp::Ordering;
use core::convert::{Infallible, TryFrom};

/// A UAVCAN node that communicates using a bxCAN peripheral
pub struct BxCanNode<N, C>
where
    N: Node,
    C: Instance,
{
    pub node: N,
    pub can: Can<C>,
    deadlines: DeadlineTracker<N::Instant>,
}

impl<N, C> BxCanNode<N, C>
where
    N: Node,
    N::FrameQueue: FrameQueueSource<N::Instant>,
    C: Instance,
{
    /// Creates a node
    pub fn new(node: N, can: Can<C>) -> Self {
        BxCanNode {
            node,
            can,
            deadlines: DeadlineTracker::new(),
        }
    }

    /// Configures the receive filters on a CAN peripheral to receive the frames that this node
    /// is currently subscribed to
    ///
    /// Caution: While the filters are being applied, there will be a period where the CAN
    /// peripheral does not accept any frames. This may cause frames to be lost if this function
    /// is called while the node is running.
    pub fn configure_filters(&mut self) -> Result<(), OutOfMemoryError>
    where
        C: FilterOwner,
    {
        // Get ideal filters
        let mut filters = self.node.frame_filters()?;
        // Reduce the filters if necessary
        let mut hardware_filters = self.can.modify_filters();
        let max_hardware_filters = hardware_filters.num_banks();
        let optimized_filters = optimize(&mut filters, max_hardware_filters.into());
        // Apply filters
        hardware_filters.clear();
        for (i, filter) in optimized_filters.iter().enumerate() {
            let id = ExtendedId::new(filter.id()).unwrap();
            let mask = ExtendedId::new(filter.mask()).unwrap();
            hardware_filters.enable_bank(
                i as u8,
                BankConfig::Mask32(Mask32::frames_with_ext_id(id, mask)),
            );
        }
        Ok(())
    }

    /// Receives all incoming CAN frames from the CAN peripheral, converts them into transfers,
    /// and passes all completed transfers to the provided handler
    pub fn receive_frames<H>(&mut self, handler: &mut H) -> Result<(), OutOfMemoryError>
    where
        H: TransferHandler<N::Instant>,
    {
        loop {
            match self.can.receive() {
                // Need to access the clock for each frame to give it an accurate timestamp.
                // When a frame completes a transfer, it may take a significant amount of time
                // to process the transfer before the next frame can be received.
                Ok(frame) => {
                    let now = self.node.clock_mut().now();
                    if let Ok(uavcan_frame) = bxcan_frame_to_uavcan(&frame, now) {
                        self.node.accept_frame(uavcan_frame, handler)?;
                    }
                }
                Err(nb::Error::Other(())) => {
                    // The receive FIFO has overflowed and at least one frame has been lost.
                    // What can we do?
                }
                Err(nb::Error::WouldBlock) => break,
            }
        }
        Ok(())
    }

    /// Sends frames from the outgoing frame queue onto the CAN bus
    ///
    /// This function also discards any frames that have not been transmitted by their deadlines.
    ///
    /// This function returns a WouldBlock error if frames are waiting to be transmitted
    /// but no suitable transmit mailbox is open.
    pub fn send_frames(&mut self) -> nb::Result<(), Infallible> {
        let now = self.node.clock_mut().now();
        self.clean_expired_frames(now);
        while let Some(frame) = self.node.frame_queue_mut().pop_frame() {
            // Check that the frame's deadline has not passed
            match frame.timestamp().overflow_safe_compare(&now) {
                Ordering::Greater | Ordering::Equal => {
                    // Deadline is now or in the future. Continue to transmit.
                    let send_status = self.send_frame(frame);
                    match send_status {
                        Ok(()) => {}
                        Err(nb::Error::Other(infallible)) => match infallible {},
                        Err(nb::Error::WouldBlock) => {
                            // The self.send_frame call already put the frame back in the queue
                            return Err(nb::Error::WouldBlock);
                        }
                    }
                }
                Ordering::Less => {
                    // Deadline passed, ignore frame
                    rtt_target::rprintln!(
                        "Dropping frame, ID {:?}, {} data bytes",
                        frame.id(),
                        frame.data().len()
                    );
                    drop(frame);
                }
            }
        }
        // All frames in the queue processed
        Ok(())
    }

    /// Puts one frame in a transmit mailbox to be sent
    ///
    /// If all mailboxes are full with frames of equal or greater priority, this function returns
    /// the frame to the outgoing frame queue and returns a WouldBlock error.
    fn send_frame(
        &mut self,
        frame: canadensis_can::Frame<N::Instant>,
    ) -> nb::Result<(), Infallible> {
        // Convert frame to BXCAN format
        let bxcan_frame = uavcan_frame_to_bxcan(&frame);
        let send_status = self.can.transmit_and_get_mailbox(&bxcan_frame);
        rtt_target::rprintln!("Send: {:?}", send_status);
        match send_status {
            Ok((None, mailbox)) => {
                // Store the deadline for the frame just submitted
                let _ = self.deadlines.replace(mailbox, frame.timestamp());
                Ok(())
            }
            Ok((Some(removed_frame), mailbox)) => {
                // Store the deadline for the frame just submitted, and get the deadline for
                // the removed frame
                let removed_frame_deadline = self
                    .deadlines
                    .replace(mailbox, frame.timestamp())
                    .expect("Bug: removed a frame from the mailbox, but no deadline");
                let removed_frame = bxcan_frame_to_uavcan(&removed_frame, removed_frame_deadline)
                    .expect("Bug: Replaced frame has invalid format");
                // Put the removed frame back in the queue to be transmitted later
                // This may return an error if it runs out of memory, but there's nothing we can
                // do about that.
                let _ = self.node.frame_queue_mut().return_frame(removed_frame);
                Ok(())
            }
            Err(nb::Error::WouldBlock) => {
                // No mailbox available for this frame. Put it back.
                // Ignore out of memory
                let _ = self.node.frame_queue_mut().return_frame(frame);

                Err(nb::Error::WouldBlock)
            }
            Err(nb::Error::Other(infallible)) => match infallible {},
        }
    }

    /// Aborts transmission for all frames placed in transmit mailboxes that have missed their
    /// transmit deadlines
    ///
    /// now: The current time
    fn clean_expired_frames(&mut self, now: N::Instant) {
        for mailbox in [Mailbox::Mailbox0, Mailbox::Mailbox1, Mailbox::Mailbox2].iter() {
            if let Some(deadline) = self.deadlines.get(mailbox.clone()) {
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

/// Converts a Canadensis frame into a bxCAN frame
///
/// # Panics
///
/// This function panics if the provided frame has more than 8 bytes of data.
fn uavcan_frame_to_bxcan<I>(frame: &canadensis_can::Frame<I>) -> bxcan::Frame {
    let bxcan_id = bxcan::ExtendedId::new(frame.id().into()).unwrap();
    let bxcan_data = bxcan::Data::new(frame.data()).expect("Frame data more than 8 bytes");
    bxcan::Frame::new_data(bxcan_id, bxcan_data)
}

/// Converts a bxCAN frame into a Canadensis frame
///
/// This function returns an error if the frame does not have an extended ID, has an ID with an
/// invalid format, or does not have any data.
fn bxcan_frame_to_uavcan<I>(
    frame: &bxcan::Frame,
    timestamp: I,
) -> Result<canadensis_can::Frame<I>, InvalidFrameFormat> {
    let id_bits = match frame.id() {
        bxcan::Id::Extended(extended_id) => extended_id.as_raw(),
        bxcan::Id::Standard(_) => return Err(InvalidFrameFormat),
    };
    let uavcan_id = canadensis_can::CanId::try_from(id_bits).map_err(|_| InvalidFrameFormat)?;
    let uavcan_data = frame.data().ok_or(InvalidFrameFormat)?;
    Ok(canadensis_can::Frame::new(
        timestamp,
        uavcan_id,
        uavcan_data.as_ref(),
    ))
}

#[derive(Debug)]
struct InvalidFrameFormat;
