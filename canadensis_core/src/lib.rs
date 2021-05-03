#![cfg_attr(not(test), no_std)]

//!
//! This library provides types used by other canadensis crates.
//!

extern crate hash32;
extern crate hash32_derive;
extern crate num_traits;

pub mod time;
pub mod transfer;

use core::convert::TryFrom;
use core::fmt;
use core::ops::RangeInclusive;
use hash32_derive::Hash32;

/// An error indicating that an unacceptable integer was provided to a TryFrom implementation
#[derive(Debug)]
pub struct InvalidValue;

/// Allowed subject ID values
const VALID_SUBJECT_IDS: RangeInclusive<u16> = 0..=8191;

/// Subject ID, in range 0..=8191
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Hash32)]
pub struct SubjectId(u16);

impl SubjectId {
    /// Creates a SubjectId from a u16, truncating to the allowed number of bits
    pub const fn from_truncating(value: u16) -> Self {
        SubjectId(value & *VALID_SUBJECT_IDS.end())
    }
}

impl TryFrom<u16> for SubjectId {
    type Error = InvalidValue;

    fn try_from(value: u16) -> core::result::Result<Self, Self::Error> {
        if VALID_SUBJECT_IDS.contains(&value) {
            Ok(SubjectId(value))
        } else {
            Err(InvalidValue)
        }
    }
}

impl From<SubjectId> for u16 {
    #[inline]
    fn from(id: SubjectId) -> Self {
        id.0
    }
}
impl From<SubjectId> for u32 {
    #[inline]
    fn from(id: SubjectId) -> Self {
        id.0 as u32
    }
}
impl From<SubjectId> for usize {
    #[inline]
    fn from(id: SubjectId) -> Self {
        id.0 as usize
    }
}

const VALID_SERVICE_IDS: RangeInclusive<u16> = 0..=511;

/// Service ID, in range 0..=511
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Hash32)]
pub struct ServiceId(u16);

impl ServiceId {
    /// Creates a ServiceId from a u16, truncating to the allowed number of bits
    pub const fn from_truncating(value: u16) -> Self {
        ServiceId(value & *VALID_SERVICE_IDS.end())
    }
}

impl TryFrom<u16> for ServiceId {
    type Error = InvalidValue;

    fn try_from(value: u16) -> core::result::Result<Self, Self::Error> {
        if VALID_SERVICE_IDS.contains(&value) {
            Ok(ServiceId(value))
        } else {
            Err(InvalidValue)
        }
    }
}

impl From<ServiceId> for u16 {
    #[inline]
    fn from(id: ServiceId) -> Self {
        id.0
    }
}
impl From<ServiceId> for u32 {
    #[inline]
    fn from(id: ServiceId) -> Self {
        id.0 as u32
    }
}
impl From<ServiceId> for usize {
    #[inline]
    fn from(id: ServiceId) -> Self {
        id.0 as usize
    }
}

/// A value that can represent a service ID (0..=511) or a subject ID (0..=8192)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Hash32)]
pub struct PortId(u16);

impl From<SubjectId> for PortId {
    fn from(subject: SubjectId) -> Self {
        PortId(subject.0)
    }
}

impl TryFrom<PortId> for SubjectId {
    type Error = InvalidValue;
    fn try_from(port: PortId) -> Result<Self, Self::Error> {
        SubjectId::try_from(port.0)
    }
}

impl From<ServiceId> for PortId {
    fn from(service: ServiceId) -> Self {
        PortId(service.0)
    }
}

impl TryFrom<PortId> for ServiceId {
    type Error = InvalidValue;
    fn try_from(port: PortId) -> Result<Self, Self::Error> {
        ServiceId::try_from(port.0)
    }
}

const VALID_NODE_IDS: RangeInclusive<u8> = 0..=127;

/// Node ID
///
/// Valid node IDs are in the range 0..=127 (7 bits). IDs 126 and 127 are reserved for diagnostic
/// and debugging tools.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeId(u8);

