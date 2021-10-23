use crate::cobs::Unescaper;
use crate::driver::ReceiveDriver;
use crate::header_collector::HeaderCollector;
use crate::{make_payload_crc, Error, SerialNodeId, SerialTransferId, SerialTransport};
use alloc::vec::Vec;
use canadensis_core::subscription::SubscriptionManager;
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, Transfer};
use canadensis_core::transport::{Receiver, Transport};
use canadensis_core::{nb, OutOfMemoryError, ServiceId, ServiceSubscribeError, SubjectId};
use core::cmp::Ordering;
use core::mem;
use fallible_collections::{FallibleVec, TryHashMap};

/// A serial transport receiver
///
/// This implementation does not support multi-frame transfers or timestamps.
pub struct SerialReceiver<I, D, S>
where
    D: ReceiveDriver,
{
    /// The driver used to receive bytes
    driver: D,
    state: State<I, D>,
    node_id: Option<SerialNodeId>,
    subscriptions: S,
}

impl<I, D, S> SerialReceiver<I, D, S>
where
    I: Instant,
    D: ReceiveDriver,
    S: SubscriptionManager<Subscription<I>> + Default,
{
    pub fn new(driver: D, node_id: SerialNodeId) -> Self {
        SerialReceiver {
            driver,
            state: State::Idle,
            node_id: Some(node_id),
            subscriptions: S::default(),
        }
    }
    pub fn new_anonymous(driver: D) -> Self {
        SerialReceiver {
            driver,
            state: State::Idle,
            node_id: None,
            subscriptions: S::default(),
        }
    }

    fn clean_expired_sessions(&mut self, now: I) {
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
        now: I,
    ) -> Result<Option<Transfer<Vec<u8>, I, SerialTransport<D::Error>>>, Error<D::Error>> {
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
                    log::debug!("Starting frame");
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
                            match header.into_header(now) {
                                Ok(header) => {
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
                                        log::debug!("Got header, but not subscribed");
                                        State::Idle
                                    }
                                }
                                Err(e) => {
                                    // Invalid header CRC or format
                                    log::debug!("Header format or CRC invalid: {:?}", e);
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

impl<I, D, S> Receiver<I> for SerialReceiver<I, D, S>
where
    I: Instant + Default,
    D: ReceiveDriver,
    S: SubscriptionManager<Subscription<I>> + Default,
{
    type Transport = SerialTransport<D::Error>;

    fn receive(
        &mut self,
        now: I,
    ) -> Result<Option<Transfer<Vec<u8>, I, Self::Transport>>, <Self::Transport as Transport>::Error>
    {
        self.clean_expired_sessions(now);
        loop {
            match self.driver.receive_byte() {
                Ok(byte) => match self.handle_byte(byte, now) {
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
        timeout: <I as Instant>::Duration,
    ) -> Result<(), <Self::Transport as Transport>::Error> {
        self.subscriptions
            .subscribe_message(subject, Subscription::new(payload_size_max, timeout))
            .map_err(Error::Memory)
    }

    fn unsubscribe_message(&mut self, subject: SubjectId) {
        self.subscriptions.unsubscribe_message(subject);
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
    ) -> Result<(), ServiceSubscribeError<<Self::Transport as Transport>::Error>> {
        if self.node_id.is_some() {
            self.subscriptions
                .subscribe_request(service, Subscription::new(payload_size_max, timeout))
                .map_err(|oom| ServiceSubscribeError::Transport(Error::Memory(oom)))
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }

    fn unsubscribe_request(&mut self, service: ServiceId) {
        self.subscriptions.unsubscribe_request(service);
    }

    fn subscribe_response(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
    ) -> Result<(), ServiceSubscribeError<<Self::Transport as Transport>::Error>> {
        if self.node_id.is_some() {
            self.subscriptions
                .subscribe_response(service, Subscription::new(payload_size_max, timeout))
                .map_err(|oom| ServiceSubscribeError::Transport(Error::Memory(oom)))
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }

    fn unsubscribe_response(&mut self, service: ServiceId) {
        self.subscriptions.unsubscribe_response(service);
    }
}

impl<I, D, S> SerialReceiver<I, D, S>
where
    I: Instant,
    D: ReceiveDriver,
    S: SubscriptionManager<Subscription<I>>,
{
    /// Finds and returns a subscription that matches the provided header (and, for service
    /// transfers, has this node as its destination) if any exists
    fn find_subscription_mut(
        &mut self,
        header: &Header<I, SerialTransport<D::Error>>,
    ) -> Option<&mut Subscription<I>> {
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
    fn is_interested(
        &self,
        header: &Header<I, SerialTransport<D::Error>>,
    ) -> Option<&Subscription<I>> {
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
        header: Header<I, SerialTransport<D::Error>>,
        mut payload_and_crc: Vec<u8>,
    ) -> Option<Transfer<Vec<u8>, I, SerialTransport<D::Error>>> {
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
                        source_node.clone(),
                        Session {
                            expiration_time: subscription.timeout + header.timestamp(),
                            last_transfer_id: header.transfer_id().clone(),
                        },
                    );
                }
                Some(Transfer { header, payload })
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

pub struct Subscription<I>
where
    I: Instant,
{
    /// The maximum payload size, in bytes
    payload_size_max: usize,
    /// Transfer ID timeout
    timeout: <I as Instant>::Duration,
    /// A session for each node (and an associated last transfer ID)
    ///
    /// This is used to remove duplicates
    sessions: TryHashMap<SerialNodeId, Session<I>>,
}

impl<I> Subscription<I>
where
    I: Instant,
{
    fn new(payload_size_max: usize, timeout: <I as Instant>::Duration) -> Self {
        Subscription {
            payload_size_max,
            timeout,
            sessions: Default::default(),
        }
    }

    /// Removes all sessions that have expired
    fn clean_expired_sessions(&mut self, now: I) {
        loop {
            let mut id_to_remove: Option<SerialNodeId> = None;
            for (id, session) in self.sessions.iter() {
                if session.expiration_time.overflow_safe_compare(&now) == Ordering::Less {
                    id_to_remove = Some(id.clone());
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

struct Session<I> {
    expiration_time: I,
    last_transfer_id: SerialTransferId,
}

/// Receiver states
enum State<I, D>
where
    D: ReceiveDriver,
{
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
        header: Header<I, SerialTransport<D::Error>>,
        payload: Vec<u8>,
    },
}
