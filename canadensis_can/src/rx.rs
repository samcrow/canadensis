//!
//! Cyphal message reception and assembly
//!

mod buildup;
mod session;
mod subscription;

use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::fmt::Debug;
use core::marker::PhantomData;

use fallible_collections::FallibleVec;

use crate::data::{CanId, Frame};
use crate::driver::ReceiveDriver;
use crate::rx::session::SessionError;
use crate::rx::subscription::{Subscription, SubscriptionError};
use crate::types::{CanNodeId, CanTransferId, CanTransport, Error};
use crate::Mtu;
use canadensis_core::time::{Clock, MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, ServiceHeader, Transfer};
use canadensis_core::transport::Receiver;
use canadensis_core::{
    nb, OutOfMemoryError, PortId, Priority, ServiceId, ServiceSubscribeError, SubjectId,
};

/// Handles subscriptions and assembles incoming frames into transfers
#[derive(Debug)]
pub struct CanReceiver<C, D> {
    /// Subscriptions for messages
    subscriptions_message: Vec<Subscription>,
    /// Subscriptions for service responses
    subscriptions_response: Vec<Subscription>,
    /// Subscriptions for service requests
    subscriptions_request: Vec<Subscription>,
    /// The ID of this node, or None if this node is anonymous
    id: Option<CanNodeId>,
    /// MTU of the transport
    mtu: Mtu,
    /// Number of transfers successfully received
    transfer_count: u64,
    /// Number of transfers that could not be received
    ///
    /// Errors include failure to allocate memory (when handling incoming frames only), missing
    /// frames, and malformed frames.
    error_count: u64,
    /// The driver that supplies incoming frames
    _driver: PhantomData<D>,
    /// The clock used to get the current time
    _clock: PhantomData<C>,
}

