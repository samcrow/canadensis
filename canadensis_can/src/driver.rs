//! CAN driver traits

use crate::data::Frame;
use canadensis_core::nb;
use core::fmt::Debug;

/// A CAN driver that can send frames
///
/// This is similar to [the embedded-can `Can` trait](embedded_can::Can), but for transmitting only.
///
/// The result type is `nb::Result`, which allows the driver to indicate that it cannot send a
/// frame.
pub trait TransmitDriver<I> {
    /// The error type
    type Error: Debug;
    /// Attempts to send a frame without blocking
    fn transmit(&mut self, frame: &Frame<I>) -> nb::Result<Option<Frame<I>>, Self::Error>;
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
    fn receive(&mut self) -> nb::Result<Frame<I>, Self::Error>;
}
