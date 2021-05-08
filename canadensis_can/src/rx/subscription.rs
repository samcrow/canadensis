use crate::rx::session::{Session, SessionError};
use crate::rx::TailByte;
use crate::{Frame, OutOfMemoryError};
use alloc::boxed::Box;
use alloc::vec::Vec;
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, Transfer};
use canadensis_core::{NodeId, PortId};
use core::fmt;
use fallible_collections::{FallibleBox, FallibleVec, TryReserveError};

/// One session per node ID
const RX_SESSIONS_PER_SUBSCRIPTION: usize = NodeId::MAX.to_u8() as usize + 1;

/// Transfer subscription state. The application can register its interest in a particular kind of data exchanged
/// over the bus by creating such subscription objects. Frames that carry data for which there is no active
/// subscription will be silently dropped by the library.
pub struct Subscription<I: Instant> {
    /// A session for each node ID
    sessions: [Option<Box<Session<I>>>; RX_SESSIONS_PER_SUBSCRIPTION],
    /// Maximum time difference between the first and last frames in a transfer
    timeout: I::Duration,
    /// Maximum number of payload bytes, including 2 bytes for the CRC if necessary
    payload_size_max: usize,
    /// Subject or service ID that this subscription is about
    port_id: PortId,
}

impl<I: Instant> fmt::Debug for Subscription<I> {
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
struct DebugSessions<'s, I>(&'s [Option<Box<Session<I>>>; RX_SESSIONS_PER_SUBSCRIPTION]);

impl<I: Instant> fmt::Debug for DebugSessions<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display as a set, showing only the non-empty entries
        f.debug_set()
            .entries(self.0.iter().flat_map(Option::as_deref))
            .finish()
    }
}

impl<I: Instant> Subscription<I> {
    /// Creates a subscription
    pub fn new(timeout: I::Duration, payload_size_max: usize, port_id: PortId) -> Self {
        Subscription {
            sessions: init_rx_sessions(),
            timeout,
            payload_size_max,
            port_id,
        }
    }

    /// Handles an incoming frame on this subscription's topic
    pub(crate) fn accept(
        &mut self,
        frame: Frame<I>,
        frame_header: Header<I>,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>, I>>, SubscriptionError> {
        if let Some(source_node) = frame_header.source() {
            self.accept_non_anonymous(frame, frame_header, source_node, tail)
        } else {
            self.accept_anonymous(frame, frame_header)
        }
    }

    fn accept_non_anonymous(
        &mut self,
        frame: Frame<I>,
        frame_header: Header<I>,
        source_node: NodeId,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>, I>>, SubscriptionError> {
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
                payload,
            };
            Ok(Some(transfer))
        } else {
            self.accept_with_session(frame, frame_header, source_node, tail)
        }
    }

    fn accept_with_session(
        &mut self,
        frame: Frame<I>,
        frame_header: Header<I>,
        source_node: NodeId,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>, I>>, SubscriptionError> {
        let max_payload_length = self.payload_size_max;
        let transfer_timeout = self.timeout;

        let slot = &mut self.sessions[usize::from(source_node)];
        let session = match slot {
            Some(session) => session,
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
                ))?);
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
        frame: Frame<I>,
        frame_header: Header<I>,
    ) -> Result<Option<Transfer<Vec<u8>, I>>, SubscriptionError> {
        // An anonymous transfer is always a single frame and does not have a corresponding session.
        // Just convert it into a transfer.
        // Remove the tail byte
        let data_without_tail = &frame.data()[..frame.data().len() - 1];

        let mut transfer_data = Vec::new();
        transfer_data.try_extend_from_slice(data_without_tail)?;

        Ok(Some(Transfer {
            header: frame_header,
            payload: transfer_data,
        }))
    }

    /// Returns the port ID of this subscription
    pub fn port_id(&self) -> PortId {
        self.port_id
    }

    /// Returns a mutable reference to the array of sessions
    pub fn sessions_mut(&mut self) -> &mut [Option<Box<Session<I>>>; RX_SESSIONS_PER_SUBSCRIPTION] {
        &mut self.sessions
    }
    /// Returns the transfer ID timeout for this subscription
    pub fn timeout(&self) -> I::Duration {
        self.timeout
    }
}

/// Errors that a subscription may encounter
#[derive(Debug)]
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
fn init_rx_sessions<I>() -> [Option<Box<Session<I>>>; RX_SESSIONS_PER_SUBSCRIPTION] {
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
