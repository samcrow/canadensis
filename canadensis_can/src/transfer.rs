//!
//! Transfer data definitions
//!

use alloc::vec::Vec;

use crate::{Microseconds, NodeId, PortId, Priority, ServiceId, SubjectId, TransferId};

/// Transfer kinds as defined by the UAVCAN Specification
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum TransferKind {
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
    pub(crate) fn kind(&self) -> TransferKind {
        match *self {
            TransferKindHeader::Message(_) => TransferKind::Message,
            TransferKindHeader::Request(_) => TransferKind::Request,
            TransferKindHeader::Response(_) => TransferKind::Response,
        }
    }

    /// Returns the port ID (subject or service ID) in this header
    pub(crate) fn port_id(&self) -> PortId {
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

/// A UAVCAN transfer (either incoming or outgoing)
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Transfer {
    /// For RX transfers: reception timestamp.
    /// For TX transfers: transmission deadline.
    /// The time system may be arbitrary as long as the clock is monotonic (steady).
    pub timestamp: Microseconds,

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

    /// The actual transfer payload.
    pub payload: Vec<u8>,
}
