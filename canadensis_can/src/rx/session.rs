use crate::rx::buildup::{Buildup, BuildupError};
use crate::rx::TailByte;
use crate::types::{CanTransferId, Header, Transfer};
use crate::Frame;
use alloc::boxed::Box;
use alloc::vec::Vec;
use canadensis_core::time::Microseconds32;
use canadensis_core::OutOfMemoryError;
use core::fmt::Debug;
use fallible_collections::FallibleBox;

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
    /// Creates a new session allocated in a `Box`
    pub fn boxed(
        transfer_timestamp: Microseconds32,
        transfer_id: CanTransferId,
        max_payload_length: usize,
        loopback: bool,
    ) -> Result<Box<Session>, OutOfMemoryError> {
        let session = Session::new(
            transfer_timestamp,
            transfer_id,
            max_payload_length,
            loopback,
        )?;
        let session_box = FallibleBox::try_new(session).map_err(|_| OutOfMemoryError)?;
        Ok(session_box)
    }

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
        // This frame looks OK. Do the reassembly.
        let maybe_transfer = self
            .buildup
            .add(frame.data())?
            .map(|data| self.handle_transfer_data(data, frame_header));
        Ok(maybe_transfer)
    }

    fn handle_transfer_data(
        &mut self,
        transfer_data: Vec<u8>,
        frame_header: Header,
    ) -> Transfer<Vec<u8>> {
        // The header for the transfer has the same priority as the final frame,
        // but the timestamp of the first frame.
        let mut transfer_header = frame_header;
        transfer_header.set_timestamp(self.transfer_timestamp);

        Transfer {
            header: transfer_header,
            loopback: self.loopback,
            payload: transfer_data,
        }
    }

    /// Returns the transfer ID of this session
    #[allow(dead_code)]
    pub fn transfer_id(&self) -> CanTransferId {
        self.buildup.transfer_id()
    }

    pub(crate) fn transfer_timestamp(&self) -> Microseconds32 {
        self.transfer_timestamp
    }
}

#[derive(Debug)]
pub enum SessionError {
    /// Reassembly failed because of an unexpected frame
    Buildup,
    /// Memory allocation failed
    Memory(OutOfMemoryError),
}

impl From<OutOfMemoryError> for SessionError {
    fn from(inner: OutOfMemoryError) -> Self {
        SessionError::Memory(inner)
    }
}
impl From<BuildupError> for SessionError {
    fn from(_inner: BuildupError) -> Self {
        SessionError::Buildup
    }
}
