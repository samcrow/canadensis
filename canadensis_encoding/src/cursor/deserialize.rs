use core::marker::PhantomData;

use half::f16;

use crate::{Deserialize, DeserializeError};

/// A cursor over a byte slice for easy deserializing of UAVCAN data types
///
/// Functions that read values will return zero when reading beyond the end of the bytes,
/// in accordance with the implicit zero extension rule (specification section 3.7.1.5)
pub struct ReadCursor<'b> {
    /// The bytes available to read from
    ///
    /// This includes any bits already read in the current byte, but excludes bytes that have
    /// already been fully read.
    bytes: *const u8,
    /// The number of valid bytes that the bytes field points to
    length: usize,
    /// The number of bits in the current byte that have already been read
    ///
    /// Multiple values within a byte are read from right to left:
    /// https://github.com/UAVCAN/specification/issues/70
    ///
    /// Invariant: This is in the range 0..=7.
    bit_index: u8,
    /// Phantom data to get the lifetimes right
    bytes_phantom: PhantomData<&'b [u8]>,
}

impl<'b> ReadCursor<'b> {
    /// Creates a cursor that will read starting at the beginning of the provided slice
    pub fn new(bytes: &'b [u8]) -> Self {
        ReadCursor {
            bytes: bytes.as_ptr(),
            length: bytes.len(),
            bit_index: 0,
            bytes_phantom: PhantomData,
        }
    }

    /// Read an x-bit unsigned integer (x must be in the range 0..=8)
    fn read_up_to_u8(&mut self, bits: u8) -> u8 {
        debug_assert!(bits <= 8);
        if bits == 0 {
            return 0;
        }

        let value = if self.bit_index <= 8 - bits {
            // Read all bits from the current byte, aligned to the right
            self.read_current() >> self.bit_index
        } else {
            // Need to split across two bytes
            // current_bits: The less significant bits are aligned to the left of the current byte.
            let current_bits = self.read_current() >> self.bit_index;
            // next_bits: The more significant bits are aligned to the right of the next byte.
            let next_bits = self.read_next() << (8 - self.bit_index);
            next_bits | current_bits
        };

        // Constrain value to fit with the correct number of bits
        // Use 16 bits to correctly handle the case when bits = 8
        let mask = ((1u16 << u16::from(bits)) - 1) as u8;
        let value = value & mask;

        self.advance_bits(usize::from(bits));
        value
    }

    /// Reads an x-bit unsigned integer (x must be in the range 1..=16)
    fn read_up_to_u16(&mut self, bits: u8) -> u16 {
        debug_assert!(bits <= 16);
        let mut shift_bits = 0;
        // Read whole bytes, least significant first
        let mut value = 0;
        if bits > 8 {
            value = u16::from(self.read_up_to_u8(8));
            shift_bits += 8;
        }
        // Write any remaining bits that don't fill up a byte
        value |= u16::from(self.read_up_to_u8(bits % 8)) << shift_bits;
        value
    }

    /// Reads an x-bit unsigned integer (x must be in the range 1..=32)
    fn read_up_to_u32(&mut self, bits: u8) -> u32 {
        debug_assert!(bits <= 32);
        let mut shift_bits = 0;
        // Read whole bytes, least significant first
        let mut value = 0;
        for _ in 0..(bits / 8) {
            value |= u32::from(self.read_up_to_u8(8)) << shift_bits;
            shift_bits += 8;
        }
        // Write any remaining bits that don't fill up a byte
        value |= u32::from(self.read_up_to_u8(bits % 8)) << shift_bits;
        value
    }

    /// Reads an x-bit unsigned integer (x must be in the range 1..=64)
    fn read_up_to_u64(&mut self, bits: u8) -> u64 {
        debug_assert!(bits <= 64);
        let mut shift_bits = 0;
        // Read whole bytes, least significant first
        let mut value = 0;
        for _ in 0..(bits / 8) {
            value |= u64::from(self.read_up_to_u8(8)) << shift_bits;
            shift_bits += 8;
        }
        // Write any remaining bits that don't fill up a byte
        value |= u64::from(self.read_up_to_u8(bits % 8)) << shift_bits;
        value
    }

