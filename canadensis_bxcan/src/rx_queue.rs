//! A driver that has a queue of incoming frames

use crate::BxCanDriver;
use bxcan::{Can, FilterOwner, Instance};
use canadensis::core::subscription::Subscription;
use canadensis::core::time::Instant;
use canadensis::core::OutOfMemoryError;
use canadensis_can::driver::{ReceiveDriver, TransmitDriver};
use canadensis_can::queue::{flush_single_queue, ArrayQueue, FrameQueue};
use canadensis_can::types::CanNodeId;
use canadensis_can::Frame;
use core::convert::Infallible;
use heapless::spsc::Queue;

/// A CAN driver with a queue of incoming frames and a queue of outgoing frames
///
/// Type parameters:
/// * `I`: A time instant
/// * `C`: The CAN peripheral
/// * `N`: The capacity (in frames) of the incoming and outgoing queues
pub struct BxCanQueuedDriver<I, C: Instance, const N: usize> {
    /// The inner driver
    driver: BxCanDriver<I, C>,
    /// Queue of outgoing frames
    ///
    /// This keeps frames in order by priority
    tx_queue: ArrayQueue<I, N>,
    /// Queue of incoming frames
    ///
    /// This does not have any special ordering properties. The ordering that happens when frames
    /// are sent is enough.
    rx_queue: Queue<Frame<I>, N>,
    /// Core clock frequency, hertz
    core_frequency: u32,
}

impl<I: Default + Clone, C: Instance, const N: usize> BxCanQueuedDriver<I, C, N> {
    /// Creates a queued driver
    ///
    /// * `can`: A CAN peripheral
    /// * `core_frequency`: The frequency of the processor core clock, in hertz.
    ///   This corresponds to HCLK on some STM32 microcontrollers.
    pub fn new(can: Can<C>, core_frequency: u32) -> Self {
        BxCanQueuedDriver {
            driver: BxCanDriver::new(can),
            tx_queue: ArrayQueue::new(),
            rx_queue: Queue::new(),
            core_frequency,
        }
    }
}

impl<I: Instant + Default + Clone, C: Instance, const N: usize> TransmitDriver<I>
    for BxCanQueuedDriver<I, C, N>
{
    type Error = Infallible;

    fn try_reserve(&mut self, frames: usize) -> Result<(), OutOfMemoryError> {
        self.tx_queue.try_reserve(frames)
    }

    fn transmit(&mut self, frame: Frame<I>, _now: I) -> nb::Result<Option<Frame<I>>, Self::Error> {
        match self.tx_queue.push_frame(frame) {
            Ok(()) => Ok(None),
            Err(_oom) => Err(nb::Error::WouldBlock),
        }
    }

    fn flush(&mut self, now: I) -> nb::Result<(), Self::Error> {
        flush_single_queue(&mut self.tx_queue, &mut self.driver, now)
    }
}

impl<I: Instant, C: Instance + FilterOwner, const N: usize> ReceiveDriver<I>
    for BxCanQueuedDriver<I, C, N>
{
    /// This matches the error type defined in bxcan
    type Error = ();

    fn receive(&mut self, now: I) -> nb::Result<Frame<I>, Self::Error> {
        // To avoid dropping incoming frames, try to receive from the driver several times to get
        // all consecutive frames

        // A frame with an extended ID, including bit stuffing and the interframe space,
        // ranges from 64 to 152 bits. At 1 megabit/second, that corresponds to 64 to 152 microseconds.
        // Wait about 100 microseconds after the first WouldBlock after a received frame.
        match self.driver.receive(now) {
            Ok(first_frame) => {
                match self.rx_queue.enqueue(first_frame) {
                    Ok(()) => {
                        let mut delayed = false;
                        loop {
                            match self.driver.receive(now) {
                                Ok(next_frame) => match self.rx_queue.enqueue(next_frame) {
                                    Ok(()) => {}
                                    Err(_) => {
                                        // No space for the frame. Break and dequeue a frame below.
                                        break;
                                    }
                                },
                                Err(nb::Error::WouldBlock) => {
                                    if delayed {
                                        // Don't delay more than once
                                        break;
                                    } else {
                                        // About 100 microseconds
                                        cortex_m::asm::delay(self.core_frequency / 10000);
                                        delayed = true;
                                    }
                                }
                                Err(nb::Error::Other(())) => break,
                            }
                        }
                    }
                    Err(_) => {
                        // No space for the frame. Continue and dequeue a frame below.
                    }
                }
            }
            Err(_) => { /* Ignore, check for a frame in the queue */ }
        }
        self.rx_queue.dequeue().ok_or(nb::Error::WouldBlock)
    }

    fn apply_filters<S>(&mut self, local_node: Option<CanNodeId>, subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>,
    {
        self.driver.apply_filters(local_node, subscriptions)
    }
}
