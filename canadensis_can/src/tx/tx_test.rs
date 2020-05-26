use super::*;
use crate::transfer::MessageHeader;
use crate::{Microseconds, NodeId, Priority, ServiceId, SubjectId, TransferId};

#[test]
fn test_make_can_id() {
    // Examples from section 4.2.3 of the specification
    // Heartbeat
    check_can_id(
        TransferHeader {
            source: NodeId::try_from(42).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject: SubjectId::try_from(32085).unwrap(),
            }),
        },
        0x107d552a,
    );
    // String primitive
    check_can_id(
        TransferHeader {
            // Anonymous pseudo-ID
            source: NodeId::try_from(0x75).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: true,
                subject: SubjectId::try_from(4919).unwrap(),
            }),
        },
        0x11133775,
    );
    // Node info request
    check_can_id(
        TransferHeader {
            source: NodeId::try_from(123).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Request(ServiceHeader {
                service: ServiceId::try_from(430).unwrap(),
                destination: NodeId::try_from(42).unwrap(),
            }),
        },
        0x136b957b,
    );
    // Node info response
    check_can_id(
        TransferHeader {
            source: NodeId::try_from(42).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Response(ServiceHeader {
                service: ServiceId::try_from(430).unwrap(),
                destination: NodeId::try_from(123).unwrap(),
            }),
        },
        0x126bbdaa,
    );
    // Array message
    check_can_id(
        TransferHeader {
            source: NodeId::try_from(59).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject: SubjectId::try_from(4919).unwrap(),
            }),
        },
        0x1013373b,
    );
}

fn check_can_id(header: TransferHeader, expected_bits: u32) {
    let actual_id = make_can_id(header);
    let expected_id = CanId::try_from(expected_bits).unwrap();
    assert_eq!(actual_id, expected_id)
}

#[test]
fn test_calculate_padding_classic_can() {
    // MTU 8, no padding ever required
    let mtu = 8;
    for payload in 0..=1024 {
        assert_eq!(0, calculate_padding(payload, mtu));
    }
}

