use core::mem;

use crate::data::*;
use alloc::vec::Vec;
use fallible_collections::{FallibleVec, TryReserveError};

/// Toggle is set to 1 for the first frame in a transfer
const TOGGLE_INIT: bool = true;

/// Breaks a transfer payload into frames
///
/// This does not account for CAN FD data length granularity limitations. It does not insert
/// any padding before the tail byte.
///
/// Before a payload is processed through a Breakdown, padding bytes and a transfer CRC should
/// be added as necessary.
pub struct Breakdown {
    /// Maximum transmission unit of the transport (this includes space for the tail byte)
    mtu: usize,
    /// The ID of this transfer
    transfer_id: TransferId,
    /// If the current frame is the first frame in the transfer
    start: bool,
    /// Toggle bit to assign to the next frame
    toggle: bool,
    /// The current frame
    ///
    /// Invariant: Between calls to add, the length of this value is less than `self.mtu`.
    frame: Vec<u8>,
}

impl Breakdown {
    pub fn new(mtu: usize, transfer_id: TransferId) -> Self {
        Breakdown {
            mtu,
            transfer_id,
            start: true,
            toggle: TOGGLE_INIT,
            frame: Vec::new(),
        }
    }

    /// Processes a byte
    ///
    /// If this operation exhausts the available memory, this function returns an error.
    ///
    /// If this byte fills up a frame, the frame is returned.
    pub fn add(&mut self, byte: u8) -> Result<Option<Vec<u8>>, TryReserveError> {
        // If the length of self.frame is equal to self.mtu - 1, we have a new byte that will need
        // to go into the next frame.
        // Add a tail byte to the current frame in preparation for returning it
        let ret_frame: Option<Vec<u8>> = if self.frame.len() == self.mtu - 1 {
            FallibleVec::try_reserve(&mut self.frame, 1)?;
            self.frame.push(make_tail_byte(
                self.start,
                false,
                self.toggle,
                self.transfer_id,
            ));
            self.start = false;
            self.toggle = !self.toggle;

            self.frame.shrink_to_fit();
            // Take out self.frame to return it
            Some(mem::replace(&mut self.frame, Vec::new()))
        } else {
            None
        };
        // Now we have either a new frame that's 0 bytes long, or a frame with one or more
        // bytes added but space for at least one byte before the tail byte.
        FallibleVec::try_push(&mut self.frame, byte)?;

        Ok(ret_frame)
    }

    /// Finishes this breakdown and returns the last frame
    pub fn finish(mut self) -> Result<Vec<u8>, TryReserveError> {
        // Add a tail byte to whatever bytes are in the current frame
        FallibleVec::try_reserve(&mut self.frame, 1)?;
        self.frame.push(make_tail_byte(
            self.start,
            true,
            self.toggle,
            self.transfer_id,
        ));
        self.frame.shrink_to_fit();
        Ok(self.frame)
    }
}

/// Encodes a tail byte
fn make_tail_byte(start: bool, end: bool, toggle: bool, transfer_id: TransferId) -> u8 {
    ((start as u8) << 7) | ((end as u8) << 6) | ((toggle as u8) << 5) | u8::from(transfer_id)
}

#[cfg(test)]
mod test {
    use super::*;
    use core::convert::TryFrom;

    #[test]
    fn test_breakdown_heartbeat() {
        // Heartbeat example from specification section 4.2.3
        for transfer_id in 0u8..=31 {
            let mut breakdown = Breakdown::new(8, TransferId::try_from(transfer_id).unwrap());
            let payload = make_heartbeat_payload(u32::from(transfer_id));

            // Expect a frame with 7 bytes of payload and a tail byte with first 1, last 1,
            // toggle 1, and the correct transfer ID
            let expected_frame: [u8; 8] = [
                payload[0],
                payload[1],
                payload[2],
                payload[3],
                payload[4],
                payload[5],
                payload[6],
                0xe0 | transfer_id,
            ];

            for byte in payload.iter() {
                assert_eq!(Ok(None), breakdown.add(*byte));
            }
            assert_eq!(Ok(expected_frame.to_vec()), breakdown.finish());
        }

        fn make_heartbeat_payload(uptime: u32) -> [u8; 7] {
            [
                // 4 bytes of uptime
                uptime as u8,
                (uptime >> 8) as u8,
                (uptime >> 16) as u8,
                (uptime >> 24) as u8,
                // Health nominal, mode operational, vendor-specific code 3471
                0x04,
                0x78,
                0x68,
            ]
        }
    }

