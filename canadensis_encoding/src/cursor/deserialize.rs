//!
//! A deserializing cursor that can be used to read values from a sequence of bytes
//!

use core::cmp;

use half::f16;

use crate::{Deserialize, DeserializeError};

/// A cursor over a byte slice for easy deserializing of Cyphal data types
///
/// Functions that read values will return zero when reading beyond the end of the bytes,
/// in accordance with the implicit zero extension rule (specification section 3.7.1.5)
#[derive(Debug)]
pub struct ReadCursor<'b> {
    /// The bytes available to read from
    ///
    /// This includes any bits already read in the current byte, but excludes bytes that have
    /// already been fully read.
    bytes: &'b [u8],
    /// The number of bits in the current byte that have already been read
    ///
    /// Multiple values within a byte are read from right to left:
    /// <https://github.com/OpenCyphal/specification/issues/70>
    ///
    /// Invariant: This is in the range 0..=7.
    bit_index: u8,
}

impl<'b> ReadCursor<'b> {
    /// Creates a cursor that will read starting at the beginning of the provided slice
    pub fn new(bytes: &'b [u8]) -> Self {
        ReadCursor {
            bytes,
            bit_index: 0,
        }
    }

    /// If this cursor is aligned to a byte boundary, this function returns the slice of bytes
    /// that remain to be read.
    ///
    /// Caution: This function bypasses the normal mechanism for keeping track of which bytes
    /// have already been read. If you consume any of the returned bytes, you must call
    /// [`advance_bytes()`](#method.advance_bytes) so that they will not be read again alter.
    ///
    pub fn as_bytes(&self) -> Option<&'b [u8]> {
        if self.bit_index == 0 {
            Some(self.bytes)
        } else {
            None
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
        if bits != 64 {
            value |= u64::from(self.read_up_to_u8(bits % 8)) << shift_bits;
        }
        value
    }

    /// Reads an 8-bit integer
    ///
    /// # Panics
    ///
    /// This function panics if the cursor is not aligned to a multiple of 8 bits.
    pub fn read_aligned_u8(&mut self) -> u8 {
        assert!(self.is_aligned_to_8_bits());
        let value = self.read_current();
        self.advance_bytes(1);
        value
    }

    /// Reads a 16-bit integer
    ///
    /// # Panics
    ///
    /// This function panics if the cursor is not aligned to a multiple of 8 bits.
    pub fn read_aligned_u16(&mut self) -> u16 {
        // Least significant byte first
        let lsb = self.read_aligned_u8();
        let msb = self.read_aligned_u8();
        (u16::from(msb) << 8) | u16::from(lsb)
    }

    /// Reads a 32-bit integer
    ///
    /// # Panics
    ///
    /// This function panics if the cursor is not aligned to a multiple of 8 bits.
    pub fn read_aligned_u32(&mut self) -> u32 {
        // Least significant byte first
        let lsbs = self.read_aligned_u16();
        let msbs = self.read_aligned_u16();
        (u32::from(msbs) << 16) | u32::from(lsbs)
    }

    /// Reads a 64-bit integer
    ///
    /// # Panics
    ///
    /// This function panics if the cursor is not aligned to a multiple of 8 bits.
    pub fn read_aligned_u64(&mut self) -> u64 {
        // Least significant byte first
        let lsbs = self.read_aligned_u32();
        let msbs = self.read_aligned_u32();
        (u64::from(msbs) << 32) | u64::from(lsbs)
    }

    /// Returns the value of the current byte being read, or 0 if the cursor is past the end
    fn read_current(&self) -> u8 {
        self.bytes.first().cloned().unwrap_or(0)
    }
    /// Returns the value of the byte after current byte being read, or 0 if that position is past
    /// the end
    fn read_next(&self) -> u8 {
        self.bytes.get(1).cloned().unwrap_or(0)
    }

    /// Advances self.bit_index and self.bytes to reflect that bits have been read
    fn advance_bits(&mut self, bits: usize) {
        let extended_bit_index = usize::from(self.bit_index) + bits;
        self.bit_index = (extended_bit_index % 8) as u8;
        let byte_increment = extended_bit_index / 8;
        self.advance_bytes(byte_increment);
    }

