//! A driver that contains frame queues only, and requires external code to actually send and
//! receive messages

use crate::driver::{ReceiveDriver, TransmitDriver};
use crate::queue::{ArrayQueue, FrameQueue};
use crate::types::CanNodeId;
use crate::Frame;
use alloc::vec::Vec;
use canadensis_core::subscription::Subscription;
use canadensis_core::time::Clock;
use canadensis_core::{nb, OutOfMemoryError};
use core::convert::Infallible;
use fallible_collections::FallibleVec;
use heapless::Deque;

/// A driver that contains frame queues only, and requires external code to actually send and
/// receive messages
///
/// This may be useful on embedded devices that do not have a hardware-managed queue of incoming
/// frames. A high-priority interrupt handler can read frames from the CAN peripheral into another
/// queue, and then pass them to this driver when there are no incoming frames waiting.
///
/// To transmit frames, this driver stores an [`ArrayQueue`](ArrayQueue) that lets external code
/// remove frames and send them to a CAN peripheral.
///
/// Type parameters:
/// * `I`: A time instant
/// * `TC`: The transmit queue capacity, in frames
/// * `RC`: The receive queue capacity, in frames
///
/// ## Filters
///
/// When a driver calls `apply_filters()`, this struct stores a `Vec` of the current subscriptions.
/// External code should get these subscriptions using the `subscriptions()` function and manually
/// send them to the CAN driver.
///
pub struct QueueOnlyDriver<I, const TC: usize, const RC: usize> {
    tx_queue: ArrayQueue<I, TC>,
    rx_queue: Deque<Frame<I>, RC>,
    subscriptions: Option<Vec<Subscription>>,
}

impl<I: Default + Clone, const TC: usize, const RC: usize> QueueOnlyDriver<I, TC, RC> {
    /// Creates a driver
    pub fn new() -> Self {
        Default::default()
    }

    /// Pushes a received frame onto the back of the receive queue
    ///
    /// The frame can be retrieved using the `receive()` function.
    pub fn push_rx_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        self.rx_queue.push_back(frame).map_err(|_| OutOfMemoryError)
    }

    /// Removes and returns a frame from the front of the transmit queue
    pub fn pop_tx_frame(&mut self) -> Option<Frame<I>> {
        self.tx_queue.pop_frame()
    }

    /// Returns a frame to the front of the transmit queue
    ///
    /// The frame will be moved back beyond any other frames already in the queue that have
    /// higher priority.
    pub fn return_tx_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        self.tx_queue.return_frame(frame)
    }

    /// Returns the subscriptions provided in the last call to `apply_filters()`
    ///
    /// This function returns None
    /// if `empty_filters()` has not been called, was called with no subscriptions, or was called
    /// but an out-of-memory error occurred while collecting the subscriptions
    pub fn subscriptions(&self) -> Option<&[Subscription]> {
        self.subscriptions.as_deref()
    }
}

impl<I: Default, const TC: usize, const RC: usize> Default for QueueOnlyDriver<I, TC, RC> {
    fn default() -> Self {
        QueueOnlyDriver {
            tx_queue: ArrayQueue::new(),
            rx_queue: Deque::new(),
            subscriptions: None,
        }
    }
}

impl<C: Clock, const TC: usize, const RC: usize> TransmitDriver<C>
    for QueueOnlyDriver<C::Instant, TC, RC>
where
    C::Instant: Default,
{
    type Error = Infallible;

    fn try_reserve(&mut self, frames: usize) -> Result<(), OutOfMemoryError> {
        self.tx_queue.try_reserve(frames)
    }

    fn transmit(
        &mut self,
        frame: Frame<C::Instant>,
        _clock: &mut C,
    ) -> nb::Result<Option<Frame<C::Instant>>, Self::Error> {
        self.tx_queue
            .push_frame(frame)
            .map(|_oom| None)
            .map_err(|_oom| nb::Error::WouldBlock)
    }

    fn flush(&mut self, _clock: &mut C) -> nb::Result<(), Self::Error> {
        // Can't do anything here. Frames have to be removed externally.
        Ok(())
    }
}

impl<I: Default + Clone, const TC: usize, const RC: usize> ReceiveDriver<I>
    for QueueOnlyDriver<I, TC, RC>
{
    type Error = Infallible;

    fn receive(&mut self, _now: I) -> nb::Result<Frame<I>, Self::Error> {
        self.rx_queue.pop_front().ok_or(nb::Error::WouldBlock)
    }

    fn apply_filters<S>(&mut self, _local_node: Option<CanNodeId>, new_subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>,
    {
        match self.subscriptions.as_mut() {
            Some(subscriptions) => {
                subscriptions.clear();
                for subscription in new_subscriptions {
                    if FallibleVec::try_push(subscriptions, subscription).is_err() {
                        // No memory. Remove subscriptions.
                        self.subscriptions = None;
                        break;
                    }
                }
            }
            None => {
                let mut subscriptions = Vec::new();
                for subscription in new_subscriptions {
                    if FallibleVec::try_push(&mut subscriptions, subscription).is_err() {
                        // No memory. Remove subscriptions.
                        self.subscriptions = None;
                        break;
                    }
                }
                self.subscriptions = Some(subscriptions);
            }
        }
    }

    fn apply_accept_all(&mut self) {
        self.subscriptions = None;
    }
}
