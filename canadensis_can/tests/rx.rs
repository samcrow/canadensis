//!
//! Receiver integration tests
//!

extern crate canadensis_can;
extern crate canadensis_core;

use core::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::collections::VecDeque;

use canadensis_can::driver::ReceiveDriver;
use canadensis_can::{CanId, CanNodeId, CanReceiver, Frame};
use canadensis_core::nb;
use canadensis_core::subscription::Subscription;
use canadensis_core::time::{Clock, MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::*;
use canadensis_core::transport::Receiver;
use canadensis_core::{Priority, ServiceId, SubjectId};

type TestInstant = Microseconds32;
type TestDuration = MicrosecondDuration32;

fn instant(ticks: u32) -> TestInstant {
    TestInstant::from_ticks(ticks)
}
fn duration(ticks: u32) -> TestDuration {
    TestDuration::from_ticks(ticks)
}

#[test]
fn test_heartbeat() {
    let mut driver = StubDriver::default();
    let mut rx = CanReceiver::new(0u8.try_into().unwrap());

    let heartbeat_subject = SubjectId::try_from(7509).unwrap();
    rx.subscribe_message(heartbeat_subject, 7, duration(0), &mut driver)
        .unwrap();

    driver.push(Frame::new(
        instant(42),
        0x107d552a.try_into().unwrap(),
        &[0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68, 0xe0],
    ));
    let clock = ClockOwner::default();
    clock.set_ticks(0);
    let transfer = rx
        .receive(&mut clock.make_clock(), &mut driver)
        .unwrap()
        .expect("Didn't get a transfer");

    let expected = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: instant(42),
            transfer_id: 0.try_into().unwrap(),
            priority: Priority::Nominal,
            subject: heartbeat_subject,
            source: Some(42u8.try_into().unwrap()),
        }),
        loopback: false,
        payload: vec![0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68],
    };
    assert_eq!(expected, transfer);
}
#[test]
#[cfg(feature = "can-fd")]
fn test_string() {
    let mut driver = StubDriver::default();
    let mut rx = CanReceiver::new(0u8.try_into().unwrap());

    let string_subject = SubjectId::try_from(4919).unwrap();
    rx.subscribe_message(string_subject, 15, duration(0), &mut driver)
        .unwrap();

    driver.push(Frame::new(
        instant(42),
        0x11133775.try_into().unwrap(),
        b"\x00\x18Hello world!\x00\xe0",
    ));
    let clock = ClockOwner::default();
    clock.set_ticks(0);
    let transfer = rx
        .receive(&mut clock.make_clock(), &mut driver)
        .unwrap()
        .expect("Didn't get a transfer");

    let expected = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: instant(42),
            transfer_id: 0.try_into().unwrap(),
            priority: Priority::Nominal,
            subject: string_subject,
            source: None,
        }),
        loopback: false,
        payload: b"\x00\x18Hello world!\x00".to_vec(),
    };
    assert_eq!(expected, transfer);
}
#[test]
fn test_node_info_request() {
    let mut driver = StubDriver::default();
    let mut rx = CanReceiver::new(42u8.try_into().unwrap());

    let service = ServiceId::try_from(430).unwrap();
    rx.subscribe_request(service, 0, duration(0), &mut driver)
        .unwrap();

    driver.push(Frame::new(
        instant(302),
        0x136b957b.try_into().unwrap(),
        &[0xe1],
    ));
    let clock = ClockOwner::default();
    clock.set_ticks(0);
    let transfer = rx
        .receive(&mut clock.make_clock(), &mut driver)
        .unwrap()
        .expect("Didn't get a transfer");

    let expected = Transfer {
        header: Header::Request(ServiceHeader {
            timestamp: instant(302),
            transfer_id: 1.try_into().unwrap(),
            priority: Priority::Nominal,
            service,
            source: 123u8.try_into().unwrap(),
            destination: 42u8.try_into().unwrap(),
        }),
        loopback: false,
        payload: vec![],
    };
    assert_eq!(expected, transfer);
}
#[test]
fn test_node_info_response() {
    let mut driver = StubDriver::default();
    let mut rx = CanReceiver::new(123u8.try_into().unwrap());

    let service = ServiceId::try_from(430).unwrap();
    rx.subscribe_response(service, 69, duration(100), &mut driver)
        .unwrap();

    let payload: [u8; 69] = [
        0x01, 0x00, // Protocol version
        0x00, 0x00, // Hardware version
        0x01, 0x00, // Software version
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // VCS revision ID
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // Unique ID
        0x24, // String length prefix
        b'o', b'r', b'g', b'.', b'u', b'a', b'v', b'c', b'a', b'n', b'.', b'p', b'y', b'u', b'a',
        b'v', b'c', b'a', b'n', b'.', b'd', b'e', b'm', b'o', b'.', b'b', b'a', b's', b'i', b'c',
        b'_', b'u', b's', b'a', b'g', b'e', // org.uavcan.pyuavcan.demo.basic_usage
        0x00, // Software image CRC length
        0x00, // Certificate of authenticity length
    ];

    let frames_and_times: [(&[u8], u32); 11] = [
        (&[0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xa1], 100),
        (&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], 102),
        (&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21], 105),
        (&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], 120),
        (b"\x00\x00\x24org.\x21", 130),
        (b"uavcan.\x01", 135),
        (b"pyuavca\x21", 160),
        (b"n.demo.\x01", 190),
        (b"basic_u\x21", 197),
        (b"sage\x00\x00\x9a\x01", 198),
        // The last frame barely makes the deadline
        (&[0xe7, 0x61], 200),
    ];

    let clock = ClockOwner::default();
    for (i, &(frame_data, frame_time)) in frames_and_times.iter().enumerate() {
        let frame = Frame::new(
            instant(frame_time),
            0x126BBDAA.try_into().unwrap(),
            frame_data,
        );
        driver.push(frame);
        clock.set_ticks(frame_time);
        if i != frames_and_times.len() - 1 {
            let maybe_transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
            assert!(maybe_transfer.is_none());
        } else {
            // End of transfer
            let transfer = rx
                .receive(&mut clock.make_clock(), &mut driver)
                .unwrap()
                .expect("Didn't get a transfer");

            let expected = Transfer {
                header: Header::Response(ServiceHeader {
                    // Timestamp matches the first frame's timestamp
                    timestamp: instant(100),
                    transfer_id: 1.try_into().unwrap(),
                    priority: Priority::Nominal,
                    service,
                    source: 42u8.try_into().unwrap(),
                    destination: 123u8.try_into().unwrap(),
                }),
                loopback: false,
                payload: payload.to_vec(),
            };
            assert_eq!(expected, transfer);
        }
    }
}
#[test]
fn test_node_info_response_timeout() {
    let mut driver = StubDriver::default();
    let clock = ClockOwner::default();
    let mut rx = CanReceiver::new(123u8.try_into().unwrap());

    let service = ServiceId::try_from(430).unwrap();
    rx.subscribe_response(service, 69, duration(100), &mut driver)
        .unwrap();

    let frames_and_times: [(&[u8], u32); 11] = [
        (&[0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xa1], 100),
        (&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], 102),
        (&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21], 105),
        (&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], 120),
        (b"\x00\x00\x24org.\x21", 130),
        (b"uavcan.\x01", 135),
        (b"pyuavca\x21", 160),
        (b"n.demo.\x01", 190),
        (b"basic_u\x21", 197),
        (b"sage\x00\x00\x9a\x01", 198),
        // The last frame barely misses the deadline
        (&[0xe7, 0x61], 201),
    ];

    for &(frame_data, frame_time) in frames_and_times.iter() {
        let frame = Frame::new(
            instant(frame_time),
            0x126BBDAA.try_into().unwrap(),
            frame_data,
        );
        driver.push(frame);
        // When the last frame is accepted, it has timed out and the whole transfer gets discarded.
        clock.set_ticks(frame_time);
        let maybe_transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
        assert!(maybe_transfer.is_none());
    }
}
#[test]
#[cfg(feature = "can-fd")]
fn test_array() {
    let mut driver = StubDriver::default();
    let clock = ClockOwner::default();
    let mut rx = CanReceiver::new(0u8.try_into().unwrap());

    let subject = SubjectId::try_from(4919).unwrap();
    rx.subscribe_message(subject, 94, duration(1), &mut driver)
        .unwrap();

    let expected = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: instant(0),
            transfer_id: 0.try_into().unwrap(),
            priority: Priority::Nominal,
            subject,
            source: Some(59u8.try_into().unwrap()),
        }),
        loopback: false,
        payload: [
            0x00, 0xb8, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
            0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35,
            0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43,
            0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51,
            0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, // Payload as sent
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, // 14 bytes of padding
        ]
        .to_vec(),
    };

    let frames: [&[u8]; 2] = [
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

    for (i, &frame_data) in frames.iter().enumerate() {
        let frame = Frame::new(
            instant(i as u32),
            0x1013373b.try_into().unwrap(),
            frame_data,
        );
        driver.push(frame);
        clock.set_ticks(i as u32);
        if i != frames.len() - 1 {
            let maybe_transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
            assert!(maybe_transfer.is_none());
        } else {
            // End of transfer
            let transfer = rx
                .receive(&mut clock.make_clock(), &mut driver)
                .unwrap()
                .expect("Didn't get a transfer");

            assert_eq!(expected, transfer);
        }
    }
}

