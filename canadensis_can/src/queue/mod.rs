//! Queues of outgoing CAN frames

mod array_queue;
mod single_frame_queue;

pub use self::array_queue::ArrayQueue;
pub use self::single_frame_queue::SingleFrameQueue;
use core::cmp::Ordering;

use crate::driver::{ReceiveDriver, TransmitDriver};
use crate::types::CanNodeId;
use crate::Frame;
use canadensis_core::subscription::Subscription;
use canadensis_core::time::Instant;
use canadensis_core::{nb, OutOfMemoryError};

/// A queue of outgoing frames
pub trait FrameQueue<I> {
    /// Attempts to reserve memory for some number of additional frames
    ///
    /// After `try_reserve(n)` returns `Ok(())` for any n, the next n calls to `push_frame()` must
    /// `return Ok(())`.
    ///
    fn try_reserve(&mut self, additional: usize) -> Result<(), OutOfMemoryError>;
    /// Attempts to free memory by reducing excess capacity in this queue
    fn shrink_to_fit(&mut self);

    /// Pushes a frame onto the back of this queue
    ///
    /// The frame must end up in front of all existing frames with a greater CAN ID, but behind all
    /// frames with an equal or lesser CAN ID. This keeps the frames in order by priority and then
    /// by first-in, first-out.
    fn push_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError>;

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

/// A single transmit queue and a single driver
pub struct SingleQueueDriver<Q, D> {
    queue: Q,
    driver: D,
}

impl<Q, D> SingleQueueDriver<Q, D> {
    /// Creates a queue and driver pair
    pub fn new(queue: Q, driver: D) -> Self {
        SingleQueueDriver { queue, driver }
    }

    /// Breaks down this queue driver into its queue and driver
    pub fn into_parts(self) -> (Q, D) {
        (self.queue, self.driver)
    }

    /// Returns a reference to the driver
    pub fn driver(&self) -> &D {
        &self.driver
    }
    /// Returns a mutable reference to the driver
    pub fn driver_mut(&mut self) -> &mut D {
        &mut self.driver
    }
}

impl<I, Q, D> TransmitDriver<I> for SingleQueueDriver<Q, D>
where
    I: Instant,
    Q: FrameQueue<I>,
    D: TransmitDriver<I>,
{
    type Error = D::Error;

    fn try_reserve(&mut self, frames: usize) -> Result<(), OutOfMemoryError> {
        self.queue.try_reserve(frames)
    }

    /// Adds a frame to the back of the queue
    ///
    /// This function returns `Err(nb::Error::WouldBlock)` if the queue is full.
    fn transmit(&mut self, frame: Frame<I>, _now: I) -> nb::Result<Option<Frame<I>>, Self::Error> {
        self.queue
            .push_frame(frame)
            .map(|_oom| None)
            .map_err(|_oom| nb::Error::WouldBlock)
    }

    /// Attempts to send all queued frames to the driver
    fn flush(&mut self, now: I) -> nb::Result<(), Self::Error> {
        flush_single_queue(&mut self.queue, &mut self.driver, now)
    }
}

impl<I, Q, D> ReceiveDriver<I> for SingleQueueDriver<Q, D>
where
    D: ReceiveDriver<I>,
{
    type Error = D::Error;

    fn receive(&mut self, now: I) -> nb::Result<Frame<I>, Self::Error> {
        self.driver.receive(now)
    }

    fn apply_filters<S>(&mut self, local_node: Option<CanNodeId>, subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>,
    {
        self.driver.apply_filters(local_node, subscriptions)
    }
}

/// Flushes from one queue to one driver
///
/// This function discards frames with a deadline less than the current time (`now`).
///
pub fn flush_single_queue<I, Q, D>(
    queue: &mut Q,
    driver: &mut D,
    now: I,
) -> nb::Result<(), D::Error>
where
    I: Instant,
    Q: FrameQueue<I>,
    D: TransmitDriver<I>,
{
    while let Some(frame) = queue.pop_frame() {
        if frame_is_expired(&frame, &now) {
            // Frame deadline has passed
            drop(frame);
            continue;
        }

        match driver.transmit(frame.clone(), now) {
            Ok(None) => { /* Transmitted, keep going and try the next frame */ }
            Ok(Some(removed_frame)) => {
                // Removed a lower-priority frame
                if !frame_is_expired(&removed_frame, &now) {
                    // Because we just popped a frame from the queue, it must have space to
                    // return a frame.
                    queue
                        .return_frame(removed_frame)
                        .expect("return_frame out of memory");
                }
                // Keep going and try the next frame
            }
            Err(nb::Error::WouldBlock) => {
                // The frame couldn't be transmitted, so put it back in the queue
                // Because we just popped a frame from the queue, it must have space to
                // return a frame.
                queue
                    .return_frame(frame)
                    .expect("return_frame out of memory");
                return Err(nb::Error::WouldBlock);
            }
            Err(nb::Error::Other(e)) => return Err(nb::Error::Other(e)),
        }
    }
    Ok(())
}

/// Returns true if this frame's deadline is in the past
fn frame_is_expired<I>(frame: &Frame<I>, now: &I) -> bool
where
    I: Instant,
{
    frame.timestamp().overflow_safe_compare(now) == Ordering::Less
}
