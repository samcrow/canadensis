use alloc::vec::Vec;
use core::mem;

use canadensis_core::TransferId;
use fallible_collections::{FallibleVec, TryReserveError};

use super::TailByte;
use crate::OutOfMemoryError;

/// Reassembles frames into a transfer
#[derive(Debug)]
pub struct Buildup {
    /// Transfer ID of expected frames
    transfer_id: TransferId,
    /// The number of frames processed
    frames: usize,
    /// If the next frame should have the start bit set
    expect_start: bool,
    /// If the next frame should have the toggle bit set
    expect_toggle: bool,
    /// The bytes collected so far, not including tail bytes
    transfer: Vec<u8>,
}

impl Buildup {
    /// Creates a transfer reassembly object
    ///
    /// This function attempts to allocate enough memory to hold the largest possible payload.
    /// It returns an error if memory allocation fails.
    pub fn new(
        transfer_id: TransferId,
        max_payload_length: usize,
    ) -> Result<Self, OutOfMemoryError> {
        Ok(Buildup {
            transfer_id,
            frames: 0,
            expect_start: true,
            expect_toggle: true,
            transfer: FallibleVec::try_with_capacity(max_payload_length)?,
        })
    }

    /// Handles an incoming frame for this transfer
    ///
    /// This function panics if the transfer ID is not equal to the transfer ID used to create
    /// this Buildup, or if the frame data is empty.
    ///
    /// If this frame is the last frame in the transfer, this function returns the reassembled
    /// payload, including the padding and transfer CRC (if applicable) but excluding any
    /// tail bytes. After the payload is returned, this Buildup must not be used again.
    pub fn add(&mut self, frame_data: &[u8]) -> Result<Option<Vec<u8>>, BuildupError> {
        self.frames += 1;
        assert!(
            !frame_data.is_empty(),
            "Can't reassemble with an empty frame"
        );
        // Check tail byte
        let tail = TailByte::parse(*frame_data.last().unwrap());
        if tail.start != self.expect_start {
            return Err(BuildupError::InvalidStart);
        }
        if tail.toggle != self.expect_toggle {
            return Err(BuildupError::InvalidToggle);
        }
        assert_eq!(
            tail.transfer_id, self.transfer_id,
            "Incorrect transfer ID for frame to be reassembled"
        );
        // Prepare for the next frame
        self.expect_start = false;
        self.expect_toggle = !self.expect_toggle;

        // Copy data
        let frame_without_tail = &frame_data[..frame_data.len() - 1];
        FallibleVec::try_extend_from_slice(&mut self.transfer, frame_without_tail)?;

        if tail.end {
            // End of transfer, return the transfer data
            let data = mem::take(&mut self.transfer);
            Ok(Some(data))
        } else {
            // Expect more frames
            Ok(None)
        }
    }

    /// Returns the number of payload bytes collected
    pub fn payload_length(&self) -> usize {
        self.transfer.len()
    }

    /// Returns the ID of the transfer that is being reassembled
    pub fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }
    /// Returns the number of frames processed
    pub fn frames(&self) -> usize {
        self.frames
    }
}

#[derive(Debug)]
pub enum BuildupError {
    OutOfMemory(TryReserveError),
    InvalidStart,
    InvalidToggle,
}

impl From<TryReserveError> for BuildupError {
    fn from(inner: TryReserveError) -> Self {
        BuildupError::OutOfMemory(inner)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::convert::TryFrom;

    #[test]
    fn test_buildup_heartbeat() {
        // Heartbeat example from specification section 4.2.3
        for transfer_id in 0u8..=31 {
            let mut buildup = Buildup::new(TransferId::try_from(transfer_id).unwrap(), 7).unwrap();
            let payload = make_heartbeat_payload(u32::from(transfer_id));

            // A frame with 7 bytes of payload and a tail byte with first 1, last 1,
            // toggle 1, and the correct transfer ID
            let frame: [u8; 8] = [
                payload[0],
                payload[1],
                payload[2],
                payload[3],
                payload[4],
                payload[5],
                payload[6],
                0xe0 | transfer_id,
            ];

            assert_eq!(Some(payload.to_vec()), buildup.add(&frame).unwrap());
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

            let mut buildup = Buildup::new(TransferId::try_from(transfer_id).unwrap(), 16).unwrap();

            // Put in the payload bytes
            assert_eq!(Some(payload.to_vec()), buildup.add(&frame).unwrap());
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
        let mut buildup = Buildup::new(TransferId::try_from(1).unwrap(), 0).unwrap();
        assert_eq!(Some(Vec::new()), buildup.add(&[0xe1]).unwrap());
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

        let frames: [&[u8]; 11] = [
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

        let mut buildup = Buildup::new(TransferId::try_from(1).unwrap(), 71).unwrap();

        for (i, frame) in frames.iter().enumerate() {
            if i != frames.len() - 1 {
                assert_eq!(None, buildup.add(*frame).unwrap());
            } else {
                assert_eq!(Some(payload.to_vec()), buildup.add(*frame).unwrap());
            }
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
        let frames: [&[u8]; 2] = [
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
        let mut buildup = Buildup::new(TransferId::try_from(0).unwrap(), 63 + 47).unwrap();

        for (i, frame) in frames.iter().enumerate() {
            if i != frames.len() - 1 {
                assert_eq!(None, buildup.add(*frame).unwrap());
            } else {
                assert_eq!(Some(payload.to_vec()), buildup.add(*frame).unwrap());
            }
        }
    }
}
