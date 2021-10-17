//! Reassembles UDP packets into transfers

use crate::header::ValidatedUdpHeader;
use crate::UdpTransferId;
use canadensis_core::{OutOfMemoryError, Priority};
use fallible_collections::{FallibleVec, TryReserveError};

// TODO: Add support for reassembling out-of-order frames

/// Collects UDP packets, reassembles them int a transfer, and calculates the CRC of the
/// entire payload (not including the header in each packet)
#[derive(Debug)]
pub struct Buildup {
    /// The transfer bytes (not including UDP frame headers) that have been collected so far
    bytes: Vec<u8>,
    /// The expected index of the next frame
    next_frame_index: u32,
    /// The priority of the first frame, which all other frames should match
    priority: Priority,
    /// The transfer ID of the first frame, which all other frames should match
    transfer_id: UdpTransferId,
}

impl Buildup {
    /// Creates a buildup from an initial frame
    ///
    /// This function attempts to allocate space for `max_length` payload bytes and returns an error
    /// if the allocation fails.
    pub fn new(
        header: &ValidatedUdpHeader,
        bytes_after_header: &[u8],
        max_length: usize,
    ) -> Result<Self, BuildupError> {
        if bytes_after_header.len() > max_length {
            return Err(BuildupError::Length);
        }
        let mut bytes: Vec<u8> = FallibleVec::try_with_capacity(max_length)?;
        bytes.extend_from_slice(bytes_after_header);

        Ok(Buildup {
            bytes,
            next_frame_index: header.frame_index + 1,
            priority: header.priority,
            transfer_id: header.transfer_id,
        })
    }

    /// Adds a frame to this buildup
    ///
    /// This function does not use dynamic memory allocation.
    pub fn push(
        &mut self,
        header: &ValidatedUdpHeader,
        bytes_after_header: &[u8],
    ) -> Result<(), BuildupError> {
        if header.frame_index != self.next_frame_index {
            return Err(BuildupError::Index);
        }
        if header.transfer_id != self.transfer_id {
            return Err(BuildupError::TransferId);
        }
        if header.priority != self.priority {
            return Err(BuildupError::Priority);
        }
        if self.bytes.len() + bytes_after_header.len() > self.bytes.capacity() {
            return Err(BuildupError::Length);
        }
        self.bytes.extend_from_slice(bytes_after_header);
        Ok(())
    }

    /// Consumes this buildup and returns the payload bytes (possibly including a CRC at the end)
    pub fn into_payload(self) -> Vec<u8> {
        self.bytes
    }
}

/// Errors that the buildup may produce
#[derive(Debug)]
pub enum BuildupError {
    /// The frame index did not match
    Index,
    /// The frame priority did not match, or a header had an invalid priority value
    Priority,
    /// The frame transfer ID did not match
    TransferId,
    /// The reassembled transfer was too long
    Length,
    /// Ran out of memory
    Memory(OutOfMemoryError),
}

impl From<OutOfMemoryError> for BuildupError {
    fn from(oom: OutOfMemoryError) -> Self {
        BuildupError::Memory(oom)
    }
}
impl From<TryReserveError> for BuildupError {
    fn from(inner: TryReserveError) -> Self {
        BuildupError::Memory(inner.into())
    }
}
