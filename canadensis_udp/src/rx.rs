use alloc::vec::Vec;
use core::convert::TryFrom;
use core::marker::PhantomData;
use core::net::Ipv4Addr;

use fallible_collections::FallibleVec;
use zerocopy::FromBytes;

use canadensis_core::crc::CrcTracker;
use canadensis_core::session::{Session, SessionTracker};
use canadensis_core::time::{Clock, MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, ServiceHeader, Transfer};
use canadensis_core::transport::Receiver;
use canadensis_core::{OutOfMemoryError, ServiceId, ServiceSubscribeError, SubjectId};
use canadensis_header::{DataSpecifier, Header as UdpHeader, RawHeader};

use crate::address::Address;
use crate::driver::UdpSocket;
use crate::rx::buildup::{Buildup, BuildupError};
use crate::rx::subscriptions::Subscriptions;
use crate::{Error, UdpNodeId, UdpTransferId, UdpTransport};
use crate::{MIN_PACKET_SIZE, TRANSFER_CRC_SIZE};

mod buildup;
mod subscriptions;

/// UDP transport receiver
pub struct UdpReceiver<C, T, S, const MTU: usize> {
    subscriptions: Subscriptions<T>,
    /// The ID of this node, or None if this node is anonymous
    node_id: Option<UdpNodeId>,
    /// The IP address of the local interface that the socket is bound to
    local_address: Ipv4Addr,
    _socket: PhantomData<S>,
    _session_tracker: PhantomData<T>,
    _clock: PhantomData<C>,
}

impl<C, T, S, const MTU: usize> UdpReceiver<C, T, S, MTU>
where
    T: SessionTracker<UdpNodeId, UdpTransferId, UdpSessionData> + Default,
    S: UdpSocket,
{
    pub fn new(node_id: Option<UdpNodeId>, interface_address: Ipv4Addr) -> Self {
        UdpReceiver {
            subscriptions: Subscriptions::new(),
            node_id,
            local_address: interface_address,
            _socket: PhantomData,
            _session_tracker: PhantomData,
            _clock: PhantomData,
        }
    }

    fn clean_expired_sessions(&mut self, now: Microseconds32)
    where
        T: SessionTracker<UdpNodeId, UdpTransferId, UdpSessionData> + Default,
    {
        for subscription in self.subscriptions.message_iter_mut() {
            subscription.clean_expired_sessions(now);
        }
        for subscription in self.subscriptions.request_iter_mut() {
            subscription.clean_expired_sessions(now);
        }
        for subscription in self.subscriptions.response_iter_mut() {
            subscription.clean_expired_sessions(now);
        }
    }

    /// Reads one incoming frame and processes it through the matching subscription
    ///
    /// Return values:
    /// * `Ok(Some(transfer))` if a transfer was received
    /// * `Ok(None)` if a packet was read, but it did not complete a transfer
    /// * `Err(nb::Error::WouldBlock)` if no packet was available to read
    /// * `Err(e)` if a socket or memory allocation error occurred
    fn accept_inner(
        &mut self,
        now: Microseconds32,
        socket: &mut S,
    ) -> Result<Option<Transfer<Vec<u8>, UdpTransport>>, Error<nb::Error<S::Error>>> {
        let mut buffer: [u8; MTU] = [0; MTU];
        let bytes_received = socket.recv(&mut buffer).map_err(Error::Socket)?;
        let buffer = &buffer[..bytes_received];

        if bytes_received < MIN_PACKET_SIZE {
            // Ignore packet
            return Ok(None);
        }
        // Check header validity, ignore frames with invalid headers
        let header =
            RawHeader::read_from_prefix(buffer).and_then(|header| UdpHeader::try_from(header).ok());
        let header: UdpHeader = match header {
            Some(header) => header,
            None => return Ok(None),
        };
        let bytes_after_header = &buffer[canadensis_header::SIZE..];

        // Look for a matching subscription
        match header.data_specifier {
            DataSpecifier::Subject { subject, .. } => {
                if let Some(subscription) =
                    self.subscriptions.find_message_subscription_mut(subject)
                {
                    return subscription
                        .handle_frame(&header, bytes_after_header, now)
                        .map_err(Error::Memory);
                } else {
                    log::trace!("No matching subject subscription");
                }
            }
            DataSpecifier::ServiceRequest { service, .. } => {
                if let Some(subscription) =
                    self.subscriptions.find_request_subscription_mut(service)
                {
                    return subscription
                        .handle_frame(&header, bytes_after_header, now)
                        .map_err(Error::Memory);
                } else {
                    log::trace!("No matching subject subscription");
                }
            }
            DataSpecifier::ServiceResponse { service, .. } => {
                if let Some(subscription) =
                    self.subscriptions.find_response_subscription_mut(service)
                {
                    return subscription
                        .handle_frame(&header, bytes_after_header, now)
                        .map_err(Error::Memory);
                } else {
                    log::trace!("No matching subject subscription");
                }
            }
        }
        Ok(None)
    }

    /// Call this function before adding a service subscription
    /// to join the multicast group if necessary
    fn service_subscribe_check_multicast(&mut self, socket: &mut S) -> Result<(), S::Error> {
        if let Some(node_id) = self.node_id {
            // If this node hasn't already subscribed to a service request/response and joined
            // its own multicast group, join the group now
            if !self.subscriptions.any_service_subscriptions() {
                socket.join_multicast_v4(&Address::Node(node_id).into(), &self.local_address)?;
            }
        }
        Ok(())
    }

    /// Call this function after removing a service subscription
    /// to leave the multicast group if necessary
    fn service_unsubscribe_check_multicast(&mut self, socket: &mut S) -> Result<(), S::Error> {
        if let Some(node_id) = self.node_id {
            // If this node has no more service request/response subscriptions, leave its
            // multicast group
            if !self.subscriptions.any_service_subscriptions() {
                socket.leave_multicast_v4(&Address::Node(node_id).into(), &self.local_address)?;
            }
        }
        Ok(())
    }
}