#[test]
fn test_multi_frame_anonymous() {
    // Multi-frame anonymous transfers must be ignored
    let mut driver = StubDriver::default();
    let mut receiver = CanReceiver::new(CanNodeId::try_from(3u8).unwrap());
    let subject_id = SubjectId::try_from(10).unwrap();
    receiver
        .subscribe_message(
            subject_id,
            8,
            MicrosecondDuration32::from_ticks(100),
            &mut driver,
        )
        .unwrap();
    // A non-anonymous 2-frame transfer works
    let non_anonymous_id: CanId = 0b100_0_0_011_0000000001010_0_1000000_u32
        .try_into()
        .unwrap();
    let frames: [&[u8]; 2] = [
        &[
            0x1,
            0x2,
            0x3,
            0x4,
            0x5,
            0x6,
            0x7,
            /* tail */ 0b101_00000,
        ],
        &[0x8, /* CRC */ 0x47, 0x92, /* tail */ 0b010_00000],
    ];
    // When the frames are not anonymous, the transfer gets received.
    let non_anonymous_transfer = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: instant(1),
            transfer_id: 0.try_into().unwrap(),
            priority: Priority::Nominal,
            subject: subject_id,
            source: Some(64u8.try_into().unwrap()),
        }),
        loopback: false,
        payload: vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8],
    };

    let clock = ClockOwner::default();
    clock.set_ticks(1);
    driver.push(Frame::new(
        TestInstant::from_ticks(1),
        non_anonymous_id,
        frames[0],
    ));
    assert_eq!(
        Ok(None),
        receiver.receive(&mut clock.make_clock(), &mut driver)
    );
    clock.set_ticks(2);
    driver.push(Frame::new(
        TestInstant::from_ticks(2),
        non_anonymous_id,
        frames[1],
    ));
    assert_eq!(
        Ok(Some(non_anonymous_transfer)),
        receiver.receive(&mut clock.make_clock(), &mut driver)
    );
    // Now make it anonymous
    let anonymous_id: CanId = 0b100_0_1_011_0000000001010_0_1000000_u32
        .try_into()
        .unwrap();
    clock.set_ticks(1);
    driver.push(Frame::new(
        TestInstant::from_ticks(1),
        anonymous_id,
        frames[0],
    ));
    assert_eq!(
        Ok(None),
        receiver.receive(&mut clock.make_clock(), &mut driver)
    );
    clock.set_ticks(2);
    driver.push(Frame::new(
        TestInstant::from_ticks(2),
        anonymous_id,
        frames[1],
    ));
    assert_eq!(
        Ok(None),
        receiver.receive(&mut clock.make_clock(), &mut driver)
    );
}

