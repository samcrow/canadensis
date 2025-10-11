//!
//! Receiver integration tests
//!

extern crate canadensis_can;
extern crate canadensis_core;

use core::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::iter;

use canadensis_can::driver::ReceiveDriver;
use canadensis_can::{CanId, CanNodeId, CanReceiver, Frame, FRAME_CAPACITY};
use canadensis_core::nb;
use canadensis_core::subscription::Subscription;
use canadensis_core::time::{milliseconds, Clock, MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::*;
use canadensis_core::transport::Receiver;
use canadensis_core::{InvalidValue, Priority, ServiceId, SubjectId};

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
fn test_node_info_response_no_timeout() {
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
        // The last frame is 101 ticks after the first frame
        (&[0xe7, 0x61], 201),
    ];

    let (last_frame, frames) = frames_and_times.split_last().unwrap();
    let frame_id = 0x126BBDAA.try_into().unwrap();
    for &(frame_data, frame_time) in frames {
        let frame = Frame::new(instant(frame_time), frame_id, frame_data);
        driver.push(frame);
        clock.set_ticks(frame_time);
        let maybe_transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
        assert!(maybe_transfer.is_none());
    }
    // The last frame completes the transfer even though the time difference between the first
    // and last frames is greater than the transfer-ID timeout.
    let frame = Frame::new(instant(last_frame.1), frame_id, last_frame.0);
    driver.push(frame);
    clock.set_ticks(last_frame.1);
    let maybe_transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    // TODO: Check transfer content
    assert!(maybe_transfer.is_some());
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
            0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b,
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

/// Tests messages with consecutive transfer-IDs (including cyclic ID rollover)
/// Some transfers are invalid; receiver should track transfer-ID appropriately
#[test]
fn test_consecutive_messages() {
    let mut driver = StubDriver::default();
    let clock = ClockOwner::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(127u8.try_into().unwrap());
    let subject = SubjectId::try_from(1337).unwrap();
    rx.subscribe_message(subject, 8, duration(2_000_000), &mut driver)
        .unwrap();
    // Nominal priority, message transfer, non-anonymous
    let frame_id = 0b1000_0011_0010100111001_01001011.try_into().unwrap();
    // Transfer-ID 29
    driver.push(Frame::new(
        instant(13309),
        frame_id,
        &[0x53, 0x4f, 0x4d, 0x45, 0x42, 0x4f, 0x44, 0b101_11101],
    ));
    driver.push(Frame::new(
        instant(13320),
        frame_id,
        &[
            // Payload
            0x59,
            // CRC
            0xcb,
            0xfa,
            // Tail byte
            0b010_11101,
        ],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(13309),
                transfer_id: 29.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x53, 0x4f, 0x4d, 0x45, 0x42, 0x4f, 0x44, 0x59],
        })
    );

    // Transfer-ID 30
    driver.push(Frame::new(
        instant(13331),
        frame_id,
        &[0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0b101_11110],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(transfer, None);
    driver.push(Frame::new(
        instant(13416),
        frame_id,
        &[
            // payload
            0x21,
            // CRC
            0x0f,
            0x99,
            // tail byte
            0b010_11110,
        ],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(13331),
                transfer_id: 30.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0x21],
        })
    );

    // Transfer-ID 31, but the CRC is wrong
    driver.push(Frame::new(
        instant(13450),
        frame_id,
        &[0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0b101_11111],
    ));
    driver.push(Frame::new(
        instant(13478),
        frame_id,
        &[
            // payload
            0x21,
            // CRC
            0x1f,
            0x99,
            // tail byte
            0b010_11111,
        ],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(transfer, None);

    // Should accept another transfer with transfer-ID 31 with the correct CRC
    driver.push(Frame::new(
        instant(13490),
        frame_id,
        &[0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0b101_11111],
    ));
    driver.push(Frame::new(
        instant(13500),
        frame_id,
        &[
            // payload
            0x21,
            // CRC
            0x0f,
            0x99,
            // tail byte
            0b010_11111,
        ],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(13490),
                transfer_id: 31.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0x21],
        })
    );

    // Transfer-ID 0, but missing a start frame
    driver.push(Frame::new(
        instant(13508),
        frame_id,
        &[
            // payload
            0x21,
            // CRC
            0x0f,
            0x99,
            // tail byte
            0b010_00000,
        ],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(transfer, None);

    // Should accept the same transfer sent properly
    driver.push(Frame::new(
        instant(13513),
        frame_id,
        &[0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0b101_00000],
    ));
    driver.push(Frame::new(
        instant(13521),
        frame_id,
        &[
            // payload
            0x21,
            // CRC
            0x0f,
            0x99,
            // tail byte
            0b010_00000,
        ],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(13513),
                transfer_id: 0.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0x21],
        })
    );

    // Should deduplicate within timeout
    driver.push(Frame::new(
        instant(13531),
        frame_id,
        &[0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0b101_00000],
    ));
    driver.push(Frame::new(
        instant(13532),
        frame_id,
        &[
            // payload
            0x21,
            // CRC
            0x0f,
            0x99,
            // tail byte
            0b010_00000,
        ],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(transfer, None);

    // Send first frame of transfer-ID 1, but don't complete transfer
    driver.push(Frame::new(
        instant(13831),
        frame_id,
        &[0x74, 0x30, 0x6c, 0x64, 0x5f, 0xff, 0x00, 0b101_00001],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(transfer, None);
    // Receiver should skip ID 1 and emit next valid transfer with ID 2
    driver.push(Frame::new(
        instant(13862),
        frame_id,
        &[0x74, 0x30, 0x6c, 0x64, 0x5f, 0x00, 0x00, 0b111_00010],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(13862),
                transfer_id: 2.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x74, 0x30, 0x6c, 0x64, 0x5f, 0x00, 0x00],
        })
    );
}

/// Tests that non-consecutive transfer-IDs are still received properly
/// (excluding the case where the transfer-ID is reused within the timeout period),
/// so long as the actual transfers are valid.
/// This accommodates nodes/sessions getting reset, whole transfers being dropped, etc.
#[test]
fn test_confusing_transfer_ids() {
    let mut driver = StubDriver::default();
    let clock = ClockOwner::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(127u8.try_into().unwrap());
    let subject = SubjectId::try_from(1337).unwrap();
    rx.subscribe_message(subject, 4, duration(2_000_000), &mut driver)
        .unwrap();
    // Nominal priority, message transfer, non-anonymous
    let frame_id = 0b1000_0011_0010100111001_01001011.try_into().unwrap();
    // Transfer-ID 3
    driver.push(Frame::new(
        instant(59),
        frame_id,
        &[0x01, 0x02, 0xca, 0xfe, 0x00, 0x00, 0x00, 0b111_00011],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(59),
                transfer_id: 3.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x01, 0x02, 0xca, 0xfe]
        })
    );

    // Transfer-ID 2 (== rolled back 1, or forwards 31)
    driver.push(Frame::new(
        instant(199),
        frame_id,
        &[0x13, 0x47, 0x84, 0x20, 0x00, 0x00, 0x00, 0b111_00010],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(199),
                transfer_id: 2.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x13, 0x47, 0x84, 0x20],
        })
    );

    // Transfer-ID 3.
    // This is within the transfer-ID timeout window for the initial transfer with ID 3,
    // but handling a transfer with ID 2 means that ID 3 is now expected.
    driver.push(Frame::new(
        instant(211),
        frame_id,
        &[0xf1, 0x02, 0x8a, 0xf1, 0x00, 0x00, 0x00, 0b111_00011],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(211),
                transfer_id: 3.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0xf1, 0x02, 0x8a, 0xf1]
        })
    );

    // Transfer-ID 19
    driver.push(Frame::new(
        instant(478),
        frame_id,
        &[0xa3, 0x47, 0x84, 0x20, 0x00, 0x00, 0x00, 0b111_10011],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(478),
                transfer_id: 19.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0xa3, 0x47, 0x84, 0x20],
        })
    );

    // Transfer-ID 0
    driver.push(Frame::new(
        instant(799),
        frame_id,
        &[0xa3, 0xb2, 0xee, 0x20, 0x00, 0x00, 0x00, 0b111_00000],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(799),
                transfer_id: 0.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0xa3, 0xb2, 0xee, 0x20],
        })
    );

    // Transfer-ID 31
    driver.push(Frame::new(
        instant(802),
        frame_id,
        &[0x0f, 0x22, 0x84, 0x2c, 0x00, 0x00, 0x00, 0b111_11111],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(802),
                transfer_id: 31.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x0f, 0x22, 0x84, 0x2c],
        })
    );
}

