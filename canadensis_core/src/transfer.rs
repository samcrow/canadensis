//!
//! Transfer data definitions
//!

use crate::transport::Transport;
use crate::{PortId, ServiceId, SubjectId};
use core::fmt::{Debug, Formatter};

/// The header of a message transfer
pub struct MessageHeader<I, T: Transport + ?Sized> {
    /// For RX transfers: the time when the first frame was received
    /// For TX transfers: the transmission deadline for all frames
    pub timestamp: I,
    /// The identifier of this transfer
    pub transfer_id: T::TransferId,
    /// The priority of this transfer
    pub priority: T::Priority,
    // ============ Message-specific fields below
    /// The subject of this message
    pub subject: SubjectId,
    /// The source node, or None if this transfer is anonymous
    pub source: Option<T::NodeId>,
}

impl<I, T: Transport + ?Sized> Debug for MessageHeader<I, T>
where
    I: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MessageHeader")
            .field("timestamp", &self.timestamp)
            .field("transfer_id", &self.transfer_id)
            .field("priority", &self.priority)
            .field("subject", &self.subject)
            .field("source", &self.source)
            .finish()
    }
}

impl<I, T: Transport + ?Sized> Clone for MessageHeader<I, T>
where
    I: Clone,
    T::TransferId: Clone,
    T::Priority: Clone,
    T::NodeId: Clone,
{
    fn clone(&self) -> Self {
        MessageHeader {
            timestamp: self.timestamp.clone(),
            transfer_id: self.transfer_id.clone(),
            priority: self.priority.clone(),
            subject: self.subject.clone(),
            source: self.source.clone(),
        }
    }
}

impl<I, T: Transport + ?Sized> PartialEq for MessageHeader<I, T>
where
    I: PartialEq,
    T::TransferId: PartialEq,
    T::Priority: PartialEq,
    T::NodeId: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.transfer_id == other.transfer_id
            && self.priority == other.priority
            && self.subject == other.subject
            && self.source == other.source
    }
}

/// The header of a service transfer
pub struct ServiceHeader<I, T: Transport + ?Sized> {
    /// For RX transfers: the time when the first frame was received
    /// For TX transfers: the transmission deadline for all frames
    pub timestamp: I,
    /// The identifier of this transfer
    pub transfer_id: T::TransferId,
    /// The priority of this transfer
    pub priority: T::Priority,
    // ============ Service-specific fields below
    /// The ID of this service
    pub service: ServiceId,
    /// The source node (the node sending this transfer)
    pub source: T::NodeId,
    /// The destination node (the node receiving this transfer)
    pub destination: T::NodeId,
}

impl<I, T: Transport + ?Sized> Debug for ServiceHeader<I, T>
where
    I: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MessageHeader")
            .field("timestamp", &self.timestamp)
            .field("transfer_id", &self.transfer_id)
            .field("priority", &self.priority)
            .field("service", &self.service)
            .field("source", &self.source)
            .field("destination", &self.destination)
            .finish()
    }
}

impl<I, T: Transport + ?Sized> Clone for ServiceHeader<I, T>
where
    I: Clone,
    T::TransferId: Clone,
    T::Priority: Clone,
    T::NodeId: Clone,
{
    fn clone(&self) -> Self {
        ServiceHeader {
            timestamp: self.timestamp.clone(),
            transfer_id: self.transfer_id.clone(),
            priority: self.priority.clone(),
            service: self.service.clone(),
            source: self.source.clone(),
            destination: self.destination.clone(),
        }
    }
}

impl<I, T: Transport + ?Sized> PartialEq for ServiceHeader<I, T>
where
    I: PartialEq,
    T::TransferId: PartialEq,
    T::Priority: PartialEq,
    T::NodeId: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.transfer_id == other.transfer_id
            && self.priority == other.priority
            && self.service == other.service
            && self.source == other.source
            && self.destination == other.destination
    }
}

/// Header fields for a message, request, or response
pub enum Header<I, T: Transport + ?Sized> {
    /// A message header
    Message(MessageHeader<I, T>),
    /// A service request header
    Request(ServiceHeader<I, T>),
    /// A service response header
    Response(ServiceHeader<I, T>),
}
impl<I, T: Transport + ?Sized> Debug for Header<I, T>
where
    I: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Header::Message(inner) => f.debug_tuple("Message").field(inner).finish(),
            Header::Request(inner) => f.debug_tuple("Request").field(inner).finish(),
            Header::Response(inner) => f.debug_tuple("Response").field(inner).finish(),
        }
    }
}
impl<I, T: Transport + ?Sized> Clone for Header<I, T>
where
    I: Clone,
    T::TransferId: Clone,
    T::Priority: Clone,
    T::NodeId: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Header::Message(inner) => Header::Message(inner.clone()),
            Header::Request(inner) => Header::Request(inner.clone()),
            Header::Response(inner) => Header::Response(inner.clone()),
        }
    }
}

