//! Cyclic redundancy checks (CRCs) used in multiple transports

/// Tracks the CRC of bytes processed so far and the last four bytes,
/// which may be the transfer CRC
///
/// Internally, this uses CRC-32C.
#[derive(Debug)]
pub struct CrcTracker {
    /// CRC of all bytes before last_four_bytes
    crc: Crc32c,
    /// Most recent four bytes processed, with the oldest byte
    /// in the least significant position
    last_four_bytes: u32,
    bytes_processed: u8,
}

impl CrcTracker {
    /// Creates an empty CRC tracker
    pub fn new() -> CrcTracker {
        CrcTracker {
            crc: Crc32c::new(),
            last_four_bytes: 0x0,
            bytes_processed: 0,
        }
    }

    /// Handles a byte
    ///
    /// If this tracker has already processed four bytes, this function returns the
    /// byte before the most recent four bytes.
    pub fn digest(&mut self, byte: u8) -> Option<u8> {
        let byte_out = if self.bytes_processed >= 4 {
            Some(self.last_four_bytes as u8)
        } else {
            None
        };
        if let Some(byte_out) = byte_out {
            self.crc.digest(byte_out);
        }
        self.last_four_bytes = (u32::from(byte) << 24) | (self.last_four_bytes >> 8);
        self.bytes_processed = self.bytes_processed.saturating_add(1);
        byte_out
    }
    /// Returns true if the most recent four bytes contain a little-endian value that matches the
    /// CRC of the previous bytes
    pub fn correct(&self) -> bool {
        self.bytes_processed >= 4 && self.crc.get_crc() == self.last_four_bytes
    }
}

impl Default for CrcTracker {
    fn default() -> Self {
        CrcTracker::new()
    }
}

#[cfg(test)]
mod tests {
    use super::CrcTracker;

    #[test]
    fn crc_tracker_empty() {
        let tracker = CrcTracker::new();
        assert!(!tracker.correct());
    }
    #[test]
    fn crc_tracker_zero() {
        let mut tracker = CrcTracker::new();
        tracker.digest(0x00);
        tracker.digest(0x00);
        tracker.digest(0x00);
        tracker.digest(0x00);
        assert!(tracker.correct());
    }
    #[test]
    fn crc_tracker_four_bytes() {
        let mut tracker = CrcTracker::new();
        IntoIterator::into_iter([0x39, 0x52, 0xee, 0x11, 0x68, 0x81, 0x3e, 0xc8]).for_each(
            |byte| {
                tracker.digest(byte);
            },
        );
        assert!(tracker.correct());
    }
    #[test]
    fn crc_tracker_long() {
        let mut tracker = CrcTracker::new();
        IntoIterator::into_iter([
            0xc2, 0xcf, 0xcc, 0xc0, 0x1c, 0xd7, 0x90, 0x5f, 0x95, 0x9e, 0xa4, 0x7c, 0x91, 0xe0,
            0xa0, 0xe4, 0xbd, 0xf9, 0x4a, 0x9d, 0x44, 0xc7, 0x7c, 0x7f, 0x59, 0xcb, 0x5b, 0x2e,
        ])
        .for_each(|byte| {
            tracker.digest(byte);
        });
        assert!(tracker.correct());
    }
}

const CRC32_XOR: u32 = 0xffff_ffff;
const CRC32_REFLECTED_POLY: u32 = 0x82f6_3b78;

/// A CRC-32C calculator
///
/// The Cyphal/Serial and Cyphal/UDP transfer CRC uses this.
#[derive(Debug)]
pub struct Crc32c {
    value: u32,
}

impl Crc32c {
    /// Creates a new CRC calculation
    pub fn new() -> Crc32c {
        Crc32c { value: CRC32_XOR }
    }

    /// Returns the CRC calculated over the bytes added through previous calls
    pub fn get_crc(&self) -> u32 {
        self.value ^ CRC32_XOR
    }

    /// Adds a byte to the CRC calculation
    pub fn digest(&mut self, byte: u8) {
        self.value ^= u32::from(byte);
        for _ in 0..8 {
            self.value = if (self.value & 0x1) != 0 {
                (self.value >> 1) ^ CRC32_REFLECTED_POLY
            } else {
                self.value >> 1
            };
        }
    }

    /// Adds all the bytes in a slice to the CRC calculation
    pub fn digest_bytes(&mut self, bytes: &[u8]) {
        bytes.iter().for_each(|&byte| self.digest(byte));
    }
}

impl Default for Crc32c {
    fn default() -> Self {
        Crc32c::new()
    }
}

/// CRC initial value
const CRC16_INIT: u16 = 0xffff;
/// CRC polynomial, not reversed, big endian
const CRC16_POLY: u16 = 0x1021;

/// A CRC-16-CCITT-false calculator
///
/// The Cyphal/CAN transfer CRC and Cyphal/Serial and Cyphal/UDP header CRC use this.
#[derive(Debug)]
pub struct Crc16CcittFalse {
    value: u16,
}

impl Crc16CcittFalse {
    /// Creates a new CRC calculation
    pub fn new() -> Self {
        Crc16CcittFalse { value: CRC16_INIT }
    }

    /// Adds a byte to the CRC calculation
    pub fn digest(&mut self, byte: u8) {
        self.value ^= u16::from(byte) << 8;
        for _bit in 0..8 {
            if (self.value & 0x8000) != 0 {
                self.value = (self.value << 1) ^ CRC16_POLY;
            } else {
                self.value <<= 1;
            }
        }
    }

    /// Adds all the bytes in a slice to the CRC calculation
    pub fn digest_bytes(&mut self, bytes: &[u8]) {
        bytes.iter().for_each(|&byte| self.digest(byte));
    }

    /// Returns the CRC calculated over the bytes added through previous calls
    pub fn get_crc(&self) -> u16 {
        self.value
    }
}

impl Default for Crc16CcittFalse {
    fn default() -> Self {
        Self::new()
    }
}