#[test]
fn test_interleaved_frame_rejection() {
    let mut driver = StubDriver::default();
    let clock = ClockOwner::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(120u8.try_into().unwrap());
    let subject = SubjectId::try_from(39).unwrap();
    rx.subscribe_message(subject, 8, duration(100), &mut driver)
        .unwrap();
    // Nominal priority, message transfer, non-anonymous
    let frame_id = 0b1000_0011_0000000100111_01001011.try_into().unwrap();
    // Frame 1 of 2 of transfer 1
    driver.push(Frame::new(
        instant(13309),
        frame_id,
        // SOF, !EOF, TOGGLE = 1, transfer-ID 1
        &[0x53, 0x4f, 0x4d, 0x45, 0x42, 0x4f, 0x44, 0b101_00001],
    ));
    // Frame 1 of 2 of transfer 2
    driver.push(Frame::new(
        instant(13311),
        frame_id,
        // SOF, !EOF, TOGGLE = 1, transfer-ID 2
        &[0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0b101_00010],
    ));
    // Frame 2 of 2 of transfer 2
    driver.push(Frame::new(
        instant(13316),
        frame_id,
        &[
            // payload
            0x21,
            // CRC
            0x0f, 0x99,
            // tail byte: !SOF, EOF, TOGGLE = 0, transfer-ID 2
            0b010_00010
        ],
    ));

    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(13311),
                transfer_id: 2.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x74, 0x30, 0x6c, 0x64, 0x5f, 0x6d, 0x33, 0x21],
        })
    );

    // Frame 2 of 2 of transfer 1
    driver.push(Frame::new(
        instant(13320),
        frame_id,
        &[
            // Payload
            0x59,
            // Transfer CRC
            0xcb,
            0xfa,
            // Tail byte
            0b010_00001,
        ],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    // Shouldn't reassemble transfer 1 if we've already reassembled transfer 2
    assert_eq!(
        transfer,
        None
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
    rx.subscribe_message(subject, 3, duration(0), &mut driver)
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
            payload: vec![0xab, 0x19, 0x7f],
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

/// Replicates this example:
/// <https://forum.opencyphal.org/t/amendment-to-the-transfer-reception-state-machine-implementations/1870>
#[test]
fn slow_multi_frame_no_timeout() {
    let mut driver = StubDriver::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(120u8.try_into().unwrap());
    let subject = SubjectId::try_from(1100).unwrap();
    // Transfer ID timeout is 2 ms. Some frames are separated by longer than that.
    // The transfer is still valid.
    rx.subscribe_message(subject, 62, milliseconds(2), &mut driver)
        .unwrap();
    let frame_id = 0x10644c7f.try_into().unwrap();
    let frames = [
        Frame::new(
            instant(288644),
            frame_id,
            &[0x09, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1],
        ),
        Frame::new(
            instant(291624),
            frame_id,
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11],
        ),
        Frame::new(
            instant(294662),
            frame_id,
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31],
        ),
        Frame::new(
            instant(297647),
            frame_id,
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11],
        ),
        Frame::new(
            instant(300635),
            frame_id,
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31],
        ),
        Frame::new(
            instant(303616),
            frame_id,
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11],
        ),
        Frame::new(
            instant(306614),
            frame_id,
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31],
        ),
        Frame::new(
            instant(309578),
            frame_id,
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11],
        ),
        Frame::new(
            instant(312569),
            frame_id,
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x31],
        ),
    ];
    for frame in IntoIterator::into_iter(frames) {
        driver.push(frame);
    }
    driver.push(Frame::new(instant(315564), frame_id, &[0x4a, 0x51]));
    let clock = ClockOwner::default();
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: instant(288644),
                transfer_id: 17.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(127u8).unwrap()),
            }),
            loopback: false,
            payload: vec![
                0x09, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, // Frame 0
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Frame 1
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Frame 2
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Frame 3
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Frame 4
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Frame 5
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Frame 6
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Frame 7
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Frame 8
            ],
        })
    );
}

