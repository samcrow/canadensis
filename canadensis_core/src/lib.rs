#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

//!
//! This library provides types used by other canadensis crates.
//!

extern crate alloc;
extern crate fallible_collections;
extern crate hash32;
extern crate hash32_derive;
extern crate heapless;
extern crate log;
pub extern crate nb;

mod error;
pub mod session;
pub mod subscription;
pub mod time;
pub mod transfer;
pub mod transport;

use core::convert::TryFrom;
use core::ops::RangeInclusive;
use core::str::FromStr;
use hash32_derive::Hash32;

/// An error indicating that an unacceptable integer was provided to a TryFrom implementation
#[derive(Debug)]
pub struct InvalidValue;
pub use crate::error::{OutOfMemoryError, ServiceSubscribeError};

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

impl FromStr for SubjectId {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u16 = s.parse().map_err(|_| InvalidValue)?;
        SubjectId::try_from(value)
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

/// Transfer priority level mnemonics per the recommendations given in the UAVCAN Specification
#[allow(missing_docs)]
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
