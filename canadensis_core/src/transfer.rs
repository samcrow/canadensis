//!
//! Transfer data definitions
//!

use crate::{NodeId, PortId, Priority, ServiceId, SubjectId, TransferId};

/// The header of a message transfer
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MessageHeader<I> {
    /// For RX transfers: the time when the first frame was received
    /// For TX transfers: the transmission deadline for all frames
    pub timestamp: I,
    /// The identifier of this transfer
    pub transfer_id: TransferId,
    /// The priority of this transfer
    pub priority: Priority,
    // ============ Message-specific fields below
    /// The subject of this message
    pub subject: SubjectId,
    /// The source node, or None if this transfer is anonymous
    pub source: Option<NodeId>,
}

/// The header of a service transfer
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ServiceHeader<I> {
    /// For RX transfers: the time when the first frame was received
    /// For TX transfers: the transmission deadline for all frames
    pub timestamp: I,
    /// The identifier of this transfer
    pub transfer_id: TransferId,
    /// The priority of this transfer
    pub priority: Priority,
    // ============ Service-specific fields below
    /// The ID of this service
    pub service: ServiceId,
    /// The source node (the node sending this transfer)
    pub source: NodeId,
    /// The destination node (the node receiving this transfer)
    pub destination: NodeId,
}

/// Header fields for a message, request, or response
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Header<I> {
    Message(MessageHeader<I>),
    Request(ServiceHeader<I>),
    Response(ServiceHeader<I>),
}

impl<I> Header<I> {
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
    pub fn priority(&self) -> Priority {
        match self {
            Header::Message(ref message_header) => message_header.priority.clone(),
            Header::Request(ref service_header) | Header::Response(ref service_header) => {
                service_header.priority.clone()
            }
        }
    }
    /// Sets the priority of this header
    pub fn set_priority(&mut self, priority: Priority) {
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
    pub fn source(&self) -> Option<NodeId> {
        match self {
            Header::Message(ref message_header) => message_header.source.clone(),
            Header::Request(ref service_header) | Header::Response(ref service_header) => {
                Some(service_header.source.clone())
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
    pub fn transfer_id(&self) -> TransferId {
        match self {
            Header::Message(ref message_header) => message_header.transfer_id,
            Header::Request(ref service_header) | Header::Response(ref service_header) => {
                service_header.transfer_id
            }
        }
    }
}

/// A UAVCAN transfer (either incoming or outgoing)
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Transfer<P, I> {
    /// The transfer header
    pub header: Header<I>,
    /// The actual transfer payload
    ///
    /// The type P usually implements `AsRef<[u8]>`. It is often a `Vec<u8>` or a `&[u8]`.
    pub payload: P,
}

/// A type of transfer that is always a message transfer
pub struct MessageTransfer<P, I> {
    /// The transfer header
    pub header: MessageHeader<I>,
    /// The actual transfer payload
    ///
    /// The type P usually implements `AsRef<[u8]>`. It is often a `Vec<u8>` or a `&[u8]`.
    pub payload: P,
}

/// A type of transfer that is always a service request or response
pub struct ServiceTransfer<P, I> {
    /// The transfer header
    pub header: ServiceHeader<I>,
    /// The actual transfer payload
    ///
    /// The type P usually implements `AsRef<[u8]>`. It is often a `Vec<u8>` or a `&[u8]`.
    pub payload: P,
}
