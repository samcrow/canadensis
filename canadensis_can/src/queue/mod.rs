//! Queues of outgoing CAN frames

mod array_queue;

pub use self::array_queue::ArrayQueue;

use crate::Frame;
use canadensis_core::OutOfMemoryError;

/// A queue of outgoing frames that a transmitter uses to send transfers
pub trait FrameSink<I> {
    /// Attempts to reserve memory for some number of additional frames
    ///
    /// After `try_reserve(n)` returns `Ok(())` for any n, the next n calls to `push_frame()` must
    /// `return Ok(())`.
    ///
    fn try_reserve(&mut self, additional: usize) -> Result<(), OutOfMemoryError>;
    /// Attempts to free memory by reducing excess capacity in this queue
    fn shrink_to_fit(&mut self);

    /// Pushes a frame onto this queue
    ///
    /// The frame must end up in front of all existing frames with a greater CAN ID, but behind all
    /// frames with an equal or lesser CAN ID. This keeps the frames in order by priority and then
    /// by first-in, first-out.
    fn push_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError>;
}

/// A queue of outgoing frames that can be used to copy frames to a CAN controller driver
///
/// All queue implementations must order frames by ID, so that the frame with the lowest CAN ID
/// is at the front. Frames with the same CAN ID must have first-in, first-out ordering.
///
pub trait FrameQueueSource<I> {
    /// Returns a reference to the frame at the front of the queue
    fn peek_frame(&self) -> Option<&Frame<I>>;
    /// Removes and returns the frame at the front of the queue
    fn pop_frame(&mut self) -> Option<Frame<I>>;
    /// Returns a not-yet-transmitted frame to the queue
    ///
    /// This function is used when a frame is displaced from a transmit mailbox and must be stored
    /// for later transmission.
    ///
    /// The frame must end up behind all existing frames with a lesser CAN ID, but in front of all
    /// frames with a greater or equal CAN ID.
    fn return_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError>;
}
