use alloc::vec::Vec;
use core::convert::TryFrom;
use core::marker::PhantomData;
use embedded_nal::Ipv4Addr;

use fallible_collections::FallibleVec;
use zerocopy::FromBytes;

use canadensis_core::session::{Session, SessionTracker};
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, MessageHeader, ServiceHeader, Transfer};
use canadensis_core::transport::Receiver;
use canadensis_core::{OutOfMemoryError, ServiceId, ServiceSubscribeError, SubjectId};
use canadensis_header::{DataSpecifier, Header as UdpHeader, RawHeader};

use crate::address::Address;
use crate::driver::UdpSocket;
use crate::rx::buildup::Buildup;
use crate::rx::subscriptions::Subscriptions;
use crate::{data_crc, MIN_PACKET_SIZE, TRANSFER_CRC_SIZE};
use crate::{Error, UdpNodeId, UdpTransferId, UdpTransport};

mod buildup;
mod subscriptions;

/// UDP transport receiver
pub struct UdpReceiver<I, T, S, const MTU: usize>
where
    I: Instant,
{
    subscriptions: Subscriptions<I, T>,
    /// The ID of this node, or None if this node is anonymous
    node_id: Option<UdpNodeId>,
    /// The IP address of the local interface that the socket is bound to
    local_address: Ipv4Addr,
    _socket: PhantomData<S>,
    _session_tracker: PhantomData<T>,
}

