//!
//! Utilities for running Cyphal nodes on Linux using the SocketCAN interface
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
use canadensis_core::time::{Clock, Microseconds32};
use canadensis_core::{nb, OutOfMemoryError};
use socketcan::frame::AsPtr;
use socketcan::{EmbeddedFrame, Id, Socket, SocketOptions};
use std::convert::TryInto;
use std::io;
use std::io::ErrorKind;
use std::os::fd::AsRawFd;

/// An adapter between SocketCAN and the canadensis frame format
pub struct LinuxCan<S: Socket> {
    socket: S,
}

impl<S: Socket> LinuxCan<S> {
    /// Creates a Linux CAN adapter around a SocketCAN socket
    pub fn new(socket: S) -> Self {
        LinuxCan { socket }
    }
}

impl<S: Socket> TransmitDriver<SystemClock> for LinuxCan<S>
where
    <S as Socket>::FrameType: EmbeddedFrame + AsPtr,
{
    type Error = io::Error;

    fn try_reserve(&mut self, _frames: usize) -> Result<(), OutOfMemoryError> {
        // Assume there's enough space
        Ok(())
    }

    fn transmit(
        &mut self,
        frame: Frame,
        clock: &mut SystemClock,
    ) -> nb::Result<Option<Frame>, Self::Error> {
        // Drop this frame if its deadline has passed
        let now = clock.now();
        if frame.timestamp() < now {
            log::warn!("Dropping frame that has missed its deadline");
            return Ok(None);
        }
        let socketcan_frame = S::FrameType::new(
            socketcan::Id::Extended(
                socketcan::ExtendedId::new(frame.id().into()).expect("Invalid CAN ID"),
            ),
            frame.data(),
        )
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

    fn flush(&mut self, _clock: &mut SystemClock) -> canadensis_core::nb::Result<(), Self::Error> {
        // Presumably this happens automatically
        Ok(())
    }
}

impl<SK: Socket + SocketOptions> ReceiveDriver<SystemClock> for LinuxCan<SK>
where
    <SK as Socket>::FrameType: EmbeddedFrame,
{
    type Error = io::Error;

    fn receive(&mut self, clock: &mut SystemClock) -> nb::Result<Frame, Self::Error> {
        loop {
            let socketcan_frame = self.socket.read_frame()?;
            if socketcan_frame.data().len() <= canadensis_can::FRAME_CAPACITY {
                let raw_id = match socketcan_frame.id() {
                    Id::Standard(_) => continue,
                    Id::Extended(id) => id.as_raw(),
                };
                let cyphal_frame = canadensis_can::Frame::new(
                    clock.now(),
                    raw_id.try_into().expect("Invalid CAN ID"),
                    socketcan_frame.data(),
                );
                return Ok(cyphal_frame);
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
                .map(|filter| socketcan::CanFilter::new(filter.id(), filter.mask()))
                .collect::<Vec<_>>();
            self.socket.set_filters(&socketcan_filters).unwrap();
        })
        .unwrap()
    }

    fn apply_accept_all(&mut self) {
        self.socket.set_filter_accept_all().unwrap();
    }
}

impl<S: Socket> AsRawFd for LinuxCan<S> {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.socket.as_raw_fd()
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
    fn now(&mut self) -> Microseconds32 {
        let since_start = std::time::Instant::now().duration_since(self.start_time);
        let microseconds = since_start.as_micros();
        Microseconds32::from_ticks(microseconds as u32)
    }
}
