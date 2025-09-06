use crate::rx::session::{Session, SessionError};
use crate::rx::TailByte;
use crate::types::{CanNodeId, Header, Transfer};
use crate::{CanTransferId, Frame};
use alloc::boxed::Box;
use alloc::vec::Vec;
use canadensis_core::time::{MicrosecondDuration32, Microseconds32};
use canadensis_core::{OutOfMemoryError, PortId};
use core::fmt;
use core::fmt::Debug;
use fallible_collections::{FallibleVec, TryReserveError};

const NUM_NODE_IDS: usize = CanNodeId::MAX.to_u8() as usize + 1;
const NUM_TRANSFER_IDS: usize = CanTransferId::MAX.to_u8() as usize + 1;

/// Transfer subscription state. The application can register its interest in a particular kind of data exchanged
/// over the bus by creating such subscription objects. Frames that carry data for which there is no active
/// subscription will be silently dropped by the library.
pub struct Subscription {
    /// Transfer-ID timeout for this subscription
    ///
    /// This is not the maximum time between the first and last frames in a multi-frame transfer.
    /// Instead, it's more like the minimum time between transfers with the same transfer-ID.
    timeout: MicrosecondDuration32,
    /// Maximum number of payload bytes to receive
    ///
    /// This does not need to include space for the padding or transfer CRC. If the transfer is
    /// longer than this, this code will check the CRC of the complete transfer but save only this
    /// number of bytes.
    payload_size_max: usize,
    /// Subject or service ID that this subscription is about
    port_id: PortId,
    /// State information from each possible node ID
    states: PerNodeStates,
}

impl Debug for Subscription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subscription")
            .field("timeout", &self.timeout)
            .field("payload_size_max", &self.payload_size_max)
            .field("port_id", &self.port_id)
            .field("states", &self.states)
            .finish()
    }
}