impl<I, T, S, const MTU: usize> UdpReceiver<I, T, S, MTU>
where
    I: Instant,
    T: SessionTracker<I, UdpNodeId, UdpTransferId, UdpSessionData> + Default,
    S: UdpSocket,
{
    pub fn new(node_id: Option<UdpNodeId>, interface_address: Ipv4Addr) -> Self {
        UdpReceiver {
            subscriptions: Subscriptions::new(),
            node_id,
            local_address: interface_address,
            _socket: PhantomData,
            _session_tracker: PhantomData,
        }
    }

    fn clean_expired_sessions(&mut self, now: I)
    where
        T: SessionTracker<I, UdpNodeId, UdpTransferId, UdpSessionData> + Default,
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
        now: I,
        socket: &mut S,
    ) -> Result<Option<Transfer<Vec<u8>, I, UdpTransport>>, Error<nb::Error<S::Error>>> {
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

impl<I, T, S, const MTU: usize> Receiver<I> for UdpReceiver<I, T, S, MTU>
where
    I: Instant,
    T: SessionTracker<I, UdpNodeId, UdpTransferId, UdpSessionData> + Default,
    S: UdpSocket,
{
    type Transport = UdpTransport;
    type Driver = S;
    type Error = Error<S::Error>;

    fn receive(
        &mut self,
        now: I,
        socket: &mut S,
    ) -> Result<Option<Transfer<Vec<u8>, I, Self::Transport>>, Self::Error> {
        // Loop until all incoming packets have been read
        let result = loop {
            match self.accept_inner(now, socket) {
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
        self.clean_expired_sessions(now);
        result
    }

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
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
        timeout: <I as Instant>::Duration,
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
        timeout: <I as Instant>::Duration,
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
}

pub struct Subscription<I, T>
where
    I: Instant,
{
    payload_size_max: usize,
    timeout: <I as Instant>::Duration,
    sessions: T,
}

impl<I, T> Subscription<I, T>
where
    I: Instant,
    T: SessionTracker<I, UdpNodeId, UdpTransferId, UdpSessionData> + Default,
{
    /// Creates a subscription
    fn new(payload_size_max: usize, timeout: <I as Instant>::Duration) -> Self {
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
        now: I,
    ) -> Result<Option<Transfer<Vec<u8>, I, UdpTransport>>, OutOfMemoryError> {
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

            if let Ok(Some(_)) = &result {
                // Successfully received
                // Don't need the session anymore
                self.sessions.remove(source_node_id);
            }

            self.convert_reassembly_result(result, header, now)
        } else {
            // Special case for anonymous transfers, which must be single-frame
            let mut session = Session::new(now, self.timeout, None, UdpSessionData::default());
            let result = session.handle_frame(header, bytes_after_header, self.payload_size_max);
            self.convert_reassembly_result(result, header, now)
        }
    }

    fn convert_reassembly_result(
        &self,
        result: Result<Option<Vec<u8>>, OutOfMemoryError>,
        header: &UdpHeader,
        now: I,
    ) -> Result<Option<Transfer<Vec<u8>, I, UdpTransport>>, OutOfMemoryError> {
        match result {
            Ok(Some(reassembled)) => {
                // Add the transfer headers and record the completed transfer
                let header = match header.data_specifier {
                    DataSpecifier::Subject { from, subject, .. } => {
                        Header::Message(MessageHeader {
                            timestamp: now,
                            transfer_id: header.transfer_id,
                            priority: header.priority,
                            subject,
                            source: from,
                        })
                    }
                    DataSpecifier::ServiceRequest { service, from, to } => {
                        Header::Request(ServiceHeader {
                            timestamp: now,
                            transfer_id: header.transfer_id,
                            priority: header.priority,
                            service,
                            source: from,
                            destination: to,
                        })
                    }
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
                Ok(Some(Transfer {
                    header,
                    loopback: false,
                    payload: reassembled,
                }))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn clean_expired_sessions(&mut self, now: I) {
        self.sessions.remove_expired(now)
    }
}

#[derive(Default)]
pub struct UdpSessionData {
    buildup: Option<Buildup>,
}

trait UdpSession<I>
where
    I: Instant,
{
    fn handle_frame(
        &mut self,
        header: &UdpHeader,
        bytes_after_header: &[u8],
        max_payload_length: usize,
    ) -> Result<Option<Vec<u8>>, OutOfMemoryError>;
}

impl<I> UdpSession<I> for Session<I, UdpTransferId, UdpSessionData>
where
    I: Instant,
{
    /// Handles a frame
    ///
    /// If the frame successfully completed a transfer, this function returns the assembled transfer
    /// payload.
    fn handle_frame(
        &mut self,
        header: &UdpHeader,
        bytes_after_header: &[u8],
        max_payload_length: usize,
    ) -> Result<Option<Vec<u8>>, OutOfMemoryError> {
        // The buildup will collect the payload and the transfer CRC in the last frame, so it
        // needs extra capacity
        let max_payload_and_crc_length = max_payload_length + TRANSFER_CRC_SIZE;
        if bytes_after_header.len() < TRANSFER_CRC_SIZE + 1 {
            // Frame not long enough
            return Ok(None);
        }
        let payload_bytes = &bytes_after_header[..bytes_after_header.len() - TRANSFER_CRC_SIZE];
        // Every frame has a transfer CRC at the end.
        // If this is the last frame, the CRC covers the whole transfer (not including the CRCs of
        // the previous frames).
        // Otherwise, the CRC covers the data in this frame only.
        // 4 cases:
        let first_frame = header.frame_index == 0;
        let last_frame = header.last_frame;
        match (first_frame, last_frame) {
            (true, true) => {
                // Frame index 0 and last (single-frame transfer):
                // Check frame CRC, no buildup, return payload only
                if check_frame_crc(bytes_after_header) {
                    let mut payload: Vec<u8> = FallibleVec::try_with_capacity(payload_bytes.len())?;
                    payload.extend_from_slice(payload_bytes);
                    Ok(Some(payload))
                } else {
                    Ok(None)
                }
            }
            (true, false) => {
                // Frame index 0 and not last (beginning):
                // Check frame CRC, create buildup and add payload only
                if check_frame_crc(bytes_after_header) {
                    let buildup =
                        match Buildup::new(header, payload_bytes, max_payload_and_crc_length) {
                            Ok(buildup) => buildup,
                            Err(_) => {
                                // payload_bytes was greater than max_payload_and_crc_length
                                return Ok(None);
                            }
                        };
                    self.data_mut().buildup = Some(buildup);
                } else {
                    log::debug!("Incorrect first frame CRC");
                }
                Ok(None)
            }
            (false, false) => {
                // Frame index >0 and not last (middle):
                // Check frame CRC, add payload only to buildup
                if check_frame_crc(bytes_after_header) {
                    if let Some(buildup) = self.data_mut().buildup.as_mut() {
                        let _ = buildup.push(header, payload_bytes);
                    }
                }
                Ok(None)
            }
            (false, true) => {
                // Frame index >0 and last (end):
                // Add payload and transfer CRC to buildup, extract combined payload and transfer CRC,
                // check full transfer CRC, return combined payload
                if let Some(mut buildup) = self.data_mut().buildup.take() {
                    if buildup.push(header, bytes_after_header).is_ok() {
                        let payload_and_crc = buildup.into_payload();
                        if check_frame_crc(&payload_and_crc) {
                            // Remove CRC from the end
                            let mut payload = payload_and_crc;
                            payload.truncate(payload.len() - TRANSFER_CRC_SIZE);
                            Ok(Some(payload))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                } else {
                    log::debug!("No buildup");
                    Ok(None)
                }
            }
        }
    }
}

fn check_frame_crc(bytes_after_header: &[u8]) -> bool {
    let crc_start = bytes_after_header.len() - TRANSFER_CRC_SIZE;
    let expected_crc = {
        let mut expected_crc_bytes: [u8; 4] = [0; 4];
        expected_crc_bytes.copy_from_slice(&bytes_after_header[crc_start..]);
        u32::from_le_bytes(expected_crc_bytes)
    };

    let bytes_to_crc = &bytes_after_header[..crc_start];

    let mut crc = data_crc();
    crc.digest(bytes_to_crc);
    crc.get_crc() == expected_crc
}
