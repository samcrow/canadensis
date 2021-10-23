//! CAN driver traits

use crate::data::Frame;
use crate::types::CanNodeId;
use canadensis_core::subscription::Subscription;
use canadensis_core::{nb, OutOfMemoryError};
use core::fmt::Debug;

/// A CAN driver that can send frames
///
/// This may be a basic driver that can only send a few frames at a time, or it may have an
/// additional in-memory queue of outgoing frames.
///
/// This is similar to [the embedded-can `Can` trait](embedded_can::Can), but for transmitting only.
///
/// The result type is `nb::Result`, which allows the driver to indicate that it cannot send a
/// frame.
pub trait TransmitDriver<I> {
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
    fn transmit(&mut self, frame: Frame<I>, now: I) -> nb::Result<Option<Frame<I>>, Self::Error>;
    /// Attempts to flush all frames out of any in-memory queues that may exist and transmit
    /// them
    fn flush(&mut self, now: I) -> nb::Result<(), Self::Error>;
}

/// A CAN driver that can receive frames
///
/// This is similar to [the embedded-can `Can` trait](embedded_can::Can), but for receiving only.
///
/// The result type is `nb::Result`, which allows the driver to indicate that no frame is available
/// to receive.
pub trait ReceiveDriver<I> {
    /// The error type
    type Error: Debug;
    /// Attempts to receive a frame without blocking
    fn receive(&mut self, now: I) -> nb::Result<Frame<I>, Self::Error>;

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
    fn apply_filters<S>(&mut self, local_node: Option<CanNodeId>, subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>;
}
