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
extern crate fallible_collections;
extern crate log;
extern crate nb;

pub mod pnp;

use alloc::vec::Vec;
use bxcan::filter::{BankConfig, Mask32};
use bxcan::{Can, ExtendedId, FilterOwner, Instance, Mailbox, Tx};
use canadensis::core::subscription::Subscription;
use canadensis::core::time::Instant;
use canadensis::core::{OutOfMemoryError, ServiceId, SubjectId};
use canadensis::filter::Filter;
use canadensis::Node;
use canadensis_can::driver::{ReceiveDriver, TransmitDriver};
use canadensis_can::types::CanNodeId;
use canadensis_can::Frame;
use canadensis_filter_config::optimize;
use core::cmp::Ordering;
use core::convert::{Infallible, TryFrom};
use fallible_collections::FallibleVec;

/// A CAN driver that wraps a bxCAN device and keeps track of deadlines for queued frames
pub struct BxCanDriver<I, N>
where
    N: Instance,
{
    can: Can<N>,
    deadlines: DeadlineTracker<I>,
}

impl<I, N> BxCanDriver<I, N>
where
    N: Instance,
{
    /// Creates a CAN driver
    pub fn new(can: Can<N>) -> Self {
        BxCanDriver {
            can,
            deadlines: DeadlineTracker::new(),
        }
    }
}

