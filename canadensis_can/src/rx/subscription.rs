use crate::rx::session::{Session, SessionError};
use crate::rx::TailByte;
use crate::types::{CanNodeId, Header, Transfer};
use crate::{Frame, Mtu};
use alloc::boxed::Box;
use alloc::vec::Vec;
use canadensis_core::time::MicrosecondDuration32;
use canadensis_core::{OutOfMemoryError, PortId};
use core::fmt;
use core::fmt::Debug;
use defmt::Format;
use fallible_collections::{FallibleBox, FallibleVec, TryReserveError};
use l0g::debug;

/// One session per node ID
const RX_SESSIONS_PER_SUBSCRIPTION: usize = CanNodeId::MAX.to_u8() as usize + 1;

/// Transfer subscription state. The application can register its interest in a particular kind of data exchanged
/// over the bus by creating such subscription objects. Frames that carry data for which there is no active
/// subscription will be silently dropped by the library.
pub struct Subscription {
    /// A session for each node ID
    sessions: [Option<Box<Session>>; RX_SESSIONS_PER_SUBSCRIPTION],
    /// Maximum time difference between the first and last frames in a transfer
    timeout: MicrosecondDuration32,
    /// Maximum number of payload bytes, space for the padding and CRC if necessary
    payload_size_max: usize,
    /// Subject or service ID that this subscription is about
    port_id: PortId,
}

impl fmt::Debug for Subscription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subscription")
            .field("sessions", &DebugSessions(&self.sessions))
            .field("transfer_id_timeout", &self.timeout)
            .field("payload_size_max", &self.payload_size_max)
            .field("port_id", &self.port_id)
            .finish()
    }
}

/// A debug adapter for the session list
struct DebugSessions<'s>(&'s [Option<Box<Session>>; RX_SESSIONS_PER_SUBSCRIPTION]);

impl fmt::Debug for DebugSessions<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display as a set, showing only the non-empty entries
        f.debug_set()
            .entries(self.0.iter().flat_map(Option::as_deref))
            .finish()
    }
}

impl Subscription {
    /// Creates a subscription
    ///
    /// The `payload_size_max` value is the maximum number of payload bytes that can be received,
    /// not including space for the padding and transfer CRC.
    pub fn new(
        timeout: MicrosecondDuration32,
        payload_size_max: usize,
        port_id: PortId,
        mtu: Mtu,
    ) -> Self {
        Subscription {
            sessions: init_rx_sessions(),
            timeout,
            payload_size_max: add_padding_and_crc_space(payload_size_max, mtu),
            port_id,
        }
    }