    /// Returns the value of the current byte being read, or 0 if the cursor is past the end
    fn read_current(&self) -> u8 {
        if self.length != 0 {
            // A byte is available; read it
            unsafe { *self.bytes }
        } else {
            // Past the end, just return 0
            0
        }
    }
    /// Returns the value of the byte after current byte being read, or 0 if that position is past
    /// the end
    fn read_next(&self) -> u8 {
        if self.length > 1 {
            // A byte is available; read it
            unsafe {
                let offset = self.bytes.add(1);
                *offset
            }
        } else {
            // Past the end, just return 0
            0
        }
    }

    /// Advances self.bit_index, self.bytes, and self.length to reflect that bits have been read
    fn advance_bits(&mut self, bits: usize) {
        let extended_bit_index = usize::from(self.bit_index) + bits;
        self.bit_index = (extended_bit_index % 8) as u8;
        let byte_increment = extended_bit_index / 8;
        if self.length >= byte_increment {
            self.length -= byte_increment;
            unsafe {
                // This offset operation is safe even if it puts self.bytes one byte past the end of
                // a byte array.
                self.bytes = self.bytes.add(byte_increment);
            }
        } else {
            // Reached or passed the end
            // Advance self.bytes to one past the end
            unsafe { self.bytes = self.bytes.add(self.length) };
            self.length = 0;
        }
    }

    /// Skips up to 7 bits so that this cursor is aligned to 8 bits (one byte)
    fn align_to_8_bits(&mut self) {
        if self.bit_index != 0 {
            self.advance_bits(8 - usize::from(self.bit_index))
        }
    }

    /// Reads a 16-bit floating-point value
    #[inline]
    pub fn read_f16(&mut self) -> f16 {
        f16::from_bits(self.read_u16())
    }

    /// Reads a 32-bit floating-point value
    #[inline]
    pub fn read_f32(&mut self) -> f32 {
        f32::from_bits(self.read_u32())
    }

    /// Reads a 64-bit floating-point value
    #[inline]
    pub fn read_f64(&mut self) -> f64 {
        f64::from_bits(self.read_u64())
    }

    /// Reads a byte array
    pub fn read_bytes(&mut self, bytes: &mut [u8]) {
        for byte in bytes {
            *byte = self.read_u8();
        }
    }

    /// Reads a fixed-length array of values
    pub fn read_array<T>(&mut self, items: &mut [T]) -> Result<(), DeserializeError>
    where
        T: Deserialize,
    {
        for item in items {
            item.deserialize_in_place(self)?;
        }
        Ok(())
    }

    /// Reads a composite object
    ///
    /// This function returns an error if T is delimited and the delimiter header has an
    /// invalid length.
    ///
    /// It also return an error if T's deserialize implementation encounters an error.
    pub fn read_composite<T>(&mut self) -> Result<T, DeserializeError>
    where
        T: Deserialize,
    {
        if T::EXTENSIBILITY.is_delimited() {
            // Read the 32-bit delimiter header
            let length_bytes = self.read_u32();
            let length_bits = length_bytes * 8;
            if !T::in_bit_length_set(length_bits as usize) {
                // Invalid length
                return Err(DeserializeError::DelimitedLength);
            }
        }
        todo!()
    }

    /// Reads a boolean value (1 bit)
    pub fn read_bool(&mut self) -> bool {
        self.read_u1() == 1
    }
}

