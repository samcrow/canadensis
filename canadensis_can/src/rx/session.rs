use crate::rx::buildup::{Buildup, BuildupError};
use crate::rx::TailByte;
use crate::{Frame, OutOfMemoryError, TransferCrc};
use alloc::vec::Vec;
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, Transfer};
use canadensis_core::TransferId;

/// A receive session, associated with a particular port ID and source node
#[derive(Debug)]
pub struct Session<I> {
    /// Timestamp of the first frame received in this transfer
    transfer_timestamp: I,
    /// Transfer reassembly
    buildup: Buildup,
}

impl<I> Session<I>
where
    I: Instant,
{
    pub fn new(transfer_timestamp: I, transfer_id: TransferId) -> Self {
        Session {
            transfer_timestamp,
            buildup: Buildup::new(transfer_id),
        }
    }

    /// Accepts a frame associated with this session
    ///
    /// If this frame completes a transfer, this function returns the transfer.
    ///
    /// The `max_payload_length` value must include space for the transfer CRC and/or padding bytes
    /// that may be inserted, depending on the transport MTU and frame length constraints.
    pub(crate) fn accept(
        &mut self,
        frame: Frame<I>,
        frame_header: Header<I>,
        tail: TailByte,
        max_payload_length: usize,
        transfer_timeout: I::Duration,
    ) -> Result<Option<Transfer<Vec<u8>, I>>, SessionError> {
        if tail.transfer_id != self.buildup.transfer_id() {
            // This is a frame from some other transfer. Ignore it, but keep this session to receive
            // possible later frames.
            debugln!("Frame transfer ID does not match, ignoring");
            return Ok(None);
        }
        // Check if this frame will make the transfer exceed the maximum length
        let new_payload_length = self.buildup.payload_length() + (frame.data().len() - 1);
        if new_payload_length > max_payload_length {
            debugln!(
                "Payload too large ({} + {} > {}), ending session",
                self.buildup.payload_length(),
                frame.data().len() - 1,
                max_payload_length
            );
            return Err(SessionError::PayloadLength);
        }
        // Check if this frame is too late
        let time_since_first_frame = frame.timestamp().duration_since(&self.transfer_timestamp);

        if time_since_first_frame > transfer_timeout {
            // Frame arrived too late. Give up on this session.
            debugln!("Frame timeout expired, ending session");
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
        frame_header: Header<I>,
    ) -> Result<Option<Transfer<Vec<u8>, I>>, SessionError> {
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
            payload: transfer_data,
        }))
    }

    /// Returns the timestamp of this transfer, which is equal to the timestamp of the first frame
    pub fn transfer_timestamp(&self) -> I {
        self.transfer_timestamp
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