    /// Handles an incoming frame on this subscription's topic
    pub(crate) fn accept(
        &mut self,
        frame: Frame,
        frame_header: Header,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>>>, SubscriptionError> {
        if let Some(source_node) = frame_header.source().cloned() {
            self.accept_non_anonymous(frame, frame_header, source_node, tail)
        } else {
            self.accept_anonymous(frame, frame_header)
        }
    }

    fn accept_non_anonymous(
        &mut self,
        frame: Frame,
        frame_header: Header,
        source_node: CanNodeId,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>>>, SubscriptionError> {
        let max_payload_length = self.payload_size_max;

        if tail.start && tail.end {
            // Special case: Everything fits into one frame, so we don't need to allocate a session
            if frame.data().len() > max_payload_length + 1 {
                return Err(SubscriptionError::Session(SessionError::PayloadLength));
            }
            // Make a transfer from this frame (remove the tail byte)
            let data_without_tail = &frame.data()[..frame.data().len() - 1];
            let mut payload = Vec::new();
            payload.try_extend_from_slice(data_without_tail)?;
            let transfer = Transfer {
                header: frame_header,
                loopback: frame.loopback(),
                payload,
            };
            Ok(Some(transfer))
        } else {
            self.accept_with_session(frame, frame_header, source_node, tail)
        }
    }

    fn accept_with_session(
        &mut self,
        frame: Frame,
        frame_header: Header,
        source_node: CanNodeId,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>>>, SubscriptionError> {
        let max_payload_length = self.payload_size_max;
        let transfer_timeout = self.timeout;

        let slot = &mut self.sessions[usize::from(source_node)];
        let session = match slot {
            Some(session) => {
                debug!(
                    "Using existing session with transfer ID {:?} for port {:?} (frame transfer ID {:?})",
                    session.transfer_id(),
                    self.port_id,
                    tail.transfer_id,
                );
                session
            }
            None => {
                // Check if this frame is appropriate for creating a new session
                if !tail.start {
                    // Not the start of a transfer, so it must be a fragment of some other transfer.
                    return Err(SubscriptionError::NotStart);
                }
                // Create a new session
                *slot = Some(FallibleBox::try_new(Session::new(
                    frame_header.timestamp(),
                    tail.transfer_id,
                    self.payload_size_max,
                    frame.loopback(),
                )?)?);
                debug!(
                    "Created new session for transfer ID {:?} on port {:?}",
                    tail.transfer_id, self.port_id
                );
                slot.as_deref_mut().unwrap()
            }
        };

        let accept_status = session.accept(
            frame,
            frame_header,
            tail,
            max_payload_length,
            transfer_timeout,
        );
        match accept_status {
            Ok(Some(transfer)) => {
                // Transfer received, this session has served its purpose and can be deleted.
                *slot = None;
                Ok(Some(transfer))
            }
            Ok(None) => Ok(None),
            Err(e) => {
                // This is either out-of-memory or an unexpected frame that invalidates
                // the session. Delete the session to free memory.
                *slot = None;
                Err(e.into())
            }
        }
    }

    fn accept_anonymous(
        &mut self,
        frame: Frame,
        frame_header: Header,
    ) -> Result<Option<Transfer<Vec<u8>>>, SubscriptionError> {
        // An anonymous transfer is always a single frame and does not have a corresponding session.
        // Just convert it into a transfer.
        // Remove the tail byte
        let data_without_tail = &frame.data()[..frame.data().len() - 1];

        let mut transfer_data = Vec::new();
        transfer_data.try_extend_from_slice(data_without_tail)?;

        Ok(Some(Transfer {
            header: frame_header,
            loopback: frame.loopback(),
            payload: transfer_data,
        }))
    }

    /// Returns the port ID of this subscription
    pub fn port_id(&self) -> PortId {
        self.port_id
    }

    /// Returns a mutable reference to the array of sessions
    pub fn sessions_mut(&mut self) -> &mut [Option<Box<Session>>; RX_SESSIONS_PER_SUBSCRIPTION] {
        &mut self.sessions
    }
    /// Returns the transfer ID timeout for this subscription
    pub fn timeout(&self) -> MicrosecondDuration32 {
        self.timeout
    }
}

/// Errors that a subscription may encounter
#[derive(Debug, Format)]
pub enum SubscriptionError {
    /// Received a frame with no corresponding session, but its start bit was not set
    NotStart,
    /// An error within the session
    Session(SessionError),
    /// Memory allocation failed
    Memory(OutOfMemoryError),
}

impl From<SessionError> for SubscriptionError {
    fn from(inner: SessionError) -> Self {
        SubscriptionError::Session(inner)
    }
}
impl From<OutOfMemoryError> for SubscriptionError {
    fn from(inner: OutOfMemoryError) -> Self {
        SubscriptionError::Memory(inner)
    }
}
impl From<TryReserveError> for SubscriptionError {
    fn from(_inner: TryReserveError) -> Self {
        SubscriptionError::Memory(OutOfMemoryError)
    }
}

/// Returns 128 Nones
fn init_rx_sessions() -> [Option<Box<Session>>; RX_SESSIONS_PER_SUBSCRIPTION] {
    [
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
    ]
}

/// Adds space for padding and a transfer CRC to the maximum payload size (if required) and returns
/// the new maximum payload size
fn add_padding_and_crc_space(payload_size_max: usize, mtu: Mtu) -> usize {
    let stats = crate::calculate_frame_stats(payload_size_max, mtu as usize);
    let crc_space = if stats.frames > 1 { 2 } else { 0 };
    payload_size_max + stats.last_frame_padding + crc_space
}

#[cfg(test)]
mod test {
    use super::add_padding_and_crc_space;
    use crate::Mtu;

    #[test]
    fn space_classic_can() {
        for size in 0..=7 {
            assert_eq!(size, add_padding_and_crc_space(size, Mtu::Can8));
        }
        for size in 8..=1024 {
            assert_eq!(size + 2, add_padding_and_crc_space(size, Mtu::Can8));
        }
    }

    #[test]
    #[cfg(feature = "can-fd")]
    fn space_can_fd() {
        // One frame
        for size in 0..=7 {
            assert_eq!(size, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
        for size in 8..=11 {
            assert_eq!(11, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
        for size in 12..=15 {
            assert_eq!(15, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
        for size in 16..=19 {
            assert_eq!(19, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
        for size in 20..=23 {
            assert_eq!(23, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
        for size in 24..=31 {
            assert_eq!(31, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
        for size in 32..=47 {
            assert_eq!(47, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
        for size in 48..=63 {
            assert_eq!(63, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
        // Two frames
        // 63 payload bytes + 1 tail byte in frame 1
        // 1..=5 payload bytes + 2 CRC bytes + 1 tail byte in frame 2
        for size in 64..=68 {
            assert_eq!(size + 2, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
        // Two frames
        // 63 payload bytes + 1 tail byte in frame 1
        // 6..=9 payload bytes + padding + 2 CRC bytes + 1 tail byte = 12 bytes in frame 2
        for size in 69..=72 {
            assert_eq!(74, add_padding_and_crc_space(size, Mtu::CanFd64));
        }
    }
}
