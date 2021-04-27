use half::f16;

use crate::Serialize;
use core::convert::TryInto;

/// A cursor over a byte slice for easy serializing of UAVCAN data types
///
/// Functions that write values will panic if no space is available in the slice.
pub struct WriteCursor<'b> {
    /// The bytes available to write to
    ///
    /// This includes any bits already written in the current byte, but excludes bytes that have
    /// already been filled up.
    bytes: &'b mut [u8],
    /// The number of bytes in `bytes` that have been fully written
    bytes_written: usize,
    /// The number of bits in the current byte that have already been filled
    ///
    /// Multiple values within a byte are filled from right to left:
    /// https://github.com/UAVCAN/specification/issues/70
    ///
    /// Invariant: This is in the range 0..=7.
    bit_index: u8,
}

impl<'b> WriteCursor<'b> {
    /// Creates a cursor that will write starting at the beginning of the provided slice
    pub fn new(bytes: &'b mut [u8]) -> Self {
        // Reset all the bytes to zero
        bytes.iter_mut().for_each(|b| *b = 0);
        WriteCursor {
            bytes,
            bytes_written: 0,
            bit_index: 0,
        }
    }

    /// Returns a reference to the slice of remaining bytes that can be written
    /// (after the first self.bytes_written bytes). This may be an empty slice.
    ///
    /// # Panics
    ///
    /// This function panics if self.bytes_written is greater than self.bytes.len()
    fn remaining_bytes(&mut self) -> &mut [u8] {
        &mut self.bytes[self.bytes_written..]
    }

    fn is_aligned_to_8_bits(&self) -> bool {
        self.bit_index == 0
    }

    /// Writes an x-bit unsigned integer (x must be in the range 1..=64)
    fn write_up_to_u64(&mut self, mut value: u64, bits: u8) {
        debug_assert!(bits <= 64);
        // Write whole bytes, least significant first
        for _ in 0..(bits / 8) {
            self.write_up_to_u8(value as u8, 8);
            value >>= 8;
        }
        // Write any remaining bits that don't fill up a byte
        self.write_up_to_u8(value as u8, bits % 8);
    }

    /// Writes an x-bit unsigned integer (x must be in the range 1..=32)
    fn write_up_to_u32(&mut self, mut value: u32, bits: u8) {
        debug_assert!(bits <= 32);
        // Write whole bytes, least significant first
        for _ in 0..(bits / 8) {
            self.write_up_to_u8(value as u8, 8);
            value >>= 8;
        }
        // Write any remaining bits that don't fill up a byte
        self.write_up_to_u8(value as u8, bits % 8);
    }

    /// Writes an x-bit unsigned integer (x must be in the range 1..=16)
    fn write_up_to_u16(&mut self, mut value: u16, bits: u8) {
        debug_assert!(bits <= 16);
        // Write whole bytes, least significant first
        for _ in 0..(bits / 8) {
            self.write_up_to_u8(value as u8, 8);
            value >>= 8;
        }
        // Write any remaining bits that don't fill up a byte
        self.write_up_to_u8(value as u8, bits % 8);
    }

    /// Writes an x-bit unsigned integer (x must be in the range 0..=8)
    fn write_up_to_u8(&mut self, value: u8, bits: u8) {
        debug_assert!(bits <= 8);
        if bits == 0 {
            return;
        }
        self.check_length(usize::from(bits));
        // Constrain value to fit with the correct number of bits
        // Use 16 bits to correctly handle the case when bits = 8
        let mask = ((1u16 << u16::from(bits)) - 1) as u8;
        let value = value & mask;

        if self.bit_index <= 8 - bits {
            // Write all bits to the current byte, aligned to the right
            let bit_index = self.bit_index;
            let remaining_bytes = self.remaining_bytes();
            remaining_bytes[0] |= value << bit_index;
        } else {
            // Need to split across two bytes
            // current_bits: The less significant bits are aligned to the left of the current byte.
            let current_bits = value << self.bit_index;
            // next_bits: The more significant bits are aligned to the right of the next byte.
            let next_bits = value >> (8 - self.bit_index);
            let remaining_bytes = self.remaining_bytes();
            remaining_bytes[0] |= current_bits;
            remaining_bytes[1] |= next_bits;
        }

        self.advance_bits(usize::from(bits));
    }

