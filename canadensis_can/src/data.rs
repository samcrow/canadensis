//!
//! Common UAVCAN data types
//!

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt;

use canadensis_core::{InvalidValue, Microseconds};

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
            Err(InvalidValue)
        }
    }
}

impl From<CanId> for u32 {
    fn from(id: CanId) -> Self {
        id.0
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
