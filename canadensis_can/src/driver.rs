//! CAN driver traits

use crate::data::Frame;
use crate::types::CanNodeId;
use alloc::vec::Vec;
use canadensis_core::subscription::Subscription;
use canadensis_core::time::Clock;
use canadensis_core::{nb, OutOfMemoryError, ServiceId, SubjectId};
use canadensis_filter_config::{optimize, Filter};
use core::fmt::Debug;
use fallible_collections::FallibleVec;

/// A CAN driver that can send frames
///
/// This may be a basic driver that can only send a few frames at a time, or it may have an
/// additional in-memory queue of outgoing frames.
///
/// The result type is `nb::Result`, which allows the driver to indicate that it cannot send a
/// frame.
pub trait TransmitDriver<C>
where
    C: Clock,
{
    /// The error type
    type Error: Debug;

    /// Attempts to reserve space to transmit `frames` additional frames
    ///
    /// If this driver does not contain a queue, this function may return an error if `frames`
    /// is greater than 1.
    fn try_reserve(&mut self, frames: usize) -> Result<(), OutOfMemoryError>;

    /// Attempts to send a frame without blocking
    ///
    /// If this driver contains a queue, this function may add the frame to the queue and not
    /// immediately transmit until `flush()` is called.
    fn transmit(
        &mut self,
        frame: Frame<C::Instant>,
        clock: &mut C,
    ) -> nb::Result<Option<Frame<C::Instant>>, Self::Error>;
    /// Attempts to flush all frames out of any in-memory queues that may exist and transmit
    /// them
    fn flush(&mut self, clock: &mut C) -> nb::Result<(), Self::Error>;
}

/// A CAN driver that can receive frames
///
/// The result type is `nb::Result`, which allows the driver to indicate that no frame is available
/// to receive.
pub trait ReceiveDriver<C>
where
    C: Clock,
{
    /// The error type
    type Error: Debug;
    /// Attempts to receive a frame without blocking
    fn receive(&mut self, clock: &mut C) -> nb::Result<Frame<C::Instant>, Self::Error>;

    /// Sets up frame reception filters to accept only frames matching the provided subscriptions
    ///
    /// The filters may allow frames that this node is not subscribed to (false positives), but
    /// they must not block any frames that this node is not subscribed to (false negatives).
    ///
    /// `local_node` is the ID of this node, which can be used to filter service transfers based
    /// on the destination address. If this is None, the filters should block all service transfers
    /// (because anonymous nodes can't participate in service transfers)
    ///
    /// If the hardware does not support filtering, this function may be empty and the
    /// hardware should receive all available frames.
    ///
    /// The [`optimize_filters`](crate::driver::optimize_filters) function may be helpful when
    /// implementing this function.
    fn apply_filters<S>(&mut self, local_node: Option<CanNodeId>, subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>;
    /// Sets up frame reception filters to accept all incoming frames
    fn apply_accept_all(&mut self);
}

/// Creates a set of filters from the provided subscription, optimizes them to reduce the number
/// of filters, and passes the resulting filters to a callback
pub fn optimize_filters<F, S>(
    local_node: Option<CanNodeId>,
    subscriptions: S,
    max_filters: usize,
    f: F,
) -> Result<(), OutOfMemoryError>
where
    F: FnOnce(&[Filter]),
    S: IntoIterator<Item = Subscription>,
{
    let mut filters: Vec<Filter> = Vec::new();
    for subscription in subscriptions {
        if let Some(filter) = make_filter(subscription, local_node) {
            filters.try_push(filter)?;
        }
    }
    let filters = optimize(&mut filters, max_filters);
    f(filters);
    Ok(())
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
