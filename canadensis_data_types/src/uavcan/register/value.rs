use crate::bits::BitArray;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};
use core::fmt;
use half::f16;

/// uavcan.register.Value 1.0
///
/// This type is hand-written to avoid having to hand-write a separate type for each variant.
#[derive(PartialEq, Clone)]
pub enum Value {
    Empty,
    String(heapless::Vec<u8, 256>),
    Unstructured(heapless::Vec<u8, 256>),
    /// Up to 2048 bits
    Bit(BitArray<{ 2048 / 8 }>),

    Integer64(heapless::Vec<i64, 32>),
    Integer32(heapless::Vec<i32, 64>),
    Integer16(heapless::Vec<i16, 128>),
    Integer8(heapless::Vec<i8, 256>),

    Natural64(heapless::Vec<u64, 32>),
    Natural32(heapless::Vec<u32, 64>),
    Natural16(heapless::Vec<u16, 128>),
    Natural8(heapless::Vec<u8, 256>),

    Real64(heapless::Vec<f64, 32>),
    Real32(heapless::Vec<f32, 64>),
    Real16(heapless::Vec<f16, 128>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Empty => f.debug_tuple("Empty").finish(),
            Value::String(_) => f.debug_tuple("String").finish(),
            Value::Unstructured(_) => f.debug_tuple("Unstructured").finish(),
            Value::Bit(_) => f.debug_tuple("Bit").finish(),
            Value::Integer64(_) => f.debug_tuple("Integer64").finish(),
            Value::Integer32(_) => f.debug_tuple("Integer32").finish(),
            Value::Integer16(_) => f.debug_tuple("Integer16").finish(),
            Value::Integer8(_) => f.debug_tuple("Integer8").finish(),
            Value::Natural64(_) => f.debug_tuple("Natural64").finish(),
            Value::Natural32(_) => f.debug_tuple("Natural32").finish(),
            Value::Natural16(_) => f.debug_tuple("Natural16").finish(),
            Value::Natural8(_) => f.debug_tuple("Natural8").finish(),
            Value::Real64(_) => f.debug_tuple("Real64").finish(),
            Value::Real32(_) => f.debug_tuple("Real32").finish(),
            Value::Real16(_) => f.debug_tuple("Real16").finish(),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Empty
    }
}

impl Message for Value {}

