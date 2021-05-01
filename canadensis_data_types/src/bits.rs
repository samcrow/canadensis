use canadensis_encoding::{ReadCursor, WriteCursor};

/// A set of bits in a format compatible with UAVCAN serialization
///
/// Because the const generics feature is incomplete, the integer generic parameter is a number
/// of bytes (= 8 bits), not a number of bits. The functions still use bit indexes.
#[derive(Debug, Clone)]
pub struct BitSet<const BYTES: usize> {
    bytes: [u8; BYTES],
    bit_length: usize,
}

impl<const BYTES: usize> BitSet<BYTES> {
    /// Creates a bit set with all bits set to zero
    pub fn new(bit_length: usize) -> Self {
        assert!(bit_length <= BYTES * 8);
        BitSet {
            bytes: [0; BYTES],
            bit_length,
        }
    }

    /// Returns the value of a bit at the provided bit index
    pub fn get(&self, bit_index: usize) -> bool {
        let (byte_index, bit_in_byte) = self.split_index(bit_index);
        let byte = self.bytes[byte_index];
        let bit = (byte >> bit_in_byte) & 1;
        bit == 1
    }

    /// Sets the value of a bit at the provided bit index
    pub fn set(&mut self, bit_index: usize, value: bool) {
        let (byte_index, bit_in_byte) = self.split_index(bit_index);
        let mask = 1 << bit_in_byte;
        let byte = &mut self.bytes[byte_index];
        if value {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    /// Serializes this bit set
    ///
    /// Note: This doesn't implement DataType, Serialize, or Deserialize because it is not a
    /// composite type and has an alignment of only 1 bit.
    pub fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        if self.bit_length == BYTES * 8 && cursor.is_aligned_to_8_bits() {
            cursor.write_aligned_bytes(&self.bytes);
        } else {
            for bit_index in 0..self.bit_length {
                cursor.write_bool(self.get(bit_index));
            }
        }
    }

    pub fn deserialize_in_place(&mut self, cursor: &mut ReadCursor<'_>) {
        if self.bit_length % 8 == 0 && cursor.is_aligned_to_8_bits() {
            self.bytes.fill_with(|| cursor.read_aligned_u8());
        } else {
            for i in 0..self.bit_length {
                self.set(i, cursor.read_bool());
            }
        }
    }

    pub fn deserialize(bit_length: usize, cursor: &mut ReadCursor<'_>) -> Self {
        let mut set = BitSet::new(bit_length);
        set.deserialize_in_place(cursor);
        set
    }

    fn split_index(&self, bit_index: usize) -> (usize, u8) {
        assert!(bit_index < self.bit_length);
        // The UAVCAN serialization makes this simple
        let byte = bit_index / 8;
        let bit_in_byte = (bit_index % 8) as u8;
        (byte, bit_in_byte)
    }
}