#[test]
fn handle_transfer_id_timeout_with_clock_overflow() {
    let mut driver = StubDriver::default();
    let clock = ClockOwner::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(77u8.try_into().unwrap());
    let subject = SubjectId::try_from(1234).unwrap();
    let timestamp_before_overflow = instant(u32::MAX - 500);
    let timestamp_after_overflow = instant(10);
    let timestamp_after_timeout_window = instant(505);
    rx.subscribe_message(subject, 4, duration(1000), &mut driver)
        .unwrap();
    let frame_id = 0b1000_0011_0010011010010_01001011.try_into().unwrap();

    driver.push(Frame::new(
        timestamp_before_overflow,
        frame_id,
        &[0xff, 0x4f, 0x5a, 0xa5, 0x00, 0x00, 0x00, 0b111_10101],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: timestamp_before_overflow,
                transfer_id: 21.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0xff, 0x4f, 0x5a, 0xa5],
        })
    );

    // Duplicated transfer within timeout window (wrapping clock around)
    driver.push(Frame::new(
        timestamp_after_overflow,
        frame_id,
        &[0xff, 0x4f, 0x5a, 0xa5, 0x00, 0x00, 0x00, 0b111_10101],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(transfer, None);

    // Now it should be accepted
    driver.push(Frame::new(
        timestamp_after_timeout_window,
        frame_id,
        &[0xff, 0x4f, 0x5a, 0xa5, 0x00, 0x00, 0x00, 0b111_10101],
    ));
    let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: timestamp_after_timeout_window,
                transfer_id: 21.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(75u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0xff, 0x4f, 0x5a, 0xa5],
        })
    );
}

