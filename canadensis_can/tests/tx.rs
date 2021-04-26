//!
//! Transmitter integration tests
//!

extern crate canadensis_can;
extern crate canadensis_core;

use core::convert::TryFrom;

use canadensis_can::{CanId, Frame, Mtu, Transmitter};
use canadensis_core::time::PrimitiveInstant;
use canadensis_core::transfer::*;
use canadensis_core::{NodeId, Priority, ServiceId, SubjectId, TransferId};

fn instant(ticks: u16) -> PrimitiveInstant<u16> {
    PrimitiveInstant::new(ticks)
}

#[test]
fn test_heartbeat() {
    let mut tx = Transmitter::new(Mtu::Can8);
    tx.push(Transfer {
        timestamp: instant(0),
        header: TransferHeader {
            source: NodeId::try_from(42).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject: SubjectId::try_from(7509).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(0).unwrap(),
        payload: &[0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68],
    })
    .unwrap();

    assert_eq!(
        Some(Frame::new(
            instant(0),
            CanId::try_from(0x107d552a).unwrap(),
            &[0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68, 0xe0]
        )),
        tx.pop()
    );
    assert_eq!(None, tx.pop());

    // New transaction ID, new uptime
    tx.push(Transfer {
        timestamp: instant(0),
        header: TransferHeader {
            source: NodeId::try_from(42).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject: SubjectId::try_from(7509).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(1).unwrap(),
        payload: &[0x01, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68],
    })
    .unwrap();

    assert_eq!(
        Some(Frame::new(
            instant(0),
            CanId::try_from(0x107d552a).unwrap(),
            &[0x01, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68, 0xe1]
        )),
        tx.pop()
    );
    assert_eq!(None, tx.pop());
}

#[test]
fn test_string() {
    let mut tx = Transmitter::new(Mtu::CanFd64);
    tx.push(Transfer {
        timestamp: instant(0),
        header: TransferHeader {
            // Anonymous pseudo-ID
            source: NodeId::try_from(0x75).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: true,
                subject: SubjectId::try_from(4919).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(0).unwrap(),
        payload: &[
            0x00, 0x18, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,
        ],
    })
    .unwrap();

    assert_eq!(
        Some(Frame::new(
            instant(0),
            CanId::try_from(0x11733775).unwrap(),
            &[
                0x00, 0x18, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,
                0x00, 0xe0
            ]
        )),
        tx.pop()
    );
    assert_eq!(None, tx.pop());
}

#[test]
fn test_node_info_request() {
    let mut tx = Transmitter::new(Mtu::Can8);
    tx.push(Transfer {
        timestamp: instant(0),
        header: TransferHeader {
            source: NodeId::try_from(123).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Request(ServiceHeader {
                service: ServiceId::try_from(430).unwrap(),
                destination: NodeId::try_from(42).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(1).unwrap(),
        payload: &[],
    })
    .unwrap();

    assert_eq!(
        Some(Frame::new(
            instant(0),
            CanId::try_from(0x136b957b).unwrap(),
            &[0xe1]
        )),
        tx.pop()
    );
    assert_eq!(None, tx.pop());
}

#[test]
fn test_node_info_response() {
    let mut tx = Transmitter::new(Mtu::Can8);
    tx.push(Transfer {
        timestamp: instant(0),
        header: TransferHeader {
            source: NodeId::try_from(42).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Response(ServiceHeader {
                service: ServiceId::try_from(430).unwrap(),
                destination: NodeId::try_from(123).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(1).unwrap(),
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
    })
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
        assert_eq!(Some(expected_frame), tx.pop());
    }
    assert_eq!(None, tx.pop());
}

#[test]
fn test_array() {
    let mut tx = Transmitter::new(Mtu::CanFd64);
    tx.push(Transfer {
        timestamp: instant(0),
        header: TransferHeader {
            source: NodeId::try_from(59).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject: SubjectId::try_from(4919).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(0).unwrap(),
        payload: &[
            0x00, 0xb8, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
            0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
            0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35,
            0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43,
            0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51,
            0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b,
        ][..],
    })
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
        assert_eq!(Some(expected_frame), tx.pop());
    }
    assert_eq!(None, tx.pop());
}
