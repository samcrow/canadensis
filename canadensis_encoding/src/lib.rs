//!
//! # UAVCAN data type serialization and deserialization
//!

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

extern crate half;

mod cursor;

pub use crate::cursor::deserialize::ReadCursor;
pub use crate::cursor::serialize::WriteCursor;

/// Trait for types that can be encoded into UAVCAN transfers, or decoded from transfers
pub trait DataType {
    /// The sealed or delimited property of this type
    const EXTENT_BYTES: Option<u32>;
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
    /// [`size_bits()`](#tymethod.size_bits) function.
    fn serialize(&self, cursor: &mut WriteCursor<'_>);

    /// A convenience function that creates a cursor around the provided bytes and calls
    /// [`serialize`](#tymethod.serialize)
    fn serialize_to_bytes(&self, bytes: &mut [u8]) {
        let mut cursor = WriteCursor::new(bytes);
        self.serialize(&mut cursor);
    }
}

/// Trait for types that can be deserialized from UAVCAN transfers
pub trait Deserialize: DataType {
    /// Deserializes a value and returns it
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized;

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
