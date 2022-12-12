//! IP addresses and ports for nodes
//!
//! All Cyphal addresses are multicast addresses in 239.0.0.0/10
//!
//! Notes:
//! * The Cyphal specification allows subject IDs in the range [0, 8191] (13 bits). The IP address
//!   format and Cyphal/UDP header have 15 bits for subject IDs, but this code rejects subject
//!   IDs greater than 8191.
//! * Similarly, service IDs are in the range [0, 512] (9 bits). The Cyphal/UDP header has 14 bits
//!   for service IDs, but this code rejects subject IDs greater than 511.
//!

use crate::UdpNodeId;
use canadensis_core::{InvalidValue, SubjectId};
use core::convert::TryFrom;
use std::net::Ipv4Addr;

/// Fixed parts of a Cyphal multicast group address, without the subnet and subject
const MULTICAST_BASE: u32 = 0b1110_1111_0000_0000_0000_0000_0000_0000;
/// The bits that must match `MULTICAST_BASE` for the address to be a valid Cyphal multicast address
const MULTICAST_MASK: u32 = 0b1111_1111_1111_1110_0000_0000_0000_0000;
/// The bit that marks a service address
const SERVICE_NOT_MESSAGE_BIT: u32 = 0x0001_0000;
/// Reserved bit in service addresses (should be zero)
const SUBJECT_RESERVED_BIT: u32 = 0x0000_8000;

impl From<Address> for Ipv4Addr {
    fn from(address: Address) -> Self {
        match address {
            Address::Node(node_id) => {
                let bits = MULTICAST_BASE | SERVICE_NOT_MESSAGE_BIT | u32::from(node_id);
                bits.into()
            }
            Address::Multicast(subject) => {
                let bits = MULTICAST_BASE | u32::from(subject);
                bits.into()
            }
        }
    }
}

impl TryFrom<Ipv4Addr> for Address {
    type Error = InvalidValue;

    /// Parses a Cyphal/UDP address from an IP address
    fn try_from(ip: Ipv4Addr) -> Result<Self, Self::Error> {
        let bits = u32::from(ip);
        if (bits & MULTICAST_MASK) != MULTICAST_BASE {
            // Not a valid multicast address
            return Err(InvalidValue);
        }
        if (bits & SERVICE_NOT_MESSAGE_BIT) == SERVICE_NOT_MESSAGE_BIT {
            // Service address
            let destination_node_id = UdpNodeId::try_from(bits as u16)?;
            Ok(Address::Node(destination_node_id))
        } else {
            // Message address
            // Bit 15 is reserved
            if (bits & SUBJECT_RESERVED_BIT) != 0 {
                return Err(InvalidValue);
            }
            let subject = SubjectId::try_from(bits as u16)?;
            Ok(Address::Multicast(subject))
        }
    }
}

/// An IP address used for Cyphal/UDP
pub enum Address {
    /// The address of a specific node, containing its destination node ID
    Node(UdpNodeId),
    /// A multicast address for a subject
    Multicast(SubjectId),
}