#[test]
fn single_frame_deduplicate_basic() {
    single_frame_deduplicate(Microseconds32::from_ticks(1000));
}

#[test]
fn single_frame_deduplicate_with_clock_overflow() {
    // The second frame adds 100 ticks to this value, making it overflow.
    single_frame_deduplicate(Microseconds32::from_ticks(u32::MAX - 50));
}

fn single_frame_deduplicate(start_time: Microseconds32) {
    let mut driver = StubDriver::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(120u8.try_into().unwrap());
    let subject = SubjectId::try_from(1100).unwrap();
    rx.subscribe_message(subject, 62, milliseconds(1000), &mut driver)
        .unwrap();
    let frame_id = 0x10644c7f.try_into().unwrap();
    let clock = ClockOwner::default();

    let first_transfer_time = start_time + MicrosecondDuration32::from_ticks(10);
    let expected_transfer = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: first_transfer_time,
            transfer_id: 27.try_into().unwrap(),
            priority: Priority::Nominal,
            subject,
            source: Some(CanNodeId::try_from(127u8).unwrap()),
        }),
        loopback: false,
        payload: vec![0x09, 0x30],
    };

    driver.push(Frame::new(
        first_transfer_time,
        frame_id,
        &[0x09, 0x30, tail(true, true, true, 27)],
    ));
    assert_eq!(
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap(),
        Some(expected_transfer.clone())
    );
    // Send the same frame again later. The receiver should ignore it.
    driver.push(Frame::new(
        start_time + MicrosecondDuration32::from_ticks(100),
        frame_id,
        &[0x09, 0x30, tail(true, true, true, 27)],
    ));
    assert_eq!(
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap(),
        None
    );
    // The transfer ID timeout has now expired. The receiver should accept a frame with the same
    // transfer ID.
    let second_transfer_time = start_time + MicrosecondDuration32::from_ticks(1_000_000 + 10 + 1);
    driver.push(Frame::new(
        second_transfer_time,
        frame_id,
        &[0x09, 0x30, tail(true, true, true, 27)],
    ));
    assert_eq!(
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap(),
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: second_transfer_time,
                transfer_id: 27.try_into().unwrap(),
                priority: Priority::Nominal,
                subject,
                source: Some(CanNodeId::try_from(127u8).unwrap()),
            }),
            loopback: false,
            payload: vec![0x09, 0x30],
        })
    );
}