/// Tests an anonymous receiver receiving a multi-frame non-anonymous transfer
#[test]
fn test_anonymous_receive_multi_frame() {
    let mut driver = StubDriver::default();
    let mut rx = CanReceiver::new_anonymous();
    rx.subscribe_message(
        8166.try_into().unwrap(),
        9,
        duration(1_000_000),
        &mut driver,
    )
    .unwrap();

    let message_id = CanId::try_from(0x107fe67e).unwrap();
    let frames = [
        Frame::new(
            instant(10),
            message_id,
            &[190, 159, 33, 213, 34, 64, 1, 174],
        ),
        Frame::new(instant(120), message_id, &[103, 0, 143, 70, 78]),
    ];
    let expected_transfer = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: instant(10),
            transfer_id: 14.try_into().unwrap(),
            priority: Priority::Nominal,
            subject: 8166.try_into().unwrap(),
            source: Some(126u8.try_into().unwrap()),
        }),
        loopback: false,
        payload: vec![190, 159, 33, 213, 34, 64, 1, 103, 0],
    };

    let clock = ClockOwner::default();
    clock.set_ticks(frames[0].timestamp().ticks());
    driver.push(frames[0].clone());
    assert_eq!(Ok(None), rx.receive(&mut clock.make_clock(), &mut driver));
    clock.set_ticks(frames[1].timestamp().ticks());
    driver.push(frames[1].clone());
    assert_eq!(
        Ok(Some(expected_transfer)),
        rx.receive(&mut clock.make_clock(), &mut driver)
    );
}

#[test]
fn test_ignore_request_to_other_node() {
    let mut driver = StubDriver::default();
    let mut rx = CanReceiver::new(43u8.try_into().unwrap());

    let service = ServiceId::try_from(430).unwrap();
    rx.subscribe_request(service, 0, duration(0), &mut driver)
        .unwrap();
    // This transfer is going to node 42.
    driver.push(Frame::new(
        instant(302),
        0x136b957b.try_into().unwrap(),
        &[0xe1],
    ));
    let clock = ClockOwner::default();
    clock.set_ticks(302);
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();

    assert_eq!(transfer, None);
}

