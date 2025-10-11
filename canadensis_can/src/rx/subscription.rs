use crate::rx::session::{Session, SessionError};
use crate::rx::TailByte;
use crate::types::{CanNodeId, Header, Transfer};
use crate::{CanTransferId, Frame};
use alloc::boxed::Box;
use alloc::vec::Vec;
use canadensis_core::time::{MicrosecondDuration32, Microseconds32};
use canadensis_core::transport::TransferId;
use canadensis_core::{OutOfMemoryError, PortId};
use core::array;
use core::fmt;
use core::fmt::Debug;
use fallible_collections::{FallibleVec, TryReserveError};

const NUM_NODE_IDS: usize = CanNodeId::MAX.to_u8() as usize + 1;

/// The Subscription layer handles transfer-ID filtering and the transfer-ID timeout. The Session
/// and Buildup layers are not aware of transfer-IDs and do not check or track them.

#[derive(Debug)]
struct SessionState {
    expected_transfer_id: CanTransferId,
    last_transfer_time: Option<Microseconds32>,
    session: Option<Box<Session>>,
}

/// Transfer subscription state. The application can register its interest in a particular kind of data exchanged
/// over the bus by creating such subscription objects. Frames that carry data for which there is no active
/// subscription will be silently dropped by the library.
pub struct Subscription {
    /// Transfer-ID timeout for this subscription
    ///
    /// This is not the maximum time between the first and last frames in a multi-frame transfer.
    /// Instead, it's effectively the time that must pass before we consider a transfer with the
    /// same transfer-ID as the previous transfer to be a new transfer (due to e.g. the
    /// transmitter resetting) instead of a duplicate.
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
    states: SessionStates,
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
            states: SessionStates::new(),
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
            self.checked_accept_non_anonymous(frame, frame_header, source_node, tail)
        } else {
            self.accept_anonymous(frame, frame_header)
        }
    }

    /// Performs transfer-ID checks before passing the frame further up the chain
    fn checked_accept_non_anonymous(
        &mut self,
        frame: Frame,
        frame_header: Header,
        source: CanNodeId,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>>>, SubscriptionError> {
        let expected_transfer_id = self.states.get(source).expected_transfer_id;
        if tail.transfer_id == expected_transfer_id {
            self.accept_non_anonymous(frame, frame_header, source, tail)
        } else if tail.transfer_id.increment() == expected_transfer_id
            && !self.has_transfer_id_timed_out(source, frame.timestamp())
        {
            // Drop frame, as we consider this to be a duplicate transfer.
            Ok(None)
        } else {
            // Either the new frame has messed with the transfer ID that isn't a simple duplication
            // (e.g. they've reset the counter to some other value), or it uses the previous
            // transfer-ID but we've timed out. Therefore, reset the session and the expected transfer-ID.
            self.states.get_mut(source).session = None;
            self.states.get_mut(source).expected_transfer_id = tail.transfer_id;
            self.accept_non_anonymous(frame, frame_header, source, tail)
        }
    }

    fn has_transfer_id_timed_out(&self, source: CanNodeId, frame_time: Microseconds32) -> bool {
        let last_transfer_time = self.states.get(source).last_transfer_time;
        match last_transfer_time {
            None => true,
            Some(t) => {
                let age = frame_time.checked_duration_since(t);
                match age {
                    // Session is old enough that this new frame starts a new session
                    Some(age) if age > self.timeout => true,
                    Some(_) => false, // Have a recent transfer with the same ID
                    None => true,     // Problem comparing timestamps
                }
            }
        }
    }

    fn accept_non_anonymous(
        &mut self,
        frame: Frame,
        frame_header: Header,
        source: CanNodeId,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>>>, SubscriptionError> {
        debug_assert!(tail.transfer_id == self.states.get(source).expected_transfer_id);
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
            self.states
                .flag_successful_transfer(source, frame.timestamp());
            Ok(Some(transfer))
        } else {
            self.accept_with_session(frame, frame_header, source, tail)
        }
    }

    fn accept_with_session(
        &mut self,
        frame: Frame,
        frame_header: Header,
        source: CanNodeId,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>>>, SubscriptionError> {
        debug_assert!(tail.transfer_id == self.states.get(source).expected_transfer_id);
        let slot: &mut Option<Box<Session>> = &mut self.states.get_mut(source).session;
        let session: &mut Box<Session> = match slot {
            Some(session) => {
                log::debug!(
                    "Using existing session with transfer ID {:?} for port {:?}",
                    tail.transfer_id,
                    self.port_id,
                );
                session
            }
            None => {
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
                    self.payload_size_max,
                    frame.loopback(),
                )?;
                *slot = Some(session);
                match slot {
                    Some(session) => session,
                    None => {
                        unreachable!("We just allocated this session, so it can't be unallocated.")
                    }
                }
            }
        };

        let accept_status = session.accept(frame, frame_header);
        match accept_status {
            Ok(Some(transfer)) => {
                // Transfer received, update state
                let completion_time = session.transfer_timestamp();
                self.states
                    .flag_successful_transfer(source, completion_time);
                Ok(Some(transfer))
            }
            Ok(None) => Ok(None),
            Err(e) => {
                // This is either out-of-memory or an unexpected frame that invalidates the
                // session.
                self.states.get_mut(source).session = None;
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

        // Don't flag as successful transfer, since it's anonymous.
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
struct SessionStates {
    states: [SessionState; NUM_NODE_IDS],
}

impl SessionStates {
    pub fn new() -> Self {
        SessionStates {
            states: array::from_fn(|_| SessionState {
                expected_transfer_id: CanTransferId::default(),
                last_transfer_time: None,
                session: None,
            }),
        }
    }

    pub fn get(&self, node: CanNodeId) -> &SessionState {
        &self.states[usize::from(node)]
    }
    pub fn get_mut(&mut self, node: CanNodeId) -> &mut SessionState {
        &mut self.states[usize::from(node)]
    }

    /// Called on receipt of a transfer from a given node on this subscription, for the purposes of
    /// the transfer-ID timeout.
    pub fn flag_successful_transfer(&mut self, node: CanNodeId, timestamp: Microseconds32) {
        let session_state = &self.states[usize::from(node)];
        let next_transfer_id = session_state.expected_transfer_id.increment();
        self.states[usize::from(node)].last_transfer_time = Some(timestamp);
        self.states[usize::from(node)].expected_transfer_id = next_transfer_id;
        self.states[usize::from(node)].session = None;
    }
}