#[test]
fn multi_frame_each_frame_duplicated() {
    let mut driver = StubDriver::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(120u8.try_into().unwrap());
    let subject = SubjectId::try_from(8003).unwrap();
    rx.subscribe_message(subject, 12, milliseconds(1000), &mut driver)
        .unwrap();
    let frame_id = 0b101_00011_1111101000011_0_1111000.try_into().unwrap();
    let clock = ClockOwner::default();

    let first_transfer_time = Microseconds32::from_ticks(10);
    let expected_transfer = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: first_transfer_time,
            transfer_id: 3.try_into().unwrap(),
            priority: Priority::Low,
            subject,
            source: Some(CanNodeId::try_from(120u8).unwrap()),
        }),
        loopback: false,
        payload: vec![
            0x30, 0x10, 0x09, 0xff, 0xae, 0x69, 0xa2, 0x01, 0x10, 0x13, 0x22, 0x99,
        ],
    };

    let frames = [
        Frame::new(
            first_transfer_time,
            frame_id,
            &[
                0x30,
                0x10,
                0x09,
                0xff,
                0xae,
                0x69,
                0xa2,
                tail(true, false, true, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + milliseconds(120),
            frame_id,
            &[
                0x01,
                0x10,
                0x13,
                0x22,
                0x99,
                0xaa,
                0xed,
                tail(false, false, false, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + milliseconds(340),
            frame_id,
            &[
                0x00,
                0x20,
                0x41,
                0x44,
                0x7a,
                0x69,
                tail(false, true, true, 3),
            ],
        ),
    ];

    let (last_frame, other_frames) = frames.split_last().unwrap();
    for frame in other_frames {
        clock.set_ticks(frame.timestamp().ticks());
        driver.push(frame.clone());
        driver.push(delay_frame(
            frame.clone(),
            MicrosecondDuration32::from_ticks(10 * 1000),
        ));
        assert_eq!(
            None,
            rx.receive(&mut clock.make_clock(), &mut driver).unwrap()
        );
    }
    clock.set_ticks(last_frame.timestamp().ticks());
    driver.push(last_frame.clone());
    driver.push(delay_frame(
        last_frame.clone(),
        MicrosecondDuration32::from_ticks(10 * 1000),
    ));
    assert_eq!(
        Some(expected_transfer),
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap()
    );
    // No duplicate
    assert_eq!(
        None,
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap()
    );
}

#[test]
fn multi_frame_all_frames_duplicated() {
    let mut driver = StubDriver::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(120u8.try_into().unwrap());
    let subject = SubjectId::try_from(8003).unwrap();
    rx.subscribe_message(subject, 12, milliseconds(1000), &mut driver)
        .unwrap();
    let frame_id = 0b101_00011_1111101000011_0_1111000.try_into().unwrap();
    let clock = ClockOwner::default();

    let first_transfer_time = Microseconds32::from_ticks(10);
    let expected_transfer = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: first_transfer_time,
            transfer_id: 3.try_into().unwrap(),
            priority: Priority::Low,
            subject,
            source: Some(CanNodeId::try_from(120u8).unwrap()),
        }),
        loopback: false,
        payload: vec![
            0x30, 0x10, 0x09, 0xff, 0xae, 0x69, 0xa2, 0x01, 0x10, 0x13, 0x22, 0x99,
        ],
    };

    let frames = [
        Frame::new(
            first_transfer_time,
            frame_id,
            &[
                0x30,
                0x10,
                0x09,
                0xff,
                0xae,
                0x69,
                0xa2,
                tail(true, false, true, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + milliseconds(120),
            frame_id,
            &[
                0x01,
                0x10,
                0x13,
                0x22,
                0x99,
                0xaa,
                0xed,
                tail(false, false, false, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + milliseconds(340),
            frame_id,
            &[
                0x00,
                0x20,
                0x41,
                0x44,
                0x7a,
                0x69,
                tail(false, true, true, 3),
            ],
        ),
    ];

    // Send all the frames once, and then send them all again (within the transfer ID timeout)
    for frame in &frames {
        driver.push(frame.clone());
    }
    for frame in frames {
        driver.push(delay_frame(
            frame.clone(),
            MicrosecondDuration32::from_ticks(220 * 1000),
        ));
    }

    assert_eq!(
        Some(expected_transfer),
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap()
    );
    // No duplicate transfer
    assert_eq!(
        None,
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap()
    );
}

#[test]
fn multi_frame_missed_frame_recovery() {
    let mut driver = StubDriver::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(120u8.try_into().unwrap());
    let subject = SubjectId::try_from(8003).unwrap();
    rx.subscribe_message(subject, 12, milliseconds(1000), &mut driver)
        .unwrap();
    let frame_id = 0b101_00011_1111101000011_0_1111000.try_into().unwrap();
    let clock = ClockOwner::default();

    let first_transfer_time = Microseconds32::from_ticks(10);
    let delay_between_transfers = MicrosecondDuration32::from_ticks(1200 * 1000);

    let frames = [
        Frame::new(
            first_transfer_time,
            frame_id,
            &[
                0x30,
                0xdd,
                0x09,
                0xff,
                0xae,
                0x69,
                0xa2,
                tail(true, false, true, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + milliseconds(120),
            frame_id,
            &[
                0x01,
                0x10,
                0x13,
                0x22,
                0x99,
                0xaa,
                0xed,
                tail(false, false, false, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + milliseconds(340),
            frame_id,
            &[
                0x00,
                0x20,
                0x41,
                0x44,
                0x7a,
                0x69,
                tail(false, true, true, 3),
            ],
        ),
    ];
    // Another transfer later with the same transfer ID but different content
    let second_transfer_frames = [
        Frame::new(
            first_transfer_time + delay_between_transfers,
            frame_id,
            &[
                0x30,
                0x10,
                0x09,
                0xf3,
                0xae,
                0x32,
                0xb6,
                tail(true, false, true, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + delay_between_transfers + milliseconds(120),
            frame_id,
            &[
                0x01,
                0x10,
                0x13,
                0x1f,
                0x99,
                0xaa,
                0xed,
                tail(false, false, false, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + delay_between_transfers + milliseconds(340),
            frame_id,
            &[
                0x00,
                0x20,
                0x41,
                0x44,
                0xd4,
                0x49,
                tail(false, true, true, 3),
            ],
        ),
    ];
    let third_transfer_frames = [
        Frame::new(
            first_transfer_time + delay_between_transfers * 2,
            frame_id,
            &[
                0x29,
                0xab,
                0x09,
                0x3c,
                0xae,
                0x32,
                0xb6,
                tail(true, false, true, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + delay_between_transfers * 2 + milliseconds(120),
            frame_id,
            &[
                0x01,
                0x10,
                0x13,
                0x1f,
                0x99,
                0xaa,
                0xed,
                tail(false, false, false, 3),
            ],
        ),
        Frame::new(
            first_transfer_time + delay_between_transfers * 2 + milliseconds(340),
            frame_id,
            &[
                0x00,
                0x20,
                0x41,
                0x44,
                0xd2,
                0x6c,
                tail(false, true, true, 3),
            ],
        ),
    ];
    let expected_transfer = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: first_transfer_time + delay_between_transfers * 2,
            transfer_id: 3.try_into().unwrap(),
            priority: Priority::Low,
            subject,
            source: Some(CanNodeId::try_from(120u8).unwrap()),
        }),
        loopback: false,
        payload: vec![
            0x29, 0xab, 0x09, 0x3c, 0xae, 0x32, 0xb6, 0x01, 0x10, 0x13, 0x1f, 0x99,
        ],
    };

    // Skip the middle frame
    driver.push(frames[0].clone());
    driver.push(frames[2].clone());
    assert_eq!(
        None,
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap()
    );
    // Later, after the transfer ID timeout, send another transfer
    for frame in second_transfer_frames {
        driver.push(frame);
    }
    // Also lost the second transfer although all three frames arrived
    assert_eq!(
        None,
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap()
    );
    for frame in third_transfer_frames {
        driver.push(frame);
    }
    // Got the third transfer
    assert_eq!(
        Some(expected_transfer),
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap()
    );
    // No duplicate transfer
    assert_eq!(
        None,
        rx.receive(&mut clock.make_clock(), &mut driver).unwrap()
    );
}

#[test]
fn test_stressed_transfer_ids_from_0() {
    test_stressed_transfer_ids_from(0);
}

#[test]
fn test_stressed_transfer_ids_from_31() {
    test_stressed_transfer_ids_from(31);
}

/// Send 1024 valid consecutive message transfers, one every tick.
fn test_stressed_transfer_ids_from(start_tid: u8) {
    let mut driver = StubDriver::default();
    let clock = ClockOwner::default();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(77u8.try_into().unwrap());
    let subject = SubjectId::try_from(1234).unwrap();
    rx.subscribe_message(subject, 4, duration(1000), &mut driver)
        .unwrap();
    let frame_id = 0b1000_0011_0010011010010_01001011.try_into().unwrap();

    let mut tid = start_tid;
    for i in 0..1024 {
        driver.push(Frame::new(
            instant(i),
            frame_id,
            // the tail function masks out last the 5 bits of i for us
            &[
                0xca,
                0xfe,
                0xbe,
                0xef,
                0x00,
                0x00,
                0x00,
                tail(true, true, true, tid),
            ],
        ));
        let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
        assert_eq!(
            transfer,
            Some(Transfer {
                header: Header::Message(MessageHeader {
                    timestamp: instant(i),
                    transfer_id: tid.try_into().unwrap(),
                    priority: Priority::Nominal,
                    subject,
                    source: Some(CanNodeId::try_from(75u8).unwrap()),
                }),
                loopback: false,
                payload: vec![0xca, 0xfe, 0xbe, 0xef],
            })
        );
        tid = (tid + 1) % 32;
    }
}

// Some helpers for the monster stress test.

struct TestCanNodeIds {
    current: u8,
}

struct TestPortIds {
    max_subject: u16,
    max_service: u16,
    current_subject: u16,
    current_service: u16,
}

impl TestCanNodeIds {
    fn new(_seed: usize) -> Self {
        // The seed leaves open the possibility of the iterator returning IDs in pseudo-random order in future testing.
        // An LFSR would be cool since we want to cycle through 127 values...
        TestCanNodeIds { current: 0 }
    }
}

// Prefixed with underscore so as not to confuse with the library PortId definition, which is just
// a wrapper for a u32. Might be sensible to change the library PortId definition...?
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum _PortId {
    SubjectId(SubjectId),
    ServiceId(ServiceId),
}

impl TestPortIds {
    fn new(max_subject: u16, max_service: u16, _seed: usize) -> Self {
        // Seed argument reserved as in TestCanNodeIds.
        if max_subject > u16::from(SubjectId::MAX) {
            panic!("Subject-IDs cannot be greater than {}", SubjectId::MAX);
        }
        if max_service > u16::from(ServiceId::MAX) {
            panic!("Service-IDs cannot be greater than {}", ServiceId::MAX);
        }
        TestPortIds {
            max_subject,
            max_service,
            current_subject: 0,
            current_service: 0,
        }
    }
}

impl iter::Iterator for TestCanNodeIds {
    type Item = CanNodeId;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > u8::from(CanNodeId::MAX) {
            None
        } else {
            let next = CanNodeId::from_truncating(self.current);
            self.current += 1;
            Some(next)
        }
    }
}

impl iter::Iterator for TestPortIds {
    type Item = _PortId;
    fn next(&mut self) -> Option<Self::Item> {
        // Currently just iterates through all subjects in order, then all services
        if self.current_subject > self.max_subject {
            if self.current_service > self.max_service {
                None
            } else {
                let next = _PortId::ServiceId(ServiceId::from_truncating(self.current_service));
                self.current_service += 1;
                Some(next)
            }
        } else {
            let next = _PortId::SubjectId(SubjectId::from_truncating(self.current_subject));
            self.current_subject += 1;
            Some(next)
        }
    }
}

fn make_message_frame_id(
    priority: Priority,
    subject_id: SubjectId,
    source_node_id: CanNodeId,
) -> Result<CanId, InvalidValue> {
    const MESSAGE_BASE: u32 = 0b000_00011_0000000000000_0_0000000;
    (MESSAGE_BASE
        | ((priority as u32) << 26)
        | (u32::from(subject_id) << 8)
        | u32::from(source_node_id))
    .try_into()
}
fn make_request_frame_id(
    priority: Priority,
    service_id: ServiceId,
    destination_node_id: CanNodeId,
    source_node_id: CanNodeId,
) -> Result<CanId, InvalidValue> {
    const REQUEST_BASE: u32 = 0b000_110_000000000_0000000_0000000;
    (REQUEST_BASE
        | ((priority as u32) << 26)
        | (u32::from(service_id) << 14)
        | (u32::from(destination_node_id) << 7)
        | u32::from(source_node_id))
    .try_into()
}

const NON_TAIL_BYTES_PER_FRAME: usize = FRAME_CAPACITY - 1;

fn num_frames_for_payload(payload_size: usize) -> usize {
    if payload_size <= NON_TAIL_BYTES_PER_FRAME {
        1
    } else {
        (payload_size).div_ceil(NON_TAIL_BYTES_PER_FRAME)
    }
}

/// Subscribe to many messages and services. Send frames from all 128 nodes across all messages and
/// services with all transfers being assembled concurrently. Check all transfers were received.
#[test]
fn test_stressed_concurrent_subscriptions() {
    const NUM_SUBJECT_IDS: u16 = 256;
    const NUM_SERVICE_IDS: u16 = 256;
    // payload size includes CRC. it's this big so this test still stresses CAN FD
    const PAYLOAD_SIZE: usize = 74;
    const PAYLOAD: [u8; PAYLOAD_SIZE] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
        0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34,
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
        0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
        0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34,
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
        0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
        // CRC
        0x57, 0x59,
    ];

    let num_frames: usize = num_frames_for_payload(PAYLOAD_SIZE);
    assert!(
        num_frames > 1,
        "Stressed subscriptions test requires its transfers to be multi-frame"
    );

    let mut driver = StubDriver::default();
    let clock = ClockOwner::default();
    let dest_node: CanNodeId = 77u8.try_into().unwrap();
    let mut rx: CanReceiver<StubClock, StubDriver> = CanReceiver::new(dest_node);

    // Subscribe to many subjects and service requests
    for subject_id in 0..NUM_SUBJECT_IDS {
        rx.subscribe_message(
            SubjectId::try_from(subject_id).unwrap(),
            PAYLOAD_SIZE,
            duration(0),
            &mut driver,
        )
        .unwrap();
    }
    for service_id in 0..NUM_SERVICE_IDS {
        rx.subscribe_request(
            ServiceId::try_from(service_id).unwrap(),
            PAYLOAD_SIZE,
            duration(0),
            &mut driver,
        )
        .unwrap();
    }

    // Perform many subject and service request transfers concurrently
    let mut t = 1000;
    let mut expected_transfer_times: HashMap<(CanNodeId, _PortId), TestInstant> = HashMap::new();
    for msg_i in 0..num_frames {
        for src_node in TestCanNodeIds::new(msg_i) {
            for port_id in TestPortIds::new(NUM_SUBJECT_IDS - 1, NUM_SERVICE_IDS - 1, msg_i) {
                // send frame msg_i of a message/request on port_id from node_id
                let frame_id = match port_id {
                    _PortId::SubjectId(subject_id) => {
                        make_message_frame_id(Priority::Nominal, subject_id, src_node).unwrap()
                    }
                    _PortId::ServiceId(service_id) => {
                        make_request_frame_id(Priority::Nominal, service_id, dest_node, src_node)
                            .unwrap()
                    }
                };
                // construct a slice of variable length containing the payload bytes we want
                let num_remaining_bytes = PAYLOAD_SIZE - msg_i * NON_TAIL_BYTES_PER_FRAME;
                let start_i = msg_i * NON_TAIL_BYTES_PER_FRAME;
                let bytes_to_send = if num_remaining_bytes >= NON_TAIL_BYTES_PER_FRAME {
                    &PAYLOAD[start_i..start_i + NON_TAIL_BYTES_PER_FRAME]
                } else {
                    &PAYLOAD[start_i..PAYLOAD_SIZE]
                };

                driver.push(Frame::new(
                    instant(t),
                    frame_id,
                    [
                        bytes_to_send,
                        &[tail(
                            msg_i == 0,
                            msg_i == num_frames - 1,
                            msg_i % 2 == 0,
                            24,
                        )],
                    ]
                    .concat()
                    .as_slice(),
                ));
                let transfer = rx.receive(&mut clock.make_clock(), &mut driver).unwrap();
                if msg_i == 0 {
                    expected_transfer_times.insert((src_node, port_id), instant(t));
                    assert_eq!(transfer, None);
                } else if msg_i == (num_frames - 1) {
                    // last frame of the transfer, so we should actually get something
                    match port_id {
                        _PortId::SubjectId(subject_id) => {
                            assert_eq!(
                                transfer,
                                Some(Transfer {
                                    header: Header::Message(MessageHeader {
                                        timestamp: *expected_transfer_times
                                            .get(&(src_node, port_id))
                                            .unwrap(),
                                        transfer_id: 24.try_into().unwrap(),
                                        priority: Priority::Nominal,
                                        subject: subject_id,
                                        source: Some(src_node),
                                    }),
                                    loopback: false,
                                    // exclude last two CRC bytes
                                    payload: Vec::from(&PAYLOAD[0..PAYLOAD_SIZE - 2]),
                                })
                            );
                        }
                        _PortId::ServiceId(service_id) => {
                            assert_eq!(
                                transfer,
                                Some(Transfer {
                                    header: Header::Request(ServiceHeader {
                                        timestamp: *expected_transfer_times
                                            .get(&(src_node, port_id))
                                            .unwrap(),
                                        transfer_id: 24.try_into().unwrap(),
                                        priority: Priority::Nominal,
                                        service: service_id,
                                        source: src_node,
                                        destination: dest_node,
                                    }),
                                    loopback: false,
                                    payload: Vec::from(&PAYLOAD[0..PAYLOAD_SIZE - 2]),
                                })
                            );
                        }
                    }
                } else {
                    assert_eq!(transfer, None);
                }

                t += 1;
            }
        }
    }
}

fn delay_frame(frame: Frame, delay: MicrosecondDuration32) -> Frame {
    Frame::new(frame.timestamp() + delay, frame.id(), frame.data())
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

/// Creates a tail byte
fn tail(start: bool, end: bool, toggle: bool, transfer: u8) -> u8 {
    ((start as u8) << 7) | ((end as u8) << 6) | ((toggle as u8) << 5) | (transfer & 0x1f)
}
