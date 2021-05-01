//!
//! Transfer data definitions
//!

use crate::{NodeId, PortId, Priority, ServiceId, SubjectId, TransferId};
use core::convert::TryFrom;

/// Transfer kinds as defined by the UAVCAN Specification
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TransferKind {
    /// Multicast, from publisher to all subscribers
    Message,
    /// Point-to-point, from server to client
    Response,
    /// Point-to-point, from client to server
    Request,
}

/// Fields specific to a message transfer
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MessageHeader {
    /// True if the sender is anonymous
    pub anonymous: bool,
    /// The subject of this message
    pub subject: SubjectId,
}

/// Fields specific to a service transfer
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ServiceHeader {
    /// The ID of this service
    pub service: ServiceId,
    /// The destination node
    pub destination: NodeId,
}

/// Header fields for a message, request, or response
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TransferKindHeader {
    Message(MessageHeader),
    Request(ServiceHeader),
    Response(ServiceHeader),
}

impl TransferKindHeader {
    /// Returns the corresponding transfer kind for this header
    pub fn kind(&self) -> TransferKind {
        match *self {
            TransferKindHeader::Message(_) => TransferKind::Message,
            TransferKindHeader::Request(_) => TransferKind::Request,
            TransferKindHeader::Response(_) => TransferKind::Response,
        }
    }

    /// Returns the port ID (subject or service ID) in this header
    pub fn port_id(&self) -> PortId {
        match *self {
            TransferKindHeader::Message(ref header) => PortId::from(header.subject),
            TransferKindHeader::Request(ref header) => PortId::from(header.service),
            TransferKindHeader::Response(ref header) => PortId::from(header.service),
        }
    }

    /// If this is a service request or response, returns the service header
    pub fn service_header(&self) -> Option<&ServiceHeader> {
        match *self {
            TransferKindHeader::Message(_) => None,
            TransferKindHeader::Request(ref service) => Some(service),
            TransferKindHeader::Response(ref service) => Some(service),
        }
    }
}
/// The complete header for a transfer
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TransferHeader {
    /// The node ID of the source of this transfer
    pub source: NodeId,
    /// The priority of this transfer
    pub priority: Priority,
    /// The type of this transfer, and the associated data
    pub kind: TransferKindHeader,
}

impl TransferHeader {
    /// Returns true if this header is for an anonymous message transfer
    ///
    /// This function returns false if the transfer is not a message transfer.
    pub fn is_anonymous(&self) -> bool {
        matches!(
            &self.kind,
            TransferKindHeader::Message(MessageHeader {
                anonymous: true,
                ..
            })
        )
    }
}

/// A UAVCAN transfer (either incoming or outgoing)
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Transfer<P, I> {
    /// For RX transfers: the time when the first frame was received
    /// For TX transfers: the transmission deadline for all frames
    pub timestamp: I,

    /// The transfer header
    ///
    /// Per the Specification, all frames belonging to a given transfer shall share the same priority level.
    /// If this is not the case, then this field contains the priority level of the last frame to arrive.
    pub header: TransferHeader,

    /// When responding to a service request, the response transfer SHALL have the same transfer-ID value as the
    /// request because the client will match the response with the request based on that.
    ///
    /// When publishing a message transfer, the value SHALL be one greater than the previous transfer under the same
    /// subject-ID; the initial value should be zero.
    ///
    /// When publishing a service request transfer, the value SHALL be one greater than the previous transfer under
    /// the same service-ID addressed to the same server node-ID; the initial value should be zero.
    ///
    /// Upon overflow, the value SHALL be reset back to zero.
    ///
    /// A simple and robust way of managing transfer-ID counters is to keep a separate static variable per subject-ID
    /// and per (service-ID, server-node-ID) pair.
    pub transfer_id: TransferId,

    /// The actual transfer payload
    ///
    /// The type P usually implements `AsRef<[u8]>`. It is often a `Vec<u8>` or a `&[u8]`.
    pub payload: P,
}

