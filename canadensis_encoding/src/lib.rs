#![cfg_attr(not(test), no_std)]

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