#[test]
fn test_calculate_padding_can_fd() {
    let mtu = 64;
    // Just one frame
    // 0 to 7 bytes, everything fits in one frame, no padding
    for payload in 0..=7 {
        assert_eq!(0, calculate_padding(payload, mtu));
    }
    // 8 to 11 bytes, frame size 12 bytes
    assert_eq!(3, calculate_padding(8, mtu));
    assert_eq!(2, calculate_padding(9, mtu));
    assert_eq!(1, calculate_padding(10, mtu));
    assert_eq!(0, calculate_padding(11, mtu));
    // 12 to 15 bytes, frame size 16 bytes
    assert_eq!(3, calculate_padding(12, mtu));
    assert_eq!(2, calculate_padding(13, mtu));
    assert_eq!(1, calculate_padding(14, mtu));
    assert_eq!(0, calculate_padding(15, mtu));
    // 16 to 19 bytes, frame size 20 bytes
    assert_eq!(3, calculate_padding(16, mtu));
    assert_eq!(2, calculate_padding(17, mtu));
    assert_eq!(1, calculate_padding(18, mtu));
    assert_eq!(0, calculate_padding(19, mtu));
    // 20 to 23 bytes, frame size 24 bytes
    assert_eq!(3, calculate_padding(20, mtu));
    assert_eq!(2, calculate_padding(21, mtu));
    assert_eq!(1, calculate_padding(22, mtu));
    assert_eq!(0, calculate_padding(23, mtu));
    // 24 to 31 bytes, frame size 32 bytes
    assert_eq!(7, calculate_padding(24, mtu));
    assert_eq!(6, calculate_padding(25, mtu));
    assert_eq!(5, calculate_padding(26, mtu));
    assert_eq!(4, calculate_padding(27, mtu));
    assert_eq!(3, calculate_padding(28, mtu));
    assert_eq!(2, calculate_padding(29, mtu));
    assert_eq!(1, calculate_padding(30, mtu));
    assert_eq!(0, calculate_padding(31, mtu));
    // 24 to 31 bytes, frame size 32 bytes
    assert_eq!(7, calculate_padding(24, mtu));
    assert_eq!(6, calculate_padding(25, mtu));
    assert_eq!(5, calculate_padding(26, mtu));
    assert_eq!(4, calculate_padding(27, mtu));
    assert_eq!(3, calculate_padding(28, mtu));
    assert_eq!(2, calculate_padding(29, mtu));
    assert_eq!(1, calculate_padding(30, mtu));
    assert_eq!(0, calculate_padding(31, mtu));
    // 32 to 47 bytes, frame size 48 bytes
    assert_eq!(15, calculate_padding(32, mtu));
    assert_eq!(14, calculate_padding(33, mtu));
    assert_eq!(13, calculate_padding(34, mtu));
    assert_eq!(12, calculate_padding(35, mtu));
    assert_eq!(11, calculate_padding(36, mtu));
    assert_eq!(10, calculate_padding(37, mtu));
    assert_eq!(9, calculate_padding(38, mtu));
    assert_eq!(8, calculate_padding(39, mtu));
    assert_eq!(7, calculate_padding(40, mtu));
    assert_eq!(6, calculate_padding(41, mtu));
    assert_eq!(5, calculate_padding(42, mtu));
    assert_eq!(4, calculate_padding(43, mtu));
    assert_eq!(3, calculate_padding(44, mtu));
    assert_eq!(2, calculate_padding(45, mtu));
    assert_eq!(1, calculate_padding(46, mtu));
    assert_eq!(0, calculate_padding(47, mtu));
    // 48 to 63 bytes, frame size 64 bytes
    assert_eq!(15, calculate_padding(48, mtu));
    assert_eq!(14, calculate_padding(49, mtu));
    assert_eq!(13, calculate_padding(50, mtu));
    assert_eq!(12, calculate_padding(51, mtu));
    assert_eq!(11, calculate_padding(52, mtu));
    assert_eq!(10, calculate_padding(53, mtu));
    assert_eq!(9, calculate_padding(54, mtu));
    assert_eq!(8, calculate_padding(55, mtu));
    assert_eq!(7, calculate_padding(56, mtu));
    assert_eq!(6, calculate_padding(57, mtu));
    assert_eq!(5, calculate_padding(58, mtu));
    assert_eq!(4, calculate_padding(59, mtu));
    assert_eq!(3, calculate_padding(60, mtu));
    assert_eq!(2, calculate_padding(61, mtu));
    assert_eq!(1, calculate_padding(62, mtu));
    assert_eq!(0, calculate_padding(63, mtu));
    // Two frames
    // 64 bytes -> 2 CRC bytes, two tail bytes
    // Last frame has 1 byte of data, 2 CRC, 1 tail
    // 68 bytes -> last frame has 5 bytes of data, 2 CRC, 1 tail
    for payload in 64..=68 {
        // Last frame length 1..=8, no padding
        assert_eq!(0, calculate_padding(payload, mtu));
    }
    // 69 to 72 bytes, last frame size 12 bytes
    assert_eq!(3, calculate_padding(69, mtu));
    assert_eq!(2, calculate_padding(70, mtu));
    assert_eq!(1, calculate_padding(71, mtu));
    assert_eq!(0, calculate_padding(72, mtu));
    // 73 to 76 bytes, last frame size 16 bytes
    assert_eq!(3, calculate_padding(73, mtu));
    assert_eq!(2, calculate_padding(74, mtu));
    assert_eq!(1, calculate_padding(75, mtu));
    assert_eq!(0, calculate_padding(76, mtu));
    // 77 to 80 bytes, last frame size 20 bytes
    assert_eq!(3, calculate_padding(77, mtu));
    assert_eq!(2, calculate_padding(78, mtu));
    assert_eq!(1, calculate_padding(79, mtu));
    assert_eq!(0, calculate_padding(80, mtu));
    // 81 to 84 bytes, last frame size 24 bytes
    assert_eq!(3, calculate_padding(81, mtu));
    assert_eq!(2, calculate_padding(82, mtu));
    assert_eq!(1, calculate_padding(83, mtu));
    assert_eq!(0, calculate_padding(84, mtu));
    // 85 to 92 bytes, frame size 32 bytes
    assert_eq!(7, calculate_padding(85, mtu));
    assert_eq!(6, calculate_padding(86, mtu));
    assert_eq!(5, calculate_padding(87, mtu));
    assert_eq!(4, calculate_padding(88, mtu));
    assert_eq!(3, calculate_padding(89, mtu));
    assert_eq!(2, calculate_padding(90, mtu));
    assert_eq!(1, calculate_padding(91, mtu));
    assert_eq!(0, calculate_padding(92, mtu));
    // 93 to 108 bytes, frame size 48 bytes
    assert_eq!(15, calculate_padding(93, mtu));
    assert_eq!(14, calculate_padding(94, mtu));
    assert_eq!(13, calculate_padding(95, mtu));
    assert_eq!(12, calculate_padding(96, mtu));
    assert_eq!(11, calculate_padding(97, mtu));
    assert_eq!(10, calculate_padding(98, mtu));
    assert_eq!(9, calculate_padding(99, mtu));
    assert_eq!(8, calculate_padding(100, mtu));
    assert_eq!(7, calculate_padding(101, mtu));
    assert_eq!(6, calculate_padding(102, mtu));
    assert_eq!(5, calculate_padding(103, mtu));
    assert_eq!(4, calculate_padding(104, mtu));
    assert_eq!(3, calculate_padding(105, mtu));
    assert_eq!(2, calculate_padding(106, mtu));
    assert_eq!(1, calculate_padding(107, mtu));
    assert_eq!(0, calculate_padding(108, mtu));
    // 109 to 124 bytes, frame size 64 bytes
    assert_eq!(15, calculate_padding(109, mtu));
    assert_eq!(14, calculate_padding(110, mtu));
    assert_eq!(13, calculate_padding(111, mtu));
    assert_eq!(12, calculate_padding(112, mtu));
    assert_eq!(11, calculate_padding(113, mtu));
    assert_eq!(10, calculate_padding(114, mtu));
    assert_eq!(9, calculate_padding(115, mtu));
    assert_eq!(8, calculate_padding(116, mtu));
    assert_eq!(7, calculate_padding(117, mtu));
    assert_eq!(6, calculate_padding(118, mtu));
    assert_eq!(5, calculate_padding(119, mtu));
    assert_eq!(4, calculate_padding(120, mtu));
    assert_eq!(3, calculate_padding(121, mtu));
    assert_eq!(2, calculate_padding(122, mtu));
    assert_eq!(1, calculate_padding(123, mtu));
    assert_eq!(0, calculate_padding(124, mtu));
}