impl<C, T, S, const MTU: usize> Receiver<C> for UdpReceiver<C, T, S, MTU>
where
    C: Clock,
    T: SessionTracker<UdpNodeId, UdpTransferId, UdpSessionData> + Default,
    S: UdpSocket,
{
    type Transport = UdpTransport;
    type Driver = S;
    type Error = Error<S::Error>;

    fn receive(
        &mut self,
        clock: &mut C,
        socket: &mut S,
    ) -> Result<Option<Transfer<Vec<u8>, Self::Transport>>, Self::Error> {
        // Loop until all incoming packets have been read
        let result = loop {
            match self.accept_inner(clock.now(), socket) {
                Ok(Some(transfer)) => break Ok(Some(transfer)),
                Ok(None) => { /* Keep going and try to read another packet */ }
                Err(Error::Socket(nb::Error::WouldBlock)) => {
                    // Can't read any more
                    break Ok(None);
                }
                Err(Error::Memory(e)) => break Err(Error::Memory(e)),
                Err(Error::Socket(nb::Error::Other(e))) => break Err(Error::Socket(e)),
            }
        };
        self.clean_expired_sessions(clock.now());
        result
    }

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
        socket: &mut S,
    ) -> Result<(), Self::Error> {
        socket
            .join_multicast_v4(&Address::Multicast(subject).into(), &self.local_address)
            .map_err(Error::Socket)?;
        self.subscriptions
            .subscribe_message(subject, Subscription::new(payload_size_max, timeout));
        Ok(())
    }

    fn unsubscribe_message(&mut self, subject: SubjectId, socket: &mut S) {
        let _ = socket.leave_multicast_v4(&Address::Multicast(subject).into(), &self.local_address);
        self.subscriptions.unsubscribe_message(subject);
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
        socket: &mut S,
    ) -> Result<(), ServiceSubscribeError<Self::Error>> {
        self.service_subscribe_check_multicast(socket)
            .map_err(|e| ServiceSubscribeError::Transport(Error::Socket(e)))?;
        if self.node_id.is_some() {
            let subscription = Subscription::new(payload_size_max, timeout);
            self.subscriptions.subscribe_request(service, subscription);
            Ok(())
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }

    fn unsubscribe_request(&mut self, service: ServiceId, socket: &mut S) {
        self.subscriptions.unsubscribe_request(service);
        let _ = self.service_unsubscribe_check_multicast(socket);
    }

    fn subscribe_response(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
        socket: &mut S,
    ) -> Result<(), ServiceSubscribeError<Self::Error>> {
        self.service_subscribe_check_multicast(socket)
            .map_err(|e| ServiceSubscribeError::Transport(Error::Socket(e)))?;
        if self.node_id.is_some() {
            let subscription = Subscription::new(payload_size_max, timeout);
            self.subscriptions.subscribe_response(service, subscription);
            Ok(())
        } else {
            Err(ServiceSubscribeError::Anonymous)
        }
    }

    fn unsubscribe_response(&mut self, service: ServiceId, socket: &mut S) {
        self.subscriptions.unsubscribe_response(service);
        let _ = self.service_unsubscribe_check_multicast(socket);
    }

    fn set_id(&mut self, id: Option<UdpNodeId>) {
        self.node_id = id;
    }

    fn subscribers(&self) -> impl Iterator<Item = SubjectId> {
        self.subscriptions.subscribers()
    }

    fn servers(&self) -> impl Iterator<Item = ServiceId> {
        self.subscriptions.servers()
    }
}

pub struct Subscription<T> {
    payload_size_max: usize,
    timeout: MicrosecondDuration32,
    sessions: T,
}