    #[test]
    fn test_string() {
        // String example from specification section 4.2.3
        for transfer_id in 0u8..=3 {
            let payload: [u8; 15] = [
                0x00, 0x18, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,
                0x00, // One byte of padding to make the frame 16 bytes
            ];
            let frame = make_frame(&payload, transfer_id);

            let mut breakdown = Breakdown::new(64, TransferId::try_from(transfer_id).unwrap());

            // Put in the payload bytes
            for payload_byte in payload.iter() {
                assert_eq!(Ok(None), breakdown.add(*payload_byte));
            }
            // Finish, get the whole frame
            assert_eq!(Ok(frame.to_vec()), breakdown.finish());
        }

        fn make_frame(payload: &[u8; 15], transfer_id: u8) -> [u8; 16] {
            let mut frame = [0u8; 16];
            // Copy in the first 15 bytes
            for (frame_byte, payload_byte) in frame.iter_mut().zip(payload.iter()).take(15) {
                *frame_byte = *payload_byte;
            }
            // Tail byte
            frame[15] = 0xe0 | transfer_id;
            frame
        }
    }

    #[test]
    fn test_node_info_request() {
        let breakdown = Breakdown::new(8, TransferId::try_from(1).unwrap());
        assert_eq!(Ok([0xe1].to_vec()), breakdown.finish());
    }

    #[test]
    fn test_node_info_response() {
        let payload: [u8; 71] = [
            0x01, 0x00, // Protocol version
            0x00, 0x00, // Hardware version
            0x01, 0x00, // Software version
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // VCS revision ID
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, // Unique ID
            0x24, // String length prefix
            b'o', b'r', b'g', b'.', b'u', b'a', b'v', b'c', b'a', b'n', b'.', b'p', b'y', b'u',
            b'a', b'v', b'c', b'a', b'n', b'.', b'd', b'e', b'm', b'o', b'.', b'b', b'a', b's',
            b'i', b'c', b'_', b'u', b's', b'a', b'g',
            b'e', // org.uavcan.pyuavcan.demo.basic_usage
            0x00, // Software image CRC length
            0x00, // Certificate of authenticity length
            0x9a, 0xe7, // Transfer CRC
        ];

        let expected_frames: [&[u8]; 11] = [
            &[0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xa1],
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21],
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
            b"\x00\x00\x24org.\x21",
            b"uavcan.\x01",
            b"pyuavca\x21",
            b"n.demo.\x01",
            b"basic_u\x21",
            b"sage\x00\x00\x9a\x01",
            &[0xe7, 0x61],
        ];

        let mut breakdown = Breakdown::new(8, TransferId::try_from(1).unwrap());

        let mut frames: Vec<Vec<u8>> = payload
            .iter()
            .flat_map(|&byte| breakdown.add(byte).unwrap())
            .collect();
        frames.push(breakdown.finish().unwrap());

        assert_eq!(expected_frames.len(), frames.len());
        for (expected, actual) in expected_frames.iter().zip(frames.into_iter()) {
            assert_eq!(*expected, &*actual);
        }
    }

    #[test]
    fn test_array() {
        let payload: [u8; 63 + 47] = [
            0x00, 0xb8, // Array length = 92
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29,
            0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
            0x38, 0x39, 0x3a, 0x3b, 0x3c, // 63 payload bytes go into the first frame
            0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a,
            0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
            0x59, 0x5a, 0x5b, // The rest of 0..=91
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, // 14 padding bytes
            0xc0, 0x48, // Transfer CRC
        ];
        let expected_frames: [&[u8]; 2] = [
            &[
                0x00, 0xb8, // Array length = 92
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
                0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29,
                0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
                0x38, 0x39, 0x3a, 0x3b, 0x3c, 0xa0,
            ],
            &[
                0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a,
                0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
                0x59, 0x5a, 0x5b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0xc0, 0x48, 0x40,
            ],
        ];
        let mut breakdown = Breakdown::new(64, TransferId::try_from(0).unwrap());

        let mut frames: Vec<Vec<u8>> = payload
            .iter()
            .flat_map(|&byte| breakdown.add(byte).unwrap())
            .collect();
        frames.push(breakdown.finish().unwrap());

        assert_eq!(expected_frames.len(), frames.len());
        for (expected, actual) in expected_frames.iter().zip(frames.into_iter()) {
            assert_eq!(*expected, &*actual);
        }
    }
}