    /// Marks some bytes has having already been read
    ///
    /// This function should only be used with [`as_bytes()`](#method.as_bytes) when manually
    /// handling bytes.
    pub fn advance_bytes(&mut self, byte_increment: usize) {
        // Advance by the byte increment or number of bytes remaining, whichever is less
        // If the number of bytes remaining is smaller,
        // self.bytes will end up empty.
        let real_byte_increment = cmp::min(byte_increment, self.bytes.len());
        self.bytes = &self.bytes[real_byte_increment..];
    }

    /// Skips up to 7 bits so that this cursor is aligned to 8 bits (one byte)
    pub fn align_to_8_bits(&mut self) {
        if self.bit_index != 0 {
            self.advance_bits(8 - usize::from(self.bit_index))
        }
    }

    /// Returns true if this cursor is aligned to a multiple of 8 bits
    pub fn is_aligned_to_8_bits(&self) -> bool {
        self.bit_index == 0
    }

    /// Reads a 16-bit floating-point value
    #[inline]
    pub fn read_f16(&mut self) -> f16 {
        f16::from_bits(self.read_u16())
    }

    /// Reads a 16-bit floating-point value
    ///
    /// # Panics
    ///
    /// This function panics if the cursor is not aligned to a multiple of 8 bits.
    #[inline]
    pub fn read_aligned_f16(&mut self) -> f16 {
        f16::from_bits(self.read_aligned_u16())
    }

    /// Reads a 32-bit floating-point value
    #[inline]
    pub fn read_f32(&mut self) -> f32 {
        f32::from_bits(self.read_u32())
    }

    /// Reads a 32-bit floating-point value
    ///
    /// # Panics
    ///
    /// This function panics if the cursor is not aligned to a multiple of 8 bits.
    #[inline]
    pub fn read_aligned_f32(&mut self) -> f32 {
        f32::from_bits(self.read_aligned_u32())
    }

    /// Reads a 64-bit floating-point value
    #[inline]
    pub fn read_f64(&mut self) -> f64 {
        f64::from_bits(self.read_u64())
    }

    /// Reads a 64-bit floating-point value
    ///
    /// # Panics
    ///
    /// This function panics if the cursor is not aligned to a multiple of 8 bits.
    #[inline]
    pub fn read_aligned_f64(&mut self) -> f64 {
        f64::from_bits(self.read_aligned_u64())
    }

    /// Reads a byte array
    pub fn read_bytes(&mut self, bytes: &mut [u8]) {
        for byte in bytes {
            *byte = self.read_u8();
        }
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
        self.align_to_8_bits();
        let status = if T::EXTENT_BYTES.is_some() {
            // This is a delimited type. Read the header and fork to read the object
            let composite_length_bytes = self.read_aligned_u32() as usize;
            if composite_length_bytes > self.bytes.len() {
                Err(DeserializeError::DelimitedLength)
            } else {
                let mut forked = self.fork(composite_length_bytes);
                T::deserialize(&mut forked)
            }
        } else {
            // Sealed type, read directly
            T::deserialize(self)
        };
        self.align_to_8_bits();
        status
    }

    /// Reads a boolean value (1 bit)
    pub fn read_bool(&mut self) -> bool {
        self.read_u1() == 1
    }

    /// Creates another cursor to read a specified number of bytes, and skips this cursor past
    /// those bytes
    ///
    /// The number of bytes to fork must be less than or equal to the number of bytes available
    /// for this cursor to read.
    ///
    /// The returned cursor will read `bytes` bytes starting at the position of this cursor before
    /// the call to `fork`, and then will read implicit zero bytes.
    ///
    /// After this function is called, this cursor will be advanced to just past the end of the
    /// bytes that the returned cursor can read.
    ///
    /// # Panics
    ///
    /// This function will panic if this cursor is not aligned to a byte boundary (8 bits),
    /// or if bytes is less then the number of bytes remaining for this cursor to read.
    fn fork(&mut self, fork_bytes: usize) -> Self {
        assert_eq!(self.bit_index, 0, "fork(): Not aligned to a byte");
        assert!(
            fork_bytes <= self.bytes.len(),
            "fork(): Not enough bytes available to fork"
        );

        let forked_cursor = ReadCursor {
            bytes: &self.bytes[..fork_bytes],
            bit_index: 0,
        };
        self.bytes = &self.bytes[fork_bytes..];
        forked_cursor
    }
}

