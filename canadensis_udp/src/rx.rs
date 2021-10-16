mod buildup;
mod subscriptions;

use alloc::vec::Vec;
use core::convert::TryFrom;
use core::marker::PhantomData;
use crc_any::CRCu32;
use fallible_collections::FallibleVec;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use zerocopy::FromBytes;

use canadensis_core::session::{Session, SessionTracker};
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, MessageHeader, ServiceHeader, Transfer};
use canadensis_core::transport::{Receiver, Transport};
use canadensis_core::{ServiceId, ServiceSubscribeError, SubjectId};

use crate::address::{Address, UdpPort};
use crate::header;
use crate::header::{UdpHeader, ValidatedUdpHeader};
use crate::rx::buildup::Buildup;
use crate::rx::subscriptions::Subscriptions;
use crate::{Error, NodeAddress, UdpNodeId, UdpTransferId, UdpTransport};

/// UDP transport receiver
pub struct UdpReceiver<I, T, const MTU: usize>
where
    I: Instant,
{
    subscriptions: Subscriptions<I, T>,
    address: NodeAddress,
    _session_tracker: PhantomData<T>,
}

impl<I, T, const MTU: usize> UdpReceiver<I, T, MTU>
where
    I: Instant,
    T: SessionTracker<I, UdpNodeId, UdpTransferId, UdpSessionData> + Default,
{
    pub fn new(address: NodeAddress) -> Self {
        UdpReceiver {
            subscriptions: Subscriptions::new(),
            address,
            _session_tracker: PhantomData,
        }
    }

    fn clean_expired_sessions(&mut self, now: I)
    where
        T: SessionTracker<I, UdpNodeId, UdpTransferId, UdpSessionData> + Default,
    {
        for subscription in self.subscriptions.message_iter_mut() {
            subscription.clean_expired_sessions(now.clone());
        }
        for subscription in self.subscriptions.request_iter_mut() {
            subscription.clean_expired_sessions(now.clone());
        }
        for subscription in self.subscriptions.response_iter_mut() {
            subscription.clean_expired_sessions(now.clone());
        }
    }

    fn accept_inner(
        &mut self,
        timestamp: I,
    ) -> Result<Option<Transfer<Vec<u8>, I, UdpTransport<I>>>, Error> {
        for subscription in self.subscriptions.message_iter_mut() {
            match subscription.check_for_frames::<MTU>(&self.address, timestamp) {
                Ok(Some(transfer)) => return Ok(Some(transfer)),
                Ok(None) => {}
                Err(Error::Socket(e)) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Nothing available to read right now, keep going and check everything else
                }
                Err(e) => return Err(e),
            }
        }
        for subscription in self.subscriptions.request_iter_mut() {
            match subscription.check_for_frames::<MTU>(&self.address, timestamp) {
                Ok(Some(transfer)) => return Ok(Some(transfer)),
                Ok(None) => {}
                Err(Error::Socket(e)) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Nothing available to read right now, keep going and check everything else
                }
                Err(e) => return Err(e),
            }
        }
        for subscription in self.subscriptions.response_iter_mut() {
            match subscription.check_for_frames::<MTU>(&self.address, timestamp) {
                Ok(Some(transfer)) => return Ok(Some(transfer)),
                Ok(None) => {}
                Err(Error::Socket(e)) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Nothing available to read right now, keep going and check everything else
                }
                Err(e) => return Err(e),
            }
        }
        Err(Error::Socket(std::io::Error::new(
            std::io::ErrorKind::WouldBlock,
            "No packets available",
        )))
    }
}

impl<I, T, const MTU: usize> Receiver<I> for UdpReceiver<I, T, MTU>
where
    I: Instant,
    T: SessionTracker<I, UdpNodeId, UdpTransferId, UdpSessionData> + Default,
{
    type Transport = UdpTransport<I>;

    /// Checks all subscriptions for incoming frames
    ///
    /// Return values:
    /// * `Ok(Some(transfer))` if a transfer was received
    /// * `Ok(None)` if at least one packet was read from a socket, but it did not complete a transfer
    /// * `Err(Error::Socket(e))` with `e.kind() == ErrorKind::WouldBlock` if no packet was available
    ///    to read
    /// * `Err(e)` if a socket or memory allocation error occurred
    fn accept(
        &mut self,
        timestamp: I,
    ) -> Result<Option<Transfer<Vec<u8>, I, UdpTransport<I>>>, Error> {
        let result = self.accept_inner(timestamp);
        self.clean_expired_sessions(timestamp);
        result
    }

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
    ) -> Result<(), Error> {
        self.subscriptions.subscribe_message(
            subject,
            Subscription::new_message(subject, payload_size_max, timeout, &self.address)?,
        )?;
        Ok(())
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
        let subscription =
            Subscription::new_request(service, payload_size_max, timeout, &self.address)
                .map_err(|io_error| ServiceSubscribeError::Transport(Error::Socket(io_error)))?;
        self.subscriptions
            .subscribe_request(service, subscription)
            .map_err(|oom| ServiceSubscribeError::Transport(Error::Memory(oom)))?;
        Ok(())
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
        let subscription =
            Subscription::new_response(service, payload_size_max, timeout, &self.address)
                .map_err(|io_error| ServiceSubscribeError::Transport(Error::Socket(io_error)))?;
        self.subscriptions
            .subscribe_response(service, subscription)
            .map_err(|oom| ServiceSubscribeError::Transport(Error::Memory(oom)))?;
        Ok(())
    }

    fn unsubscribe_response(&mut self, service: ServiceId) {
        self.subscriptions.unsubscribe_response(service);
    }
}

