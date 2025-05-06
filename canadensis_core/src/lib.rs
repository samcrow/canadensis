#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

//!
//! This library provides types used by other canadensis crates.
//!

extern crate alloc;
extern crate fallible_collections;
extern crate fugit;
extern crate heapless;
pub extern crate nb;

mod error;
pub mod session;
pub mod subscription;
pub mod time;
pub mod transfer;
pub mod transport;

pub use crate::error::{OutOfMemoryError, ServiceSubscribeError};
use core::convert::TryFrom;
use core::ops::RangeInclusive;
use core::str::FromStr;
use defmt::Format;

/// An error indicating that an unacceptable integer was provided to a TryFrom implementation
#[derive(Debug, Format)]
pub struct InvalidValue;

/// Allowed subject ID values
const VALID_SUBJECT_IDS: RangeInclusive<u16> = 0..=8191;

/// Subject ID, in range 0..=8191
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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

impl FromStr for SubjectId {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u16 = s.parse().map_err(|_| InvalidValue)?;
        SubjectId::try_from(value)
    }
}

const VALID_SERVICE_IDS: RangeInclusive<u16> = 0..=511;

/// Service ID, in range 0..=511
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Format)]
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

/// Basic transfer priority levels that all transports should support
///
/// Transports can define their own priority levels with more detail.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Priority {
    /// The bus designer can ignore these messages when calculating bus load since they should
    /// only be sent when a total system failure has occurred. For example, a self-destruct message
    /// on a rocket would use this priority. Another analogy is an NMI on a microcontroller.
    Exceptional = 0,
    /// Immediate is a “high priority message” but with additional latency constraints. Since
    /// exceptional messages are not considered when designing a bus, the latency of immediate
    /// messages can be determined by considering only immediate messages.
    Immediate = 1,
    /// Fast and immediate are both “high priority messages” but with additional latency
    /// constraints. Since exceptional messages are not considered when designing a bus, the latency
    /// of fast messages can be determined by considering only immediate and fast messages.
    Fast = 2,
    /// High priority messages are more important than nominal messages but have looser latency
    /// requirements than fast messages. This priority is used so that, in the presence of rogue
    /// nominal messages,important commands can be received. For example, one might envision a
    /// failure mode where a temperature sensor starts to load a vehicle bus with nominal messages.
    /// The vehicle remains operational (for a time) because the controller is exchanging fast and
    /// immediate messages with sensors and actuators. A system safety monitor is able to detect the
    /// distressed bus and command the vehicle to a safe state by sending high priority messages to
    /// the controller.
    High = 3,
    /// This is what all messages should use by default. Specifically the heartbeat messages should
    /// use this priority.
    Nominal = 4,
    /// Low priority messages are expected to be sent on a bus under all conditions but cannot
    /// prevent the delivery of nominal messages. They are allowed to be delayed but latency should
    /// be constrained by the bus designer.
    Low = 5,
    /// Slow messages are low priority messages that have no time sensitivity at all. The bus
    /// designer need only ensure that, for all possible system states, these messages will
    /// eventually be sent.
    Slow = 6,
    /// These messages might never be sent (theoretically) for some possible system states. The
    /// system shall tolerate never exchanging optional messages in every possible state. The bus
    /// designer can ignore these messages when calculating bus load. This should be the priority
    /// used for diagnostic or debug messages that are not required on an operational system.
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

mod fmt_impl {
    use super::SubjectId;
    use core::fmt::{Display, Formatter, Result};

    impl Display for SubjectId {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            Display::fmt(&self.0, f)
        }
    }
}
