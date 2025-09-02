//! Reassembles UDP packets into transfers

use alloc::vec::Vec;

use fallible_collections::{FallibleVec, TryReserveError};

use canadensis_core::crc::CrcTracker;
use canadensis_core::{OutOfMemoryError, Priority};
use canadensis_header::Header;

use crate::UdpTransferId;

// TODO: Add support for reassembling out-of-order frames

/// Collects UDP packets and reassembles them into a transfer
///
/// This checks the following properties:
/// * Frames have consecutive frame indices (out-of-order reassembly is not currently supported)
/// * Frames have the same transfer ID
/// * Frames have the same priority
///
#[derive(Debug)]
pub struct Buildup {
    /// The transfer bytes (not including UDP frame headers) that have been collected so far,
    /// excluding any bytes after `max_length`
    bytes: Vec<u8>,
    /// The CRC of all bytes collected so far
    crc: CrcTracker,
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
        header: &Header,
        bytes_after_header: &[u8],
        max_length: usize,
    ) -> Result<Self, OutOfMemoryError> {
        let mut bytes: Vec<u8> = FallibleVec::try_with_capacity(max_length)?;
        let mut crc = CrcTracker::new();
        bytes_after_header.iter().for_each(|&byte| {
            if let Some(digested) = crc.digest(byte) {
                if bytes.len() < bytes.capacity() {
                    bytes.push(digested);
                }
            }
        });

        Ok(Buildup {
            bytes,
            crc,
            next_frame_index: header.frame_index + 1,
            priority: header.priority,
            transfer_id: header.transfer_id,
        })
    }

    pub fn crc_correct(&self) -> bool {
        self.crc.correct()
    }

    /// Adds a frame to this buildup
    ///
    /// This function does not use dynamic memory allocation.
    pub fn push(&mut self, header: &Header, bytes_after_header: &[u8]) -> Result<(), BuildupError> {
        if header.frame_index != self.next_frame_index {
            return Err(BuildupError::Index);
        }
        if header.transfer_id != self.transfer_id {
            return Err(BuildupError::TransferId);
        }
        if header.priority != self.priority {
            return Err(BuildupError::Priority);
        }
        bytes_after_header.iter().for_each(|&byte| {
            if let Some(digested) = self.crc.digest(byte) {
                if self.bytes.len() < self.bytes.capacity() {
                    self.bytes.push(digested);
                }
            }
        });
        self.next_frame_index += 1;
        Ok(())
    }

    /// Consumes this buildup and returns the payload bytes
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
    /// Ran out of memory
    Memory(OutOfMemoryError),
    /// The payload CRC was incorrect
    Crc,
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