impl Subscription {
    /// Creates a subscription
    ///
    /// The `payload_size_max` value is the maximum number of payload bytes that can be received,
    /// not including space for the padding and transfer CRC.
    pub fn new(timeout: MicrosecondDuration32, payload_size_max: usize, port_id: PortId) -> Self {
        Subscription {
            timeout,
            payload_size_max,
            port_id,
            states: PerNodeStates::new(),
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
        if !self.check_transfer_id_timeout(source_node, tail.transfer_id, frame.timestamp()) {
            return Ok(None);
        }
        if tail.start && tail.end {
            // Special case: Everything fits into one frame, so we don't need to allocate a session
            // Make a transfer from this frame (remove the tail byte)
            let usable_data_len = self.payload_size_max.min(frame.data().len() - 1);
            let data_without_tail = &frame.data()[..usable_data_len];
            let mut payload = Vec::new();
            payload.try_extend_from_slice(data_without_tail)?;
            let transfer = Transfer {
                header: frame_header,
                loopback: frame.loopback(),
                payload,
            };
            // Record that we got a transfer with this ID
            *self.states.get_mut(source_node).get_mut(tail.transfer_id) =
                TransferIdState::Complete(frame.timestamp());
            Ok(Some(transfer))
        } else {
            self.accept_with_session(frame, frame_header, source_node, tail)
        }
    }

    /// Based on the transfer ID and received timestamp of an incoming frame, returns true
    /// to continue processing this frame or false to ignore it
    fn check_transfer_id_timeout(
        &self,
        source: CanNodeId,
        transfer_id: CanTransferId,
        frame_time: Microseconds32,
    ) -> bool {
        match self.states.get(source).get(transfer_id) {
            // First transfer with this ID
            TransferIdState::None => true,
            // Send this frame into the current session even if the session is too old.
            // This may sometimes mix frames from an old and new transfer with the same transfer ID,
            // possibly causing a CRC error.
            // The CRC error will make the state change to Complete.
            TransferIdState::Active(_) => true,
            TransferIdState::Complete(last_transfer_time) => {
                let age = frame_time.checked_duration_since(*last_transfer_time);
                match age {
                    // Session is old enough that this new frame starts a new session
                    Some(age) if age > self.timeout => true,
                    Some(_) => false, // Have a recent transfer with the same ID
                    None => true,     // Problem comparing timestamps
                }
            }
        }
    }

    fn accept_with_session(
        &mut self,
        frame: Frame,
        frame_header: Header,
        source_node: CanNodeId,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>>>, SubscriptionError> {
        let slot = self.states.get_mut(source_node).get_mut(tail.transfer_id);
        let session = match slot {
            TransferIdState::Active(session) => {
                log::debug!(
                    "Using existing session with transfer ID {:?} for port {:?} (frame transfer ID {:?})",
                    session.transfer_id(),
                    self.port_id,
                    tail.transfer_id,
                );
                session
            }
            TransferIdState::None | TransferIdState::Complete(_) => {
                // Check if this frame is appropriate for creating a new session
                if !tail.start {
                    // Not the start of a transfer, so it must be a fragment of some other transfer.
                    return Err(SubscriptionError::NotStart);
                }
                log::debug!(
                    "Creating new session for transfer ID {:?} on port {:?}",
                    tail.transfer_id,
                    self.port_id
                );
                let session = Session::boxed(
                    frame_header.timestamp(),
                    tail.transfer_id,
                    self.payload_size_max,
                    frame.loopback(),
                )?;
                *slot = TransferIdState::Active(session);
                match slot {
                    TransferIdState::Active(session) => &mut *session,
                    TransferIdState::Complete(_) | TransferIdState::None => unreachable!(
                        "We just set this transfer to active, so it can't have any other state."
                    ),
                }
            }
        };

        let accept_status = session.accept(frame, frame_header, tail);
        match accept_status {
            Ok(Some(transfer)) => {
                // Transfer received, this session has served its purpose and can be deleted.
                *slot = TransferIdState::Complete(session.transfer_timestamp());
                Ok(Some(transfer))
            }
            Ok(None) => Ok(None),
            Err(e) => {
                // This is either out-of-memory or an unexpected frame that invalidates
                // the session. Delete the session to free memory.
                *slot = TransferIdState::None;
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

#[derive(Debug)]
struct PerNodeStates {
    states: [TransferIdStates; NUM_NODE_IDS],
}

impl PerNodeStates {
    pub fn new() -> PerNodeStates {
        const DEFAULT: TransferIdStates = TransferIdStates::new();
        PerNodeStates {
            states: [DEFAULT; NUM_NODE_IDS],
        }
    }
    pub fn get(&self, source: CanNodeId) -> &TransferIdStates {
        &self.states[usize::from(source)]
    }
    pub fn get_mut(&mut self, source: CanNodeId) -> &mut TransferIdStates {
        &mut self.states[usize::from(source)]
    }
}

#[derive(Debug)]
struct TransferIdStates {
    states: [TransferIdState; NUM_TRANSFER_IDS],
}

impl TransferIdStates {
    pub const fn new() -> TransferIdStates {
        const DEFAULT: TransferIdState = TransferIdState::None;
        TransferIdStates {
            states: [DEFAULT; NUM_TRANSFER_IDS],
        }
    }
    pub fn get(&self, id: CanTransferId) -> &TransferIdState {
        &self.states[usize::from(id)]
    }
    pub fn get_mut(&mut self, id: CanTransferId) -> &mut TransferIdState {
        &mut self.states[usize::from(id)]
    }
}

/// Information about what is happening with a specific transfer ID
#[derive(Debug)]
enum TransferIdState {
    /// We haven't received any frames with this transfer ID, or we received some frames but
    /// had a problem validating or reassembling them
    None,
    /// We got at least one frame and are in the process of reassembling a transfer
    ///
    /// If the transfer finishes or encounters an error, this moves to the `Complete` state.
    Active(Box<Session>),
    /// Got a frame with this transfer ID, and reassembly finished successfully
    ///
    /// The enclosed timestamp is the time we received the first frame of the most recent transfer.
    Complete(Microseconds32),
}
