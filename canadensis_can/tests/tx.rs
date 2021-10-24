//!
//! Transmitter integration tests
//!

extern crate canadensis_can;
extern crate canadensis_core;

use core::convert::TryFrom;
use std::collections::VecDeque;
use std::convert::Infallible;

use canadensis_can::driver::TransmitDriver;
use canadensis_can::types::{CanNodeId, CanTransferId};
use canadensis_can::{CanId, CanTransmitter, Frame, Mtu};
use canadensis_core::time::{Clock, Microseconds32};
use canadensis_core::transfer::*;
use canadensis_core::transport::Transmitter;
use canadensis_core::{OutOfMemoryError, Priority, ServiceId, SubjectId};

fn instant(ticks: u32) -> Microseconds32 {
    Microseconds32::new(ticks)
}

#[test]
fn test_heartbeat() {
    let mut driver = MockDriver::default();
    let mut tx = CanTransmitter::new(Mtu::Can8);
    tx.push(
        Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(0),
                transfer_id: CanTransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(7509).unwrap(),
                source: Some(CanNodeId::try_from(42u8).unwrap()),
            }),
            payload: &[0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68],
        },
        &mut ZeroClock,
        &mut driver,
    )
    .unwrap();

    assert_eq!(
        Some(Frame::new(
            instant(0),
            CanId::try_from(0x107d552a).unwrap(),
            &[0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68, 0xe0]
        )),
        driver.pop_frame()
    );
    assert_eq!(None, driver.pop_frame());

    // New transaction ID, new uptime
    tx.push(
        Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(0),
                transfer_id: CanTransferId::try_from(1).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(7509).unwrap(),
                source: Some(CanNodeId::try_from(42u8).unwrap()),
            }),
            payload: &[0x01, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68],
        },
        &mut ZeroClock,
        &mut driver,
    )
    .unwrap();

    assert_eq!(
        Some(Frame::new(
            instant(0),
            CanId::try_from(0x107d552a).unwrap(),
            &[0x01, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68, 0xe1]
        )),
        driver.pop_frame()
    );
    assert_eq!(None, driver.pop_frame());
}

#[test]
#[cfg(feature = "can-fd")]
fn test_string() {
    let mut driver = MockDriver::default();
    let mut tx = CanTransmitter::new(Mtu::CanFd64);
    tx.push(
        Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(0),
                transfer_id: CanTransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(4919).unwrap(),
                source: None,
            }),
            payload: &[
                0x00, 0x18, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,
            ],
        },
        &mut ZeroClock,
        &mut driver,
    )
    .unwrap();

    assert_eq!(
        Some(Frame::new(
            instant(0),
            CanId::try_from(0x1173376c).unwrap(),
            &[
                0x00, 0x18, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,
                0x00, 0xe0
            ]
        )),
        driver.pop_frame()
    );
    assert_eq!(None, driver.pop_frame());
}

#[test]
fn test_node_info_request() {
    let mut driver = MockDriver::default();
    let mut tx = CanTransmitter::new(Mtu::Can8);
    tx.push(
        Transfer {
            header: Header::Request(ServiceHeader {
                timestamp: instant(0),
                transfer_id: CanTransferId::try_from(1).unwrap(),
                priority: Priority::Nominal,
                service: ServiceId::try_from(430).unwrap(),
                source: CanNodeId::try_from(123u8).unwrap(),
                destination: CanNodeId::try_from(42u8).unwrap(),
            }),
            payload: &[],
        },
        &mut ZeroClock,
        &mut driver,
    )
    .unwrap();

    assert_eq!(
        Some(Frame::new(
            instant(0),
            CanId::try_from(0x136b957b).unwrap(),
            &[0xe1]
        )),
        driver.pop_frame()
    );
    assert_eq!(None, driver.pop_frame());
}

