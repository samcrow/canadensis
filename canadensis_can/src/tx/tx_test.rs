use crate::types::{CanNodeId, CanTransferId, Header};
use canadensis_core::transfer::MessageHeader;
use canadensis_core::{Priority, ServiceId, SubjectId};

use crate::{calculate_frame_stats, FrameStats};

use super::*;

fn calculate_padding(payload_length: usize, mtu: usize) -> usize {
    calculate_frame_stats(payload_length, mtu).last_frame_padding
}

#[test]
fn test_make_can_id() {
    // Examples from section 4.2.3 of the specification
    // Heartbeat
    check_can_id(
        Header::Message(MessageHeader {
            timestamp: (),
            transfer_id: CanTransferId::try_from(0).unwrap(),
            priority: Priority::Nominal,
            subject: SubjectId::try_from(7509).unwrap(),
            source: Some(CanNodeId::try_from(42u8).unwrap()),
        }),
        &[],
        0x107d552a,
    );
    // String primitive
    check_can_id(
        Header::Message(MessageHeader {
            timestamp: (),
            transfer_id: CanTransferId::try_from(0).unwrap(),
            priority: Priority::Nominal,
            subject: SubjectId::try_from(4919).unwrap(),
            source: None,
        }),
        // This payload will result in an anonymous pseudo-ID of 0x75. The pseudo-ID generation
        // method is really an implementation detail.
        &[0x20],
        0x11733775,
    );
    // Node info request
    check_can_id(
        Header::Request(ServiceHeader {
            timestamp: (),
            transfer_id: Default::default(),
            priority: Priority::Nominal,
            service: ServiceId::try_from(430).unwrap(),
            source: CanNodeId::try_from(123u8).unwrap(),
            destination: CanNodeId::try_from(42u8).unwrap(),
        }),
        &[],
        0x136b957b,
    );
    // Node info response
    check_can_id(
        Header::Response(ServiceHeader {
            timestamp: (),
            transfer_id: Default::default(),
            priority: Priority::Nominal,
            service: ServiceId::try_from(430).unwrap(),
            source: CanNodeId::try_from(42u8).unwrap(),
            destination: CanNodeId::try_from(123u8).unwrap(),
        }),
        &[],
        0x126bbdaa,
    );
    // Array message
    check_can_id(
        Header::Message(MessageHeader {
            timestamp: (),
            transfer_id: Default::default(),
            priority: Priority::Nominal,
            subject: SubjectId::try_from(4919).unwrap(),
            source: Some(CanNodeId::try_from(59u8).unwrap()),
        }),
        &[],
        0x1073373b,
    );
}

fn check_can_id<I>(header: Header<I>, payload: &[u8], expected_bits: u32) {
    let actual_id = make_can_id(&header, payload);
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
fn test_frame_stats_classic_can() {
    let mtu = 8;
    for length in 0..8 {
        // 1 tail byte, up to 7 data bytes
        assert_eq!(
            FrameStats {
                frames: 1,
                last_frame_padding: 0
            },
            calculate_frame_stats(length, mtu)
        );
    }
    for length in 8..13 {
        // 2 tail bytes, 2 CRC bytes, up to 12 data bytes
        assert_eq!(
            FrameStats {
                frames: 2,
                last_frame_padding: 0
            },
            calculate_frame_stats(length, mtu)
        );
    }
    for length in 13..20 {
        // 3 tail bytes, 2 CRC bytes, up to 19 data bytes
        assert_eq!(
            FrameStats {
                frames: 3,
                last_frame_padding: 0
            },
            calculate_frame_stats(length, mtu)
        );
    }
    for length in 20..27 {
        // 4 tail bytes, 2 CRC bytes, up to 26 data bytes
        assert_eq!(
            FrameStats {
                frames: 4,
                last_frame_padding: 0
            },
            calculate_frame_stats(length, mtu)
        );
    }
}

#[test]
fn test_frame_stats_can_fd() {
    let mtu = 64;
    // Part 1: Transfers fit into one frame (up to 64 bytes, possibly with padding)
    for length in 0..8 {
        // 1 tail byte, up to 63 data bytes
        assert_eq!(
            FrameStats {
                frames: 1,
                last_frame_padding: 0
            },
            calculate_frame_stats(length, mtu)
        );
    }
    for length in 8..12 {
        // 1 tail byte, up to 63 data bytes
        assert_eq!(
            FrameStats {
                frames: 1,
                last_frame_padding: 11 - length
            },
            calculate_frame_stats(length, mtu)
        );
    }
    // ...
    for length in 48..64 {
        // 1 tail byte, up to 63 data bytes
        assert_eq!(
            FrameStats {
                frames: 1,
                last_frame_padding: 63 - length
            },
            calculate_frame_stats(length, mtu)
        );
    }
    // Two frames
    for length in 64..69 {
        // Frame 1: 63 bytes of data, tail byte
        // Frame 2: up to 5 bytes of data, 2 bytes CRC, tail byte
        assert_eq!(
            FrameStats {
                frames: 2,
                last_frame_padding: 0
            },
            calculate_frame_stats(length, mtu)
        );
    }
    for length in 69..73 {
        // Frame 1: 63 bytes of data, tail byte
        // Frame 2: up to 9 bytes of data, 2 bytes CRC, tail byte (padded to 12 bytes)
        assert_eq!(
            FrameStats {
                frames: 2,
                last_frame_padding: 72 - length
            },
            calculate_frame_stats(length, mtu)
        );
    }
}