impl DataType for Value {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for Value {
    fn size_bits(&self) -> usize {
        let tag_bits = 8;
        let variant_bits = match self {
            Value::Empty => 0,
            Value::String(bytes) | Value::Unstructured(bytes) => 16 + 8 * bytes.len(),
            Value::Bit(bits) => 16 + bits.len(),
            Value::Integer64(values) => 8 + 64 * values.len(),
            Value::Integer32(values) => 8 + 32 * values.len(),
            Value::Integer16(values) => 8 + 16 * values.len(),
            // Integer8 and Natural8 need a 16-bit length because they can contain 256 values
            Value::Integer8(values) => 16 + 8 * values.len(),
            Value::Natural64(values) => 8 + 64 * values.len(),
            Value::Natural32(values) => 8 + 32 * values.len(),
            Value::Natural16(values) => 8 + 16 * values.len(),
            // Integer8 and Natural8 need a 16-bit length because they can contain 256 values
            Value::Natural8(values) => 16 + 8 * values.len(),
            Value::Real64(values) => 8 + 64 * values.len(),
            Value::Real32(values) => 8 + 32 * values.len(),
            Value::Real16(values) => 8 + 16 * values.len(),
        };
        tag_bits + variant_bits
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        match self {
            Value::Empty => {
                // Tag only
                cursor.write_aligned_u8(0);
            }
            Value::String(bytes) => {
                cursor.write_aligned_u8(1);
                cursor.write_aligned_u16(bytes.len() as u16);
                cursor.write_aligned_bytes(&bytes);
            }
            Value::Unstructured(bytes) => {
                cursor.write_aligned_u8(2);
                cursor.write_aligned_u16(bytes.len() as u16);
                cursor.write_aligned_bytes(&bytes);
            }
            Value::Bit(bit_array) => {
                cursor.write_aligned_u8(3);
                cursor.write_aligned_u16(bit_array.len() as u16);
                bit_array.serialize(cursor);
            }
            Value::Integer64(values) => {
                cursor.write_aligned_u8(4);
                cursor.write_aligned_u8(values.len() as u8);
                for value in values {
                    cursor.write_aligned_u64(*value as u64);
                }
            }
            Value::Integer32(values) => {
                cursor.write_aligned_u8(5);
                cursor.write_aligned_u8(values.len() as u8);
                for value in values {
                    cursor.write_aligned_u32(*value as u32);
                }
            }
            Value::Integer16(values) => {
                cursor.write_aligned_u8(6);
                cursor.write_aligned_u8(values.len() as u8);
                for value in values {
                    cursor.write_aligned_u16(*value as u16);
                }
            }
            Value::Integer8(values) => {
                cursor.write_aligned_u8(7);
                cursor.write_aligned_u16(values.len() as u16);
                for value in values {
                    cursor.write_aligned_u8(*value as u8);
                }
            }
            Value::Natural64(values) => {
                cursor.write_aligned_u8(8);
                cursor.write_aligned_u8(values.len() as u8);
                for value in values {
                    cursor.write_u64(*value);
                }
            }
            Value::Natural32(values) => {
                cursor.write_aligned_u8(9);
                cursor.write_aligned_u8(values.len() as u8);
                for value in values {
                    cursor.write_u32(*value);
                }
            }
            Value::Natural16(values) => {
                cursor.write_aligned_u8(10);
                cursor.write_aligned_u8(values.len() as u8);
                for value in values {
                    cursor.write_u16(*value);
                }
            }
            Value::Natural8(values) => {
                cursor.write_aligned_u8(11);
                cursor.write_aligned_u16(values.len() as u16);
                for value in values {
                    cursor.write_aligned_u8(*value);
                }
            }
            Value::Real64(values) => {
                cursor.write_aligned_u8(12);
                cursor.write_aligned_u8(values.len() as u8);
                for value in values {
                    cursor.write_f64(*value)
                }
            }
            Value::Real32(values) => {
                cursor.write_aligned_u8(13);
                cursor.write_aligned_u8(values.len() as u8);
                for value in values {
                    cursor.write_f32(*value)
                }
            }
            Value::Real16(values) => {
                cursor.write_aligned_u8(14);
                cursor.write_aligned_u8(values.len() as u8);
                for value in values {
                    cursor.write_f16(*value)
                }
            }
        }
    }
}

impl Deserialize for Value {
    fn in_bit_length_set(bit_length: usize) -> bool {
        // This might be too permissive, but since the type is sealed it won't really be used.
        if bit_length % 8 == 0 {
            let byte_length = bit_length / 8;
            (1..=259).contains(&byte_length)
        } else {
            false
        }
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        let tag = cursor.read_aligned_u8();
        match tag {
            // Empty
            0 => *self = Value::Empty,
            1 | 2 => {
                // String or Unstructured
                let length = cursor.read_aligned_u16();
                let mut bytes = heapless::Vec::new();
                if usize::from(length) <= bytes.capacity() {
                    for _ in 0..length {
                        bytes.push(cursor.read_aligned_u8()).unwrap();
                    }
                    if tag == 1 {
                        *self = Value::String(bytes);
                    } else {
                        *self = Value::Unstructured(bytes);
                    }
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            3 => {
                // Bit
                let length: usize = cursor.read_aligned_u16().into();
                if length <= 2048 {
                    let bits = BitArray::deserialize(length, cursor);
                    *self = Value::Bit(bits)
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            4 => {
                // Integer64
                let length = cursor.read_aligned_u8();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_aligned_u64() as i64).unwrap();
                    }
                    *self = Value::Integer64(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            5 => {
                // Integer32
                let length = cursor.read_aligned_u8();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_aligned_u32() as i32).unwrap();
                    }
                    *self = Value::Integer32(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            6 => {
                // Integer16
                let length = cursor.read_aligned_u8();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_aligned_u16() as i16).unwrap();
                    }
                    *self = Value::Integer16(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            7 => {
                // Integer8
                let length = cursor.read_aligned_u16();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_aligned_u8() as i8).unwrap();
                    }
                    *self = Value::Integer8(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            8 => {
                // Natural64
                let length = cursor.read_aligned_u8();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_aligned_u64()).unwrap();
                    }
                    *self = Value::Natural64(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            9 => {
                // Natural32
                let length = cursor.read_aligned_u8();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_aligned_u32()).unwrap();
                    }
                    *self = Value::Natural32(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            10 => {
                // Natural16
                let length = cursor.read_aligned_u8();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_aligned_u16()).unwrap();
                    }
                    *self = Value::Natural16(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            11 => {
                // Natural8
                let length = cursor.read_aligned_u16();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_aligned_u8()).unwrap();
                    }
                    *self = Value::Natural8(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            12 => {
                // Real64
                let length = cursor.read_aligned_u8();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_f64()).unwrap();
                    }
                    *self = Value::Real64(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            13 => {
                // Real32
                let length = cursor.read_aligned_u8();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_f32()).unwrap();
                    }
                    *self = Value::Real32(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            14 => {
                // Real16
                let length = cursor.read_aligned_u8();
                let mut values = heapless::Vec::new();
                if usize::from(length) <= values.capacity() {
                    for _ in 0..length {
                        values.push(cursor.read_f16()).unwrap();
                    }
                    *self = Value::Real16(values);
                } else {
                    return Err(DeserializeError::ArrayLength);
                }
            }
            _ => return Err(DeserializeError::UnionTag),
        }
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = Value::Empty;
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
