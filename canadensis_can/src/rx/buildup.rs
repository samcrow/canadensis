use alloc::collections::TryReserveError;
use alloc::vec::Vec;
use core::mem;

use super::TailByte;
use canadensis_core::crc::Crc16CcittFalse as TransferCrc;
use canadensis_core::OutOfMemoryError;

/// Reassembles frames into a transfer
#[derive(Debug)]
pub struct Buildup {
    /// The number of frames processed
    frames: usize,
    /// The number of payload bytes processed, not including the transfer CRC or tail bytes
    ///
    /// This may be greater than payload_size_max
    payload_size: usize,
    /// If the next frame should have the start bit set
    expect_start: bool,
    /// If the next frame should have the toggle bit set
    expect_toggle: bool,
    /// Tail flags from the previous frame, for the purposes of duplicate detection
    prev_expect_start: bool,
    prev_expect_toggle: bool,
    /// The bytes collected so far, not including tail bytes
    ///
    /// The length of this never exceeds payload_size_max.
    transfer: Vec<u8>,
    /// The CRC of the bytes collected so far, excluding tail bytes (possibly more than
    /// payload_size_max)
    crc: TransferCrc,
}

impl Buildup {
    /// Creates a transfer reassembly object
    ///
    /// This function attempts to allocate enough memory to hold the largest possible payload.
    /// It returns an error if memory allocation fails.
    pub fn new(
        payload_size_max: usize,
    ) -> Result<Self, OutOfMemoryError> {
        let mut transfer = Vec::new();
        transfer.try_reserve_exact(payload_size_max)?;

        Ok(Buildup {
            frames: 0,
            payload_size: 0,
            expect_start: true,
            expect_toggle: true,
            prev_expect_start: false,
            prev_expect_toggle: false,
            transfer,
            crc: TransferCrc::new(),
        })
    }

    /// Handles an incoming frame for this transfer
    ///
    /// This function panics if the transfer ID is not equal to the transfer ID used to create
    /// this Buildup, or if the frame data is empty.
    ///
    /// If this frame is the last frame in the transfer, this function returns the reassembled
    /// payload, including the padding but excluding the transfer CRC and any
    /// tail bytes. After the payload is returned, this Buildup must not be used again.
    pub fn add(&mut self, frame_data: &[u8]) -> Result<Option<Vec<u8>>, BuildupError> {
        self.frames += 1;
        assert!(
            !frame_data.is_empty(),
            "Can't reassemble with an empty frame"
        );
        // Check if frame a likely duplicate of the previous
        if self.frames > 1 && self.expect_start == self.prev_expect_start && self.expect_toggle == self.prev_expect_toggle {
            // Drop it
            return Ok(None);
        }
        let tail = TailByte::parse(*frame_data.last().unwrap());
        // Check if new (non-duplicate) start frame
        if tail.start {
            // Restart buildup and continue
            self.frames = 1;
            self.expect_start = true;
            self.expect_toggle = true;
            self.transfer.clear();
            self.crc = TransferCrc::new();
        }
        // Check tail byte
        if tail.start != self.expect_start {
            return Ok(None);
        }
        if tail.toggle != self.expect_toggle {
            return Ok(None);
        }
        // Prepare for the next frame
        self.prev_expect_start = self.expect_start;
        self.prev_expect_toggle = self.expect_toggle;
        self.expect_start = false;
        self.expect_toggle = !self.expect_toggle;

        let frame_without_tail = &frame_data[..frame_data.len() - 1];
        let capacity_remaining = self.transfer.capacity() - self.transfer.len();
        let bytes_to_copy = capacity_remaining.min(frame_without_tail.len());
        self.transfer
            .extend_from_slice(&frame_without_tail[..bytes_to_copy]);

        self.crc.digest_bytes(frame_without_tail);
        self.payload_size += frame_without_tail.len();
        // Remove CRC
        if tail.end && !tail.start {
            self.payload_size -= 2;
        }

        let skip_crc = tail.start && tail.end;
        if tail.end {
            if skip_crc || self.crc.get_crc() == 0 {
                // End of transfer, return the transfer data without the CRC
                self.transfer.truncate(self.payload_size);
                let data = mem::take(&mut self.transfer);
                Ok(Some(data))
            } else {
                log::debug!("Incorrect transfer CRC");
                Err(BuildupError::Crc)
            }
        } else {
            // Expect more frames
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub enum BuildupError {
    OutOfMemory,
    Crc,
}

impl From<TryReserveError> for BuildupError {
    fn from(_inner: TryReserveError) -> Self {
        BuildupError::OutOfMemory
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_buildup_heartbeat() {
        // Heartbeat example from specification section 4.2.3
        for transfer_id in 0u8..=31 {
            let mut buildup =
                Buildup::new(7).unwrap();
            let payload = make_heartbeat_payload(u32::from(transfer_id));

            // A frame with 7 bytes of payload and a tail byte with first 1, last 1,
            // toggle 1, and correct transfer ID
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

            let mut buildup =
                Buildup::new(16).unwrap();

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
        let mut buildup = Buildup::new(0).unwrap();
        assert_eq!(Some(Vec::new()), buildup.add(&[0xe1]).unwrap());
    }

    #[test]
    fn test_node_info_response() {
        let payload: [u8; 69] = [
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

        let mut buildup = Buildup::new(71).unwrap();

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
        let payload: [u8; 63 + 45] = [
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
        let mut buildup = Buildup::new(63 + 47).unwrap();

        for (i, frame) in frames.iter().enumerate() {
            if i != frames.len() - 1 {
                assert_eq!(None, buildup.add(*frame).unwrap());
            } else {
                assert_eq!(Some(payload.to_vec()), buildup.add(*frame).unwrap());
            }
        }
    }
}
