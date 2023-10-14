use crate::rx::buildup::{Buildup, BuildupError};
use crate::rx::TailByte;
use crate::types::{CanTransferId, Header, Transfer};
use crate::{Frame, TransferCrc};
use alloc::vec::Vec;
use canadensis_core::time::{MicrosecondDuration32, Microseconds32};
use canadensis_core::OutOfMemoryError;
use core::fmt::Debug;

/// A receive session, associated with a particular port ID and source node
#[derive(Debug)]
pub struct Session {
    /// Timestamp of the first frame received in this transfer
    transfer_timestamp: Microseconds32,
    /// Loopback flag of the first frame received in this transfer
    loopback: bool,
    /// Transfer reassembly
    buildup: Buildup,
}

impl Session {
    /// Creates a new session
    ///
    /// This function attempts to allocate `max_payload_length` bytes of memory, which will be
    /// used to assemble the received frames.
    pub fn new(
        transfer_timestamp: Microseconds32,
        transfer_id: CanTransferId,
        max_payload_length: usize,
        loopback: bool,
    ) -> Result<Self, OutOfMemoryError> {
        Ok(Session {
            transfer_timestamp,
            loopback,
            buildup: Buildup::new(transfer_id, max_payload_length)?,
        })
    }

    /// Accepts a frame associated with this session
    ///
    /// If this frame completes a transfer, this function returns the transfer.
    ///
    /// The `max_payload_length` value must include space for the transfer CRC and/or padding bytes
    /// that may be inserted, depending on the transport MTU and frame length constraints.
    pub(crate) fn accept(
        &mut self,
        frame: Frame,
        frame_header: Header,
        tail: TailByte,
        max_payload_length: usize,
        transfer_timeout: MicrosecondDuration32,
    ) -> Result<Option<Transfer<Vec<u8>>>, SessionError> {
        if tail.transfer_id != self.buildup.transfer_id() {
            // This is a frame from some other transfer. Ignore it, but keep this session to receive
            // possible later frames.
            log::info!("Frame transfer ID does not match, ignoring");
            return Ok(None);
        }
        if frame.loopback() != self.loopback {
            log::info!("Frame loopback flag does not match, ignoring");
            return Ok(None);
        }
        // Check if this frame will make the transfer exceed the maximum length
        let new_payload_length = self.buildup.payload_length() + (frame.data().len() - 1);
        if new_payload_length > max_payload_length {
            log::warn!(
                "Payload too large ({} + {} > {}), ending session",
                self.buildup.payload_length(),
                frame.data().len() - 1,
                max_payload_length
            );
            return Err(SessionError::PayloadLength);
        }
        // Check if this frame is too late
        let time_since_first_frame = frame.timestamp().duration_since(self.transfer_timestamp);

        if time_since_first_frame > transfer_timeout {
            // Frame arrived too late. Give up on this session.
            log::info!("Frame timeout expired, ending session");
            return Err(SessionError::Timeout);
        }
        // This frame looks OK. Do the reassembly.
        match self.buildup.add(frame.data())? {
            Some(transfer_data) => self.handle_transfer_data(transfer_data, frame_header),
            None => {
                // Reassembly still in progress
                Ok(None)
            }
        }
    }

    fn handle_transfer_data(
        &mut self,
        mut transfer_data: Vec<u8>,
        frame_header: Header,
    ) -> Result<Option<Transfer<Vec<u8>>>, SessionError> {
        // Check CRC, if this transfer used more than one frame
        if self.buildup.frames() > 1 {
            let mut crc = TransferCrc::new();
            crc.add_bytes(&transfer_data);
            if crc.get() != 0 {
                // Invalid CRC, drop transfer
                return Err(SessionError::Crc);
            }
            // Remove the CRC bytes from the transfer data
            transfer_data.truncate(transfer_data.len() - 2);
        }

        // The header for the transfer has the same priority as the final frame,
        // but the timestamp of the first frame.
        let mut transfer_header = frame_header;
        transfer_header.set_timestamp(self.transfer_timestamp);

        Ok(Some(Transfer {
            header: transfer_header,
            loopback: self.loopback,
            payload: transfer_data,
        }))
    }

    /// Returns the timestamp of this transfer, which is equal to the timestamp of the first frame
    pub fn transfer_timestamp(&self) -> Microseconds32 {
        self.transfer_timestamp
    }

    /// Returns the transfer ID of this session
    #[allow(dead_code)]
    pub fn transfer_id(&self) -> CanTransferId {
        self.buildup.transfer_id()
    }
}

#[derive(Debug)]
pub enum SessionError {
    /// A transfer CRC was invalid
    Crc,
    /// The payload was too long
    PayloadLength,
    /// The session timed out because a frame arrived too lage
    Timeout,
    /// Reassembly failed because of an unexpected frame
    Buildup(BuildupError),
    /// Memory allocation failed
    Memory(OutOfMemoryError),
}

impl From<OutOfMemoryError> for SessionError {
    fn from(inner: OutOfMemoryError) -> Self {
        SessionError::Memory(inner)
    }
}
impl From<BuildupError> for SessionError {
    fn from(inner: BuildupError) -> Self {
        SessionError::Buildup(inner)
    }
}
