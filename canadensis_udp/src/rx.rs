use alloc::vec::Vec;
use core::convert::TryFrom;
use core::marker::PhantomData;
use core::net::Ipv4Addr;

use zerocopy::FromBytes;

use canadensis_core::crc::CrcTracker;
use canadensis_core::session::{ActiveSession, Session, SessionTracker};
use canadensis_core::time::{Clock, MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, ServiceHeader, Transfer};
use canadensis_core::transport::Receiver;
use canadensis_core::{OutOfMemoryError, ServiceId, ServiceSubscribeError, SubjectId};
use canadensis_header::{DataSpecifier, Header as UdpHeader, NodeId16, RawHeader};

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
    C: Clock,
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

    /// Reads one incoming frame and processes it through the matching subscription
    ///
    /// Return values:
    /// * `Ok(Some(transfer))` if a transfer was received
    /// * `Ok(None)` if a packet was read, but it did not complete a transfer
    /// * `Err(nb::Error::WouldBlock)` if no packet was available to read
    /// * `Err(e)` if a socket or memory allocation error occurred
    fn accept_inner(
        &mut self,
        clock: &mut C,
        socket: &mut S,
    ) -> Result<Option<Transfer<Vec<u8>, UdpTransport>>, Error<nb::Error<S::Error>>> {
        let mut buffer: [u8; MTU] = [0; MTU];
        let bytes_received = socket.recv(&mut buffer).map_err(Error::Socket)?;
        let buffer = &buffer[..bytes_received];
        let frame_time = clock.now();

        if bytes_received < MIN_PACKET_SIZE {
            // Ignore packet
            return Ok(None);
        }
        // Check header validity, ignore frames with invalid headers
        let header = RawHeader::read_from_prefix(buffer)
            .ok()
            .and_then(|(header, _)| UdpHeader::try_from(header).ok());
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
                        .handle_frame(&header, bytes_after_header, frame_time)
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
                        .handle_frame(&header, bytes_after_header, frame_time)
                        .map_err(Error::Memory);
                } else {
                    log::trace!("No matching service request subscription");
                }
            }
            DataSpecifier::ServiceResponse { service, .. } => {
                if let Some(subscription) =
                    self.subscriptions.find_response_subscription_mut(service)
                {
                    return subscription
                        .handle_frame(&header, bytes_after_header, frame_time)
                        .map_err(Error::Memory);
                } else {
                    log::trace!("No matching service response subscription");
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
        loop {
            match self.accept_inner(clock, socket) {
                Ok(Some(transfer)) => break Ok(Some(transfer)),
                Ok(None) => { /* Keep going and try to read another packet */ }
                Err(Error::Socket(nb::Error::WouldBlock)) => {
                    // Can't read any more
                    break Ok(None);
                }
                Err(Error::Memory(e)) => break Err(Error::Memory(e)),
                Err(Error::Socket(nb::Error::Other(e))) => break Err(Error::Socket(e)),
            }
        }
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
        if let Some(source_node_id) = header.data_specifier.source_node_id() {
            self.handle_frame_non_anonymous(header, source_node_id, now, bytes_after_header)
        } else {
            // Special case for anonymous transfers, which must be single-frame
            let mut session = ActiveSession {
                time: now,
                transfer_id: header.transfer_id,
                data: UdpSessionData::default(),
            };
            let result = session.handle_frame(header, bytes_after_header, self.payload_size_max);
            match result {
                Ok(Some(payload)) => Ok(Some(self.convert_reassembly_result(payload, header, now))),
                Ok(None) => Ok(None),
                Err(BuildupError::Memory(_)) => Err(OutOfMemoryError),
                Err(_) => Ok(None),
            }
        }
    }

    fn handle_frame_non_anonymous(
        &mut self,
        header: &UdpHeader,
        source_node_id: NodeId16,
        now: Microseconds32,
        bytes_after_header: &[u8],
    ) -> Result<Option<Transfer<Vec<u8>, UdpTransport>>, OutOfMemoryError> {
        let session = self.sessions.get_mut_or_insert_with(source_node_id, || {
            Session::Active(ActiveSession {
                time: now,
                transfer_id: header.transfer_id,
                data: UdpSessionData::default(),
            })
        })?;
        // Check transfer ID
        if let Session::Complete {
            time: last_transfer_time,
            transfer_id: last_transfer_id,
        } = session
        {
            let last_transfer_age = now - *last_transfer_time;
            let sequence_correct = header.transfer_id > *last_transfer_id;
            if !(sequence_correct || last_transfer_age > self.timeout) {
                // Duplicate
                return Ok(None);
            }
            *session = Session::Active(ActiveSession {
                time: now,
                transfer_id: header.transfer_id,
                data: UdpSessionData::default(),
            });
        }
        let active_session = match session {
            Session::Active(active) => active,
            Session::Complete { .. } => {
                unreachable!("Session must be active due to logic above")
            }
        };
        let result = active_session.handle_frame(header, bytes_after_header, self.payload_size_max);
        match result {
            Ok(Some(payload)) => {
                // Successfully received
                let first_frame_time = active_session.time;
                *session = Session::Complete {
                    time: active_session.time,
                    transfer_id: active_session.transfer_id,
                };
                Ok(Some(self.convert_reassembly_result(
                    payload,
                    header,
                    first_frame_time,
                )))
            }
            Ok(None) => Ok(None),
            Err(e) => {
                log::warn!("Buildup error {:?}, removing session", e);
                self.sessions.remove(source_node_id);
                Ok(None)
            }
        }
    }

    fn convert_reassembly_result(
        &self,
        reassembled: Vec<u8>,
        header: &UdpHeader,
        first_frame_time: Microseconds32,
    ) -> Transfer<Vec<u8>, UdpTransport> {
        // Add the transfer headers and record the completed transfer
        let header = match header.data_specifier {
            DataSpecifier::Subject { from, subject, .. } => Header::Message(MessageHeader {
                timestamp: first_frame_time,
                transfer_id: header.transfer_id,
                priority: header.priority,
                subject,
                source: from,
            }),
            DataSpecifier::ServiceRequest { service, from, to } => Header::Request(ServiceHeader {
                timestamp: first_frame_time,
                transfer_id: header.transfer_id,
                priority: header.priority,
                service,
                source: from,
                destination: to,
            }),
            DataSpecifier::ServiceResponse { service, from, to } => {
                Header::Response(ServiceHeader {
                    timestamp: first_frame_time,
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
}

#[derive(Default, Debug)]
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

impl UdpSession for ActiveSession<UdpTransferId, UdpSessionData> {
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

        if first_frame && last_frame {
            // Special case for a single-frame transfer
            let mut payload: Vec<u8> = Vec::new();
            payload.try_reserve_exact(max_payload_length)?;
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
            match self.data.buildup.take() {
                Some(mut buildup) => {
                    buildup.push(header, bytes_after_header)?;
                    if last_frame {
                        if buildup.crc_correct() {
                            Ok(Some(buildup.into_payload()))
                        } else {
                            Err(BuildupError::Crc)
                        }
                    } else {
                        self.data.buildup = Some(buildup);
                        Ok(None)
                    }
                }
                None => {
                    self.data.buildup = Some(Buildup::new(
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

#[cfg(test)]
mod tests {
    use super::UdpSession;
    use crate::UdpSessionData;
    use canadensis_core::session::ActiveSession;
    use canadensis_core::time::Microseconds32;
    use canadensis_core::Priority;
    use canadensis_header::{DataSpecifier, Header};
    use std::convert::TryInto;

    #[test]
    fn small_capacity_single_frame() {
        let mut session = ActiveSession {
            time: Microseconds32::from_ticks(1000),
            transfer_id: 19034.try_into().unwrap(),
            data: UdpSessionData::default(),
        };
        let header = Header {
            priority: Priority::Optional,
            data_specifier: DataSpecifier::Subject {
                from: Some(309.try_into().unwrap()),
                subject: 903.try_into().unwrap(),
            },
            transfer_id: 19034.try_into().unwrap(),
            frame_index: 0,
            last_frame: true,
            data: 0,
        };
        let received_payload = session
            .handle_frame(
                &header,
                &[0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0x8a, 0xe9, 0xde, 0xb3],
                3,
            )
            .expect("Reassembly error");
        assert_eq!(received_payload, Some(vec![0xa0, 0xa1, 0xa2]));
    }
}