impl ReadCursor<'_> {
    /// Reads a 1-bit unsigned integer
    #[inline]
    pub fn read_u1(&mut self) -> u8 {
        self.read_up_to_u8(1)
    }
    /// Reads a 2-bit unsigned integer
    #[inline]
    pub fn read_u2(&mut self) -> u8 {
        self.read_up_to_u8(2)
    }
    /// Reads a 3-bit unsigned integer
    #[inline]
    pub fn read_u3(&mut self) -> u8 {
        self.read_up_to_u8(3)
    }
    /// Reads a 4-bit unsigned integer
    #[inline]
    pub fn read_u4(&mut self) -> u8 {
        self.read_up_to_u8(4)
    }
    /// Reads a 5-bit unsigned integer
    #[inline]
    pub fn read_u5(&mut self) -> u8 {
        self.read_up_to_u8(5)
    }
    /// Reads a 6-bit unsigned integer
    #[inline]
    pub fn read_u6(&mut self) -> u8 {
        self.read_up_to_u8(6)
    }
    /// Reads a 7-bit unsigned integer
    #[inline]
    pub fn read_u7(&mut self) -> u8 {
        self.read_up_to_u8(7)
    }
    /// Reads a 8-bit unsigned integer
    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        self.read_up_to_u8(8)
    }
    /// Reads a 9-bit unsigned integer
    #[inline]
    pub fn read_u9(&mut self) -> u16 {
        self.read_up_to_u16(9)
    }
    /// Reads a 10-bit unsigned integer
    #[inline]
    pub fn read_u10(&mut self) -> u16 {
        self.read_up_to_u16(10)
    }
    /// Reads a 11-bit unsigned integer
    #[inline]
    pub fn read_u11(&mut self) -> u16 {
        self.read_up_to_u16(11)
    }
    /// Reads a 12-bit unsigned integer
    #[inline]
    pub fn read_u12(&mut self) -> u16 {
        self.read_up_to_u16(12)
    }
    /// Reads a 13-bit unsigned integer
    #[inline]
    pub fn read_u13(&mut self) -> u16 {
        self.read_up_to_u16(13)
    }
    /// Reads a 14-bit unsigned integer
    #[inline]
    pub fn read_u14(&mut self) -> u16 {
        self.read_up_to_u16(14)
    }
    /// Reads a 15-bit unsigned integer
    #[inline]
    pub fn read_u15(&mut self) -> u16 {
        self.read_up_to_u16(15)
    }
    /// Reads a 16-bit unsigned integer
    #[inline]
    pub fn read_u16(&mut self) -> u16 {
        self.read_up_to_u32(16) as u16
    }
    /// Reads a 17-bit unsigned integer
    #[inline]
    pub fn read_u17(&mut self) -> u32 {
        self.read_up_to_u32(17)
    }
    /// Reads a 18-bit unsigned integer
    #[inline]
    pub fn read_u18(&mut self) -> u32 {
        self.read_up_to_u32(18)
    }
    /// Reads a 19-bit unsigned integer
    #[inline]
    pub fn read_u19(&mut self) -> u32 {
        self.read_up_to_u32(19)
    }
    /// Reads a 20-bit unsigned integer
    #[inline]
    pub fn read_u20(&mut self) -> u32 {
        self.read_up_to_u32(20)
    }
    /// Reads a 21-bit unsigned integer
    #[inline]
    pub fn read_u21(&mut self) -> u32 {
        self.read_up_to_u32(21)
    }
    /// Reads a 22-bit unsigned integer
    #[inline]
    pub fn read_u22(&mut self) -> u32 {
        self.read_up_to_u32(22)
    }
    /// Reads a 23-bit unsigned integer
    #[inline]
    pub fn read_u23(&mut self) -> u32 {
        self.read_up_to_u32(23)
    }
    /// Reads a 24-bit unsigned integer
    #[inline]
    pub fn read_u24(&mut self) -> u32 {
        self.read_up_to_u32(24)
    }
    /// Reads a 25-bit unsigned integer
    #[inline]
    pub fn read_u25(&mut self) -> u32 {
        self.read_up_to_u32(25)
    }
    /// Reads a 26-bit unsigned integer
    #[inline]
    pub fn read_u26(&mut self) -> u32 {
        self.read_up_to_u32(26)
    }
    /// Reads a 27-bit unsigned integer
    #[inline]
    pub fn read_u27(&mut self) -> u32 {
        self.read_up_to_u32(27)
    }
    /// Reads a 28-bit unsigned integer
    #[inline]
    pub fn read_u28(&mut self) -> u32 {
        self.read_up_to_u32(28)
    }
    /// Reads a 29-bit unsigned integer
    #[inline]
    pub fn read_u29(&mut self) -> u32 {
        self.read_up_to_u32(29)
    }
    /// Reads a 30-bit unsigned integer
    #[inline]
    pub fn read_u30(&mut self) -> u32 {
        self.read_up_to_u32(30)
    }
    /// Reads a 31-bit unsigned integer
    #[inline]
    pub fn read_u31(&mut self) -> u32 {
        self.read_up_to_u32(31)
    }
    /// Reads a 32-bit unsigned integer
    #[inline]
    pub fn read_u32(&mut self) -> u32 {
        self.read_up_to_u64(32) as u32
    }
    /// Reads a 33-bit unsigned integer
    #[inline]
    pub fn read_u33(&mut self) -> u64 {
        self.read_up_to_u64(33)
    }
    /// Reads a 34-bit unsigned integer
    #[inline]
    pub fn read_u34(&mut self) -> u64 {
        self.read_up_to_u64(34)
    }
    /// Reads a 35-bit unsigned integer
    #[inline]
    pub fn read_u35(&mut self) -> u64 {
        self.read_up_to_u64(35)
    }
    /// Reads a 36-bit unsigned integer
    #[inline]
    pub fn read_u36(&mut self) -> u64 {
        self.read_up_to_u64(36)
    }
    /// Reads a 37-bit unsigned integer
    #[inline]
    pub fn read_u37(&mut self) -> u64 {
        self.read_up_to_u64(37)
    }
    /// Reads a 38-bit unsigned integer
    #[inline]
    pub fn read_u38(&mut self) -> u64 {
        self.read_up_to_u64(38)
    }
    /// Reads a 39-bit unsigned integer
    #[inline]
    pub fn read_u39(&mut self) -> u64 {
        self.read_up_to_u64(39)
    }
    /// Reads a 40-bit unsigned integer
    #[inline]
    pub fn read_u40(&mut self) -> u64 {
        self.read_up_to_u64(40)
    }
    /// Reads a 41-bit unsigned integer
    #[inline]
    pub fn read_u41(&mut self) -> u64 {
        self.read_up_to_u64(41)
    }
    /// Reads a 42-bit unsigned integer
    #[inline]
    pub fn read_u42(&mut self) -> u64 {
        self.read_up_to_u64(42)
    }
    /// Reads a 43-bit unsigned integer
    #[inline]
    pub fn read_u43(&mut self) -> u64 {
        self.read_up_to_u64(43)
    }
    /// Reads a 44-bit unsigned integer
    #[inline]
    pub fn read_u44(&mut self) -> u64 {
        self.read_up_to_u64(44)
    }
    /// Reads a 45-bit unsigned integer
    #[inline]
    pub fn read_u45(&mut self) -> u64 {
        self.read_up_to_u64(45)
    }
    /// Reads a 46-bit unsigned integer
    #[inline]
    pub fn read_u46(&mut self) -> u64 {
        self.read_up_to_u64(46)
    }
    /// Reads a 47-bit unsigned integer
    #[inline]
    pub fn read_u47(&mut self) -> u64 {
        self.read_up_to_u64(47)
    }
    /// Reads a 48-bit unsigned integer
    #[inline]
    pub fn read_u48(&mut self) -> u64 {
        self.read_up_to_u64(48)
    }
    /// Reads a 49-bit unsigned integer
    #[inline]
    pub fn read_u49(&mut self) -> u64 {
        self.read_up_to_u64(49)
    }
    /// Reads a 50-bit unsigned integer
    #[inline]
    pub fn read_u50(&mut self) -> u64 {
        self.read_up_to_u64(50)
    }
    /// Reads a 51-bit unsigned integer
    #[inline]
    pub fn read_u51(&mut self) -> u64 {
        self.read_up_to_u64(51)
    }
    /// Reads a 52-bit unsigned integer
    #[inline]
    pub fn read_u52(&mut self) -> u64 {
        self.read_up_to_u64(52)
    }
    /// Reads a 53-bit unsigned integer
    #[inline]
    pub fn read_u53(&mut self) -> u64 {
        self.read_up_to_u64(53)
    }
    /// Reads a 54-bit unsigned integer
    #[inline]
    pub fn read_u54(&mut self) -> u64 {
        self.read_up_to_u64(54)
    }
    /// Reads a 55-bit unsigned integer
    #[inline]
    pub fn read_u55(&mut self) -> u64 {
        self.read_up_to_u64(55)
    }
    /// Reads a 56-bit unsigned integer
    #[inline]
    pub fn read_u56(&mut self) -> u64 {
        self.read_up_to_u64(56)
    }
    /// Reads a 57-bit unsigned integer
    #[inline]
    pub fn read_u57(&mut self) -> u64 {
        self.read_up_to_u64(57)
    }
    /// Reads a 58-bit unsigned integer
    #[inline]
    pub fn read_u58(&mut self) -> u64 {
        self.read_up_to_u64(58)
    }
    /// Reads a 59-bit unsigned integer
    #[inline]
    pub fn read_u59(&mut self) -> u64 {
        self.read_up_to_u64(59)
    }
    /// Reads a 60-bit unsigned integer
    #[inline]
    pub fn read_u60(&mut self) -> u64 {
        self.read_up_to_u64(60)
    }
    /// Reads a 61-bit unsigned integer
    #[inline]
    pub fn read_u61(&mut self) -> u64 {
        self.read_up_to_u64(61)
    }
    /// Reads a 62-bit unsigned integer
    #[inline]
    pub fn read_u62(&mut self) -> u64 {
        self.read_up_to_u64(62)
    }
    /// Reads a 63-bit unsigned integer
    #[inline]
    pub fn read_u63(&mut self) -> u64 {
        self.read_up_to_u64(63)
    }
    /// Reads a 64-bit unsigned integer
    #[inline]
    pub fn read_u64(&mut self) -> u64 {
        self.read_up_to_u64(64)
    }
}