    pub fn write_aligned_u8(&mut self, value: u8) {
        assert!(self.is_aligned_to_8_bits());
        self.remaining_bytes()[0] = value;
        self.advance_bits(8);
    }

    pub fn write_aligned_u16(&mut self, value: u16) {
        assert!(self.is_aligned_to_8_bits());
        let space = &mut self.remaining_bytes()[..2];
        space.copy_from_slice(&value.to_le_bytes());
        self.advance_bits(2 * 8);
    }

    pub fn write_aligned_u32(&mut self, value: u32) {
        assert!(self.is_aligned_to_8_bits());
        let space = &mut self.remaining_bytes()[..4];
        space.copy_from_slice(&value.to_le_bytes());
        self.advance_bits(4 * 8);
    }

    pub fn write_aligned_u64(&mut self, value: u64) {
        assert!(self.is_aligned_to_8_bits());
        let space = &mut self.remaining_bytes()[..8];
        space.copy_from_slice(&value.to_le_bytes());
        self.advance_bits(8 * 8);
    }

    /// Checks that enough space is available to write the specified number of bits, and panics
    /// if space is not available
    fn check_length(&self, bits: usize) {
        let extended_bit_index = usize::from(self.bit_index) + bits;
        let byte_increment = extended_bit_index / 8;
        assert!(self.bytes.len() - self.bytes_written >= byte_increment);
    }

    /// Advances to reflect that bits have been
    /// written
    fn advance_bits(&mut self, bits: usize) {
        self.check_length(bits);
        let extended_bit_index = usize::from(self.bit_index) + bits;
        self.bit_index = (extended_bit_index % 8) as u8;
        let byte_increment = extended_bit_index / 8;
        self.bytes_written += byte_increment;
    }

    fn skip_bits(&mut self, bits: u8) {
        self.check_length(usize::from(bits));
        self.advance_bits(usize::from(bits));
    }

    /// Advances the cursor to a byte boundary (a multiple of 8 bits)
    pub fn align_to_8_bits(&mut self) {
        if self.bit_index != 0 {
            self.skip_bits(8 - self.bit_index);
        }
    }

    /// Writes a 16-bit floating-point value
    #[inline]
    pub fn write_f16(&mut self, value: f16) {
        self.write_u16(value.to_bits());
    }

    /// Writes a 32-bit floating-point value
    #[inline]
    pub fn write_f32(&mut self, value: f32) {
        self.write_u32(value.to_bits());
    }

    /// Writes a 64-bit floating-point value
    #[inline]
    pub fn write_f64(&mut self, value: f64) {
        self.write_u64(value.to_bits());
    }

    /// Writes a byte array
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u8(*byte);
        }
    }

    pub fn write_aligned_bytes(&mut self, bytes: &[u8]) {
        let remaining_bytes = self.remaining_bytes();
        assert!(remaining_bytes.len() >= bytes.len());
        remaining_bytes[..bytes.len()].copy_from_slice(bytes);
        self.advance_bits(8 * bytes.len());
    }

    // /// Writes a fixed-length array of values
    // pub fn write_fixed_array<T>(&mut self, items: &[T])
    // where
    //     T: Serialize,
    // {
    //     for item in items {
    //         item.serialize(self);
    //     }
    // }
    //
    // /// Writes a variable-length array of values
    // pub fn write_variable_array<T>(&mut self, max_length: usize, items: &[T])
    // where
    //     T: Serialize,
    // {
    //     // TODO: The alignment of an array equals the alignment of its element type
    //     let length = items.len();
    //     assert!(length <= max_length);
    //     let length_bits = ceiling_log_2(max_length + 1);
    //     // Round up the length bits to 8, 16, 32, or 64
    //     // https://github.com/UAVCAN/specification/issues/75
    //     match length_bits {
    //         0..=8 => self.write_u8(length as u8),
    //         9..=16 => self.write_u16(length as u16),
    //         17..=32 => self.write_u32(length as u32),
    //         33..=64 => self.write_u64(length as u64),
    //         _ => panic!("Bug: Number of bits required for array size is too large"),
    //     };
    //     self.write_fixed_array(items);
    // }

    /// Writes a composite value, aligned to 8 bits
    pub fn write_composite<T>(&mut self, value: &T)
    where
        T: Serialize,
    {
        self.align_to_8_bits();
        if T::EXTENT_BYTES.is_some() {
            // Add delimiter header
            let composite_size_bits = value.size_bits();
            // Convert bits to bytes, round up
            let composite_size_bytes: u32 = ((composite_size_bits + 7) / 8)
                .try_into()
                .expect("Composite too large for u32");
            self.write_u32(composite_size_bytes);
        }
        // Now serialize the components
        value.serialize(self);
    }

    /// Writes a boolean value (1 bit)
    pub fn write_bool(&mut self, value: bool) {
        self.write_u1(value as u8)
    }
}