impl ReadCursor<'_> {
    #[inline]
    pub fn read_u1(&mut self) -> u8 {
        self.read_up_to_u8(1)
    }
    #[inline]
    pub fn read_u2(&mut self) -> u8 {
        self.read_up_to_u8(2)
    }
    #[inline]
    pub fn read_u3(&mut self) -> u8 {
        self.read_up_to_u8(3)
    }
    #[inline]
    pub fn read_u4(&mut self) -> u8 {
        self.read_up_to_u8(4)
    }
    #[inline]
    pub fn read_u5(&mut self) -> u8 {
        self.read_up_to_u8(5)
    }
    #[inline]
    pub fn read_u6(&mut self) -> u8 {
        self.read_up_to_u8(6)
    }
    #[inline]
    pub fn read_u7(&mut self) -> u8 {
        self.read_up_to_u8(7)
    }
    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        self.read_up_to_u8(8)
    }
    #[inline]
    pub fn read_u9(&mut self) -> u16 {
        self.read_up_to_u16(9)
    }
    #[inline]
    pub fn read_u10(&mut self) -> u16 {
        self.read_up_to_u16(10)
    }
    #[inline]
    pub fn read_u11(&mut self) -> u16 {
        self.read_up_to_u16(11)
    }
    #[inline]
    pub fn read_u12(&mut self) -> u16 {
        self.read_up_to_u16(12)
    }
    #[inline]
    pub fn read_u13(&mut self) -> u16 {
        self.read_up_to_u16(13)
    }
    #[inline]
    pub fn read_u14(&mut self) -> u16 {
        self.read_up_to_u16(14)
    }
    #[inline]
    pub fn read_u15(&mut self) -> u16 {
        self.read_up_to_u16(15)
    }
    #[inline]
    pub fn read_u16(&mut self) -> u16 {
        self.read_up_to_u16(16)
    }
    #[inline]
    pub fn read_u17(&mut self) -> u32 {
        self.read_up_to_u32(17)
    }
    #[inline]
    pub fn read_u18(&mut self) -> u32 {
        self.read_up_to_u32(18)
    }
    #[inline]
    pub fn read_u19(&mut self) -> u32 {
        self.read_up_to_u32(19)
    }
    #[inline]
    pub fn read_u20(&mut self) -> u32 {
        self.read_up_to_u32(20)
    }
    #[inline]
    pub fn read_u21(&mut self) -> u32 {
        self.read_up_to_u32(21)
    }
    #[inline]
    pub fn read_u22(&mut self) -> u32 {
        self.read_up_to_u32(22)
    }
    #[inline]
    pub fn read_u23(&mut self) -> u32 {
        self.read_up_to_u32(23)
    }
    #[inline]
    pub fn read_u24(&mut self) -> u32 {
        self.read_up_to_u32(24)
    }
    #[inline]
    pub fn read_u25(&mut self) -> u32 {
        self.read_up_to_u32(25)
    }
    #[inline]
    pub fn read_u26(&mut self) -> u32 {
        self.read_up_to_u32(26)
    }
    #[inline]
    pub fn read_u27(&mut self) -> u32 {
        self.read_up_to_u32(27)
    }
    #[inline]
    pub fn read_u28(&mut self) -> u32 {
        self.read_up_to_u32(28)
    }
    #[inline]
    pub fn read_u29(&mut self) -> u32 {
        self.read_up_to_u32(29)
    }
    #[inline]
    pub fn read_u30(&mut self) -> u32 {
        self.read_up_to_u32(30)
    }
    #[inline]
    pub fn read_u31(&mut self) -> u32 {
        self.read_up_to_u32(31)
    }
    #[inline]
    pub fn read_u32(&mut self) -> u32 {
        self.read_up_to_u32(32)
    }
    #[inline]
    pub fn read_u33(&mut self) -> u64 {
        self.read_up_to_u64(33)
    }
    #[inline]
    pub fn read_u34(&mut self) -> u64 {
        self.read_up_to_u64(34)
    }
    #[inline]
    pub fn read_u35(&mut self) -> u64 {
        self.read_up_to_u64(35)
    }
    #[inline]
    pub fn read_u36(&mut self) -> u64 {
        self.read_up_to_u64(36)
    }
    #[inline]
    pub fn read_u37(&mut self) -> u64 {
        self.read_up_to_u64(37)
    }
    #[inline]
    pub fn read_u38(&mut self) -> u64 {
        self.read_up_to_u64(38)
    }
    #[inline]
    pub fn read_u39(&mut self) -> u64 {
        self.read_up_to_u64(39)
    }
    #[inline]
    pub fn read_u40(&mut self) -> u64 {
        self.read_up_to_u64(40)
    }
    #[inline]
    pub fn read_u41(&mut self) -> u64 {
        self.read_up_to_u64(41)
    }
    #[inline]
    pub fn read_u42(&mut self) -> u64 {
        self.read_up_to_u64(42)
    }
    #[inline]
    pub fn read_u43(&mut self) -> u64 {
        self.read_up_to_u64(43)
    }
    #[inline]
    pub fn read_u44(&mut self) -> u64 {
        self.read_up_to_u64(44)
    }
    #[inline]
    pub fn read_u45(&mut self) -> u64 {
        self.read_up_to_u64(45)
    }
    #[inline]
    pub fn read_u46(&mut self) -> u64 {
        self.read_up_to_u64(46)
    }
    #[inline]
    pub fn read_u47(&mut self) -> u64 {
        self.read_up_to_u64(47)
    }
    #[inline]
    pub fn read_u48(&mut self) -> u64 {
        self.read_up_to_u64(48)
    }
    #[inline]
    pub fn read_u49(&mut self) -> u64 {
        self.read_up_to_u64(49)
    }
    #[inline]
    pub fn read_u50(&mut self) -> u64 {
        self.read_up_to_u64(50)
    }
    #[inline]
    pub fn read_u51(&mut self) -> u64 {
        self.read_up_to_u64(51)
    }
    #[inline]
    pub fn read_u52(&mut self) -> u64 {
        self.read_up_to_u64(52)
    }
    #[inline]
    pub fn read_u53(&mut self) -> u64 {
        self.read_up_to_u64(53)
    }
    #[inline]
    pub fn read_u54(&mut self) -> u64 {
        self.read_up_to_u64(54)
    }
    #[inline]
    pub fn read_u55(&mut self) -> u64 {
        self.read_up_to_u64(55)
    }
    #[inline]
    pub fn read_u56(&mut self) -> u64 {
        self.read_up_to_u64(56)
    }
    #[inline]
    pub fn read_u57(&mut self) -> u64 {
        self.read_up_to_u64(57)
    }
    #[inline]
    pub fn read_u58(&mut self) -> u64 {
        self.read_up_to_u64(58)
    }
    #[inline]
    pub fn read_u59(&mut self) -> u64 {
        self.read_up_to_u64(59)
    }
    #[inline]
    pub fn read_u60(&mut self) -> u64 {
        self.read_up_to_u64(60)
    }
    #[inline]
    pub fn read_u61(&mut self) -> u64 {
        self.read_up_to_u64(61)
    }
    #[inline]
    pub fn read_u62(&mut self) -> u64 {
        self.read_up_to_u64(62)
    }
    #[inline]
    pub fn read_u63(&mut self) -> u64 {
        self.read_up_to_u64(63)
    }
    #[inline]
    pub fn read_u64(&mut self) -> u64 {
        self.read_up_to_u64(64)
    }
}