impl ReadCursor<'_> {
    /// Advances the cursor to skip 1 bit
    #[inline]
    pub fn skip_1(&mut self) {
        self.advance_bits(1)
    }
    /// Advances the cursor to skip 2 bits
    #[inline]
    pub fn skip_2(&mut self) {
        self.advance_bits(2)
    }
    /// Advances the cursor to skip 3 bits
    #[inline]
    pub fn skip_3(&mut self) {
        self.advance_bits(3)
    }
    /// Advances the cursor to skip 4 bits
    #[inline]
    pub fn skip_4(&mut self) {
        self.advance_bits(4)
    }
    /// Advances the cursor to skip 5 bits
    #[inline]
    pub fn skip_5(&mut self) {
        self.advance_bits(5)
    }
    /// Advances the cursor to skip 6 bits
    #[inline]
    pub fn skip_6(&mut self) {
        self.advance_bits(6)
    }
    /// Advances the cursor to skip 7 bits
    #[inline]
    pub fn skip_7(&mut self) {
        self.advance_bits(7)
    }
    /// Advances the cursor to skip 8 bits
    #[inline]
    pub fn skip_8(&mut self) {
        self.advance_bits(8)
    }
    /// Advances the cursor to skip 9 bits
    #[inline]
    pub fn skip_9(&mut self) {
        self.advance_bits(9)
    }
    /// Advances the cursor to skip 10 bits
    #[inline]
    pub fn skip_10(&mut self) {
        self.advance_bits(10)
    }
    /// Advances the cursor to skip 11 bits
    #[inline]
    pub fn skip_11(&mut self) {
        self.advance_bits(11)
    }
    /// Advances the cursor to skip 12 bits
    #[inline]
    pub fn skip_12(&mut self) {
        self.advance_bits(12)
    }
    /// Advances the cursor to skip 13 bits
    #[inline]
    pub fn skip_13(&mut self) {
        self.advance_bits(13)
    }
    /// Advances the cursor to skip 14 bits
    #[inline]
    pub fn skip_14(&mut self) {
        self.advance_bits(14)
    }
    /// Advances the cursor to skip 15 bits
    #[inline]
    pub fn skip_15(&mut self) {
        self.advance_bits(15)
    }
    /// Advances the cursor to skip 16 bits
    #[inline]
    pub fn skip_16(&mut self) {
        self.advance_bits(16)
    }
    /// Advances the cursor to skip 17 bits
    #[inline]
    pub fn skip_17(&mut self) {
        self.advance_bits(17)
    }
    /// Advances the cursor to skip 18 bits
    #[inline]
    pub fn skip_18(&mut self) {
        self.advance_bits(18)
    }
    /// Advances the cursor to skip 19 bits
    #[inline]
    pub fn skip_19(&mut self) {
        self.advance_bits(19)
    }
    /// Advances the cursor to skip 20 bits
    #[inline]
    pub fn skip_20(&mut self) {
        self.advance_bits(20)
    }
    /// Advances the cursor to skip 21 bits
    #[inline]
    pub fn skip_21(&mut self) {
        self.advance_bits(21)
    }
    /// Advances the cursor to skip 22 bits
    #[inline]
    pub fn skip_22(&mut self) {
        self.advance_bits(22)
    }
    /// Advances the cursor to skip 23 bits
    #[inline]
    pub fn skip_23(&mut self) {
        self.advance_bits(23)
    }
    /// Advances the cursor to skip 24 bits
    #[inline]
    pub fn skip_24(&mut self) {
        self.advance_bits(24)
    }
    /// Advances the cursor to skip 25 bits
    #[inline]
    pub fn skip_25(&mut self) {
        self.advance_bits(25)
    }
    /// Advances the cursor to skip 26 bits
    #[inline]
    pub fn skip_26(&mut self) {
        self.advance_bits(26)
    }
    /// Advances the cursor to skip 27 bits
    #[inline]
    pub fn skip_27(&mut self) {
        self.advance_bits(27)
    }
    /// Advances the cursor to skip 28 bits
    #[inline]
    pub fn skip_28(&mut self) {
        self.advance_bits(28)
    }
    /// Advances the cursor to skip 29 bits
    #[inline]
    pub fn skip_29(&mut self) {
        self.advance_bits(29)
    }
    /// Advances the cursor to skip 30 bits
    #[inline]
    pub fn skip_30(&mut self) {
        self.advance_bits(30)
    }
    /// Advances the cursor to skip 31 bits
    #[inline]
    pub fn skip_31(&mut self) {
        self.advance_bits(31)
    }
    /// Advances the cursor to skip 32 bits
    #[inline]
    pub fn skip_32(&mut self) {
        self.advance_bits(32)
    }
    /// Advances the cursor to skip 33 bits
    #[inline]
    pub fn skip_33(&mut self) {
        self.advance_bits(33)
    }
    /// Advances the cursor to skip 34 bits
    #[inline]
    pub fn skip_34(&mut self) {
        self.advance_bits(34)
    }
    /// Advances the cursor to skip 35 bits
    #[inline]
    pub fn skip_35(&mut self) {
        self.advance_bits(35)
    }
    /// Advances the cursor to skip 36 bits
    #[inline]
    pub fn skip_36(&mut self) {
        self.advance_bits(36)
    }
    /// Advances the cursor to skip 37 bits
    #[inline]
    pub fn skip_37(&mut self) {
        self.advance_bits(37)
    }
    /// Advances the cursor to skip 38 bits
    #[inline]
    pub fn skip_38(&mut self) {
        self.advance_bits(38)
    }
    /// Advances the cursor to skip 39 bits
    #[inline]
    pub fn skip_39(&mut self) {
        self.advance_bits(39)
    }
    /// Advances the cursor to skip 40 bits
    #[inline]
    pub fn skip_40(&mut self) {
        self.advance_bits(40)
    }
    /// Advances the cursor to skip 41 bits
    #[inline]
    pub fn skip_41(&mut self) {
        self.advance_bits(41)
    }
    /// Advances the cursor to skip 42 bits
    #[inline]
    pub fn skip_42(&mut self) {
        self.advance_bits(42)
    }
    /// Advances the cursor to skip 43 bits
    #[inline]
    pub fn skip_43(&mut self) {
        self.advance_bits(43)
    }
    /// Advances the cursor to skip 44 bits
    #[inline]
    pub fn skip_44(&mut self) {
        self.advance_bits(44)
    }
    /// Advances the cursor to skip 45 bits
    #[inline]
    pub fn skip_45(&mut self) {
        self.advance_bits(45)
    }
    /// Advances the cursor to skip 46 bits
    #[inline]
    pub fn skip_46(&mut self) {
        self.advance_bits(46)
    }
    /// Advances the cursor to skip 47 bits
    #[inline]
    pub fn skip_47(&mut self) {
        self.advance_bits(47)
    }
    /// Advances the cursor to skip 48 bits
    #[inline]
    pub fn skip_48(&mut self) {
        self.advance_bits(48)
    }
    /// Advances the cursor to skip 49 bits
    #[inline]
    pub fn skip_49(&mut self) {
        self.advance_bits(49)
    }
    /// Advances the cursor to skip 50 bits
    #[inline]
    pub fn skip_50(&mut self) {
        self.advance_bits(50)
    }
    /// Advances the cursor to skip 51 bits
    #[inline]
    pub fn skip_51(&mut self) {
        self.advance_bits(51)
    }
    /// Advances the cursor to skip 52 bits
    #[inline]
    pub fn skip_52(&mut self) {
        self.advance_bits(52)
    }
    /// Advances the cursor to skip 53 bits
    #[inline]
    pub fn skip_53(&mut self) {
        self.advance_bits(53)
    }
    /// Advances the cursor to skip 54 bits
    #[inline]
    pub fn skip_54(&mut self) {
        self.advance_bits(54)
    }
    /// Advances the cursor to skip 55 bits
    #[inline]
    pub fn skip_55(&mut self) {
        self.advance_bits(55)
    }
    /// Advances the cursor to skip 56 bits
    #[inline]
    pub fn skip_56(&mut self) {
        self.advance_bits(56)
    }
    /// Advances the cursor to skip 57 bits
    #[inline]
    pub fn skip_57(&mut self) {
        self.advance_bits(57)
    }
    /// Advances the cursor to skip 58 bits
    #[inline]
    pub fn skip_58(&mut self) {
        self.advance_bits(58)
    }
    /// Advances the cursor to skip 59 bits
    #[inline]
    pub fn skip_59(&mut self) {
        self.advance_bits(59)
    }
    /// Advances the cursor to skip 60 bits
    #[inline]
    pub fn skip_60(&mut self) {
        self.advance_bits(60)
    }
    /// Advances the cursor to skip 61 bits
    #[inline]
    pub fn skip_61(&mut self) {
        self.advance_bits(61)
    }
    /// Advances the cursor to skip 62 bits
    #[inline]
    pub fn skip_62(&mut self) {
        self.advance_bits(62)
    }
    /// Advances the cursor to skip 63 bits
    #[inline]
    pub fn skip_63(&mut self) {
        self.advance_bits(63)
    }
    /// Advances the cursor to skip 64 bits
    #[inline]
    pub fn skip_64(&mut self) {
        self.advance_bits(64)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u8_one() {
        let bytes = [0xABu8];
        let mut cursor = ReadCursor::new(&bytes);
        assert_eq!(cursor.read_u8(), 0xAB);
    }

    #[test]
    fn u16_one() {
        let bytes = [0xCDu8, 0xAB];
        let mut cursor = ReadCursor::new(&bytes);
        assert_eq!(cursor.read_u16(), 0xABCD);
    }

    #[test]
    fn u32_one() {
        let bytes = [0xD4u8, 0xC3, 0xB2, 0xA1];
        let mut cursor = ReadCursor::new(&bytes);
        assert_eq!(cursor.read_u32(), 0xA1B2C3D4);
    }

    #[test]
    fn u64_one() {
        let bytes = [0x67u8, 0x45, 0x23, 0x01, 0xD4, 0xC3, 0xB2, 0xA1];
        let mut cursor = ReadCursor::new(&bytes);
        assert_eq!(cursor.read_u64(), 0xA1B2C3D401234567);
    }

    #[test]
    fn f16_one() {
        let bytes = [0xCDu8, 0xAB];
        let mut cursor = ReadCursor::new(&bytes);
        assert_eq!(cursor.read_f16(), f16::from_bits(0xABCD));
    }

    #[test]
    fn f32_one() {
        let bytes = [0xD4u8, 0xC3, 0xB2, 0xA1];
        let mut cursor = ReadCursor::new(&bytes);
        assert_eq!(cursor.read_f32(), f32::from_bits(0xA1B2C3D4));
    }

    #[test]
    fn f64_one() {
        let bytes = [0x67u8, 0x45, 0x23, 0x01, 0xD4, 0xC3, 0xB2, 0xA1];
        let mut cursor = ReadCursor::new(&bytes);
        assert_eq!(cursor.read_f64(), f64::from_bits(0xA1B2C3D401234567));
    }
}
