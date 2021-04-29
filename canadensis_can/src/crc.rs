//!
//! Transfer CRC calculation
//!

/// CRC initial value
const TRANSFER_CRC_INIT: u16 = 0xffff;
/// CRC polynomial, not reversed, big endian
const TRANSFER_CRC_POLY: u16 = 0x1021;

/// Calculates the 32-bit transfer CRC
pub struct TransferCrc {
    value: u16,
}

impl TransferCrc {
    /// Creates a new CRC calculation
    pub fn new() -> Self {
        TransferCrc {
            value: TRANSFER_CRC_INIT,
        }
    }

    /// Adds a byte to the CRC calculation
    pub fn add(&mut self, byte: u8) {
        self.value ^= u16::from(byte) << 8;
        for _bit in 0..8 {
            if (self.value & 0x8000) != 0 {
                self.value = (self.value << 1) ^ TRANSFER_CRC_POLY;
            } else {
                self.value <<= 1;
            }
        }
    }

    /// Adds all the bytes in a slice to the CRC calculation
    pub fn add_bytes(&mut self, bytes: &[u8]) {
        bytes.iter().for_each(|&byte| self.add(byte));
    }

    /// Returns the CRC calculated over the bytes added through previous calls
    pub fn get(&self) -> u16 {
        self.value
    }
}
