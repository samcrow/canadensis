//!
//! Utilities for running UAVCAN nodes on Linux using the SocketCAN interface
//!

extern crate canadensis_can;
extern crate canadensis_core;
extern crate socketcan;

use canadensis_core::time::{Clock, Instant, Microseconds64};
use socketcan::CANSocket;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::io;

/// An adapter between SocketCAN and the canadensis frame format
pub struct LinuxCan {
    socket: CANSocket,
    clock: SystemClock,
}

impl LinuxCan {
    pub fn new(socket: CANSocket) -> Self {
        LinuxCan {
            socket,
            clock: SystemClock::new(),
        }
    }

    /// Receives a frame
    pub fn receive(&mut self) -> io::Result<canadensis_can::Frame<Microseconds64>> {
        loop {
            let socketcan_frame = self.socket.read_frame()?;
            if socketcan_frame.data().len() <= canadensis_can::FRAME_CAPACITY {
                let uavcan_frame = canadensis_can::Frame::new(
                    self.clock.now(),
                    socketcan_frame.id().try_into().expect("Invalid CAN ID"),
                    socketcan_frame.data(),
                );
                return Ok(uavcan_frame);
            } else {
                println!(
                    "Warning: Ignoring a frame {} bytes long, which is too large",
                    socketcan_frame.data().len()
                );
            }
        }
    }

    /// Sends a frame, or discards the frame if its deadline has passed
    pub fn send(&mut self, frame: canadensis_can::Frame<Microseconds64>) -> io::Result<()> {
        // Drop this frame if its deadline has passed
        if frame.timestamp().overflow_safe_compare(&self.clock.now()) == Ordering::Less {
            println!("Warning: Dropping frame that has missed its deadline");
            return Ok(());
        }
        let socketcan_frame =
            socketcan::CANFrame::new(frame.id().into(), frame.data(), false, false)
                .expect("Invalid frame format");
        self.socket.write_frame(&socketcan_frame)
    }
}

/// A clock that uses the operating system's clock
#[derive(Debug, Clone)]
pub struct SystemClock {
    start_time: std::time::Instant,
}

impl SystemClock {
    pub fn new() -> Self {
        SystemClock {
            start_time: std::time::Instant::now(),
        }
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
