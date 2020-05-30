#![no_std]

extern crate half;

mod cursor;

pub use crate::cursor::encode::EncodeCursor;

use core::cmp;
use core::mem::{self, MaybeUninit};
use core::ptr;
use core::slice;

/// Trait for types that can be encoded into UAVCAN transfers, or decoded from transfers
pub trait DataType {
    /// Returns a zero-copy encoding implementation for this type, if one exists
    ///
    /// The default implementation returns None.
    fn zero_copy(&self) -> Option<&dyn ZeroCopy> {
        None
    }

    /// Returns the size of the encoded form of this data type, in bits
    fn size_bits(&self) -> usize;

    /// Encodes this value into a buffer
    ///
    /// The provided cursor will allow writing at least the number of bits returned by the
    /// size_bits() function.
    fn encode(&self, cursor: &mut EncodeCursor<'_>);

    /// Decodes a value, replacing the content of self with the decoded value
    ///
    /// In accordance with section 3.7.1.5 of the specification, the provided byte slice may be
    /// shorter than expected for this data type. This function must assume that all bytes not
    /// provided are zero, as if bytes had an infinite sequence of zero values at the end.
    fn decode_in_place(&mut self, bytes: &[u8]);

    /// Decodes a value and returns it
    ///
    /// In accordance with section 3.7.1.5 of the specification, the provided byte slice may be
    /// shorter than expected for this data type. This function must assume that all bytes not
    /// provided are zero, as if bytes had an infinite sequence of zero values at the end.
    fn decode(bytes: &[u8]) -> Self
    where
        Self: Sized;
}

/// A trait for data types that have an in-memory representation that exactly matches their
/// encoded representation
pub unsafe trait ZeroCopy {
    /// Returns a slice of bytes that represent this value
    fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, mem::size_of_val(&self)) }
    }

    /// Returns a mutable slice of bytes that represent this value, and can be used to modify it
    fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut u8, mem::size_of_val(&self)) }
    }

    /// Creates a reference to a value of this type from a slice
    ///
    /// This function panics if the length of bytes is less than the size of this type.
    fn from_slice(bytes: &[u8]) -> &Self
    where
        Self: Sized,
    {
        let self_size = mem::size_of::<Self>();
        assert!(bytes.len() >= self_size);
        unsafe { &*(bytes.as_ptr() as *const Self) }
    }
}

impl<T> DataType for T
where
    T: ZeroCopy,
{
    fn zero_copy(&self) -> Option<&dyn ZeroCopy> {
        Some(self)
    }

    fn size_bits(&self) -> usize {
        mem::size_of_val(self) * 8
    }

    fn encode(&self, cursor: &mut EncodeCursor<'_>) {
        cursor.write_bytes(self.as_slice());
    }

    fn decode_in_place(&mut self, bytes: &[u8]) {
        unsafe {
            decode_in_place_ptr(self as *mut Self as *mut u8, mem::size_of_val(self), bytes);
        }
    }

    fn decode(bytes: &[u8]) -> Self
    where
        Self: Sized,
    {
        let mut value = MaybeUninit::uninit();
        unsafe {
            decode_in_place_ptr(value.as_mut_ptr() as *mut u8, mem::size_of::<Self>(), bytes);
            value.assume_init()
        }
    }
}

/// Copies up to size bytes to a "this" location. If bytes.len() < size, fills the remaining
/// bytes with zeros
unsafe fn decode_in_place_ptr(this: *mut u8, size: usize, bytes: &[u8]) {
    let copy_length = cmp::min(size, bytes.len());
    ptr::copy_nonoverlapping(bytes.as_ptr(), this, copy_length);
    // Fill in this with extra zeroes
    // This is safe even if bytes.len() > size, see the source:
    // https://doc.rust-lang.org/stable/src/core/iter/range.rs.html#207-263
    // In that case, it will have no effect.
    for i in bytes.len()..size {
        let this_offset = this.add(i);
        ptr::write(this_offset, 0u8);
    }
}


unsafe impl ZeroCopy for u8 {}
unsafe impl ZeroCopy for i8 {}

// Implement ZeroCopy for multi-byte primitive types on little endian targets
#[cfg(target_endian = "little")]
mod primitive_zero_copy {
    use half::f16;
    use super::ZeroCopy;

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
