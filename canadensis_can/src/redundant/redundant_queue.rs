use crate::driver::TransmitDriver;
use crate::Frame;
use canadensis_core::time::Clock;
use canadensis_core::{nb, OutOfMemoryError};
use defmt::Format;

/// An aggregation of two outgoing frame queues that can be used for double-redundant transports
///
/// The [`try_reserve`](#method.try_reserve) and [`push_frame`](#method.push_frame) functions will
/// return `Ok(())` if the operation succeeded on at least one of the queues.
///
/// Double-redundant drivers can be nested for use with triple-redundant transports.
///
pub struct RedundantDriver<D0, D1> {
    /// Driver 0
    driver0: D0,
    /// Driver 1
    driver1: D1,
    /// Result of the last try_reserve() call on driver 0
    status0: Result<(), OutOfMemoryError>,
    /// Result of the last try_reserve() call on driver 1
    status1: Result<(), OutOfMemoryError>,
}

impl<D0, D1> RedundantDriver<D0, D1> {
    /// Creates a redundant queue aggregation
    pub fn new(driver0: D0, driver1: D1) -> Self {
        RedundantDriver {
            driver0,
            driver1,
            status0: Ok(()),
            status1: Ok(()),
        }
    }
}

impl<C, D0, D1> TransmitDriver<C> for RedundantDriver<D0, D1>
where
    C: Clock,
    D0: TransmitDriver<C>,
    D1: TransmitDriver<C>,
{
    type Error = RedundantError<D0::Error, D1::Error>;
    /// Tries to reserve space on both queues, returning `Ok(())` if the operation succeeded
    /// on at least one queue
    fn try_reserve(&mut self, additional: usize) -> Result<(), OutOfMemoryError> {
        self.status0 = self.driver0.try_reserve(additional);
        self.status1 = self.driver1.try_reserve(additional);
        // If one queue failed, it might be full because the underlying transport is broken.
        // This is successful if space was available in at least one queue.
        self.status0.clone().or_else(|_| self.status1.clone())
    }

    /// Tries to push a frame to both drivers, returning `Ok(())` if the operation succeeded
    /// on at least one driver
    ///
    /// If a driver failed to allocate memory during the most recent call to
    /// [`try_reserve`](#method.try_reserve), this function does not attempt to push a frame onto
    /// that queue.
    fn transmit(&mut self, frame: Frame, clock: &mut C) -> nb::Result<Option<Frame>, Self::Error> {
        // If a queue failed to reserve memory in the last call to try_reserve(),
        // don't try to push the frame there.
        let push_status_0 = if self.status0.is_ok() {
            self.driver0.transmit(frame.clone(), clock)
        } else {
            Err(nb::Error::WouldBlock)
        };
        let push_status_1 = if self.status1.is_ok() {
            self.driver1.transmit(frame, clock)
        } else {
            Err(nb::Error::WouldBlock)
        };
        // This is successful if the frame got onto at least one queue.
        // If two frames were returned, send the first and ignore the second.
        match (push_status_0, push_status_1) {
            (Ok(returned0), Ok(_returned1)) => Ok(returned0),
            (Ok(returned), _) => Ok(returned),
            (_, Ok(returned)) => Ok(returned),
            (Err(nb::Error::WouldBlock), Err(nb::Error::WouldBlock)) => Err(nb::Error::WouldBlock),
            (Err(nb::Error::Other(e0)), Err(nb::Error::Other(e1))) => {
                Err(nb::Error::Other(RedundantError::Both(e0, e1)))
            }
            (Err(nb::Error::Other(e)), _) => Err(nb::Error::Other(RedundantError::Driver0(e))),
            (_, Err(nb::Error::Other(e))) => Err(nb::Error::Other(RedundantError::Driver1(e))),
        }
    }

    fn flush(&mut self, clock: &mut C) -> nb::Result<(), Self::Error> {
        let flush_status_0 = self.driver0.flush(clock);
        let flush_status_1 = self.driver1.flush(clock);
        // This is successful if at least one driver successfully flushed its frames
        match (flush_status_0, flush_status_1) {
            (Ok(()), _) => Ok(()),
            (_, Ok(())) => Ok(()),
            (Err(nb::Error::WouldBlock), Err(nb::Error::WouldBlock)) => Err(nb::Error::WouldBlock),
            (Err(nb::Error::Other(e0)), Err(nb::Error::Other(e1))) => {
                Err(nb::Error::Other(RedundantError::Both(e0, e1)))
            }
            (Err(nb::Error::Other(e)), _) => Err(nb::Error::Other(RedundantError::Driver0(e))),
            (_, Err(nb::Error::Other(e))) => Err(nb::Error::Other(RedundantError::Driver1(e))),
        }
    }
}

/// An error from a DoubleRedundantQueueDriver
#[derive(Debug, Format)]
pub enum RedundantError<E0, E1> {
    /// An error from driver 0
    Driver0(E0),
    /// An error from driver 1
    Driver1(E1),
    /// Errors from both drivers
    Both(E0, E1),
}