impl NodeId {
    /// The smallest allowed node ID (0)
    pub const MIN: NodeId = NodeId(*VALID_NODE_IDS.start());
    /// The largest allowed node ID (127)
    pub const MAX: NodeId = NodeId(*VALID_NODE_IDS.end());

    /// Returns the integer value of this node ID
    pub const fn to_u8(self) -> u8 {
        self.0
    }

    /// Creates a valid NodeID from a u8, truncating values that are out of range
    pub const fn from_truncating(value: u8) -> Self {
        NodeId(value & *VALID_NODE_IDS.end())
    }

    /// Returns true if this node ID is one of the two highest values, which are reserved for
    /// diagnostic and debugging tools
    pub fn is_diagnostic_reserved(self) -> bool {
        self.0 >= *VALID_NODE_IDS.end() - 1
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl TryFrom<u8> for NodeId {
    type Error = InvalidValue;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        if VALID_NODE_IDS.contains(&bits) {
            Ok(NodeId(bits))
        } else {
            Err(InvalidValue)
        }
    }
}

impl From<NodeId> for u8 {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0
    }
}
impl From<NodeId> for u16 {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0 as u16
    }
}
impl From<NodeId> for u32 {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0 as u32
    }
}
impl From<NodeId> for usize {
    #[inline]
    fn from(id: NodeId) -> Self {
        id.0 as usize
    }
}

const VALID_TRANSFER_IDS: RangeInclusive<u8> = 0..=31;

/// Transfer ID, 5 bits, in range 0..=31
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TransferId(u8);

impl TransferId {
    /// Returns the default transfer ID (0). This is equivalent to Default::default(), but it can
    /// be called in a constant expression.
    pub const fn const_default() -> Self {
        TransferId(0)
    }

    /// Returns the next transfer ID after this, wrapping around after reaching the maximum
    /// allowed value
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn increment(self) -> TransferId {
        if self.0 == *VALID_TRANSFER_IDS.end() {
            // Wrap around to 0
            TransferId(0)
        } else {
            TransferId(self.0 + 1)
        }
    }
}

impl TryFrom<u8> for TransferId {
    type Error = InvalidValue;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        if VALID_TRANSFER_IDS.contains(&value) {
            Ok(TransferId(value))
        } else {
            Err(InvalidValue)
        }
    }
}

impl From<TransferId> for u8 {
    #[inline]
    fn from(id: TransferId) -> Self {
        id.0
    }
}
impl From<TransferId> for u16 {
    #[inline]
    fn from(id: TransferId) -> Self {
        id.0 as u16
    }
}
impl From<TransferId> for u32 {
    #[inline]
    fn from(id: TransferId) -> Self {
        id.0 as u32
    }
}
impl From<TransferId> for usize {
    #[inline]
    fn from(id: TransferId) -> Self {
        id.0 as usize
    }
}

impl Default for TransferId {
    /// Returns a transfer ID of 0
    fn default() -> Self {
        TransferId::const_default()
    }
}

/// Transfer priority level mnemonics per the recommendations given in the UAVCAN Specification
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Priority {
    Exceptional = 0,
    Immediate = 1,
    Fast = 2,
    High = 3,
    Nominal = 4,
    Low = 5,
    Slow = 6,
    Optional = 7,
}

impl Default for Priority {
    /// Returns Nominal priority
    fn default() -> Self {
        Priority::Nominal
    }
}

impl From<Priority> for u8 {
    fn from(priority: Priority) -> Self {
        priority as u8
    }
}

impl TryFrom<u8> for Priority {
    type Error = InvalidValue;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Priority::Exceptional),
            1 => Ok(Priority::Immediate),
            2 => Ok(Priority::Fast),
            3 => Ok(Priority::High),
            4 => Ok(Priority::Nominal),
            5 => Ok(Priority::Low),
            6 => Ok(Priority::Slow),
            7 => Ok(Priority::Optional),
            _ => Err(InvalidValue),
        }
    }
}
