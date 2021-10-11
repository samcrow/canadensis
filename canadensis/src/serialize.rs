use crate::encoding::{Serialize, WriteCursor};
use alloc::vec::Vec;
use canadensis_core::OutOfMemoryError;
use core::iter;
use fallible_collections::{FallibleVec, TryReserveError};

/// Payloads above this size (in bytes) will use a dynamically allocated buffer
const STACK_THRESHOLD: usize = 64;

/// Serializes a payload into a buffer and passes the buffer to a closure
pub(crate) fn do_serialize<T, F, R, E>(payload: &T, operation: F) -> Result<R, E>
where
    T: Serialize,
    F: FnOnce(&[u8]) -> Result<R, E>,
    E: From<OutOfMemoryError>,
{
    let payload_bytes = (payload.size_bits() + 7) / 8;
    if payload_bytes > STACK_THRESHOLD {
        let mut bytes: Vec<u8> = FallibleVec::try_with_capacity(payload_bytes)
            .map_err(|e: TryReserveError| E::from(OutOfMemoryError::from(e)))?;
        bytes.extend(iter::repeat(0).take(payload_bytes));
        payload.serialize(&mut WriteCursor::new(&mut bytes));
        operation(&bytes)
    } else {
        let mut bytes = [0u8; STACK_THRESHOLD];
        let bytes = &mut bytes[..payload_bytes];
        payload.serialize(&mut WriteCursor::new(bytes));
        operation(bytes)
    }
}