// Highly repetitive functions that just delegate
impl WriteCursor<'_> {
    #[inline]
    pub fn write_u1(&mut self, value: u8) {
        self.write_up_to_u8(value, 1);
    }
    #[inline]
    pub fn write_u2(&mut self, value: u8) {
        self.write_up_to_u8(value, 2);
    }
    #[inline]
    pub fn write_u3(&mut self, value: u8) {
        self.write_up_to_u8(value, 3);
    }
    #[inline]
    pub fn write_u4(&mut self, value: u8) {
        self.write_up_to_u8(value, 4);
    }
    #[inline]
    pub fn write_u5(&mut self, value: u8) {
        self.write_up_to_u8(value, 5);
    }
    #[inline]
    pub fn write_u6(&mut self, value: u8) {
        self.write_up_to_u8(value, 6);
    }
    #[inline]
    pub fn write_u7(&mut self, value: u8) {
        self.write_up_to_u8(value, 7);
    }
    #[inline]
    pub fn write_u8(&mut self, value: u8) {
        self.write_up_to_u8(value, 8);
    }
    #[inline]
    pub fn write_u9(&mut self, value: u16) {
        self.write_up_to_u16(value, 9);
    }
    #[inline]
    pub fn write_u10(&mut self, value: u16) {
        self.write_up_to_u16(value, 10);
    }
    #[inline]
    pub fn write_u11(&mut self, value: u16) {
        self.write_up_to_u16(value, 11);
    }
    #[inline]
    pub fn write_u12(&mut self, value: u16) {
        self.write_up_to_u16(value, 12);
    }
    #[inline]
    pub fn write_u13(&mut self, value: u16) {
        self.write_up_to_u16(value, 13);
    }
    #[inline]
    pub fn write_u14(&mut self, value: u16) {
        self.write_up_to_u16(value, 14);
    }
    #[inline]
    pub fn write_u15(&mut self, value: u16) {
        self.write_up_to_u16(value, 15);
    }
    #[inline]
    pub fn write_u16(&mut self, value: u16) {
        self.write_up_to_u16(value, 16);
    }
    #[inline]
    pub fn write_u18(&mut self, value: u32) {
        self.write_up_to_u32(value, 18)
    }
    #[inline]
    pub fn write_u19(&mut self, value: u32) {
        self.write_up_to_u32(value, 19)
    }
    #[inline]
    pub fn write_u20(&mut self, value: u32) {
        self.write_up_to_u32(value, 20)
    }
    #[inline]
    pub fn write_u21(&mut self, value: u32) {
        self.write_up_to_u32(value, 21)
    }
    #[inline]
    pub fn write_u22(&mut self, value: u32) {
        self.write_up_to_u32(value, 22)
    }
    #[inline]
    pub fn write_u23(&mut self, value: u32) {
        self.write_up_to_u32(value, 23)
    }
    #[inline]
    pub fn write_u24(&mut self, value: u32) {
        self.write_up_to_u32(value, 24)
    }
    #[inline]
    pub fn write_u25(&mut self, value: u32) {
        self.write_up_to_u32(value, 25)
    }
    #[inline]
    pub fn write_u26(&mut self, value: u32) {
        self.write_up_to_u32(value, 26)
    }
    #[inline]
    pub fn write_u27(&mut self, value: u32) {
        self.write_up_to_u32(value, 27)
    }
    #[inline]
    pub fn write_u28(&mut self, value: u32) {
        self.write_up_to_u32(value, 28)
    }
    #[inline]
    pub fn write_u29(&mut self, value: u32) {
        self.write_up_to_u32(value, 29)
    }
    #[inline]
    pub fn write_u30(&mut self, value: u32) {
        self.write_up_to_u32(value, 30)
    }
    #[inline]
    pub fn write_u31(&mut self, value: u32) {
        self.write_up_to_u32(value, 31)
    }
    #[inline]
    pub fn write_u32(&mut self, value: u32) {
        self.write_up_to_u32(value, 32)
    }
    #[inline]
    pub fn write_u33(&mut self, value: u64) {
        self.write_up_to_u64(value, 33)
    }
    #[inline]
    pub fn write_u34(&mut self, value: u64) {
        self.write_up_to_u64(value, 34)
    }
    #[inline]
    pub fn write_u35(&mut self, value: u64) {
        self.write_up_to_u64(value, 35)
    }
    #[inline]
    pub fn write_u36(&mut self, value: u64) {
        self.write_up_to_u64(value, 36)
    }
    #[inline]
    pub fn write_u37(&mut self, value: u64) {
        self.write_up_to_u64(value, 37)
    }
    #[inline]
    pub fn write_u38(&mut self, value: u64) {
        self.write_up_to_u64(value, 38)
    }
    #[inline]
    pub fn write_u39(&mut self, value: u64) {
        self.write_up_to_u64(value, 39)
    }
    #[inline]
    pub fn write_u40(&mut self, value: u64) {
        self.write_up_to_u64(value, 40)
    }
    #[inline]
    pub fn write_u41(&mut self, value: u64) {
        self.write_up_to_u64(value, 41)
    }
    #[inline]
    pub fn write_u42(&mut self, value: u64) {
        self.write_up_to_u64(value, 42)
    }
    #[inline]
    pub fn write_u43(&mut self, value: u64) {
        self.write_up_to_u64(value, 43)
    }
    #[inline]
    pub fn write_u44(&mut self, value: u64) {
        self.write_up_to_u64(value, 44)
    }
    #[inline]
    pub fn write_u45(&mut self, value: u64) {
        self.write_up_to_u64(value, 45)
    }
    #[inline]
    pub fn write_u46(&mut self, value: u64) {
        self.write_up_to_u64(value, 46)
    }
    #[inline]
    pub fn write_u47(&mut self, value: u64) {
        self.write_up_to_u64(value, 47)
    }
    #[inline]
    pub fn write_u48(&mut self, value: u64) {
        self.write_up_to_u64(value, 48)
    }
    #[inline]
    pub fn write_u49(&mut self, value: u64) {
        self.write_up_to_u64(value, 49)
    }
    #[inline]
    pub fn write_u50(&mut self, value: u64) {
        self.write_up_to_u64(value, 50)
    }
    #[inline]
    pub fn write_u51(&mut self, value: u64) {
        self.write_up_to_u64(value, 51)
    }
    #[inline]
    pub fn write_u52(&mut self, value: u64) {
        self.write_up_to_u64(value, 52)
    }
    #[inline]
    pub fn write_u53(&mut self, value: u64) {
        self.write_up_to_u64(value, 53)
    }
    #[inline]
    pub fn write_u54(&mut self, value: u64) {
        self.write_up_to_u64(value, 54)
    }
    #[inline]
    pub fn write_u55(&mut self, value: u64) {
        self.write_up_to_u64(value, 55)
    }
    #[inline]
    pub fn write_u56(&mut self, value: u64) {
        self.write_up_to_u64(value, 56)
    }
    #[inline]
    pub fn write_u57(&mut self, value: u64) {
        self.write_up_to_u64(value, 57)
    }
    #[inline]
    pub fn write_u58(&mut self, value: u64) {
        self.write_up_to_u64(value, 58)
    }
    #[inline]
    pub fn write_u59(&mut self, value: u64) {
        self.write_up_to_u64(value, 59)
    }
    #[inline]
    pub fn write_u60(&mut self, value: u64) {
        self.write_up_to_u64(value, 60)
    }
    #[inline]
    pub fn write_u61(&mut self, value: u64) {
        self.write_up_to_u64(value, 61)
    }
    #[inline]
    pub fn write_u62(&mut self, value: u64) {
        self.write_up_to_u64(value, 62)
    }
    #[inline]
    pub fn write_u63(&mut self, value: u64) {
        self.write_up_to_u64(value, 63)
    }
    #[inline]
    pub fn write_u64(&mut self, value: u64) {
        self.write_up_to_u64(value, 64)
    }
}
impl WriteCursor<'_> {
    #[inline]
    pub fn skip_1(&mut self) {
        self.skip_bits(1);
    }
    #[inline]
    pub fn skip_2(&mut self) {
        self.skip_bits(2);
    }
    #[inline]
    pub fn skip_3(&mut self) {
        self.skip_bits(3);
    }
    #[inline]
    pub fn skip_4(&mut self) {
        self.skip_bits(4);
    }
    #[inline]
    pub fn skip_5(&mut self) {
        self.skip_bits(5);
    }
    #[inline]
    pub fn skip_6(&mut self) {
        self.skip_bits(6);
    }
    #[inline]
    pub fn skip_7(&mut self) {
        self.skip_bits(7);
    }
    #[inline]
    pub fn skip_8(&mut self) {
        self.skip_bits(8);
    }
    #[inline]
    pub fn skip_9(&mut self) {
        self.skip_bits(9);
    }
    #[inline]
    pub fn skip_10(&mut self) {
        self.skip_bits(10);
    }
    #[inline]
    pub fn skip_11(&mut self) {
        self.skip_bits(11);
    }
    #[inline]
    pub fn skip_12(&mut self) {
        self.skip_bits(12);
    }
    #[inline]
    pub fn skip_13(&mut self) {
        self.skip_bits(13);
    }
    #[inline]
    pub fn skip_14(&mut self) {
        self.skip_bits(14);
    }
    #[inline]
    pub fn skip_15(&mut self) {
        self.skip_bits(15);
    }
    #[inline]
    pub fn skip_16(&mut self) {
        self.skip_bits(16);
    }
    #[inline]
    pub fn skip_17(&mut self) {
        self.skip_bits(17);
    }
    #[inline]
    pub fn skip_18(&mut self) {
        self.skip_bits(18);
    }
    #[inline]
    pub fn skip_19(&mut self) {
        self.skip_bits(19);
    }
    #[inline]
    pub fn skip_20(&mut self) {
        self.skip_bits(20);
    }
    #[inline]
    pub fn skip_21(&mut self) {
        self.skip_bits(21);
    }
    #[inline]
    pub fn skip_22(&mut self) {
        self.skip_bits(22);
    }
    #[inline]
    pub fn skip_23(&mut self) {
        self.skip_bits(23);
    }
    #[inline]
    pub fn skip_24(&mut self) {
        self.skip_bits(24);
    }
    #[inline]
    pub fn skip_25(&mut self) {
        self.skip_bits(25);
    }
    #[inline]
    pub fn skip_26(&mut self) {
        self.skip_bits(26);
    }
    #[inline]
    pub fn skip_27(&mut self) {
        self.skip_bits(27);
    }
    #[inline]
    pub fn skip_28(&mut self) {
        self.skip_bits(28);
    }
    #[inline]
    pub fn skip_29(&mut self) {
        self.skip_bits(29);
    }
    #[inline]
    pub fn skip_30(&mut self) {
        self.skip_bits(30);
    }
    #[inline]
    pub fn skip_31(&mut self) {
        self.skip_bits(31);
    }
    #[inline]
    pub fn skip_32(&mut self) {
        self.skip_bits(32);
    }
    #[inline]
    pub fn skip_33(&mut self) {
        self.skip_bits(33);
    }
    #[inline]
    pub fn skip_34(&mut self) {
        self.skip_bits(34);
    }
    #[inline]
    pub fn skip_35(&mut self) {
        self.skip_bits(35);
    }
    #[inline]
    pub fn skip_36(&mut self) {
        self.skip_bits(36);
    }
    #[inline]
    pub fn skip_37(&mut self) {
        self.skip_bits(37);
    }
    #[inline]
    pub fn skip_38(&mut self) {
        self.skip_bits(38);
    }
    #[inline]
    pub fn skip_39(&mut self) {
        self.skip_bits(39);
    }
    #[inline]
    pub fn skip_40(&mut self) {
        self.skip_bits(40);
    }
    #[inline]
    pub fn skip_41(&mut self) {
        self.skip_bits(41);
    }
    #[inline]
    pub fn skip_42(&mut self) {
        self.skip_bits(42);
    }
    #[inline]
    pub fn skip_43(&mut self) {
        self.skip_bits(43);
    }
    #[inline]
    pub fn skip_44(&mut self) {
        self.skip_bits(44);
    }
    #[inline]
    pub fn skip_45(&mut self) {
        self.skip_bits(45);
    }
    #[inline]
    pub fn skip_46(&mut self) {
        self.skip_bits(46);
    }
    #[inline]
    pub fn skip_47(&mut self) {
        self.skip_bits(47);
    }
    #[inline]
    pub fn skip_48(&mut self) {
        self.skip_bits(48);
    }
    #[inline]
    pub fn skip_49(&mut self) {
        self.skip_bits(49);
    }
    #[inline]
    pub fn skip_50(&mut self) {
        self.skip_bits(50);
    }
    #[inline]
    pub fn skip_51(&mut self) {
        self.skip_bits(51);
    }
    #[inline]
    pub fn skip_52(&mut self) {
        self.skip_bits(52);
    }
    #[inline]
    pub fn skip_53(&mut self) {
        self.skip_bits(53);
    }
    #[inline]
    pub fn skip_54(&mut self) {
        self.skip_bits(54);
    }
    #[inline]
    pub fn skip_55(&mut self) {
        self.skip_bits(55);
    }
    #[inline]
    pub fn skip_56(&mut self) {
        self.skip_bits(56);
    }
    #[inline]
    pub fn skip_57(&mut self) {
        self.skip_bits(57);
    }
    #[inline]
    pub fn skip_58(&mut self) {
        self.skip_bits(58);
    }
    #[inline]
    pub fn skip_59(&mut self) {
        self.skip_bits(59);
    }
    #[inline]
    pub fn skip_60(&mut self) {
        self.skip_bits(60);
    }
    #[inline]
    pub fn skip_61(&mut self) {
        self.skip_bits(61);
    }
    #[inline]
    pub fn skip_62(&mut self) {
        self.skip_bits(62);
    }
    #[inline]
    pub fn skip_63(&mut self) {
        self.skip_bits(63);
    }
    #[inline]
    pub fn skip_64(&mut self) {
        self.skip_bits(64);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u8_one() {
        let mut bytes = [0u8];
        let mut cursor = WriteCursor::new(&mut bytes);
        cursor.write_u8(0xe6);
        assert_eq!(bytes[0], 0xe6);
    }
    #[test]
    fn u8_many() {
        let mut bytes = [0u8; 16];
        let mut cursor = WriteCursor::new(&mut bytes);
        cursor.write_u8(0xe6);
        cursor.write_u8(0x21);
        cursor.write_u8(0xff);
        cursor.write_u8(0xe9);
        cursor.write_u8(0x02);
        cursor.write_u8(0xf7);
        cursor.write_u8(0x32);
        cursor.write_u8(0x1c);
        cursor.write_u8(0xc9);
        cursor.write_u8(0xab);
        cursor.write_u8(0xca);
        cursor.write_u8(0xd2);
        cursor.write_u8(0xe9);
        cursor.write_u8(0xf0);
        cursor.write_u8(0x39);
        cursor.write_u8(0x18);

        assert_eq!(
            bytes,
            [
                0xe6, 0x21, 0xff, 0xe9, 0x02, 0xf7, 0x32, 0x1c, 0xc9, 0xab, 0xca, 0xd2, 0xe9, 0xf0,
                0x39, 0x18
            ]
        );
    }

    #[test]
    fn u1_assemble_u8() {
        let mut bytes = [0u8];
        let mut cursor = WriteCursor::new(&mut bytes);
        // Within a byte, the most significant bit is first
        cursor.write_u1(1);
        cursor.write_u1(0);
        cursor.write_u1(1);
        cursor.write_u1(1);
        cursor.write_u1(0);
        cursor.write_u1(1);
        cursor.write_u1(0);
        cursor.write_u1(1);
        assert_eq!(bytes[0], 0b10101101);
    }

    #[test]
    fn u1_4_u8_u1_4() {
        let mut bytes = [0u8; 2];
        let mut cursor = WriteCursor::new(&mut bytes);
        // 4 u1s, 1 u8 (split between bytes), 4 u1s
        cursor.write_u1(1);
        cursor.write_u1(0);
        cursor.write_u1(1);
        cursor.write_u1(0);
        assert_eq!(cursor.bit_index, 4);
        cursor.write_u8(0x37);
        assert_eq!(cursor.bit_index, 4);
        cursor.write_u1(1);
        cursor.write_u1(1);
        cursor.write_u1(1);
        cursor.write_u1(0);
        assert_eq!(bytes, [0b_0111_0101, 0b0111_0011]);
    }

    #[test]
    fn u1_3_u8_u1_5() {
        let mut bytes = [0u8; 2];
        let mut cursor = WriteCursor::new(&mut bytes);
        // 3 u1s, 1 u8 (split between bytes), 5 u1s
        cursor.write_u1(1);
        cursor.write_u1(0);
        cursor.write_u1(1);
        cursor.write_u8(0x37);
        cursor.write_u1(1);
        cursor.write_u1(1);
        cursor.write_u1(1);
        cursor.write_u1(0);
        cursor.write_u1(0);
        assert_eq!(bytes, [0b1011_1101, 0b0011_1001]);
    }

    #[test]
    fn u2_assemble_u8() {
        let mut bytes = [0u8];
        let mut cursor = WriteCursor::new(&mut bytes);
        cursor.write_u2(3);
        cursor.write_u2(1);
        cursor.write_u2(2);
        cursor.write_u2(1);
        assert_eq!(bytes[0], 0b01100111);
    }

    #[test]
    fn u1_7_u2_u1_7() {
        let mut bytes = [0u8; 2];
        let mut cursor = WriteCursor::new(&mut bytes);
        // 7 u1s, 1 u2 (split between bytes), 7 u1s
        cursor.write_u1(1);
        cursor.write_u1(0);
        cursor.write_u1(1);
        cursor.write_u1(0);
        cursor.write_u1(0);
        cursor.write_u1(0);
        cursor.write_u1(0);
        cursor.write_u2(3);
        cursor.write_u1(0);
        cursor.write_u1(0);
        cursor.write_u1(1);
        cursor.write_u1(1);
        cursor.write_u1(1);
        cursor.write_u1(0);
        cursor.write_u1(0);
        assert_eq!(bytes, [0b1000_0101, 0b0011_1001]);
    }

    /// Tests the example in section 3.7.5 of the specification
    #[test]
    fn complex_example() {
        let mut bytes = [0u8; 4];
        let mut cursor = WriteCursor::new(&mut bytes);
        cursor.write_u12(48858);
        cursor.write_u3((-1i8) as u8);
        cursor.write_u4((-5i8) as u8);
        cursor.write_u2((-1i8) as u8);
        cursor.write_u4(136);

        // This has the ordering of fields within bytes updated for
        // https://github.com/UAVCAN/specification/issues/70 . The example in the specification
        // hasn't been updated yet.
        assert_eq!(bytes, [0b1101_1010, 0b1111_1110, 0b0001_1101, 0x1]);
    }

    #[test]
    fn u64_basic() {
        let mut bytes = [0u8; 8];
        let mut cursor = WriteCursor::new(&mut bytes);
        cursor.write_u64(0xfd569a8b24bca386);
        assert_eq!(bytes, [0x86, 0xa3, 0xbc, 0x24, 0x8b, 0x9a, 0x56, 0xfd]);
    }

    #[test]
    fn u64_aligned_basic() {
        let mut bytes = [0u8; 8];
        let mut cursor = WriteCursor::new(&mut bytes);
        cursor.write_aligned_u64(0xfd569a8b24bca386);
        assert_eq!(bytes, [0x86, 0xa3, 0xbc, 0x24, 0x8b, 0x9a, 0x56, 0xfd]);
    }

    // #[test]
    // fn test_ceiling_log_2() {
    //     assert_eq!(0, ceiling_log_2(1));
    //     assert_eq!(1, ceiling_log_2(2));
    //     assert_eq!(2, ceiling_log_2(3));
    //     assert_eq!(2, ceiling_log_2(4));
    //     assert_eq!(3, ceiling_log_2(5));
    //     assert_eq!(3, ceiling_log_2(6));
    //     assert_eq!(3, ceiling_log_2(7));
    //     assert_eq!(3, ceiling_log_2(8));
    //     assert_eq!(4, ceiling_log_2(9));
    //     assert_eq!(4, ceiling_log_2(10));
    //     assert_eq!(4, ceiling_log_2(11));
    //     assert_eq!(4, ceiling_log_2(12));
    //     assert_eq!(4, ceiling_log_2(13));
    //     assert_eq!(4, ceiling_log_2(14));
    //     assert_eq!(4, ceiling_log_2(15));
    //     assert_eq!(4, ceiling_log_2(16));
    //     assert_eq!(5, ceiling_log_2(17));
    // }
}