impl<T> Subscription<T>
where
    T: SessionTracker<UdpNodeId, UdpTransferId, UdpSessionData> + Default,
{
    /// Creates a subscription
    fn new(payload_size_max: usize, timeout: MicrosecondDuration32) -> Self {
        Subscription {
            payload_size_max,
            timeout,
            sessions: T::default(),
        }
    }

    fn handle_frame(
        &mut self,
        header: &UdpHeader,
        bytes_after_header: &[u8],
        now: Microseconds32,
    ) -> Result<Option<Transfer<Vec<u8>, UdpTransport>>, OutOfMemoryError> {
        let timeout = self.timeout;
        if let Some(source_node_id) = header.data_specifier.source_node_id() {
            let session = self.sessions.get_mut_or_insert_with(source_node_id, || {
                Session::new(now, timeout, None, UdpSessionData::default())
            })?;
            // Check transfer ID
            if let Some(last_transfer_id) = session.last_transfer_id() {
                if header.transfer_id <= *last_transfer_id {
                    // Duplicate
                    log::debug!(
                        "Discarding duplicate transfer with ID {:?}",
                        header.transfer_id
                    );
                    return Ok(None);
                }
            }
            session.set_last_activity(now);
            let result = session.handle_frame(header, bytes_after_header, self.payload_size_max);
            match result {
                Ok(Some(payload)) => {
                    // Successfully received
                    // Don't need the session anymore
                    self.sessions.remove(source_node_id);
                    Ok(Some(self.convert_reassembly_result(payload, header, now)))
                }
                Ok(None) => Ok(None),
                Err(e) => {
                    log::warn!("Buildup error {:?}, removing session", e);
                    self.sessions.remove(source_node_id);
                    Ok(None)
                }
            }
        } else {
            // Special case for anonymous transfers, which must be single-frame
            let mut session = Session::new(now, self.timeout, None, UdpSessionData::default());
            let result = session.handle_frame(header, bytes_after_header, self.payload_size_max);
            match result {
                Ok(Some(payload)) => Ok(Some(self.convert_reassembly_result(payload, header, now))),
                Ok(None) => Ok(None),
                Err(BuildupError::Memory(_)) => Err(OutOfMemoryError),
                Err(_) => Ok(None),
            }
        }
    }

    fn convert_reassembly_result(
        &self,
        reassembled: Vec<u8>,
        header: &UdpHeader,
        now: Microseconds32,
    ) -> Transfer<Vec<u8>, UdpTransport> {
        // Add the transfer headers and record the completed transfer
        let header = match header.data_specifier {
            DataSpecifier::Subject { from, subject, .. } => Header::Message(MessageHeader {
                timestamp: now,
                transfer_id: header.transfer_id,
                priority: header.priority,
                subject,
                source: from,
            }),
            DataSpecifier::ServiceRequest { service, from, to } => Header::Request(ServiceHeader {
                timestamp: now,
                transfer_id: header.transfer_id,
                priority: header.priority,
                service,
                source: from,
                destination: to,
            }),
            DataSpecifier::ServiceResponse { service, from, to } => {
                Header::Response(ServiceHeader {
                    timestamp: now,
                    transfer_id: header.transfer_id,
                    priority: header.priority,
                    service,
                    source: from,
                    destination: to,
                })
            }
        };
        Transfer {
            header,
            loopback: false,
            payload: reassembled,
        }
    }

    fn clean_expired_sessions(&mut self, now: Microseconds32) {
        self.sessions.remove_expired(now)
    }
}

#[derive(Default)]
pub struct UdpSessionData {
    buildup: Option<Buildup>,
}

trait UdpSession {
    fn handle_frame(
        &mut self,
        header: &UdpHeader,
        bytes_after_header: &[u8],
        max_payload_length: usize,
    ) -> Result<Option<Vec<u8>>, BuildupError>;
}

impl UdpSession for Session<UdpTransferId, UdpSessionData> {
    /// Handles a frame
    ///
    /// If the frame successfully completed a transfer, this function returns the assembled transfer
    /// payload.
    fn handle_frame(
        &mut self,
        header: &UdpHeader,
        bytes_after_header: &[u8],
        max_payload_length: usize,
    ) -> Result<Option<Vec<u8>>, BuildupError> {
        if bytes_after_header.len() < TRANSFER_CRC_SIZE {
            // Frame not long enough
            return Ok(None);
        }
        let first_frame = header.frame_index == 0;
        let last_frame = header.last_frame;
        let data = self.data_mut();

        if first_frame && last_frame {
            // Special case for a single-frame transfer
            let mut payload: Vec<u8> = FallibleVec::try_with_capacity(max_payload_length)?;
            let mut crc = CrcTracker::new();
            bytes_after_header.iter().copied().for_each(|byte| {
                if let Some(digested) = crc.digest(byte) {
                    if payload.len() < max_payload_length {
                        payload.push(digested)
                    }
                }
            });
            if crc.correct() {
                Ok(Some(payload))
            } else {
                Err(BuildupError::Crc)
            }
        } else {
            match data.buildup.take() {
                Some(mut buildup) => {
                    buildup.push(header, bytes_after_header)?;
                    if last_frame {
                        if buildup.crc_correct() {
                            Ok(Some(buildup.into_payload()))
                        } else {
                            Err(BuildupError::Crc)
                        }
                    } else {
                        data.buildup = Some(buildup);
                        Ok(None)
                    }
                }
                None => {
                    data.buildup = Some(Buildup::new(
                        header,
                        bytes_after_header,
                        max_payload_length,
                    )?);
                    Ok(None)
                }
            }
        }
    }
}