impl<C, D> Receiver<C> for CanReceiver<C, D>
where
    C: Clock,
    D: ReceiveDriver<C>,
{
    type Transport = CanTransport;
    type Driver = D;
    type Error = Error<D::Error>;

    fn receive(
        &mut self,
        clock: &mut C,
        driver: &mut Self::Driver,
    ) -> Result<Option<Transfer<Vec<u8>, Self::Transport>>, Self::Error> {
        // The current time is equal to or greater than the frame timestamp. Use that timestamp
        // to clean up expired sessions.
        self.clean_expired_sessions(clock.now());
        // Loop until all available frames have been handled
        loop {
            match driver.receive(clock) {
                Ok(frame) => {
                    match self.accept_frame(frame) {
                        Ok(Some(transfer)) => break Ok(Some(transfer)),
                        Ok(None) => { /* Keep going and try another frame */ }
                        Err(e) => break Err(e.into()),
                    }
                }
                Err(nb::Error::WouldBlock) => break Ok(None),
                Err(nb::Error::Other(e)) => break Err(Error::Driver(e)),
            }
        }
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
    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
        driver: &mut Self::Driver,
    ) -> Result<(), Self::Error> {
        self.subscribe(
            TransferKind::Message,
            PortId::from(subject),
            payload_size_max,
            timeout,
        )
        .map_err(Error::Memory)?;
        self.apply_frame_filters(driver);
        Ok(())
    }

    /// Unsubscribes from messages on a subject
    fn unsubscribe_message(&mut self, subject: SubjectId, driver: &mut Self::Driver) {
        self.unsubscribe(TransferKind::Message, PortId::from(subject));
        self.apply_frame_filters(driver);
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
    /// This function returns an error if memory allocation fails or if this node is anonymous.
    ///
    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
        driver: &mut Self::Driver,
    ) -> Result<(), ServiceSubscribeError<Self::Error>> {
        if self.id.is_some() {
            self.subscribe(
                TransferKind::Request,
                PortId::from(service),
                payload_size_max,
                timeout,
            )
            .map_err(Error::Memory)
            .map_err(ServiceSubscribeError::Transport)?;
            self.apply_frame_filters(driver);
            Ok(())
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }

    /// Unsubscribes from requests for a service
    fn unsubscribe_request(&mut self, service: ServiceId, driver: &mut Self::Driver) {
        self.unsubscribe(TransferKind::Request, PortId::from(service));
        self.apply_frame_filters(driver);
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
    /// This function returns an error if memory allocation fails or if this node is anonymous.
    ///
    fn subscribe_response(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
        driver: &mut Self::Driver,
    ) -> Result<(), ServiceSubscribeError<Self::Error>> {
        if self.id.is_some() {
            self.subscribe(
                TransferKind::Response,
                PortId::from(service),
                payload_size_max,
                timeout,
            )
            .map_err(Error::Memory)
            .map_err(ServiceSubscribeError::Transport)?;
            self.apply_frame_filters(driver);
            Ok(())
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }
    /// Unsubscribes from responses for a service
    fn unsubscribe_response(&mut self, service: ServiceId, driver: &mut Self::Driver) {
        self.unsubscribe(TransferKind::Response, PortId::from(service));
        self.apply_frame_filters(driver);
    }

    /// Updates the identifier of this node
    ///
    /// This can be used after a node ID is identified to make this receiver capable of handling
    /// service transfers.
    fn set_id(&mut self, id: Option<CanNodeId>) {
        self.id = id;
    }

    /// Despite the use of `unwrap` this function should never panic as
    /// [`Receiver::subscribe_message`] takes [`SubjectId`] as an argument and that is the only
    /// way that [`CanReceiver::subscriptions_message`] can be modified
    fn subscribers(&self) -> impl Iterator<Item = SubjectId> {
        self.subscriptions_message
            .iter()
            .map(|x| x.port_id().try_into().unwrap())
    }

    /// Despite the use of `unwrap` this function should never panic as
    /// [`Receiver::subscribe_request`] takes [`ServiceId`] as an argument and that is the only
    /// way that [`CanReceiver::subscriptions_request`] can be modified
    fn servers(&self) -> impl Iterator<Item = ServiceId> {
        self.subscriptions_request
            .iter()
            .map(|x| x.port_id().try_into().unwrap())
    }
}

impl<C, D> CanReceiver<C, D>
where
    C: Clock,
    D: ReceiveDriver<C>,
{
    /// Creates a receiver
    ///
    /// id: The ID of this node. This is used to filter incoming service requests and responses.
    pub fn new(id: CanNodeId, mtu: Mtu) -> Self {
        Self::new_inner(Some(id), mtu)
    }

    /// Creates an anonymous receiver
    ///
    /// An anonymous receiver cannot receive service requests or responses.
    pub fn new_anonymous(mtu: Mtu) -> Self {
        Self::new_inner(None, mtu)
    }

    fn new_inner(id: Option<CanNodeId>, mtu: Mtu) -> Self {
        CanReceiver {
            subscriptions_message: Vec::new(),
            subscriptions_response: Vec::new(),
            subscriptions_request: Vec::new(),
            id,
            mtu,
            transfer_count: 0,
            error_count: 0,
            _driver: PhantomData,
            _clock: PhantomData,
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
    fn accept_frame(
        &mut self,
        frame: Frame,
    ) -> Result<Option<Transfer<Vec<u8>, CanTransport>>, OutOfMemoryError> {
        // Part 1: basic frame checks
        let (frame_header, tail) = match Self::frame_sanity_check(&frame) {
            Some(data) => data,
            None => {
                // Can't use this frame
                log::debug!("Frame failed sanity checks, ignoring");
                self.increment_error_count();
                return Ok(None);
            }
        };
        // Check that the frame is actually destined for this node, and this node can handle services
        // Exception: Loopback frames came from this node and are always accepted
        if let Header::Request(service_header) | Header::Response(service_header) = &frame_header {
            if !(frame.loopback() || self.can_accept_service(service_header)) {
                return Ok(None);
            }
        }
        self.accept_sane_frame(frame, frame_header, tail)
    }

    /// Returns true if this node is not anonymous and matches the destination node ID of the
    /// provided service header
    fn can_accept_service(&self, service_header: &ServiceHeader<CanTransport>) -> bool {
        match self.id {
            Some(local_id) if local_id == service_header.destination => true,
            Some(_) | None => false,
        }
    }

    /// Handles an incoming frame that has passed sanity checks and has a parsed header and tail byte
    fn accept_sane_frame(
        &mut self,
        frame: Frame,
        frame_header: Header<CanTransport>,
        tail: TailByte,
    ) -> Result<Option<Transfer<Vec<u8>, CanTransport>>, OutOfMemoryError> {
        let kind = TransferKind::from_header(&frame_header);
        let subscriptions = self.subscriptions_for_kind(kind);
        if let Some(subscription) = subscriptions
            .iter_mut()
            .find(|subscription| subscription.port_id() == frame_header.port_id())
        {
            match subscription.accept(frame, frame_header, tail) {
                Ok(Some(transfer)) => {
                    self.increment_transfer_count();
                    Ok(Some(transfer))
                }
                Ok(None) => Ok(None),
                Err(e) => {
                    log::info!("Receiver accept error {:?}", e);
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
    fn frame_sanity_check(frame: &Frame) -> Option<(Header<CanTransport>, TailByte)> {
        // Frame must have a tail byte to be valid
        let tail_byte = TailByte::parse(*frame.data().last()?);

        let header = parse_can_id(frame.id(), frame.timestamp(), tail_byte.transfer_id).ok()?;

        // Additional header checks
        if let Header::Message(message_header) = &header {
            if message_header.source.is_none() {
                // Anonymous message transfers must always fit into one frame
                if !(tail_byte.toggle && tail_byte.start && tail_byte.end) {
                    log::debug!("Anonymous multi-frame transfer, ignoring");
                    return None;
                }
            }
        }

        // OK
        Some((header, tail_byte))
    }

    fn subscribe(
        &mut self,
        kind: TransferKind,
        port_id: PortId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
    ) -> Result<(), OutOfMemoryError> {
        // Remove any existing subscription, ignore result
        self.unsubscribe(kind, port_id);

        // Create new subscription
        let new_subscription = Subscription::new(timeout, payload_size_max, port_id, self.mtu);

        // Add this subscription to the list for this transfer kind
        let subscriptions = self.subscriptions_for_kind(kind);
        // Reserve memory for the new subscription
        // Logical safety: If a subscription previously existed and was removed, this Vec must have
        // space for it. Therefore, this function cannot remove a subscription and fail to add
        // its replacement.
        subscriptions.try_reserve_exact(1)?;
        FallibleVec::try_push(subscriptions, new_subscription)?;
        Ok(())
    }
    fn unsubscribe(&mut self, kind: TransferKind, port_id: PortId) {
        let subscriptions = self.subscriptions_for_kind(kind);
        subscriptions.retain(|sub| sub.port_id() != port_id);
    }

    fn subscriptions_for_kind(&mut self, kind: TransferKind) -> &mut Vec<Subscription> {
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

    /// Deletes all sessions that have expired
    fn clean_expired_sessions(&mut self, now: Microseconds32) {
        clean_sessions_from_subscriptions(&mut self.subscriptions_message, now);
        clean_sessions_from_subscriptions(&mut self.subscriptions_request, now);
        clean_sessions_from_subscriptions(&mut self.subscriptions_response, now);
    }

    fn apply_frame_filters(&mut self, driver: &mut D) {
        let message_subscriptions = self.subscriptions_message.iter().map(|sub| {
            canadensis_core::subscription::Subscription::Message(sub.port_id().try_into().unwrap())
        });
        let request_subscriptions = self.subscriptions_request.iter().map(|sub| {
            canadensis_core::subscription::Subscription::Request(sub.port_id().try_into().unwrap())
        });
        let response_subscriptions = self.subscriptions_response.iter().map(|sub| {
            canadensis_core::subscription::Subscription::Response(sub.port_id().try_into().unwrap())
        });
        let all_subscriptions = message_subscriptions
            .chain(request_subscriptions)
            .chain(response_subscriptions);
        driver.apply_filters(self.id, all_subscriptions);
    }
}

fn clean_sessions_from_subscriptions(subscriptions: &mut Vec<Subscription>, now: Microseconds32) {
    for subscription in subscriptions {
        let timeout = subscription.timeout();
        for slot in subscription.sessions_mut().iter_mut() {
            if let Some(session) = slot.as_deref_mut() {
                let deadline = session.transfer_timestamp() + timeout;
                if now > deadline {
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
fn parse_can_id(
    id: CanId,
    timestamp: Microseconds32,
    transfer_id: CanTransferId,
) -> core::result::Result<Header<CanTransport>, CanIdParseError> {
    let bits = u32::from(id);

    if bits.bit_set(23) {
        return Err(CanIdParseError::Bit23Set);
    }
    // Ignore bits 22 and 21

    let priority = Priority::try_from(bits.get_u8(26)).expect("Bug: Invalid priority");
    let source_id =
        CanNodeId::try_from(bits.get_u8(0) & 0x7f).expect("Bug: Invalid source node ID");

    let header = if bits.bit_set(25) {
        // Service
        let service_header = ServiceHeader {
            timestamp,
            transfer_id,
            priority,
            service: ServiceId::try_from(bits.get_u16(14) & 0x1ff)
                .expect("Bug: Invalid service ID"),
            source: source_id,
            destination: CanNodeId::try_from(bits.get_u8(7) & 0x7f)
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

/// Basic extension trait for extracting bits from a CAN ID
//noinspection RsSelfConvention
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

    #[test]
    fn test_parse_can_id() {
        // Examples from section 4.2.3 of the specification
        // Heartbeat
        check_can_id(
            Header::Message(MessageHeader {
                timestamp: Microseconds32::from_ticks(0),
                transfer_id: CanTransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(7509).unwrap(),
                source: Some(CanNodeId::try_from(42u8).unwrap()),
            }),
            0x107d552a,
        );
        // String primitive
        check_can_id(
            Header::Message(MessageHeader {
                timestamp: Microseconds32::from_ticks(0),
                transfer_id: CanTransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(4919).unwrap(),
                source: None,
            }),
            0x11733775,
        );
        // Node info request
        check_can_id(
            Header::Request(ServiceHeader {
                timestamp: Microseconds32::from_ticks(0),
                transfer_id: CanTransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                service: ServiceId::try_from(430).unwrap(),
                source: CanNodeId::try_from(123u8).unwrap(),
                destination: CanNodeId::try_from(42u8).unwrap(),
            }),
            0x136b957b,
        );
        // Node info response
        check_can_id(
            Header::Response(ServiceHeader {
                timestamp: Microseconds32::from_ticks(0),
                transfer_id: CanTransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                service: ServiceId::try_from(430).unwrap(),
                source: CanNodeId::try_from(42u8).unwrap(),
                destination: CanNodeId::try_from(123u8).unwrap(),
            }),
            0x126bbdaa,
        );
        // Array message
        check_can_id(
            Header::Message(MessageHeader {
                timestamp: Microseconds32::from_ticks(0),
                transfer_id: CanTransferId::try_from(0).unwrap(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(4919).unwrap(),
                source: Some(CanNodeId::try_from(59u8).unwrap()),
            }),
            0x1073373b,
        );
    }

    fn check_can_id(expected_header: Header<CanTransport>, bits: u32) {
        let id = CanId::try_from(bits).unwrap();
        let actual_header = parse_can_id(
            id,
            expected_header.timestamp(),
            expected_header.transfer_id().clone(),
        )
        .unwrap();
        assert_eq!(actual_header, expected_header);
    }
}

pub(crate) struct TailByte {
    start: bool,
    end: bool,
    toggle: bool,
    transfer_id: CanTransferId,
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
    pub fn from_header(header: &Header<CanTransport>) -> Self {
        match header {
            Header::Message(_) => TransferKind::Message,
            Header::Request(_) => TransferKind::Request,
            Header::Response(_) => TransferKind::Response,
        }
    }
}