#[test]
fn test_end_to_end_heartbeat() {
    let mut tx = Transmitter::new(Mtu::Can8);
    tx.push(Transfer {
        timestamp: Microseconds(0),
        header: TransferHeader {
            source: NodeId::try_from(42).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject: SubjectId::try_from(32085).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(0).unwrap(),
        payload: vec![0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68],
    })
    .unwrap();

    assert_eq!(
        Some(Frame {
            timestamp: Microseconds(0),
            can_id: CanId::try_from(0x107d552a).unwrap(),
            payload: vec![0x00, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68, 0xe0]
        }),
        tx.pop()
    );
    assert_eq!(None, tx.pop());

    // New transaction ID, new uptime
    tx.push(Transfer {
        timestamp: Microseconds(0),
        header: TransferHeader {
            source: NodeId::try_from(42).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject: SubjectId::try_from(32085).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(1).unwrap(),
        payload: vec![0x01, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68],
    })
    .unwrap();

    assert_eq!(
        Some(Frame {
            timestamp: Microseconds(0),
            can_id: CanId::try_from(0x107d552a).unwrap(),
            payload: vec![0x01, 0x00, 0x00, 0x00, 0x04, 0x78, 0x68, 0xe1]
        }),
        tx.pop()
    );
    assert_eq!(None, tx.pop());
}

#[test]
fn test_end_to_end_string() {
    let mut tx = Transmitter::new(Mtu::CanFd64);
    tx.push(Transfer {
        timestamp: Microseconds(0),
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
        payload: vec![
            0x00, 0x18, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,
        ],
    })
    .unwrap();

    assert_eq!(
        Some(Frame {
            timestamp: Microseconds(0),
            can_id: CanId::try_from(0x11133775).unwrap(),
            payload: vec![
                0x00, 0x18, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,
                0x00, 0xe0
            ]
        }),
        tx.pop()
    );
    assert_eq!(None, tx.pop());
}

#[test]
fn test_end_to_end_node_info_request() {
    let mut tx = Transmitter::new(Mtu::Can8);
    tx.push(Transfer {
        timestamp: Microseconds(0),
        header: TransferHeader {
            source: NodeId::try_from(123).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Request(ServiceHeader {
                service: ServiceId::try_from(430).unwrap(),
                destination: NodeId::try_from(42).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(1).unwrap(),
        payload: vec![],
    })
    .unwrap();

    assert_eq!(
        Some(Frame {
            timestamp: Microseconds(0),
            can_id: CanId::try_from(0x136b957b).unwrap(),
            payload: vec![0xe1]
        }),
        tx.pop()
    );
    assert_eq!(None, tx.pop());
}

#[test]
fn test_end_to_end_node_info_response() {
    let mut tx = Transmitter::new(Mtu::Can8);
    tx.push(Transfer {
        timestamp: Microseconds(0),
        header: TransferHeader {
            source: NodeId::try_from(42).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Response(ServiceHeader {
                service: ServiceId::try_from(430).unwrap(),
                destination: NodeId::try_from(123).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(1).unwrap(),
        payload: b"\x01\x00\x00\x00\x01\x00\x00\
                    \x00\x00\x00\x00\x00\x00\x00\
                    \x00\x00\x00\x00\x00\x00\x00\
                    \x00\x00\x00\x00\x00\x00\x00\
                    \x00\x00\x24org.\
                    uavcan.\
                    pyuavca\
                    n.demo.\
                    basic_u\
                    sage\x00\x00"
            .to_vec(),
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
        let expected_frame = Frame {
            timestamp: Microseconds(0),
            can_id: expected_can_id,
            payload: expected_data.to_vec(),
        };
        assert_eq!(Some(expected_frame), tx.pop());
    }
    assert_eq!(None, tx.pop());
}

#[test]
fn test_end_to_end_array() {
    let mut tx = Transmitter::new(Mtu::CanFd64);
    tx.push(Transfer {
        timestamp: Microseconds(0),
        header: TransferHeader {
            source: NodeId::try_from(59).unwrap(),
            priority: Priority::Nominal,
            kind: TransferKindHeader::Message(MessageHeader {
                anonymous: false,
                subject: SubjectId::try_from(4919).unwrap(),
            }),
        },
        transfer_id: TransferId::try_from(0).unwrap(),
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
    })
    .unwrap();

    let expected_can_id = CanId::try_from(0x1013373b).unwrap();
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
        let expected_frame = Frame {
            timestamp: Microseconds(0),
            can_id: expected_can_id,
            payload: expected_data.to_vec(),
        };
        assert_eq!(Some(expected_frame), tx.pop());
    }
    assert_eq!(None, tx.pop());
}