impl ReadCursor<'_> {
    #[inline]
    pub fn skip_1(&mut self) {
        self.advance_bits(1)
    }
    #[inline]
    pub fn skip_2(&mut self) {
        self.advance_bits(2)
    }
    #[inline]
    pub fn skip_3(&mut self) {
        self.advance_bits(3)
    }
    #[inline]
    pub fn skip_4(&mut self) {
        self.advance_bits(4)
    }
    #[inline]
    pub fn skip_5(&mut self) {
        self.advance_bits(5)
    }
    #[inline]
    pub fn skip_6(&mut self) {
        self.advance_bits(6)
    }
    #[inline]
    pub fn skip_7(&mut self) {
        self.advance_bits(7)
    }
    #[inline]
    pub fn skip_8(&mut self) {
        self.advance_bits(8)
    }
    #[inline]
    pub fn skip_9(&mut self) {
        self.advance_bits(9)
    }
    #[inline]
    pub fn skip_10(&mut self) {
        self.advance_bits(10)
    }
    #[inline]
    pub fn skip_11(&mut self) {
        self.advance_bits(11)
    }
    #[inline]
    pub fn skip_12(&mut self) {
        self.advance_bits(12)
    }
    #[inline]
    pub fn skip_13(&mut self) {
        self.advance_bits(13)
    }
    #[inline]
    pub fn skip_14(&mut self) {
        self.advance_bits(14)
    }
    #[inline]
    pub fn skip_15(&mut self) {
        self.advance_bits(15)
    }
    #[inline]
    pub fn skip_16(&mut self) {
        self.advance_bits(16)
    }
    #[inline]
    pub fn skip_17(&mut self) {
        self.advance_bits(17)
    }
    #[inline]
    pub fn skip_18(&mut self) {
        self.advance_bits(18)
    }
    #[inline]
    pub fn skip_19(&mut self) {
        self.advance_bits(19)
    }
    #[inline]
    pub fn skip_20(&mut self) {
        self.advance_bits(20)
    }
    #[inline]
    pub fn skip_21(&mut self) {
        self.advance_bits(21)
    }
    #[inline]
    pub fn skip_22(&mut self) {
        self.advance_bits(22)
    }
    #[inline]
    pub fn skip_23(&mut self) {
        self.advance_bits(23)
    }
    #[inline]
    pub fn skip_24(&mut self) {
        self.advance_bits(24)
    }
    #[inline]
    pub fn skip_25(&mut self) {
        self.advance_bits(25)
    }
    #[inline]
    pub fn skip_26(&mut self) {
        self.advance_bits(26)
    }
    #[inline]
    pub fn skip_27(&mut self) {
        self.advance_bits(27)
    }
    #[inline]
    pub fn skip_28(&mut self) {
        self.advance_bits(28)
    }
    #[inline]
    pub fn skip_29(&mut self) {
        self.advance_bits(29)
    }
    #[inline]
    pub fn skip_30(&mut self) {
        self.advance_bits(30)
    }
    #[inline]
    pub fn skip_31(&mut self) {
        self.advance_bits(31)
    }
    #[inline]
    pub fn skip_32(&mut self) {
        self.advance_bits(32)
    }
    #[inline]
    pub fn skip_33(&mut self) {
        self.advance_bits(33)
    }
    #[inline]
    pub fn skip_34(&mut self) {
        self.advance_bits(34)
    }
    #[inline]
    pub fn skip_35(&mut self) {
        self.advance_bits(35)
    }
    #[inline]
    pub fn skip_36(&mut self) {
        self.advance_bits(36)
    }
    #[inline]
    pub fn skip_37(&mut self) {
        self.advance_bits(37)
    }
    #[inline]
    pub fn skip_38(&mut self) {
        self.advance_bits(38)
    }
    #[inline]
    pub fn skip_39(&mut self) {
        self.advance_bits(39)
    }
    #[inline]
    pub fn skip_40(&mut self) {
        self.advance_bits(40)
    }
    #[inline]
    pub fn skip_41(&mut self) {
        self.advance_bits(41)
    }
    #[inline]
    pub fn skip_42(&mut self) {
        self.advance_bits(42)
    }
    #[inline]
    pub fn skip_43(&mut self) {
        self.advance_bits(43)
    }
    #[inline]
    pub fn skip_44(&mut self) {
        self.advance_bits(44)
    }
    #[inline]
    pub fn skip_45(&mut self) {
        self.advance_bits(45)
    }
    #[inline]
    pub fn skip_46(&mut self) {
        self.advance_bits(46)
    }
    #[inline]
    pub fn skip_47(&mut self) {
        self.advance_bits(47)
    }
    #[inline]
    pub fn skip_48(&mut self) {
        self.advance_bits(48)
    }
    #[inline]
    pub fn skip_49(&mut self) {
        self.advance_bits(49)
    }
    #[inline]
    pub fn skip_50(&mut self) {
        self.advance_bits(50)
    }
    #[inline]
    pub fn skip_51(&mut self) {
        self.advance_bits(51)
    }
    #[inline]
    pub fn skip_52(&mut self) {
        self.advance_bits(52)
    }
    #[inline]
    pub fn skip_53(&mut self) {
        self.advance_bits(53)
    }
    #[inline]
    pub fn skip_54(&mut self) {
        self.advance_bits(54)
    }
    #[inline]
    pub fn skip_55(&mut self) {
        self.advance_bits(55)
    }
    #[inline]
    pub fn skip_56(&mut self) {
        self.advance_bits(56)
    }
    #[inline]
    pub fn skip_57(&mut self) {
        self.advance_bits(57)
    }
    #[inline]
    pub fn skip_58(&mut self) {
        self.advance_bits(58)
    }
    #[inline]
    pub fn skip_59(&mut self) {
        self.advance_bits(59)
    }
    #[inline]
    pub fn skip_60(&mut self) {
        self.advance_bits(60)
    }
    #[inline]
    pub fn skip_61(&mut self) {
        self.advance_bits(61)
    }
    #[inline]
    pub fn skip_62(&mut self) {
        self.advance_bits(62)
    }
    #[inline]
    pub fn skip_63(&mut self) {
        self.advance_bits(63)
    }
    #[inline]
    pub fn skip_64(&mut self) {
        self.advance_bits(64)
    }
}
