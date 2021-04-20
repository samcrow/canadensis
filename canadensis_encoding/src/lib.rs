#![no_std]

extern crate half;

mod cursor;

pub use crate::cursor::deserialize::ReadCursor;
pub use crate::cursor::serialize::WriteCursor;

use core::mem::{self, MaybeUninit};
use core::slice;

pub trait LengthSet {}

/// Whether a type is sealed or delimited
pub enum Extensibility {
    /// A sealed type
    Sealed,
    /// A delimited type
    ///
    /// The enclosed value is the type's extent in bytes (not bits)
    ///
    /// Because the extent must be a multiple of 8 bits, no information is lost by storing
    /// it as a number of bytes
    Delimited(u32),
}

impl Extensibility {
    /// Returns true if this is `Delimited`
    pub fn is_delimited(&self) -> bool {
        matches!(*self, Extensibility::Delimited(_))
    }
}

/// Trait for types that can be encoded into UAVCAN transfers, or decoded from transfers
pub trait DataType {
    /// The sealed or delimited property of this type
    const EXTENSIBILITY: Extensibility;

    /// Returns a zero-copy encoding implementation for this type, if one exists
    ///
    /// The default implementation returns None.
    fn zero_copy(&self) -> Option<&dyn ZeroCopy> {
        None
    }
}

/// Trait for types that can be serialized into UAVCAN transfers
pub trait Serialize: DataType {
    /// Returns the size of the encoded form of this value, in bits
    ///
    /// For composite types, this must be a multiple of 8.
    fn size_bits(&self) -> usize;

    /// Serializes this value into a buffer
    ///
    /// The provided cursor will allow writing at least the number of bits returned by the
    /// size_bits() function.
    fn serialize(&self, cursor: &mut WriteCursor<'_>);
}

/// Trait for types that can be deserialized from UAVCAN transfers
pub trait Deserialize: DataType {
    /// Returns true if the provided number of bits is in this type's bit length set
    ///
    /// For composite types, this function must not return true for any input that is not
    /// a multiple of 8.
    fn in_bit_length_set(bit_length: usize) -> bool;

    /// Deserializes a value, replacing the content of self with the decoded value
    fn deserialize_in_place(&mut self, cursor: &mut ReadCursor<'_>)
        -> Result<(), DeserializeError>;

    /// Deserializes a value and returns it
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

/// A trait for data types that have an in-memory representation that exactly matches their
/// encoded representation
///
/// In addition, a type that implements ZeroCopy must be valid for all possible values of the
/// memory where it is stored. It must not contain any types that have niches, like references (&T).
pub unsafe trait ZeroCopy {
    /// Returns a slice that points to the same memory as self
    fn as_slice(&self) -> &[u8] {
        zero_copy_as_slice(self)
    }
}

impl<T> Serialize for T
where
    T: ZeroCopy + DataType,
{
    fn size_bits(&self) -> usize {
        mem::size_of_val(self) * 8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_bytes(zero_copy_as_slice(self));
    }
}
impl<T> Deserialize for T
where
    T: ZeroCopy + DataType,
{
    fn in_bit_length_set(bit_length: usize) -> bool {
        // Only one valid bit length
        bit_length == mem::size_of::<T>() * 8
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        cursor.read_bytes(zero_copy_as_slice_mut(self));
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = MaybeUninit::<Self>::uninit();
        unsafe {
            for i in 0..mem::size_of::<Self>() {
                let value_ptr = (value.as_mut_ptr() as *mut u8).add(i);
                *value_ptr = cursor.read_u8();
            }
            Ok(value.assume_init())
        }
    }
}

/// Returns a slice of bytes that represent a value
fn zero_copy_as_slice<T>(value: &T) -> &[u8]
where
    T: ZeroCopy + ?Sized,
{
    unsafe { slice::from_raw_parts(value as *const T as *const u8, mem::size_of_val(value)) }
}

/// Returns a mutable slice of bytes that represent a value, and can be used to modify it
fn zero_copy_as_slice_mut<T>(value: &mut T) -> &mut [u8]
where
    T: ZeroCopy,
{
    unsafe { slice::from_raw_parts_mut(value as *mut T as *mut u8, mem::size_of_val(value)) }
}

unsafe impl ZeroCopy for u8 {}
unsafe impl ZeroCopy for i8 {}

// Implement ZeroCopy for multi-byte primitive types on little endian targets
#[cfg(target_endian = "little")]
mod primitive_zero_copy {
    use super::ZeroCopy;
    use half::f16;

    unsafe impl ZeroCopy for u16 {}
    unsafe impl ZeroCopy for u32 {}
    unsafe impl ZeroCopy for u64 {}
    unsafe impl ZeroCopy for u128 {}
    unsafe impl ZeroCopy for i16 {}
    unsafe impl ZeroCopy for i32 {}
    unsafe impl ZeroCopy for i64 {}
    unsafe impl ZeroCopy for i128 {}

    unsafe impl ZeroCopy for f16 {}
    unsafe impl ZeroCopy for f32 {}
    unsafe impl ZeroCopy for f64 {}
}

/// Errors that can occur when deserializing
#[non_exhaustive]
#[derive(Debug)]
pub enum DeserializeError {
    /// A variable-length array length field was greater than the maximum allowed length
    ArrayLength,
    /// A union tag field did not correspond to a known variant
    UnionTag,
    /// A delimiter header had a length that was not valid for the expected type
    DelimitedLength,
}
