//! IP addresses and ports for nodes

use crate::UdpNodeId;
use canadensis_core::{InvalidValue, ServiceId, SubjectId};
use core::convert::TryFrom;
use embedded_nal::Ipv4Addr;

const PREFIX_MAX: u16 = 0x1ff;
const SUBNET_MAX: u8 = 0x7f;
const PREFIX_SHIFT: u32 = 23;
const SUBNET_SHIFT: u32 = 16;

/// Fixed parts of a multicast group address, without the subnet and subject
const MULTICAST_BASE: u32 = 0b1110_1111_0000_0000_0000_0000_0000_0000;

/// An address of a node
///
/// This can be converted to and from an `Ipv4Addr`.
#[derive(Debug, Clone)]
pub struct NodeAddress {
    /// 9-bit address prefix
    prefix: u16,
    /// 7-bit subnet
    subnet: u8,
    /// 16-bit node ID
    node: UdpNodeId,
}

impl NodeAddress {
    /// Creates a node address
    ///
    /// This function returns an error if `prefix` is too large to fit into 9 bits, or `subnet`
    /// is too large to fit into 7 bits.
    pub fn new(prefix: u16, subnet: u8, node: UdpNodeId) -> Option<Self> {
        if prefix > PREFIX_MAX || subnet > SUBNET_MAX {
            None
        } else {
            Some(NodeAddress {
                prefix,
                subnet,
                node,
            })
        }
    }

    /// Creates an IPv4 address with the same prefix and subnet as this address, but a different
    /// node ID
    pub(crate) fn remote_node_address(&self, node: UdpNodeId) -> Ipv4Addr {
        NodeAddress {
            node,
            ..self.clone()
        }
        .into()
    }

    /// Creates a multicast group address used to send messages on the provided subject
    ///
    /// The returned address has the same subnet as this address.
    pub(crate) fn multicast_address(&self, subject: SubjectId) -> Ipv4Addr {
        let bits = MULTICAST_BASE | u32::from(self.subnet) << 16 | u32::from(subject);
        Ipv4Addr::from(bits)
    }
}

impl From<NodeAddress> for Ipv4Addr {
    fn from(address: NodeAddress) -> Self {
        let address_bits = u32::from(address.prefix) << PREFIX_SHIFT
            | u32::from(address.subnet) << SUBNET_SHIFT
            | u32::from(address.node);
        Ipv4Addr::from(address_bits)
    }
}

impl From<Ipv4Addr> for NodeAddress {
    fn from(address: Ipv4Addr) -> Self {
        let bits = u32::from(address);
        let prefix = ((bits >> PREFIX_SHIFT) & u32::from(PREFIX_MAX)) as u16;
        let subnet = ((bits >> SUBNET_SHIFT) & u32::from(SUBNET_MAX)) as u8;
        let node = UdpNodeId::from(bits as u16);
        NodeAddress {
            prefix,
            subnet,
            node,
        }
    }
}

/// The destination port number used for all message transfers
const SUBJECT_PORT: u16 = 16383;

/// The base port number used for service transfers
const SERVICE_BASE_PORT: u16 = 0x4000;
/// The bits in a service port number that must match `SERVICE_BASE_PORT`
const SERVICE_BASE_MASK: u16 = 0xfc00;

/// A port number associated with a service or message
///
/// Conversions are based on [the pyuavcan implementation](https://github.com/UAVCAN/pyuavcan/blob/87c27a978119d24ac77c9a7f2d6f289846ac96fd/pyuavcan/transport/udp/_ip/_endpoint_mapping.py#L172).
///
/// Messages are always sent on port 16383.
///
/// Service transfers use a specific port format:
/// ```text
/// 0100 00xx xxxx xxxR
/// ```
/// `xxxxxxxxx` is the service ID. R is et to 1 for a response or 0 for a request.
///
pub enum UdpPort {
    Message,
    Request(ServiceId),
    Response(ServiceId),
}

impl From<UdpPort> for u16 {
    /// Converts a port specifier into a port number
    fn from(port: UdpPort) -> Self {
        match port {
            UdpPort::Message => SUBJECT_PORT,
            UdpPort::Request(id) => SERVICE_BASE_PORT | (u16::from(id) << 1),
            UdpPort::Response(id) => SERVICE_BASE_PORT | (u16::from(id) << 1) + 1,
        }
    }
}

impl TryFrom<u16> for UdpPort {
    type Error = InvalidValue;

    /// Tries to parse a port from a port number
    fn try_from(port: u16) -> Result<Self, Self::Error> {
        if port == SUBJECT_PORT {
            Ok(UdpPort::Message)
        } else if (port & SERVICE_BASE_MASK) == SERVICE_BASE_PORT {
            let service = ServiceId::from_truncating(port >> 1);
            if port & 1 == 1 {
                Ok(UdpPort::Response(service))
            } else {
                Ok(UdpPort::Request(service))
            }
        } else {
            Err(InvalidValue)
        }
    }
}
