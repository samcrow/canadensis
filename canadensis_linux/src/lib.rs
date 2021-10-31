//!
//! Utilities for running UAVCAN nodes on Linux using the SocketCAN interface
//!

#![deny(missing_docs)]

extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_filter_config;
extern crate log;
extern crate socketcan;

use canadensis_can::driver::{optimize_filters, ReceiveDriver, TransmitDriver};
use canadensis_can::{CanNodeId, Frame};
use canadensis_core::subscription::Subscription;
use canadensis_core::time::{Clock, Instant, Microseconds64};
use canadensis_core::{nb, OutOfMemoryError};
use socketcan::CANSocket;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::io;
use std::io::ErrorKind;

/// An adapter between SocketCAN and the canadensis frame format
pub struct LinuxCan {
    socket: CANSocket,
}

impl LinuxCan {
    /// Creates a Linux CAN adapter around a SocketCAN socket
    pub fn new(socket: CANSocket) -> Self {
        LinuxCan { socket }
    }
}

impl TransmitDriver<Microseconds64> for LinuxCan {
    type Error = io::Error;

    fn try_reserve(&mut self, _frames: usize) -> Result<(), OutOfMemoryError> {
        // Assume there's enough space
        Ok(())
    }

    fn transmit(
        &mut self,
        frame: Frame<Microseconds64>,
        now: Microseconds64,
    ) -> nb::Result<Option<Frame<Microseconds64>>, Self::Error> {
        // Drop this frame if its deadline has passed
        if frame.timestamp().overflow_safe_compare(&now) == Ordering::Less {
            log::warn!("Dropping frame that has missed its deadline");
            return Ok(None);
        }
        let socketcan_frame =
            socketcan::CANFrame::new(frame.id().into(), frame.data(), false, false)
                .expect("Invalid frame format");
        self.socket
            .write_frame_insist(&socketcan_frame)
            .map(|()| None)
            .map_err(|e| {
                if e.kind() == ErrorKind::WouldBlock {
                    nb::Error::WouldBlock
                } else {
                    nb::Error::Other(e)
                }
            })
    }

    fn flush(&mut self, _now: Microseconds64) -> canadensis_core::nb::Result<(), Self::Error> {
        // Presumably this happens automatically
        Ok(())
    }
}

impl ReceiveDriver<Microseconds64> for LinuxCan {
    type Error = io::Error;

    fn receive(&mut self, now: Microseconds64) -> nb::Result<Frame<Microseconds64>, Self::Error> {
        loop {
            let socketcan_frame = self.socket.read_frame()?;
            if socketcan_frame.data().len() <= canadensis_can::FRAME_CAPACITY {
                let uavcan_frame = canadensis_can::Frame::new(
                    now,
                    socketcan_frame.id().try_into().expect("Invalid CAN ID"),
                    socketcan_frame.data(),
                );
                return Ok(uavcan_frame);
            } else {
                log::warn!(
                    "Ignoring a frame {} bytes long, which is too large",
                    socketcan_frame.data().len()
                );
            }
        }
    }

    fn apply_filters<S>(&mut self, local_node: Option<CanNodeId>, subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>,
    {
        optimize_filters(local_node, subscriptions, usize::MAX, |optimized| {
            let socketcan_filters = optimized
                .iter()
                .map(|filter| socketcan::CANFilter::new(filter.id(), filter.mask()).unwrap())
                .collect::<Vec<_>>();
            self.socket.set_filter(&socketcan_filters).unwrap();
        })
        .unwrap()
    }

    fn apply_accept_all(&mut self) {
        self.socket.filter_accept_all().unwrap();
    }
}

/// A clock that uses the operating system's clock
#[derive(Debug, Clone)]
pub struct SystemClock {
    start_time: std::time::Instant,
}

impl SystemClock {
    /// Creates a new system clock
    pub fn new() -> Self {
        SystemClock {
            start_time: std::time::Instant::now(),
        }
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for SystemClock {
    type Instant = Microseconds64;

    fn now(&mut self) -> Self::Instant {
        let since_start = std::time::Instant::now().duration_since(self.start_time);
        let microseconds = since_start.as_micros();
        Microseconds64::new(microseconds as u64)
    }
}
