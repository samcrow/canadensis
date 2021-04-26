//!
//! Receiver integration tests
//!

extern crate canadensis_can;
extern crate canadensis_core;

use core::convert::{TryFrom, TryInto};

use canadensis_can::{Frame, OutOfMemoryError, Receiver};
use canadensis_core::time::{Instant, PrimitiveInstant};
use canadensis_core::transfer::*;
use canadensis_core::{Priority, ServiceId, SubjectId};

type TestInstant = PrimitiveInstant<u16>;
type TestDuration = <TestInstant as Instant>::Duration;

fn instant(ticks: u16) -> TestInstant {
    TestInstant::new(ticks)
}
fn duration(ticks: u16) -> TestDuration {
    TestDuration::new(ticks)
}

#[test]
fn test_heartbeat() -> Result<(), OutOfMemoryError> {
    let mut rx = Receiver::new(0.try_into().unwrap());

    let heartbeat_subject = SubjectId::try_from(7509).unwrap();
    rx.subscribe_message(heartbeat_subject, 7, duration(0))?;

    let transfer = rx
        .accept(Frame::new(
            instant(42),
            0x107d552a.try_into().unwrap(),
            &[0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68, 0xe0],
        ))?
        .expect("Didn't get a transfer");

    let expected = Transfer {
        timestamp: instant(42),
        header: TransferHeader {
            source: 42.try_into().unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject: heartbeat_subject,
            }),
        },
        transfer_id: 0.try_into().unwrap(),
        payload: vec![0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68],
    };
    assert_eq!(expected, transfer);

    Ok(())
}
#[test]
fn test_string() -> Result<(), OutOfMemoryError> {
    let mut rx = Receiver::new(0.try_into().unwrap());

    let string_subject = SubjectId::try_from(4919).unwrap();
    rx.subscribe_message(string_subject, 16, duration(0))?;

    let transfer = rx
        .accept(Frame::new(
            instant(42),
            0x11133775.try_into().unwrap(),
            b"\x00\x18Hello world!\x00\xe0",
        ))?
        .expect("Didn't get a transfer");

    let expected = Transfer {
        timestamp: instant(42),
        header: TransferHeader {
            // Anonymous pseudo-ID
            source: 0x75.try_into().unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: true,
                subject: string_subject,
            }),
        },
        transfer_id: 0.try_into().unwrap(),
        payload: b"\x00\x18Hello world!\x00".to_vec(),
    };
    assert_eq!(expected, transfer);

    Ok(())
}
#[test]
fn test_node_info_request() -> Result<(), OutOfMemoryError> {
    let mut rx = Receiver::new(42.try_into().unwrap());

    let service = ServiceId::try_from(430).unwrap();
    rx.subscribe_request(service, 0, duration(0))?;

    let transfer = rx
        .accept(Frame::new(
            instant(302),
            0x136b957b.try_into().unwrap(),
            &[0xe1],
        ))?
        .expect("Didn't get a transfer");

    let expected = Transfer {
        timestamp: instant(302),
        header: TransferHeader {
            source: 123.try_into().unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Request(ServiceHeader {
                service,
                destination: 42.try_into().unwrap(),
            }),
        },
        transfer_id: 1.try_into().unwrap(),
        payload: vec![],
    };
    assert_eq!(expected, transfer);

    Ok(())
}
#[test]
fn test_node_info_response() -> Result<(), OutOfMemoryError> {
    let mut rx = Receiver::new(123.try_into().unwrap());

    let service = ServiceId::try_from(430).unwrap();
    rx.subscribe_response(service, 313 * 8, duration(100))?;

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

    let frames_and_times: [(&[u8], u16); 11] = [
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

    for (i, &(frame_data, frame_time)) in frames_and_times.iter().enumerate() {
        let frame = Frame::new(
            instant(frame_time),
            0x126BBDAA.try_into().unwrap(),
            frame_data,
        );
        if i != frames_and_times.len() - 1 {
            let maybe_transfer = rx.accept(frame)?;
            assert!(maybe_transfer.is_none());
        } else {
            // End of transfer
            let transfer = rx.accept(frame)?.expect("Didn't get a transfer");

            let expected = Transfer {
                // Timestamp matches the first frame's timestamp
                timestamp: instant(100),
                header: TransferHeader {
                    source: 42.try_into().unwrap(),
                    priority: Priority::Nominal,
                    kind: TransferKindHeader::Response(ServiceHeader {
                        service,
                        destination: 123.try_into().unwrap(),
                    }),
                },
                transfer_id: 1.try_into().unwrap(),
                payload: payload.to_vec(),
            };
            assert_eq!(expected, transfer);
        }
    }

    Ok(())
}
#[test]
fn test_node_info_response_timeout() -> Result<(), OutOfMemoryError> {
    let mut rx = Receiver::new(123.try_into().unwrap());

    let service = ServiceId::try_from(430).unwrap();
    rx.subscribe_response(service, 313 * 8, duration(100))?;

    let frames_and_times: [(&[u8], u16); 11] = [
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
        // When the last frame is accepted, it has timed out and the whole transfer gets discarded.
        let maybe_transfer = rx.accept(frame)?;
        assert!(maybe_transfer.is_none());
    }

    Ok(())
}
#[test]
fn test_array() -> Result<(), OutOfMemoryError> {
    let mut rx = Receiver::new(0.try_into().unwrap());

    let subject = SubjectId::try_from(4919).unwrap();
    rx.subscribe_message(subject, 1024, duration(1))?;

    let expected = Transfer {
        timestamp: instant(0),
        header: TransferHeader {
            source: 59.try_into().unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject,
            }),
        },
        transfer_id: 0.try_into().unwrap(),
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
            instant(i as u16),
            0x1013373b.try_into().unwrap(),
            frame_data,
        );
        if i != frames.len() - 1 {
            let maybe_transfer = rx.accept(frame)?;
            assert!(maybe_transfer.is_none());
        } else {
            // End of transfer
            let transfer = rx.accept(frame)?.expect("Didn't get a transfer");

            assert_eq!(expected, transfer);
        }
    }

    Ok(())
}