#[test]
fn test_message_payload_too_large_single_frame() {
    let mut driver = StubDriver::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(120u8.try_into().unwrap());
    let subject = SubjectId::try_from(39).unwrap();
    rx.subscribe_message(subject, 4, duration(0), &mut driver)
        .unwrap();
    driver.push(Frame::new(
        instant(13309),
        0b1000_0011_0000000100111_01001001.try_into().unwrap(),
        &[0xab, 0x19, 0x7f, 0x23, 0x03, 0xee, 0xca, 0b111_00011],
    ));
    let clock = ClockOwner::default();
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(13309),
                transfer_id: 3.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(73u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0xab, 0x19, 0x7f, 0x23],
        })
    );
}

#[test]
fn test_message_payload_too_large_multi_frame() {
    let mut driver = StubDriver::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(120u8.try_into().unwrap());
    let subject = SubjectId::try_from(39).unwrap();
    rx.subscribe_message(subject, 8, duration(100), &mut driver)
        .unwrap();
    let frame_id = 0b1000_0011_0000000100111_01001001.try_into().unwrap();
    driver.push(Frame::new(
        instant(13309),
        frame_id,
        &[0xab, 0x19, 0x7f, 0x23, 0x03, 0xee, 0xca, 0b101_00111],
    ));
    driver.push(Frame::new(
        instant(13399),
        frame_id,
        &[
            // Payload
            0xf1,
            0x82,
            // Transfer CRC
            0x2d,
            0x34,
            // Tail byte
            0b010_00111,
        ],
    ));
    let clock = ClockOwner::default();
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(13309),
                transfer_id: 7.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(73u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0xab, 0x19, 0x7f, 0x23, 0x03, 0xee, 0xca, 0xf1],
        })
    );
}

#[test]
fn test_message_payload_too_large_multi_frame_split_crc() {
    let mut driver = StubDriver::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(120u8.try_into().unwrap());
    let subject = SubjectId::try_from(39).unwrap();
    rx.subscribe_message(subject, 4, duration(100), &mut driver)
        .unwrap();
    let frame_id = 0b1000_0011_0000000100111_01001001.try_into().unwrap();
    driver.push(Frame::new(
        instant(13309),
        frame_id,
        &[
            0xab,
            0x19,
            0x7f,
            0x23,
            0x03,
            0xee,
            0x30,
            // Tail byte
            0b101_00111,
        ],
    ));
    driver.push(Frame::new(
        instant(13319),
        frame_id,
        &[
            0x41,
            0x49,
            0x9c,
            0xa4,
            0xfe,
            0xff,
            // First byte of transfer CRC
            0x29,
            // Tail byte
            0b000_00111,
        ],
    ));
    driver.push(Frame::new(
        instant(13399),
        frame_id,
        &[
            // Second byte of transfer CRC
            0x5f,
            // Tail byte
            0b011_00111,
        ],
    ));
    let clock = ClockOwner::default();
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(13309),
                transfer_id: 7.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(73u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0xab, 0x19, 0x7f, 0x23],
        })
    );
}

/// A driver that reads from a queue of frames
///
/// This does not keep the frames in order by priority, but it is correct as long as it is used for
/// only one transfer at a time.
#[derive(Default)]
struct StubDriver {
    frames: VecDeque<Frame>,
}

impl StubDriver {
    fn push(&mut self, frame: Frame) {
        self.frames.push_back(frame)
    }
}

impl ReceiveDriver<StubClock<'_>> for StubDriver {
    type Error = ();

    fn receive(&mut self, _clock: &mut StubClock<'_>) -> nb::Result<Frame, Self::Error> {
        self.frames.pop_front().ok_or(nb::Error::WouldBlock)
    }

    fn apply_filters<S>(&mut self, _local_node: Option<CanNodeId>, _subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>,
    {
        // Nothing to do
    }

    fn apply_accept_all(&mut self) {
        // Nothing to do
    }
}

struct StubClock<'a> {
    count: &'a Cell<u32>,
}
impl Clock for StubClock<'_> {
    fn now(&mut self) -> Microseconds32 {
        Microseconds32::from_ticks(self.count.get())
    }
}
#[derive(Default)]
struct ClockOwner {
    count: Cell<u32>,
}

impl ClockOwner {
    pub fn set_ticks(&self, count: u32) {
        self.count.set(count)
    }
    pub fn make_clock(&self) -> StubClock<'_> {
        StubClock { count: &self.count }
    }
}
