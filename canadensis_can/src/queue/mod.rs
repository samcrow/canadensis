//! Queues of outgoing CAN frames

mod array_queue;

pub use self::array_queue::ArrayQueue;

use crate::{Frame, OutOfMemoryError};

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

/// An aggregation of two frame queues that can be used for double-redundant transports
///
/// The [`try_reserve`](#method.try_reserve) and [`push_frame`](#method.push_frame) functions will
/// return `Ok(())` if the operation succeeded on at least one of the queues.
///
/// Double-redundant queue objects can be nested for use with triple-redundant transports.
pub struct DoubleRedundant<Q0, Q1> {
    /// Inner queue 0
    queue0: Q0,
    /// Inner queue 1
    queue1: Q1,
    /// Result of the last try_reserve() call on queue 0
    status0: Result<(), OutOfMemoryError>,
    /// Result of the last try_reserve() call on queue 1
    status1: Result<(), OutOfMemoryError>,
}

impl<Q0, Q1> DoubleRedundant<Q0, Q1> {
    /// Creates a redundant queue aggregation
    pub fn new(queue0: Q0, queue1: Q1) -> Self {
        DoubleRedundant {
            queue0,
            queue1,
            status0: Ok(()),
            status1: Ok(()),
        }
    }
}

impl<I, Q0, Q1> FrameSink<I> for DoubleRedundant<Q0, Q1>
where
    I: Clone,
    Q0: FrameSink<I>,
    Q1: FrameSink<I>,
{
    /// Tries to reserve space on both queues, returning `Ok(())` if the operation succeeded
    /// on at least one queue
    fn try_reserve(&mut self, additional: usize) -> Result<(), OutOfMemoryError> {
        self.status0 = self.queue0.try_reserve(additional);
        self.status1 = self.queue1.try_reserve(additional);
        // If one queue failed, it might be full because the underlying transport is broken.
        // This is successful if space was available in at least one queue.
        self.status0.clone().or(self.status1.clone())
    }

    fn shrink_to_fit(&mut self) {
        self.queue0.shrink_to_fit();
        self.queue1.shrink_to_fit();
    }

    /// Tries to push a frame onto both queues, returning `Ok(())` if the operation succeeded
    /// on at least one queue
    ///
    /// If a queue failed to allocate memory during the most recent call to
    /// [`try_reserve`](#method.try_reserve), this function does not attempt to push a frame onto
    /// that queue.
    fn push_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        // If a queue failed to reserve memory in the last call to try_reserve(),
        // don't try to push the frame there.
        let push_status_0 = if self.status0.is_ok() {
            self.queue0.push_frame(frame.clone())
        } else {
            Err(OutOfMemoryError)
        };
        let push_status_1 = if self.status1.is_ok() {
            self.queue1.push_frame(frame)
        } else {
            Err(OutOfMemoryError)
        };
        // This is successful if the frame got onto at least one queue.
        push_status_0.or(push_status_1)
    }
}