pub struct Subscription<I, T>
where
    I: Instant,
{
    /// A socket bound to the appropriate address and port
    socket: UdpSocket,
    kind: SubscriptionKind,
    payload_size_max: usize,
    timeout: <I as Instant>::Duration,
    sessions: T,
}

enum SubscriptionKind {
    Message(SubjectId),
    Request(ServiceId),
    Response(ServiceId),
}

impl<I, T> Subscription<I, T>
where
    I: Instant,
    T: SessionTracker<I, UdpNodeId, UdpTransferId, UdpSessionData> + Default,
{
    /// Creates a message subscription
    fn new_message(
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
        local_address: &NodeAddress,
    ) -> Result<Self, io::Error> {
        let multicast_address = local_address.multicast_address(subject);
        let socket = crate::bind_socket(multicast_address, UdpPort::Message.into())?;
        socket.join_multicast_v4(&multicast_address, &Ipv4Addr::from(local_address.clone()))?;

        Ok(Subscription {
            socket,
            kind: SubscriptionKind::Message(subject),
            payload_size_max,
            timeout,
            sessions: T::default(),
        })
    }
    fn new_request(
        service: ServiceId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
        local_address: &NodeAddress,
    ) -> Result<Self, io::Error> {
        let socket = crate::bind_socket(
            local_address.clone().into(),
            UdpPort::Request(service).into(),
        )?;

        Ok(Subscription {
            socket,
            kind: SubscriptionKind::Request(service),
            payload_size_max,
            timeout,
            sessions: T::default(),
        })
    }
    fn new_response(
        service: ServiceId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
        local_address: &NodeAddress,
    ) -> Result<Self, io::Error> {
        let socket = crate::bind_socket(
            local_address.clone().into(),
            UdpPort::Response(service).into(),
        )?;

        Ok(Subscription {
            socket,
            kind: SubscriptionKind::Response(service),
            payload_size_max,
            timeout,
            sessions: T::default(),
        })
    }

    /// Checks for an incoming packet and handles it, possibly returning a transfer
    ///
    /// This function returns a `WouldBlock` error if no packet is ready to read from the socket.
    /// It returns `Ok(None)` if a packet was read but it did not finish a transfer.
    fn check_for_frames<const MTU: usize>(
        &mut self,
        local_address: &NodeAddress,
        now: I,
    ) -> Result<Option<Transfer<Vec<u8>, I, UdpTransport<I>>>, Error>
    where
        I: Instant,
    {
        let mut buffer = [0u8; MTU];
        let (bytes, from) = self.socket.recv_from(&mut buffer)?;
        self.handle_frame(&buffer[..bytes], from, local_address, now)
    }

    fn handle_frame(
        &mut self,
        bytes: &[u8],
        from: SocketAddr,
        local_address: &NodeAddress,
        now: I,
    ) -> Result<Option<Transfer<Vec<u8>, I, UdpTransport<I>>>, Error> {
        // The from address should always be the address of an individual node
        let from_ip = match from.ip() {
            IpAddr::V4(v4) => v4,
            IpAddr::V6(_) => return Ok(None),
        };
        let from = match Address::try_from(from_ip) {
            Ok(Address::Node(node_address)) => node_address,
            Ok(Address::Multicast(_)) | Err(_) => return Ok(None),
        };
        if from.subnet() != local_address.subnet() {
            // Different subnet
            return Ok(None);
        }
        // The from port doesn't matter here. The socket is bound to only one port, so any incoming
        // frames must be on that port.

        // Extract header from frame bytes
        let header: ValidatedUdpHeader = match UdpHeader::read_from_prefix(bytes)
            .and_then(|header| ValidatedUdpHeader::try_from(header).ok())
        {
            Some(header) => header,
            None => {
                // Frame not large enough for header, or invalid format
                return Ok(None);
            }
        };
        self.handle_sane_frame(
            &header,
            &bytes[header::SIZE..],
            from.node_id(),
            local_address,
            now,
        )
    }

    fn handle_sane_frame(
        &mut self,
        header: &ValidatedUdpHeader,
        bytes_after_header: &[u8],
        from: UdpNodeId,
        local_address: &NodeAddress,
        now: I,
    ) -> Result<Option<Transfer<Vec<u8>, I, UdpTransport<I>>>, Error> {
        log::debug!("handle_sane_frame header {:?} from node {:?}", header, from);
        let timeout = self.timeout.clone();
        let session = self.sessions.get_mut_or_insert_with(from.clone(), || {
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
            Ok(Some(reassembled)) => {
                // Add the transfer headers and record the completed transfer
                session.set_last_transfer_id(header.transfer_id.into());

                let header = match &self.kind {
                    SubscriptionKind::Message(subject) => Header::Message(MessageHeader {
                        timestamp: now,
                        transfer_id: header.transfer_id,
                        priority: header.priority,
                        subject: *subject,
                        source: Some(from),
                    }),
                    SubscriptionKind::Request(service) => Header::Request(ServiceHeader {
                        timestamp: now,
                        transfer_id: header.transfer_id,
                        priority: header.priority,
                        service: *service,
                        source: from,
                        destination: local_address.node_id(),
                    }),
                    SubscriptionKind::Response(service) => Header::Response(ServiceHeader {
                        timestamp: now,
                        transfer_id: header.transfer_id,
                        priority: header.priority,
                        service: *service,
                        source: from,
                        destination: local_address.node_id(),
                    }),
                };
                Ok(Some(Transfer {
                    header,
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
        header: &ValidatedUdpHeader,
        bytes_after_header: &[u8],
        max_payload_length: usize,
    ) -> Result<Option<Vec<u8>>, Error>;
}

impl<I> UdpSession<I> for Session<I, UdpTransferId, UdpSessionData>
where
    I: Instant,
{
    fn handle_frame(
        &mut self,
        header: &ValidatedUdpHeader,
        bytes_after_header: &[u8],
        max_payload_length: usize,
    ) -> Result<Option<Vec<u8>>, Error> {
        log::debug!(
            "UdpSession::handle_frame, buildup = {:?}",
            &self.data().buildup
        );
        if header.frame_index == 0 {
            if header.last_frame {
                // Special case for a single-frame transfer
                if bytes_after_header.len() <= max_payload_length {
                    let mut payload: Vec<u8> =
                        FallibleVec::try_with_capacity(bytes_after_header.len())?;
                    payload.extend_from_slice(bytes_after_header);
                    Ok(Some(payload))
                } else {
                    // Frame too large
                    Ok(None)
                }
            } else {
                // Start a buildup
                log::debug!("Creating buildup for first frame");
                match Buildup::new(header, bytes_after_header, max_payload_length) {
                    Ok(buildup) => self.data_mut().buildup = Some(buildup),
                    Err(e) => {
                        // Couldn't create buildup due to a problem with the frame
                        log::warn!("Can't create buildup from first frame: {:?}", e);
                        return Ok(None);
                    }
                }
                Ok(None)
            }
        } else {
            if header.last_frame {
                match self.data_mut().buildup.take() {
                    Some(mut buildup) => {
                        match buildup.push(header, bytes_after_header) {
                            Ok(()) => {
                                // Completed reassembly
                                let mut payload_and_crc = buildup.into_payload();

                                let expected_crc =
                                    match payload_and_crc.get(payload_and_crc.len() - 4..) {
                                        Some(crc_slice) => {
                                            let mut crc_bytes = [0u8; 4];
                                            crc_bytes.copy_from_slice(crc_slice);
                                            u32::from_le_bytes(crc_bytes)
                                        }
                                        None => {
                                            // Not enough payload to include a CRC
                                            log::warn!("Payload too short to contain CRC");
                                            return Ok(None);
                                        }
                                    };
                                payload_and_crc.truncate(payload_and_crc.len() - 4);
                                let payload = payload_and_crc;

                                let calculated_crc = {
                                    let mut crc = CRCu32::crc32c();
                                    crc.digest(&payload);
                                    crc.get_crc()
                                };

                                // Check crc
                                if calculated_crc == expected_crc {
                                    Ok(Some(payload))
                                } else {
                                    // Incorrect CRC
                                    log::warn!(
                                        "Incorrect CRC: calculated {:#08x}, got {:#08x}",
                                        calculated_crc,
                                        expected_crc
                                    );
                                    Ok(None)
                                }
                            }
                            Err(e) => {
                                // Reassembly error. Give up on the reassembly.
                                log::warn!("Reassembly error on last frame: {:?}", e);
                                Ok(None)
                            }
                        }
                    }
                    None => {
                        // Should have a buildup from the first frame, but none exists
                        log::warn!("Last frame, buildup does not exist");
                        Ok(None)
                    }
                }
            } else {
                // A frame in the middle
                match self.data_mut().buildup.as_mut() {
                    Some(buildup) => {
                        match buildup.push(header, bytes_after_header) {
                            Ok(()) => Ok(None),
                            Err(e) => {
                                // Reassembly error. Give up on the reassembly.
                                log::warn!("Reassembly error on middle frame: {:?}", e);
                                self.data_mut().buildup = None;
                                Ok(None)
                            }
                        }
                    }
                    None => {
                        // Missed the first frame, can't use this transfer
                        log::warn!("Middle frame, buildup does not exist");
                        Ok(None)
                    }
                }
            }
        }
    }
}