impl<P, I> Transfer<P, I> {
    /// Returns true if this transfer is a request matching the provided service ID
    pub fn is_request_for(&self, service_id: ServiceId) -> bool {
        match &self.header.kind {
            TransferKindHeader::Request(service_header) if service_header.service == service_id => {
                true
            }
            _ => false,
        }
    }
    /// Returns true if this transfer is a response matching the provided service ID
    pub fn is_response_for(&self, service_id: ServiceId) -> bool {
        match &self.header.kind {
            TransferKindHeader::Response(service_header)
                if service_header.service == service_id =>
            {
                true
            }
            _ => false,
        }
    }
    /// Returns true if this transfer is a message matching the provided service ID
    pub fn is_message_for(&self, subject_id: SubjectId) -> bool {
        match &self.header.kind {
            TransferKindHeader::Message(message_header) if message_header.subject == subject_id => {
                true
            }
            _ => false,
        }
    }
}

/// A type of transfer header with only the fields available in a service request or response
pub struct ServiceOnlyHeader {
    /// The node ID of the source of this transfer
    pub source: NodeId,
    /// The priority of this transfer
    pub priority: Priority,
    /// The service ID and destination node
    pub service: ServiceHeader,
}

impl TryFrom<&'_ TransferHeader> for ServiceOnlyHeader {
    type Error = SubtypeError;

    fn try_from(header: &TransferHeader) -> Result<Self, Self::Error> {
        match &header.kind {
            TransferKindHeader::Message(_) => Err(SubtypeError(())),
            TransferKindHeader::Request(service_header) => Ok(ServiceOnlyHeader {
                source: header.source,
                priority: header.priority,
                service: service_header.clone(),
            }),
            TransferKindHeader::Response(_) => Err(SubtypeError(())),
        }
    }
}

/// A type of transfer header with only the fields available in a message transfer
pub struct MessageOnlyHeader {
    /// The node ID of the source of this transfer
    pub source: NodeId,
    /// The priority of this transfer
    pub priority: Priority,
    /// The anonymous flag and subject
    pub message: MessageHeader,
}

impl TryFrom<&'_ TransferHeader> for MessageOnlyHeader {
    type Error = SubtypeError;

    fn try_from(header: &TransferHeader) -> Result<Self, Self::Error> {
        match &header.kind {
            TransferKindHeader::Message(message_header) => Ok(MessageOnlyHeader {
                source: header.source,
                priority: header.priority,
                message: message_header.clone(),
            }),
            TransferKindHeader::Request(_) => Err(SubtypeError(())),
            TransferKindHeader::Response(_) => Err(SubtypeError(())),
        }
    }
}

/// A type of transfer that is always a message transfer
pub struct MessageTransfer<P, I> {
    /// The time when the first frame was received
    pub timestamp: I,

    /// The transfer header
    ///
    /// Per the Specification, all frames belonging to a given transfer shall share the same priority level.
    /// If this is not the case, then this field contains the priority level of the last frame to arrive.
    pub header: MessageOnlyHeader,

    /// The ID of this transfer
    pub transfer_id: TransferId,

    /// The actual transfer payload
    ///
    /// The type P usually implements `AsRef<[u8]>`. It is often a `Vec<u8>` or a `&[u8]`.
    pub payload: P,
}

/// A type of transfer that is always a service request or response
pub struct ServiceTransfer<P, I> {
    /// The time when the first frame was received
    pub timestamp: I,

    /// The transfer header
    ///
    /// Per the Specification, all frames belonging to a given transfer shall share the same priority level.
    /// If this is not the case, then this field contains the priority level of the last frame to arrive.
    pub header: ServiceOnlyHeader,

    /// The ID of this transfer
    pub transfer_id: TransferId,

    /// The actual transfer payload
    ///
    /// The type P usually implements `AsRef<[u8]>`. It is often a `Vec<u8>` or a `&[u8]`.
    pub payload: P,
}

#[derive(Debug)]
pub struct SubtypeError(());
