//!
//! Common UAVCAN data types
//!

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt;
use core::ops::RangeInclusive;
use core::ops::{Add, Sub};

/// An error indicating that an unacceptable integer was provided to a TryFrom implementation
#[derive(Debug)]
pub struct InvalidValue(());

const VALID_SUBJECT_IDS: RangeInclusive<u16> = 0..=32767;

/// Subject ID, in range 0..=32767
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SubjectId(u16);

impl TryFrom<u16> for SubjectId {
    type Error = InvalidValue;

    fn try_from(value: u16) -> core::result::Result<Self, Self::Error> {
        if VALID_SUBJECT_IDS.contains(&value) {
            Ok(SubjectId(value))
        } else {
            Err(InvalidValue(()))
        }
    }
}

impl From<SubjectId> for u16 {
    fn from(id: SubjectId) -> Self {
        id.0
    }
}

const VALID_SERVICE_IDS: RangeInclusive<u16> = 0..=511;

/// Service ID, in range 0..=511
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ServiceId(u16);

impl TryFrom<u16> for ServiceId {
    type Error = InvalidValue;

    fn try_from(value: u16) -> core::result::Result<Self, Self::Error> {
        if VALID_SERVICE_IDS.contains(&value) {
            Ok(ServiceId(value))
        } else {
            Err(InvalidValue(()))
        }
    }
}

impl From<ServiceId> for u16 {
    fn from(id: ServiceId) -> Self {
        id.0
    }
}

/// A value that can represent a service ID (0..=511) or a subject ID (0..=32767)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct PortId(u16);

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

pub(crate) const NORMAL_NODE_IDS: RangeInclusive<u8> = 0..=127;

/// Node ID
///
/// Normal node IDs are in the range 0..=127.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeId(u8);

impl From<u8> for NodeId {
    fn from(bits: u8) -> Self {
        NodeId(bits)
    }
}

impl From<NodeId> for u8 {
    fn from(id: NodeId) -> Self {
        id.0
    }
}

const VALID_TRANSFER_IDS: RangeInclusive<u8> = 0..=31;

/// Transfer ID, 5 bits, in range 0..=31
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TransferId(u8);

impl TryFrom<u8> for TransferId {
    type Error = InvalidValue;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        if VALID_TRANSFER_IDS.contains(&value) {
            Ok(TransferId(value))
        } else {
            Err(InvalidValue(()))
        }
    }
}

impl From<TransferId> for u8 {
    fn from(id: TransferId) -> Self {
        id.0
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
            _ => Err(InvalidValue(())),
        }
    }
}

/// Bit mask for a 29-bit CAN ID
const CAN_ID_MASK: u32 = 0x1f_ff_ff_ff;

/// A 29-bit extended CAN ID
#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct CanId(u32);

impl fmt::Debug for CanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CanId({:#010x})", self.0)
    }
}

impl TryFrom<u32> for CanId {
    type Error = InvalidValue;

    fn try_from(value: u32) -> core::result::Result<Self, Self::Error> {
        if (value & !CAN_ID_MASK) == 0 {
            // No bits set outside the mask, OK
            Ok(CanId(value))
        } else {
            Err(InvalidValue(()))
        }
    }
}

impl From<CanId> for u32 {
    fn from(id: CanId) -> Self {
        id.0
    }
}

/// A time in microseconds, from any monotonic clock
///
/// Microsecond values can wrap around. The Microsecond comparison operators will correctly handle
/// a single wraparound.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Microseconds(pub u64);

impl Add for Microseconds {
    type Output = Self;
    /// Adds, wrapping around
    fn add(self, rhs: Self) -> Self::Output {
        Microseconds(self.0.wrapping_add(rhs.0))
    }
}

impl Sub for Microseconds {
    type Output = Self;
    /// Subtracts, wrapping around
    fn sub(self, rhs: Self) -> Self::Output {
        Microseconds(self.0.wrapping_sub(rhs.0))
    }
}

impl PartialOrd for Microseconds {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Microseconds {
    fn cmp(&self, other: &Self) -> Ordering {
        // https://www.rapitasystems.com/blog/what-happened-first-handling-timer-wraparound
        if self.0 == other.0 {
            Ordering::Equal
        } else if (other.0.wrapping_sub(self.0)) < u64::MAX / 2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl fmt::Display for Microseconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} us", self.0)
    }
}

/// Allowed maximum transmission unit (MTU) values
#[derive(Debug)]
pub enum Mtu {
    /// 8 bytes, for standard CAN
    Can8 = 8,
    /// 64 bytes, for CAN FD
    CanFd64 = 64,
}

/// CAN data frame with an extended 29-bit ID
///
/// RTR/Error frames are not used and therefore not modeled here.
/// CAN frames with 11-bit ID are not used by UAVCAN/CAN and so they are not supported by the library.
#[derive(Debug, Eq, PartialEq)]
pub struct Frame {
    /// For RX frames: reception timestamp.
    /// For TX frames: transmission deadline.
    /// The time system may be arbitrary as long as the clock is monotonic (steady).
    pub timestamp: Microseconds,
    /// 29-bit extended ID
    pub can_id: CanId,
    /// The useful data in the frame
    pub payload: Vec<u8>,
}

impl PartialOrd for Frame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Frame {
    /// Compare by CAN ID
    fn cmp(&self, other: &Self) -> Ordering {
        self.can_id.cmp(&other.can_id)
    }
}
