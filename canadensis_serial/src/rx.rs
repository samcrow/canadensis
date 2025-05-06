use alloc::vec::Vec;
use core::convert::TryFrom;
use core::marker::PhantomData;
use core::mem;

use fallible_collections::{FallibleVec, TryHashMap};
use l0g::debug;

use canadensis_core::subscription::SubscriptionManager;
use canadensis_core::time::{Clock, MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::{Header, Transfer};
use canadensis_core::transport::Receiver;
use canadensis_core::{nb, OutOfMemoryError, ServiceId, ServiceSubscribeError, SubjectId};
use canadensis_header::Header as SerialHeader;

use crate::cobs::Unescaper;
use crate::driver::ReceiveDriver;
use crate::header_collector::HeaderCollector;
use crate::{make_payload_crc, Error, SerialNodeId, SerialTransferId, SerialTransport};

/// A serial transport receiver
///
/// This implementation does not support multi-frame transfers or timestamps.
pub struct SerialReceiver<C, D, S> {
    state: State,
    node_id: Option<SerialNodeId>,
    subscriptions: S,
    _driver: PhantomData<D>,
    _clock: PhantomData<C>,
}

impl<C, D, S> SerialReceiver<C, D, S>
where
    C: Clock,
    D: ReceiveDriver,
    S: SubscriptionManager<Subscription> + Default,
{
    pub fn new(node_id: SerialNodeId) -> Self {
        SerialReceiver {
            state: State::Idle,
            node_id: Some(node_id),
            subscriptions: S::default(),
            _driver: PhantomData,
            _clock: PhantomData,
        }
    }
    pub fn new_anonymous() -> Self {
        SerialReceiver {
            state: State::Idle,
            node_id: None,
            subscriptions: S::default(),
            _driver: PhantomData,
            _clock: PhantomData,
        }
    }

    fn clean_expired_sessions(&mut self, now: Microseconds32) {
        self.subscriptions
            .for_each_message_subscription_mut(|sub| sub.clean_expired_sessions(now));
        self.subscriptions
            .for_each_request_subscription_mut(|sub| sub.clean_expired_sessions(now));
        self.subscriptions
            .for_each_response_subscription_mut(|sub| sub.clean_expired_sessions(now));
    }

    fn handle_byte(
        &mut self,
        byte: u8,
        now: Microseconds32,
    ) -> Result<Option<Transfer<Vec<u8>, SerialTransport>>, Error<D::Error>> {
        let state = mem::replace(&mut self.state, State::Idle);
        self.state = match state {
            State::Idle => {
                if byte == 0 {
                    State::BetweenTransfers
                } else {
                    State::Idle
                }
            }
            State::BetweenTransfers => {
                if byte != 0 {
                    // Start decoding
                    debug!("Starting frame");
                    let mut unescaper = Unescaper::new();
                    match unescaper.accept(byte) {
                        Ok(Some(byte)) => {
                            // Got the first byte of the header
                            let mut header = HeaderCollector::new();
                            header.push(byte);
                            State::Header { unescaper, header }
                        }
                        Ok(None) => State::Header {
                            unescaper,
                            header: HeaderCollector::new(),
                        },
                        Err(_) => unreachable!("Unescaper returned an error for a non-zero input"),
                    }
                } else {
                    // Got another zero, keep waiting
                    State::BetweenTransfers
                }
            }
            State::Header {
                mut unescaper,
                mut header,
            } => {
                match unescaper.accept(byte) {
                    Ok(Some(byte)) => {
                        header.push(byte);

                        if header.is_done() {
                            // Got the complete header
                            let header = header.as_header();
                            match SerialHeader::try_from(header) {
                                Ok(header) => {
                                    let header = header.as_core_header(now);
                                    if let Some(subscription) = self.is_interested(&header) {
                                        // Try to allocate memory for the incoming transfer
                                        // (add 4 bytes at the end for the CRC)
                                        match FallibleVec::try_with_capacity(
                                            subscription.payload_size_max + 4,
                                        ) {
                                            Ok(payload) => State::Payload {
                                                unescaper,
                                                header,
                                                payload,
                                            },
                                            Err(_) => {
                                                // Not enough memory to receive this transfer
                                                self.state = State::Idle;
                                                return Err(Error::Memory(OutOfMemoryError));
                                            }
                                        }
                                    } else {
                                        // Not interested in this transfer
                                        debug!("Got header, but not subscribed");
                                        State::Idle
                                    }
                                }
                                Err(e) => {
                                    // Invalid header CRC or format
                                    debug!("Header format or CRC invalid: {:?}", e);
                                    State::Idle
                                }
                            }
                        } else {
                            // Wait for more header bytes
                            State::Header { unescaper, header }
                        }
                    }
                    Ok(None) => {
                        // Keep the same state
                        State::Header { unescaper, header }
                    }
                    // Unexpected zero byte
                    Err(_) => State::Idle,
                }
            }
            State::Payload {
                mut unescaper,
                header,
                mut payload,
            } => {
                match unescaper.accept(byte) {
                    Ok(Some(byte)) => {
                        if payload.len() == payload.capacity() {
                            // Reached maximum payload length, forced to finish the transfer
                            self.state = State::Idle;
                            return Ok(self.complete_transfer(header, payload));
                        } else {
                            // Keep collecting bytes
                            payload.push(byte);
                            State::Payload {
                                unescaper,
                                header,
                                payload,
                            }
                        }
                    }
                    Ok(None) => {
                        // Stay in the same state
                        State::Payload {
                            unescaper,
                            header,
                            payload,
                        }
                    }
                    Err(_) => {
                        // Got a zero (end delimiter)
                        self.state = State::BetweenTransfers;
                        // Check and finish the transfer
                        return Ok(self.complete_transfer(header, payload));
                    }
                }
            }
        };
        Ok(None)
    }
}

impl<C, D, S> Receiver<C> for SerialReceiver<C, D, S>
where
    C: Clock,
    D: ReceiveDriver,
    S: SubscriptionManager<Subscription> + Default,
{
    type Transport = SerialTransport;
    type Driver = D;
    type Error = Error<D::Error>;

    fn receive(
        &mut self,
        clock: &mut C,
        driver: &mut D,
    ) -> Result<Option<Transfer<Vec<u8>, Self::Transport>>, Self::Error> {
        self.clean_expired_sessions(clock.now());
        loop {
            match driver.receive_byte() {
                Ok(byte) => match self.handle_byte(byte, clock.now()) {
                    Ok(Some(transfer)) => break Ok(Some(transfer)),
                    Ok(None) => { /* Keep going and try another byte */ }
                    Err(e) => break Err(e),
                },
                Err(nb::Error::WouldBlock) => break Ok(None),
                Err(nb::Error::Other(e)) => break Err(Error::Driver(e)),
            }
        }
    }

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
        _driver: &mut D,
    ) -> Result<(), Self::Error> {
        self.subscriptions
            .subscribe_message(subject, Subscription::new(payload_size_max, timeout))
            .map_err(Error::Memory)
    }

    fn unsubscribe_message(&mut self, subject: SubjectId, _driver: &mut D) {
        self.subscriptions.unsubscribe_message(subject);
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
        _driver: &mut D,
    ) -> Result<(), ServiceSubscribeError<Self::Error>> {
        if self.node_id.is_some() {
            self.subscriptions
                .subscribe_request(service, Subscription::new(payload_size_max, timeout))
                .map_err(|oom| ServiceSubscribeError::Transport(Error::Memory(oom)))
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }

    fn unsubscribe_request(&mut self, service: ServiceId, _driver: &mut D) {
        self.subscriptions.unsubscribe_request(service);
    }

    fn subscribe_response(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
        _driver: &mut D,
    ) -> Result<(), ServiceSubscribeError<Self::Error>> {
        if self.node_id.is_some() {
            self.subscriptions
                .subscribe_response(service, Subscription::new(payload_size_max, timeout))
                .map_err(|oom| ServiceSubscribeError::Transport(Error::Memory(oom)))
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }

    fn unsubscribe_response(&mut self, service: ServiceId, _driver: &mut D) {
        self.subscriptions.unsubscribe_response(service);
    }

    fn set_id(&mut self, id: Option<SerialNodeId>) {
        self.node_id = id;
    }

    fn subscribers(&self) -> impl Iterator<Item = SubjectId> {
        self.subscriptions.subscribers()
    }

    fn servers(&self) -> impl Iterator<Item = ServiceId> {
        self.subscriptions.servers()
    }
}

impl<C, D, S> SerialReceiver<C, D, S>
where
    C: Clock,
    S: SubscriptionManager<Subscription>,
{
    /// Finds and returns a subscription that matches the provided header (and, for service
    /// transfers, has this node as its destination) if any exists
    fn find_subscription_mut(
        &mut self,
        header: &Header<SerialTransport>,
    ) -> Option<&mut Subscription> {
        match header {
            Header::Message(header) => self
                .subscriptions
                .find_message_subscription_mut(header.subject),
            Header::Request(header) => {
                if self.node_id == Some(header.destination) {
                    self.subscriptions
                        .find_request_subscription_mut(header.service)
                } else {
                    None
                }
            }
            Header::Response(header) => {
                if self.node_id == Some(header.destination) {
                    self.subscriptions
                        .find_response_subscription_mut(header.service)
                } else {
                    None
                }
            }
        }
    }

    /// Returns true if this receiver has a matching subscription, its last transfer ID is less
    /// than the provided header's transfer ID, and (for service transfers) this node is the
    /// destination
    fn is_interested(&self, header: &Header<SerialTransport>) -> Option<&Subscription> {
        self.subscriptions
            .find_subscription(header)
            .and_then(|subscription| {
                match header.source() {
                    Some(source) => {
                        match subscription.sessions.get(source) {
                            Some(session) => {
                                if session.last_transfer_id < *header.transfer_id() {
                                    Some(subscription)
                                } else {
                                    // Duplicate transfer
                                    None
                                }
                            }
                            None => {
                                // No session, accept
                                Some(subscription)
                            }
                        }
                    }
                    None => {
                        // Anonymous transfers can't take advantage of deduplication. Always accept.
                        Some(subscription)
                    }
                }
            })
    }

    fn complete_transfer(
        &mut self,
        header: Header<SerialTransport>,
        mut payload_and_crc: Vec<u8>,
    ) -> Option<Transfer<Vec<u8>, SerialTransport>> {
        if payload_and_crc.len() >= 4 {
            let mut crc_bytes = [0u8; 4];
            crc_bytes.copy_from_slice(&payload_and_crc[payload_and_crc.len() - 4..]);
            let crc = u32::from_le_bytes(crc_bytes);

            payload_and_crc.truncate(payload_and_crc.len() - 4);
            let payload = payload_and_crc;
            if crc != make_payload_crc(&payload) {
                // Incorrect CRC
                return None;
            }

            // Record that this transfer was received
            if let Some(subscription) = self.find_subscription_mut(&header) {
                if let Some(source_node) = header.source() {
                    // This may fail to allocate memory.
                    // TODO: Handle allocation failure
                    let _ = subscription.sessions.insert(
                        *source_node,
                        Session {
                            expiration_time: header.timestamp() + subscription.timeout,
                            last_transfer_id: *header.transfer_id(),
                        },
                    );
                }
                Some(Transfer {
                    header,
                    loopback: false,
                    payload,
                })
            } else {
                // The subscription was removed while receiving the transfer
                None
            }
        } else {
            // Not enough bytes for a CRC
            None
        }
    }
}

pub struct Subscription {
    /// The maximum payload size, in bytes
    payload_size_max: usize,
    /// Transfer ID timeout
    timeout: MicrosecondDuration32,
    /// A session for each node (and an associated last transfer ID)
    ///
    /// This is used to remove duplicates
    sessions: TryHashMap<SerialNodeId, Session>,
}

impl Subscription {
    fn new(payload_size_max: usize, timeout: MicrosecondDuration32) -> Self {
        Subscription {
            payload_size_max,
            timeout,
            sessions: Default::default(),
        }
    }

    /// Removes all sessions that have expired
    fn clean_expired_sessions(&mut self, now: Microseconds32) {
        loop {
            let mut id_to_remove: Option<SerialNodeId> = None;
            for (id, session) in self.sessions.iter() {
                if session.expiration_time < now {
                    id_to_remove = Some(*id);
                }
            }
            match id_to_remove {
                Some(id) => {
                    self.sessions.remove(&id);
                }
                None => break,
            }
        }
    }
}

struct Session {
    expiration_time: Microseconds32,
    last_transfer_id: SerialTransferId,
}

/// Receiver states
enum State {
    /// Waiting for the first zero byte
    Idle,
    /// Got a zero byte, waiting for the first non-zero byte to begin a transfer
    BetweenTransfers,
    /// Collecting the header
    ///
    /// When the final header byte arrives, it will be inspected
    Header {
        unescaper: Unescaper,
        header: HeaderCollector,
    },
    /// Got a header, collecting payload bytes
    ///
    /// The last 4 bytes of the payload may be the payload CRC.
    ///
    /// The capacity of the payload is set to the maximum payload length plus 4 bytes.
    Payload {
        unescaper: Unescaper,
        header: Header<SerialTransport>,
        payload: Vec<u8>,
    },
}
