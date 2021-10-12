use crate::cobs::Unescaper;
use crate::header_collector::HeaderCollector;
use crate::{make_payload_crc, SerialNodeId, SerialTransport};
use alloc::vec::Vec;
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, Transfer};
use canadensis_core::transport::{Receiver, Transport};
use canadensis_core::{OutOfMemoryError, PortId, ServiceId, ServiceSubscribeError, SubjectId};
use core::mem;
use fallible_collections::FallibleVec;

/// A serial transport receiver
///
/// This implementation does not support multi-frame transfers or timestamps.
pub struct SerialReceiver<I> {
    state: State<I>,
    node_id: Option<SerialNodeId>,
    message_subscriptions: Vec<Subscription>,
    request_subscriptions: Vec<Subscription>,
    response_subscriptions: Vec<Subscription>,
}

impl<I> SerialReceiver<I> {
    pub fn new(node_id: SerialNodeId) -> Self {
        SerialReceiver {
            state: State::Idle,
            node_id: Some(node_id),
            message_subscriptions: Vec::new(),
            request_subscriptions: Vec::new(),
            response_subscriptions: Vec::new(),
        }
    }
    pub fn new_anonymous() -> Self {
        SerialReceiver {
            state: State::Idle,
            node_id: None,
            message_subscriptions: Vec::new(),
            request_subscriptions: Vec::new(),
            response_subscriptions: Vec::new(),
        }
    }
}

impl<I: Instant + Default> Receiver<I> for SerialReceiver<I> {
    type Transport = SerialTransport;

    fn accept(
        &mut self,
        frame: u8,
    ) -> Result<Option<Transfer<Vec<u8>, I, Self::Transport>>, <Self::Transport as Transport>::Error>
    {
        let state = mem::replace(&mut self.state, State::Idle);
        self.state = match state {
            State::Idle => {
                if frame == 0 {
                    State::BetweenTransfers
                } else {
                    State::Idle
                }
            }
            State::BetweenTransfers => {
                if frame != 0 {
                    // Start decoding
                    log::debug!("Starting frame");
                    let mut unescaper = Unescaper::new();
                    match unescaper.accept(frame) {
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
                match unescaper.accept(frame) {
                    Ok(Some(byte)) => {
                        header.push(byte);

                        if header.is_done() {
                            // Got the complete header
                            let header = header.as_header();
                            match header.into_header(I::default()) {
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
                                                return Err(OutOfMemoryError);
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
                match unescaper.accept(frame) {
                    Ok(Some(byte)) => {
                        if payload.len() == payload.capacity() {
                            // Reached maximum payload length, forced to finish the transfer
                            self.state = State::Idle;
                            return Ok(complete_transfer(header, payload));
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
                        return Ok(complete_transfer(header, payload));
                    }
                }
            }
        };
        Ok(None)
    }

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        _timeout: <I as Instant>::Duration,
    ) -> Result<(), <Self::Transport as Transport>::Error> {
        subscribe(
            &mut self.message_subscriptions,
            subject.into(),
            payload_size_max,
        )
    }

    fn unsubscribe_message(&mut self, subject: SubjectId) {
        unsubscribe(&mut self.message_subscriptions, subject.into());
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        _timeout: <I as Instant>::Duration,
    ) -> Result<(), ServiceSubscribeError<<Self::Transport as Transport>::Error>> {
        if self.node_id.is_some() {
            subscribe(
                &mut self.request_subscriptions,
                service.into(),
                payload_size_max,
            )
            .map_err(ServiceSubscribeError::Transport)
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }

    fn unsubscribe_request(&mut self, service: ServiceId) {
        unsubscribe(&mut self.request_subscriptions, service.into());
    }

    fn subscribe_response(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        _timeout: <I as Instant>::Duration,
    ) -> Result<(), ServiceSubscribeError<<Self::Transport as Transport>::Error>> {
        if self.node_id.is_some() {
            subscribe(
                &mut self.response_subscriptions,
                service.into(),
                payload_size_max,
            )
            .map_err(ServiceSubscribeError::Transport)
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }

    fn unsubscribe_response(&mut self, service: ServiceId) {
        unsubscribe(&mut self.response_subscriptions, service.into());
    }
}

impl<I> SerialReceiver<I> {
    fn is_interested(&self, header: &Header<I, SerialTransport>) -> Option<&Subscription> {
        match header {
            Header::Message(header) => self
                .message_subscriptions
                .iter()
                .find(|sub| sub.port == header.subject.into()),
            Header::Request(header) => {
                if self.node_id == Some(header.destination) {
                    self.request_subscriptions
                        .iter()
                        .find(|sub| sub.port == header.service.into())
                } else {
                    None
                }
            }
            Header::Response(header) => {
                if self.node_id == Some(header.destination) {
                    self.response_subscriptions
                        .iter()
                        .find(|sub| sub.port == header.service.into())
                } else {
                    None
                }
            }
        }
    }
}

fn complete_transfer<I>(
    header: Header<I, SerialTransport>,
    mut payload_and_crc: Vec<u8>,
) -> Option<Transfer<Vec<u8>, I, SerialTransport>> {
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
        Some(Transfer { header, payload })
    } else {
        // Not enough bytes for a CRC
        None
    }
}

struct Subscription {
    port: PortId,
    payload_size_max: usize,
}

fn unsubscribe(subscriptions: &mut Vec<Subscription>, port: PortId) {
    if let Some(index) = subscriptions.iter().position(|sub| sub.port == port) {
        subscriptions.swap_remove(index);
    }
}

fn subscribe(
    subscriptions: &mut Vec<Subscription>,
    port: PortId,
    payload_size_max: usize,
) -> Result<(), OutOfMemoryError> {
    // Remove any existing subscription for this port
    unsubscribe(subscriptions, port);
    FallibleVec::try_push(
        subscriptions,
        Subscription {
            port,
            payload_size_max,
        },
    )?;
    Ok(())
}

/// Receiver states
enum State<I> {
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
        header: Header<I, SerialTransport>,
        payload: Vec<u8>,
    },
}