impl<I, T: Transport + ?Sized> PartialEq for Header<I, T>
where
    I: PartialEq,
    T::TransferId: PartialEq,
    T::Priority: PartialEq,
    T::NodeId: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Header::Message(lhs), Header::Message(rhs)) => lhs == rhs,
            (Header::Request(lhs), Header::Request(rhs)) => lhs == rhs,
            (Header::Response(lhs), Header::Response(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl<I, T: Transport + ?Sized> Header<I, T> {
    /// Returns the timestamp of this header
    pub fn timestamp(&self) -> I
    where
        I: Clone,
    {
        match self {
            Header::Message(ref message_header) => message_header.timestamp.clone(),
            Header::Request(ref service_header) | Header::Response(ref service_header) => {
                service_header.timestamp.clone()
            }
        }
    }
    /// Sets the timestamp of this header
    pub fn set_timestamp(&mut self, timestamp: I) {
        match self {
            Header::Message(ref mut message_header) => {
                message_header.timestamp = timestamp;
            }
            Header::Request(ref mut transfer_header)
            | Header::Response(ref mut transfer_header) => {
                transfer_header.timestamp = timestamp;
            }
        }
    }
    /// Returns the priority of this header
    pub fn priority(&self) -> &T::Priority {
        match self {
            Header::Message(ref message_header) => &message_header.priority,
            Header::Request(ref service_header) | Header::Response(ref service_header) => {
                &service_header.priority
            }
        }
    }
    /// Sets the priority of this header
    pub fn set_priority(&mut self, priority: T::Priority) {
        match self {
            Header::Message(ref mut message_header) => {
                message_header.priority = priority;
            }
            Header::Request(ref mut transfer_header)
            | Header::Response(ref mut transfer_header) => {
                transfer_header.priority = priority;
            }
        }
    }

    /// Returns the source node ID of this transfer, or None if this is an anonymous message
    pub fn source(&self) -> Option<&T::NodeId> {
        match self {
            Header::Message(ref message_header) => message_header.source.as_ref(),
            Header::Request(ref service_header) | Header::Response(ref service_header) => {
                Some(&service_header.source)
            }
        }
    }

    /// Returns the port ID (either a subject ID or service ID) of this transfer
    pub fn port_id(&self) -> PortId {
        match self {
            Header::Message(ref message_header) => message_header.subject.into(),
            Header::Request(ref service_header) | Header::Response(ref service_header) => {
                service_header.service.into()
            }
        }
    }
    /// Returns the transfer ID of this transfer
    pub fn transfer_id(&self) -> &T::TransferId {
        match self {
            Header::Message(ref message_header) => &message_header.transfer_id,
            Header::Request(ref service_header) | Header::Response(ref service_header) => {
                &service_header.transfer_id
            }
        }
    }
}

/// A UAVCAN transfer (either incoming or outgoing)
pub struct Transfer<A, I, T: Transport + ?Sized> {
    /// The transfer header
    pub header: Header<I, T>,
    /// The actual transfer payload
    ///
    /// The type A usually implements `AsRef<[u8]>`. It is often a `Vec<u8>` or a `&[u8]`.
    pub payload: A,
}

impl<A, I, T: Transport + ?Sized> Debug for Transfer<A, I, T>
where
    A: Debug,
    I: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Transfer")
            .field("header", &self.header)
            .field("payload", &self.payload)
            .finish()
    }
}

impl<A, I, T: Transport + ?Sized> PartialEq for Transfer<A, I, T>
where
    A: PartialEq,
    I: PartialEq,
    T::TransferId: PartialEq,
    T::Priority: PartialEq,
    T::NodeId: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.payload == other.payload
    }
}
impl<A, I, T: Transport + ?Sized> Clone for Transfer<A, I, T>
where
    A: Clone,
    I: Clone,
    T::TransferId: Clone,
    T::Priority: Clone,
    T::NodeId: Clone,
{
    fn clone(&self) -> Self {
        Transfer {
            header: self.header.clone(),
            payload: self.payload.clone(),
        }
    }
}

/// A type of transfer that is always a message transfer
#[derive(Clone)]
pub struct MessageTransfer<A, I, T: Transport + ?Sized> {
    /// The transfer header
    pub header: MessageHeader<I, T>,
    /// The actual transfer payload
    ///
    /// The type A usually implements `AsRef<[u8]>`. It is often a `Vec<u8>` or a `&[u8]`.
    pub payload: A,
}

impl<A, I, T: Transport + ?Sized> Debug for MessageTransfer<A, I, T>
where
    A: Debug,
    I: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Transfer")
            .field("header", &self.header)
            .field("payload", &self.payload)
            .finish()
    }
}

impl<A, I, T: Transport + ?Sized> PartialEq for MessageTransfer<A, I, T>
where
    A: PartialEq,
    I: PartialEq,
    T::TransferId: PartialEq,
    T::Priority: PartialEq,
    T::NodeId: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.payload == other.payload
    }
}

/// A type of transfer that is always a service request or response
#[derive(Clone)]
pub struct ServiceTransfer<A, I, T: Transport + ?Sized> {
    /// The transfer header
    pub header: ServiceHeader<I, T>,
    /// The actual transfer payload
    ///
    /// The type A usually implements `AsRef<[u8]>`. It is often a `Vec<u8>` or a `&[u8]`.
    pub payload: A,
}

impl<A, I, T: Transport + ?Sized> Debug for ServiceTransfer<A, I, T>
where
    A: Debug,
    I: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Transfer")
            .field("header", &self.header)
            .field("payload", &self.payload)
            .finish()
    }
}

impl<A, I, T: Transport + ?Sized> PartialEq for ServiceTransfer<A, I, T>
where
    A: PartialEq,
    I: PartialEq,
    T::TransferId: PartialEq,
    T::Priority: PartialEq,
    T::NodeId: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.payload == other.payload
    }
}