#[test]
fn test_node_info_response() {
    let mut driver = MockDriver::default();
    let mut tx = CanTransmitter::new(Mtu::Can8);
    tx.push(
        Transfer {
            header: Header::Response(ServiceHeader {
                timestamp: instant(0),
                transfer_id: CanTransferId::try_from(1).unwrap(),
                priority: Priority::Nominal,
                service: ServiceId::try_from(430).unwrap(),
                source: CanNodeId::try_from(42u8).unwrap(),
                destination: CanNodeId::try_from(123u8).unwrap(),
            }),
            payload: &b"\x01\x00\x00\x00\x01\x00\x00\
                    \x00\x00\x00\x00\x00\x00\x00\
                    \x00\x00\x00\x00\x00\x00\x00\
                    \x00\x00\x00\x00\x00\x00\x00\
                    \x00\x00\x24org.\
                    uavcan.\
                    pyuavca\
                    n.demo.\
                    basic_u\
                    sage\x00\x00"[..],
        },
        &mut ZeroClock,
        &mut driver,
    )
    .unwrap();

    let expected_can_id = CanId::try_from(0x126bbdaa).unwrap();
    let expected_frame_data: [&[u8]; 11] = [
        b"\x01\x00\x00\x00\x01\x00\x00\xa1",
        b"\x00\x00\x00\x00\x00\x00\x00\x01",
        b"\x00\x00\x00\x00\x00\x00\x00\x21",
        b"\x00\x00\x00\x00\x00\x00\x00\x01",
        b"\x00\x00\x24org.\x21",
        b"uavcan.\x01",
        b"pyuavca\x21",
        b"n.demo.\x01",
        b"basic_u\x21",
        b"sage\x00\x00\x9a\x01",
        b"\xe7\x61",
    ];

    for &expected_data in expected_frame_data.iter() {
        let expected_frame = Frame::new(instant(0), expected_can_id, expected_data);
        assert_eq!(Some(expected_frame), driver.pop_frame());
    }
    assert_eq!(None, driver.pop_frame());
}

#[test]
#[cfg(feature = "can-fd")]
fn test_array() {
    let mut driver = MockDriver::default();
    let mut tx = CanTransmitter::new(Mtu::CanFd64);
    tx.push(
        Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(0),
                transfer_id: CanTransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(4919).unwrap(),
                source: Some(CanNodeId::try_from(59u8).unwrap()),
            }),
            payload: &[
                0x00, 0xb8, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
                0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
                0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
                0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35,
                0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43,
                0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51,
                0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b,
            ][..],
        },
        &mut ZeroClock,
        &mut driver,
    )
    .unwrap();

    let expected_can_id = CanId::try_from(0x1073373b).unwrap();
    let expected_frame_data: [&[u8]; 2] = [
        &[
            0x00, 0xb8, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
            0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35,
            0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0xa0,
        ],
        &[
            0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a,
            0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
            0x59, 0x5a, 0x5b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0xc0, 0x48, 0x40,
        ],
    ];

    for &expected_data in expected_frame_data.iter() {
        let expected_frame = Frame::new(instant(0), expected_can_id, expected_data);
        assert_eq!(Some(expected_frame), driver.pop_frame());
    }
    assert_eq!(None, driver.pop_frame());
}

/// A simple driver that uses a `VecDeque`
///
/// This does not keep the frames in order by priority, but it is correct as long as it is used for
/// only one transfer at a time.
#[derive(Default)]
struct MockDriver<I> {
    queue: VecDeque<Frame<I>>,
}

impl<I> MockDriver<I> {
    fn pop_frame(&mut self) -> Option<Frame<I>> {
        self.queue.pop_front()
    }
}

impl<I> TransmitDriver<I> for MockDriver<I> {
    type Error = Infallible;

    fn try_reserve(&mut self, frames: usize) -> Result<(), OutOfMemoryError> {
        self.queue.reserve(frames);
        Ok(())
    }

    fn transmit(
        &mut self,
        frame: Frame<I>,
        _now: I,
    ) -> canadensis_core::nb::Result<Option<Frame<I>>, Self::Error> {
        self.queue.push_back(frame);
        Ok(None)
    }

    fn flush(&mut self, _now: I) -> canadensis_core::nb::Result<(), Self::Error> {
        Ok(())
    }
}

/// A clock that produces a Microseconds32 value that is always zero
struct ZeroClock;

impl Clock for ZeroClock {
    type Instant = Microseconds32;

    fn now(&mut self) -> Self::Instant {
        Microseconds32::new(0)
    }
}
