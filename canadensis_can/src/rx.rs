//!
//! UAVCAN reception
//!

mod buildup;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::fmt;

use fallible_collections::{FallibleBox, FallibleVec, TryReserveError};

use crate::crc::TransferCrc;
use crate::data::{CanId, Frame};
use crate::error::OutOfMemoryError;
use crate::rx::buildup::{Buildup, BuildupError};
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, MessageHeader, ServiceHeader, Transfer};
use canadensis_core::{NodeId, PortId, Priority, ServiceId, SubjectId, TransferId};
use canadensis_filter_config::Filter;

/// One session per node ID
const RX_SESSIONS_PER_SUBSCRIPTION: usize = NodeId::MAX.to_u8() as usize + 1;

/// Transfer subscription state. The application can register its interest in a particular kind of data exchanged
/// over the bus by creating such subscription objects. Frames that carry data for which there is no active
/// subscription will be silently dropped by the library.
struct Subscription<I: Instant> {
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
    pub fn accept(
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
        let transfer_timeout = self.timeout.clone();

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

/// A receive session, associated with a particular port ID and source node
#[derive(Debug)]
struct Session<I> {
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

    pub fn accept(
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
            return Ok(None);
        }
        // Check if this frame will make the transfer exceed the maximum length
        let new_payload_length = self.buildup.payload_length() + (frame.data().len() - 1);
        if new_payload_length > max_payload_length {
            return Err(SessionError::PayloadLength);
        }
        // Check if this frame is too late
        let time_since_first_frame = frame.timestamp().duration_since(&self.transfer_timestamp);

        if time_since_first_frame > transfer_timeout {
            // Frame arrived too late. Give up on this session.
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
        transfer_header.set_timestamp(self.transfer_timestamp.clone());

        Ok(Some(Transfer {
            header: transfer_header,
            payload: transfer_data,
        }))
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

/// Handles subscriptions and assembles incoming frames into transfers
#[derive(Debug)]
pub struct Receiver<I: Instant> {
    /// Subscriptions for messages
    subscriptions_message: Vec<Subscription<I>>,
    /// Subscriptions for service responses
    subscriptions_response: Vec<Subscription<I>>,
    /// Subscriptions for service requests
    subscriptions_request: Vec<Subscription<I>>,
    /// The ID of this node, or None if this node is anonymous
    id: Option<NodeId>,
    /// Number of transfers successfully received
    transfer_count: u64,
    /// Number of transfers that could not be received
    ///
    /// Errors include failure to allocate memory (when handling incoming frames only), missing
    /// frames, and malformed frames.
    error_count: u64,
}

impl<I: Instant> Receiver<I> {
    /// Creates a receiver
    ///
    /// id: The ID of this node. This is used to filter incoming service requests and responses.
    pub fn new(id: NodeId) -> Self {
        Self::new_inner(Some(id))
    }

    /// Creates an anonymous receiver
    ///
    /// An anonymous receiver cannot receive service requests or responses.
    pub fn new_anonymous() -> Self {
        Self::new_inner(None)
    }

    fn new_inner(id: Option<NodeId>) -> Self {
        Receiver {
            subscriptions_message: Vec::new(),
            subscriptions_response: Vec::new(),
            subscriptions_request: Vec::new(),
            id,
            transfer_count: 0,
            error_count: 0,
        }
    }

    /// Handles an incoming CAN or CAN FD frame
    ///
    /// If this frame is the last frame in a transfer, this function returns the completed transfer.
    /// The transfer type is `Transfer<Vec<u8>>`, which owns the payload buffer.
    ///
    /// The payload of the returned transfer does not include any tail bytes or CRC.
    ///
    /// This function will return an error if memory allocation has failed. Other unexpected
    /// situations, such as duplicate or malformed frames, do not cause this function to return
    /// an error but do increment the error counter. Valid frames on subjects that this receiver is
    /// not subscribed to will be silently ignored.
    pub fn accept(
        &mut self,
        frame: Frame<I>,
    ) -> Result<Option<Transfer<Vec<u8>, I>>, OutOfMemoryError> {
        // The current time is equal to or greater than the frame timestamp. Use that timestamp
        // to clean up expired sessions.
        self.clean_expired_sessions(frame.timestamp());

        // Part 1: basic frame checks
        let (frame_header, tail) = match Self::frame_sanity_check(&frame) {
            Some(data) => data,
            None => {
                // Can't use this frame
                #[cfg(test)]
                std::eprintln!("Frame failed sanity checks, ignoring");
                self.increment_error_count();
                return Ok(None);
            }
        };
        // Check that the frame is actually destined for this node, and this node can handle services
        if let Header::Request(service_header) | Header::Response(service_header) = &frame_header {
            if let Some(this_id) = self.id {
                if service_header.destination != this_id {
                    // This frame is a service request or response going to some other node
                    return Ok(None);
                }
            } else {
                // This node is anonymous, so it must ignore all service frames
                return Ok(None);
            }
        }

        self.accept_sane_frame(frame, frame_header, tail)
    }

    fn accept_sane_frame(
        &mut self,
        frame: Frame<I>,
        frame_header: Header<I>,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>, I>>, OutOfMemoryError> {
        let kind = TransferKind::from_header(&frame_header);
        let subscriptions = self.subscriptions_for_kind(kind);
        if let Some(subscription) = subscriptions
            .iter_mut()
            .find(|subscription| subscription.port_id == frame_header.port_id())
        {
            match subscription.accept(frame, frame_header, tail) {
                Ok(Some(transfer)) => {
                    self.increment_transfer_count();
                    Ok(Some(transfer))
                }
                Ok(None) => Ok(None),
                Err(e) => {
                    #[cfg(test)]
                    std::eprintln!("Receiver accept error {:?}", e);
                    self.increment_error_count();
                    match e {
                        SubscriptionError::Session(SessionError::Memory(e))
                        | SubscriptionError::Memory(e) => Err(e),
                        _ => {
                            // Ignore non-memory errors
                            Ok(None)
                        }
                    }
                }
            }
        } else {
            // No subscription for this port, ignore frame
            Ok(None)
        }
    }

    /// Runs basic sanity checks on an incoming frame. Returns the header and tail byte if the frame
    /// is valid.
    fn frame_sanity_check(frame: &Frame<I>) -> Option<(Header<I>, TailByte)> {
        // Frame must have a tail byte to be valid
        let tail_byte = TailByte::parse(frame.data().last()?.clone());

        let header = parse_can_id(frame.id(), frame.timestamp(), tail_byte.transfer_id).ok()?;

        // Additional header checks
        if let Header::Message(message_header) = &header {
            if message_header.source.is_none() {
                // Anonymous message transfers must always fit into one frame
                if !(tail_byte.toggle && tail_byte.start && tail_byte.end) {
                    #[cfg(test)]
                    std::eprintln!("Anonymous multi-frame transfer, ignoring");
                    return None;
                }
            }
        }

        // OK
        Some((header, tail_byte))
    }

    /// Subscribes to messages on a subject
    ///
    /// This will enable incoming transfers from all nodes on the specified subject ID.
    ///
    /// subject: The subject ID to subscribe to
    ///
    /// payload_size_max: The maximum number of payload bytes expected on this subject
    /// (longer transfers will be dropped)
    ///
    /// timeout: The maximum time between the first and last frames in a transfer (transfers that
    /// do not finish within this time will be dropped)
    ///
    /// If all transfers fit into one frame, the timeout has no meaning and may be zero.
    ///
    pub fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: I::Duration,
    ) -> Result<(), OutOfMemoryError> {
        self.subscribe(
            TransferKind::Message,
            PortId::from(subject),
            payload_size_max,
            timeout,
        )
    }

    /// Unsubscribes from messages on a subject
    pub fn unsubscribe_message(&mut self, subject: SubjectId) {
        self.unsubscribe(TransferKind::Message, PortId::from(subject));
    }

    /// Subscribes to requests for a service
    ///
    /// This will enable incoming service request transfers from all nodes on the specified service
    /// ID.
    ///
    /// service: The service ID to subscribe to
    ///
    /// payload_size_max: The maximum number of payload bytes expected on this subject
    /// (longer transfers will be dropped)
    ///
    /// timeout: The maximum time between the first and last frames in a transfer (transfers that
    /// do not finish within this time will be dropped)
    ///
    /// If all transfers fit into one frame, the timeout has no meaning and may be zero.
    ///
    pub fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: I::Duration,
    ) -> Result<(), OutOfMemoryError> {
        self.subscribe(
            TransferKind::Request,
            PortId::from(service),
            payload_size_max,
            timeout,
        )
    }

    /// Unsubscribes from requests for a service
    pub fn unsubscribe_request(&mut self, service: ServiceId) {
        self.unsubscribe(TransferKind::Request, PortId::from(service));
    }

    /// Subscribes to responses for a service
    ///
    /// This will enable incoming service response transfers from all nodes on the specified service
    /// ID.
    ///
    /// service: The service ID to subscribe to
    ///
    /// payload_size_max: The maximum number of payload bytes expected on this subject
    /// (longer transfers will be dropped)
    ///
    /// timeout: The maximum time between the first and last frames in a transfer (transfers that
    /// do not finish within this time will be dropped)
    ///
    /// If all transfers fit into one frame, the timeout has no meaning and may be zero.
    ///
    pub fn subscribe_response(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: I::Duration,
    ) -> Result<(), OutOfMemoryError> {
        self.subscribe(
            TransferKind::Response,
            PortId::from(service),
            payload_size_max,
            timeout,
        )
    }
    /// Unsubscribes from responses for a service
    pub fn unsubscribe_response(&mut self, service: ServiceId) {
        self.unsubscribe(TransferKind::Response, PortId::from(service));
    }

    fn subscribe(
        &mut self,
        kind: TransferKind,
        port_id: PortId,
        payload_size_max: usize,
        timeout: I::Duration,
    ) -> Result<(), OutOfMemoryError> {
        // Remove any existing subscription, ignore result
        self.unsubscribe(kind, port_id);

        // Create new subscription
        let new_subscription = Subscription::new(timeout, payload_size_max, port_id);

        // Add this subscription to the list for this transfer kind
        let subscriptions = self.subscriptions_for_kind(kind);
        // Reserve memory for the new subscription
        // Logical safety: If a subscription previously existed and was removed, this Vec must have
        // space for it. Therefore, this function cannot remove a subscription and fail to add
        // its replacement.
        FallibleVec::try_push(subscriptions, new_subscription)?;
        Ok(())
    }
    fn unsubscribe(&mut self, kind: TransferKind, port_id: PortId) {
        let subscriptions = self.subscriptions_for_kind(kind);
        subscriptions.retain(|sub| sub.port_id != port_id);
    }

    fn subscriptions_for_kind(&mut self, kind: TransferKind) -> &mut Vec<Subscription<I>> {
        match kind {
            TransferKind::Message => &mut self.subscriptions_message,
            TransferKind::Response => &mut self.subscriptions_response,
            TransferKind::Request => &mut self.subscriptions_request,
        }
    }

    /// Returns the number of transfers successfully received
    pub fn transfer_count(&self) -> u64 {
        self.transfer_count
    }
    /// Returns the number of transfers that could not be received correctly
    ///
    /// Errors include failure to allocate memory (when handling incoming frames only), missing
    /// frames, and malformed frames.
    pub fn error_count(&self) -> u64 {
        self.error_count
    }

    fn increment_transfer_count(&mut self) {
        self.transfer_count = self.transfer_count.wrapping_add(1)
    }
    fn increment_error_count(&mut self) {
        self.error_count = self.error_count.wrapping_add(1)
    }

    fn clean_expired_sessions(&mut self, now: I) {
        clean_sessions_from_subscriptions(&mut self.subscriptions_message, &now);
        clean_sessions_from_subscriptions(&mut self.subscriptions_request, &now);
        clean_sessions_from_subscriptions(&mut self.subscriptions_response, &now);
    }
}

fn clean_sessions_from_subscriptions<I: Instant>(
    subscriptions: &mut Vec<Subscription<I>>,
    now: &I,
) {
    for subscription in subscriptions {
        for slot in subscription.sessions.iter_mut() {
            if let Some(session) = slot.as_deref_mut() {
                let time_since_first_frame = now.duration_since(&session.transfer_timestamp);
                if time_since_first_frame > subscription.timeout {
                    // This session has timed out, delete it.
                    *slot = None;
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum CanIdParseError {
    /// Reserved bit 23 was set
    Bit23Set,
    /// On a message header, reserved bit 7 was set
    Bit7Set,
}

/// Parses a transfer header from a CAN ID, frame timestamp, and frame transfer ID
fn parse_can_id<I>(
    id: CanId,
    timestamp: I,
    transfer_id: TransferId,
) -> core::result::Result<Header<I>, CanIdParseError> {
    let bits = u32::from(id);

    if bits.bit_set(23) {
        return Err(CanIdParseError::Bit23Set);
    }
    // Ignore bits 22 and 21

    let priority = Priority::try_from(bits.get_u8(26)).expect("Bug: Invalid priority");
    let source_id = NodeId::try_from(bits.get_u8(0) & 0x7f).expect("Bug: Invalid source node ID");

    let header = if bits.bit_set(25) {
        // Service
        let service_header = ServiceHeader {
            timestamp,
            transfer_id,
            priority,
            service: ServiceId::try_from(bits.get_u16(14) & 0x1ff)
                .expect("Bug: Invalid service ID"),
            source: source_id,
            destination: NodeId::try_from(bits.get_u8(7) & 0x7f)
                .expect("Bug: Invalid destination node ID"),
        };
        if bits.bit_set(24) {
            // Request
            Header::Request(service_header)
        } else {
            // Response
            Header::Response(service_header)
        }
    } else {
        // Message
        if bits.bit_set(7) {
            return Err(CanIdParseError::Bit7Set);
        }
        // Don't report an anonymous pseudo-ID for anonymous transfers
        let anonymous = bits.bit_set(24);
        let message_source_id = if anonymous { None } else { Some(source_id) };
        let message_header = MessageHeader {
            // Subject ID is 13 bits, 0..=8191
            timestamp,
            transfer_id,
            priority,
            subject: SubjectId::try_from(bits.get_u16(8) & 0x1fff)
                .expect("Bug: Invalid subject ID"),
            source: message_source_id,
        };
        Header::Message(message_header)
    };
    Ok(header)
}

/// Returns a filter that matches message transfers on one subject
///
/// Criteria:
/// * Priority: any
/// * Anonymous: any
/// * Subject ID: matching the provided subject ID
/// * Source node ID: any
pub fn subject_filter(subject: SubjectId) -> Filter {
    let m_id: u32 = 0b0_0000_0110_0000_0000_0000_0000_0000 | u32::from(subject) << 8;
    let mask: u32 = 0b0_0010_1001_1111_1111_1111_1000_0000;
    Filter::new(mask, m_id)
}

/// Returns a filter that matches service request transfers for one service to one node ID
///
/// Criteria:
/// * Priority: any
/// * Request or response: request
/// * Service ID: matching the provided service ID
/// * Destination: matching the provided node ID
/// * Source: any
pub fn request_filter(service: ServiceId, client: NodeId) -> Filter {
    let dynamic_id_bits = u32::from(service) << 14 | u32::from(client) << 7;
    let m_id: u32 = 0b0_0011_0000_0000_0000_0000_0000_0000 | dynamic_id_bits;
    let mask: u32 = 0b0_0011_1111_1111_1111_1111_1000_0000;
    Filter::new(mask, m_id)
}

/// Returns a filter that matches service response transfers for one service to one node ID
///
/// Criteria:
/// * Priority: any
/// * Request or response: response
/// * Service ID: matching the provided service ID
/// * Destination: matching the provided node ID
/// * Source: any
pub fn response_filter(service: ServiceId, server: NodeId) -> Filter {
    let dynamic_id_bits = u32::from(u16::from(service)) << 14 | u32::from(u8::from(server)) << 7;
    let m_id: u32 = 0b0_0010_0000_0000_0000_0000_0000_0000 | dynamic_id_bits;
    let mask: u32 = 0b0_0011_1111_1111_1111_1111_1000_0000;
    Filter::new(mask, m_id)
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

/// Basic extension trait for extracting bits from a CAN ID
trait GetBits {
    fn bit_set(self, offset: u32) -> bool;
    fn get_u8(self, offset: u32) -> u8;
    fn get_u16(self, offset: u32) -> u16;
}

impl GetBits for u32 {
    fn bit_set(self, offset: u32) -> bool {
        ((self >> offset) & 1) == 1
    }
    fn get_u8(self, offset: u32) -> u8 {
        (self >> offset) as u8
    }
    fn get_u16(self, offset: u32) -> u16 {
        (self >> offset) as u16
    }
}
impl GetBits for u8 {
    fn bit_set(self, offset: u32) -> bool {
        ((self >> offset as u8) & 1) == 1
    }

    fn get_u8(self, offset: u32) -> u8 {
        self >> offset as u8
    }

    fn get_u16(self, offset: u32) -> u16 {
        u16::from(self.get_u8(offset))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use canadensis_core::transfer::Header;
    use canadensis_core::{ServiceId, SubjectId};
    use core::fmt::Debug;

    #[test]
    fn test_parse_can_id() {
        // Examples from section 4.2.3 of the specification
        // Heartbeat
        check_can_id(
            Header::Message(MessageHeader {
                timestamp: (),
                transfer_id: TransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(7509).unwrap(),
                source: Some(NodeId::try_from(42).unwrap()),
            }),
            0x107d552a,
        );
        // String primitive
        check_can_id(
            Header::Message(MessageHeader {
                timestamp: (),
                transfer_id: TransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(4919).unwrap(),
                source: None,
            }),
            0x11733775,
        );
        // Node info request
        check_can_id(
            Header::Request(ServiceHeader {
                timestamp: (),
                transfer_id: TransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                service: ServiceId::try_from(430).unwrap(),
                source: NodeId::try_from(123).unwrap(),
                destination: NodeId::try_from(42).unwrap(),
            }),
            0x136b957b,
        );
        // Node info response
        check_can_id(
            Header::Response(ServiceHeader {
                timestamp: (),
                transfer_id: TransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                service: ServiceId::try_from(430).unwrap(),
                source: NodeId::try_from(42).unwrap(),
                destination: NodeId::try_from(123).unwrap(),
            }),
            0x126bbdaa,
        );
        // Array message
        check_can_id(
            Header::Message(MessageHeader {
                timestamp: (),
                transfer_id: TransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(4919).unwrap(),
                source: Some(NodeId::try_from(59).unwrap()),
            }),
            0x1073373b,
        );
    }

    fn check_can_id<I: Clone + PartialEq + Debug>(expected_header: Header<I>, bits: u32) {
        let id = CanId::try_from(bits).unwrap();
        let actual_header = parse_can_id(
            id,
            expected_header.timestamp(),
            expected_header.transfer_id(),
        )
        .unwrap();
        assert_eq!(actual_header, expected_header);
    }
}

struct TailByte {
    start: bool,
    end: bool,
    toggle: bool,
    transfer_id: TransferId,
}

impl TailByte {
    pub fn parse(bits: u8) -> Self {
        TailByte {
            start: bits.bit_set(7),
            end: bits.bit_set(6),
            toggle: bits.bit_set(5),
            transfer_id: (bits & 0x1f).try_into().expect("Bug: Invalid transfer ID"),
        }
    }
}

/// Types of transfers
#[derive(Debug, Copy, Clone)]
enum TransferKind {
    Message,
    Request,
    Response,
}

impl TransferKind {
    pub fn from_header<I>(header: &Header<I>) -> Self {
        match header {
            Header::Message(_) => TransferKind::Message,
            Header::Request(_) => TransferKind::Request,
            Header::Response(_) => TransferKind::Response,
        }
    }
}
