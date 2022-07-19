//!
//! # Cyphal data type serialization and deserialization
//!

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

extern crate half;
extern crate zerocopy;

pub mod bits;
mod cursor;

pub use crate::cursor::deserialize::ReadCursor;
pub use crate::cursor::serialize::WriteCursor;
use core::cmp;
use zerocopy::{AsBytes, FromBytes};

/// Trait for types that can be encoded into Cyphal transfers, or decoded from transfers
pub trait DataType {
    /// The sealed or delimited property of this type
    const EXTENT_BYTES: Option<u32>;
}

/// Trait for types that can be serialized into Cyphal transfers
pub trait Serialize: DataType {
    /// Returns the size of the encoded form of this value, in bits
    ///
    /// The returned value may not be a multiple of 8.
    fn size_bits(&self) -> usize;

    /// Serializes this value into a buffer
    ///
    /// The provided cursor will allow writing at least the number of bits returned by the
    /// [`size_bits()`](#tymethod.size_bits) function.
    fn serialize(&self, cursor: &mut WriteCursor<'_>);

    /// A convenience function that creates a cursor around the provided bytes and calls
    /// [`serialize`](#tymethod.serialize)
    fn serialize_to_bytes(&self, bytes: &mut [u8]) {
        let mut cursor = WriteCursor::new(bytes);
        self.serialize(&mut cursor);
    }
}

/// Trait for types that can be deserialized from Cyphal transfers
pub trait Deserialize: DataType {
    /// Deserializes a value and returns it
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized;

    /// Deserializes a value from a slice of bytes and returns it
    ///
    /// This is available only for types that implement [`Sized`], [`AsBytes`], and [`FromBytes`].
    ///
    /// # Panics
    ///
    /// This function panics if the provided cursor is not aligned to a byte boundary.
    fn deserialize_zero_copy(cursor: &mut ReadCursor<'_>) -> Self
    where
        Self: Sized + AsBytes + FromBytes,
    {
        // This isn't quite zero-copy. It's one-copy, but it eliminates handling each field
        // individually.
        let cursor_bytes = cursor.as_bytes().expect("Cursor not aligned");
        let mut value = Self::new_zeroed();

        let value_bytes = value.as_bytes_mut();
        // To apply implicit truncation and zero extension, copy whatever bytes we can
        let bytes_to_copy = cmp::min(value_bytes.len(), cursor_bytes.len());
        value_bytes[..bytes_to_copy].copy_from_slice(&cursor_bytes[..bytes_to_copy]);

        cursor.advance_bytes(bytes_to_copy);

        value
    }

    /// A convenience function that creates a cursor around the provided bytes and calls
    /// [`deserialize`](#tymethod.deserialize)
    fn deserialize_from_bytes(bytes: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut cursor = ReadCursor::new(bytes);
        Self::deserialize(&mut cursor)
    }
}

/// Marker for message data types
pub trait Message {}
/// Marker for service request data types
pub trait Request {}
/// Marker for service response data types
pub trait Response {}

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