impl<I, N> TransmitDriver<I> for BxCanDriver<I, N>
where
    N: Instance,
{
    type Error = Infallible;

    fn transmit(&mut self, frame: &Frame<I>, now: I) -> nb::Result<Option<Frame<I>>, Self::Error> {
        clean_expired_frames(&mut self.deadlines, &mut self.can, now);
        // Check that the frame's deadline has not passed
        let deadline = frame.timestamp();
        match deadline.overflow_safe_compare(&now) {
            Ordering::Greater | Ordering::Equal => {
                // Deadline is now or in the future. Continue to transmit.
                let frame = uavcan_frame_to_bxcan(frame);
                match self.can.transmit(&frame) {
                    Ok(status) => {
                        // Store the deadline for this frame
                        let replaced_deadline = self.deadlines.replace(status.mailbox(), deadline);
                        if let (Some(removed_frame), Some(removed_frame_deadline)) =
                            (status.dequeued_frame(), replaced_deadline)
                        {
                            if let Ok(removed_frame) =
                                bxcan_frame_to_uavcan(removed_frame, removed_frame_deadline)
                            {
                                Ok(Some(removed_frame))
                            } else {
                                // Frame that was removed is not compatible with UAVCAN, so ignore it
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
            Ordering::Less => {
                // Deadline passed, ignore frame
                Ok(None)
            }
        }
    }
}

impl<I, N> ReceiveDriver<I> for BxCanDriver<I, N>
where
    I: Instant,
    N: Instance + FilterOwner,
{
    /// This matches the error type defined in bxcan
    type Error = ();

    fn receive(&mut self, now: I) -> nb::Result<Frame<I>, Self::Error> {
        loop {
            match self.can.receive() {
                Ok(frame) => match bxcan_frame_to_uavcan(&frame, now) {
                    Ok(frame) => break Ok(frame),
                    Err(_) => {
                        // Remote or basic ID, not compatible with UAVCAN
                        // Try to receive another frame
                    }
                },
                Err(nb::Error::WouldBlock) => break Err(nb::Error::WouldBlock),
                Err(nb::Error::Other(e)) => break Err(nb::Error::Other(e)),
            }
        }
    }

    fn apply_filters<S>(&mut self, local_node: Option<CanNodeId>, subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>,
    {
        match configure_node_filters(&mut self.can, local_node, subscriptions) {
            Ok(()) => { /* Done */ }
            Err(_) => {
                // Out of memory. Set the filters to accept all frames.
                self.can
                    .modify_filters()
                    .clear()
                    .enable_bank(0, Mask32::accept_all());
            }
        }
    }
}

/// Configures filters on a CAN peripheral to accept all frames that match the provided subscription
pub fn configure_node_filters<I, S>(
    can: &mut Can<I>,
    local_node: Option<CanNodeId>,
    subscriptions: S,
) -> Result<(), OutOfMemoryError>
where
    I: Instance + FilterOwner,
    S: IntoIterator<Item = Subscription>,
{
    let mut subscriptions = subscriptions.into_iter();
    let mut filters: Vec<Filter> = Vec::new();
    for subscription in subscriptions {
        if let Some(filter) = make_filter(subscription, local_node) {
            filters.try_push(filter)?;
        }
    }
    optimize_and_apply_filters(&mut filters, can);
    Ok(())
}

/// Aborts transmission for all frames placed in transmit mailboxes that have missed their
/// transmit deadlines
///
/// now: The current time
fn clean_expired_frames<I, C>(deadlines: &mut DeadlineTracker<I>, can: &mut Tx<C>, now: I)
where
    I: Instant,
    C: Instance,
{
    for mailbox in [Mailbox::Mailbox0, Mailbox::Mailbox1, Mailbox::Mailbox2].iter() {
        if let Some(deadline) = deadlines.get(mailbox.clone()) {
            if now.overflow_safe_compare(&deadline) == Ordering::Greater {
                // Deadline has passed, abort transmission
                // Ignore if the mailbox is really empty or the frame has been transmitted.
                can.abort(mailbox.clone());
            }
        }
    }
}

/// Keeps track of the deadline for each frame in a CAN transmit mailbox
///
/// This struct does not have any public associated functions except `new()`.
pub struct DeadlineTracker<I> {
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
    pub(crate) fn get(&self, mailbox: Mailbox) -> Option<I> {
        self.deadlines[mailbox as usize].clone()
    }
    /// Stores the deadline for a mailbox and returns the deadline for the previous frame in that
    /// mailbox, if any
    pub(crate) fn replace(&mut self, mailbox: Mailbox, new_deadline: I) -> Option<I> {
        let slot = &mut self.deadlines[mailbox as usize];
        slot.replace(new_deadline)
    }
}

/// Converts a Canadensis frame into a bxCAN frame
///
/// # Panics
///
/// This function panics if the provided frame has more than 8 bytes of data.
pub fn uavcan_frame_to_bxcan<I>(frame: &canadensis_can::Frame<I>) -> bxcan::Frame {
    let bxcan_id = bxcan::ExtendedId::new(frame.id().into()).unwrap();
    let bxcan_data = bxcan::Data::new(frame.data()).expect("Frame data more than 8 bytes");
    bxcan::Frame::new_data(bxcan_id, bxcan_data)
}

/// Converts a bxCAN frame into a Canadensis frame
///
/// This function returns an error if the frame does not have an extended ID, has an ID with an
/// invalid format, or does not have any data.
pub fn bxcan_frame_to_uavcan<I>(
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

/// An error indicating that a frame did not have the correct format for use with UAVCAN
#[derive(Debug)]
pub struct InvalidFrameFormat;

/// Optimizes the provided list and applies filters to a CAN peripheral
fn optimize_and_apply_filters<I>(ideal_filters: &mut [Filter], can: &mut Can<I>)
where
    I: Instance + FilterOwner,
{
    // Reduce the filters if necessary
    let mut hardware_filters = can.modify_filters();
    let max_hardware_filters = hardware_filters.num_banks();
    let optimized_filters = optimize(ideal_filters, max_hardware_filters.into());
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
}

/// Creates and returns a filter that matches the provided subscription, or None if the subscription
/// is a request or response subscription and local_node is None.
fn make_filter(subscription: Subscription, local_node: Option<CanNodeId>) -> Option<Filter> {
    match subscription {
        Subscription::Message(subject) => Some(subject_filter(subject)),
        Subscription::Request(service) => {
            local_node.map(|local_node| request_filter(service, local_node))
        }
        Subscription::Response(service) => {
            local_node.map(|local_node| response_filter(service, local_node))
        }
    }
}

/// Returns a filter that matches message transfers on one subject
///
/// Criteria:
/// * Priority: any
/// * Anonymous: any
/// * Subject ID: matching the provided subject ID
/// * Source node ID: any
fn subject_filter(subject: SubjectId) -> Filter {
    let m_id: u32 = 0b0_0000_0110_0000_0000_0000_0000_0000 | u32::from(subject) << 8;
    let mask: u32 = 0b0_0010_1001_1111_1111_1111_1000_0000;
    Filter::new(mask, m_id)
}

/// Returns a filter that matches service request transfers for one service to one node ID
///
/// Criteria:
/// * Priority: any
/// * Request or response: request
/// * Service ID: matching the provided service ID
/// * Destination: matching the provided node ID
/// * Source: any
fn request_filter(service: ServiceId, destination: CanNodeId) -> Filter {
    let dynamic_id_bits = u32::from(service) << 14 | u32::from(destination) << 7;
    let m_id: u32 = 0b0_0011_0000_0000_0000_0000_0000_0000 | dynamic_id_bits;
    let mask: u32 = 0b0_0011_1111_1111_1111_1111_1000_0000;
    Filter::new(mask, m_id)
}

/// Returns a filter that matches service response transfers for one service to one node ID
///
/// Criteria:
/// * Priority: any
/// * Request or response: response
/// * Service ID: matching the provided service ID
/// * Destination: matching the provided node ID
/// * Source: any
fn response_filter(service: ServiceId, destination: CanNodeId) -> Filter {
    let dynamic_id_bits =
        u32::from(u16::from(service)) << 14 | u32::from(u8::from(destination)) << 7;
    let m_id: u32 = 0b0_0010_0000_0000_0000_0000_0000_0000 | dynamic_id_bits;
    let mask: u32 = 0b0_0011_1111_1111_1111_1111_1000_0000;
    Filter::new(mask, m_id)
}
