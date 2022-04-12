//! IP addresses and ports for nodes
//!
//! Some important address ranges from <https://en.wikipedia.org/wiki/IPv4#Special-use_addresses>:
//! * `10.0.0.0/8`: Private addresses
//! * `172.16.0.0/12`: Private addresses
//! * `192.168.0.0/16`: Private addresses
//! * `224.0.0.0/4` (four most significant bits are `1110`): Multicast
//!   * Cyphal/UDP further specifies that the nine most significant bits should be `1110 1111 0`.
//! * `127.0.0.0/8`: Loopback
//!
//! This module assumes that any address with `1110 1111 0` in its nine most significant bits is a
//! multicast address associated with a subject, and any other address is the address of a single
//! node.
//!
//! Addresses with `1110` in their four most significant bits, but other values in the next five
//! bits, are multicast addresses but are not valid for Cyphal/UDP.
//!

use crate::UdpNodeId;
use canadensis_core::{InvalidValue, ServiceId, SubjectId};
use core::convert::TryFrom;
use std::net::Ipv4Addr;

const PREFIX_MAX: u16 = 0x1ff;
const SUBNET_MAX: u8 = 0x7f;
const PREFIX_SHIFT: u32 = 23;
const SUBNET_SHIFT: u32 = 16;

/// Fixed parts of a multicast address
const GENERIC_MULTICAST_BASE: u32 = 0b1110_0000_0000_0000_0000_0000_0000_0000;
/// The bits that must match `GENERIC_MULTICAST_BASE` for the address to be a multicast address
const GENERIC_MULTICAST_MASK: u32 = 0b1111_0000_0000_0000_0000_0000_0000_0000;

/// Fixed parts of a Cyphal multicast group address, without the subnet and subject
const MULTICAST_BASE: u32 = 0b1110_1111_0000_0000_0000_0000_0000_0000;
/// The bits that must match `MULTICAST_BASE` for the address to be a valid Cyphal multicast address
const MULTICAST_MASK: u32 = 0b1111_1111_1000_0000_1110_0000_0000_0000;

impl From<Address> for Ipv4Addr {
    fn from(address: Address) -> Self {
        match address {
            Address::Node(node_address) => node_address.into(),
            Address::Multicast(multicast_address) => multicast_address.into(),
        }
    }
}

impl TryFrom<Ipv4Addr> for Address {
    type Error = InvalidValue;

    /// Parses a Cyphal/UDP address from an IP address
    fn try_from(ip: Ipv4Addr) -> Result<Self, Self::Error> {
        let bits = u32::from(ip);
        if (bits & GENERIC_MULTICAST_MASK) == GENERIC_MULTICAST_BASE {
            if (bits & MULTICAST_MASK) == MULTICAST_BASE {
                let subnet = (bits >> SUBNET_SHIFT) as u8 & SUBNET_MAX;
                let subject = SubjectId::from_truncating(bits as u16);
                Ok(Address::Multicast(MulticastAddress { subnet, subject }))
            } else {
                // Multicast, but not valid for Cyphal
                Err(InvalidValue)
            }
        } else {
            // Specific node
            Ok(Address::Node(NodeAddress::try_from(ip)?))
        }
    }
}

/// An IP address used for Cyphal/UDP
pub enum Address {
    /// The address of a specific node
    Node(NodeAddress),
    /// A multicast address for a subject
    Multicast(MulticastAddress),
}

impl Address {
    /// Returns true if this address has the same subnet as another address
    pub fn same_subnet(&self, other: &Address) -> bool {
        self.subnet() == other.subnet()
    }
    /// Returns the subnet of this address
    pub fn subnet(&self) -> u8 {
        match self {
            Address::Node(inner) => inner.subnet,
            Address::Multicast(inner) => inner.subnet,
        }
    }
}

/// A multicast address for a subject
pub struct MulticastAddress {
    subnet: u8,
    subject: SubjectId,
}

impl MulticastAddress {
    pub fn subject(&self) -> SubjectId {
        self.subject
    }
}

impl From<MulticastAddress> for Ipv4Addr {
    fn from(address: MulticastAddress) -> Self {
        let bits = MULTICAST_BASE | (u32::from(address.subnet) << 16) | u32::from(address.subject);
        bits.into()
    }
}

/// An address of a node
///
/// This can be converted to and from an `Ipv4Addr`.
#[derive(Debug, Clone)]
pub struct NodeAddress {
    /// 9-bit address prefix
    ///
    /// Invariant: the four most significant bits are not `1110`
    prefix: u16,
    /// 7-bit subnet
    subnet: u8,
    /// 16-bit node ID
    node: UdpNodeId,
}

impl NodeAddress {
    /// Creates an IPv4 address with the same prefix and subnet as this address, but a different
    /// node ID
    pub(crate) fn remote_node_address(&self, node: UdpNodeId) -> Ipv4Addr {
        Address::Node(NodeAddress {
            node,
            ..self.clone()
        })
        .into()
    }

    /// Creates a multicast group address used to send messages on the provided subject
    ///
    /// The returned address has the same subnet as this address.
    pub(crate) fn multicast_address(&self, subject: SubjectId) -> Ipv4Addr {
        Address::Multicast(MulticastAddress {
            subnet: self.subnet,
            subject,
        })
        .into()
    }

    pub fn subnet(&self) -> u8 {
        self.subnet
    }

    pub fn node_id(&self) -> UdpNodeId {
        self.node
    }
}

impl From<NodeAddress> for Ipv4Addr {
    fn from(address: NodeAddress) -> Self {
        let bits = (u32::from(address.prefix) << 23)
            | (u32::from(address.subnet) << 16)
            | u32::from(address.node);
        bits.into()
    }
}

impl TryFrom<Ipv4Addr> for NodeAddress {
    type Error = InvalidValue;

    fn try_from(address: Ipv4Addr) -> Result<Self, Self::Error> {
        let bits = u32::from(address);
        if (bits & GENERIC_MULTICAST_MASK) != GENERIC_MULTICAST_BASE {
            let node_id_bits = bits as u16;
            let subnet = (bits >> SUBNET_SHIFT) as u8 & SUBNET_MAX;
            let prefix = (bits >> PREFIX_SHIFT) as u16 & PREFIX_MAX;
            Ok(NodeAddress {
                prefix,
                subnet,
                node: UdpNodeId::from(node_id_bits),
            })
        } else {
            Err(InvalidValue)
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
/// Conversions are based on [the pycyphal implementation](https://github.com/OpenCyphal/pycyphal/blob/87c27a978119d24ac77c9a7f2d6f289846ac96fd/pyuavcan/transport/udp/_ip/_endpoint_mapping.py#L172).
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
            UdpPort::Response(id) => SERVICE_BASE_PORT | ((u16::from(id) << 1) + 1),
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
