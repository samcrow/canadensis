//!
//! Common UAVCAN data types
//!

use core::convert::TryFrom;
use core::fmt;

use canadensis_core::InvalidValue;

/// Bit mask for a 29-bit CAN ID
const CAN_ID_MASK: u32 = 0x1f_ff_ff_ff;

/// A 29-bit extended CAN ID
#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Default)]
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
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Mtu {
    /// 8 bytes, for standard CAN
    Can8 = 8,
    /// 64 bytes, for CAN FD
    #[cfg(feature = "can-fd")]
    CanFd64 = 64,
}

impl Mtu {
    /// Returns the number of bytes that this MTU represents
    pub fn as_bytes(&self) -> usize {
        *self as usize
    }
}

/// Maximum number of bytes in a frame
#[cfg(feature = "can-fd")]
pub const FRAME_CAPACITY: usize = 64;
/// Maximum number of bytes in a frame
#[cfg(not(feature = "can-fd"))]
pub const FRAME_CAPACITY: usize = 8;

/// CAN or CAN FD data frame with up to 64 bytes of data and an extended 29-bit ID
///
/// RTR/Error frames are not used and therefore not modeled here.
/// CAN frames with 11-bit ID are not used by UAVCAN/CAN and so they are not supported by the library.
#[derive(Clone, PartialEq, Default)]
pub struct Frame<I> {
    /// For RX frames: reception timestamp.
    /// For TX frames: transmission deadline.
    /// The time system may be arbitrary as long as the clock is monotonic (steady).
    timestamp: I,
    /// 29-bit extended ID
    id: CanId,
    /// The frame data
    data: heapless::Vec<u8, FRAME_CAPACITY>,
}

impl<I> Frame<I> {
    /// Creates a frame
    ///
    /// # Panics
    /// This function will panic if the length of data is greater than FRAME_CAPACITY.
    pub fn new(timestamp: I, id: CanId, data: &[u8]) -> Self {
        Frame {
            timestamp,
            id,
            data: heapless::Vec::from_slice(data).expect("Data to large for a frame"),
        }
    }
    /// Returns the ID of this frame
    #[inline]
    pub fn id(&self) -> CanId {
        self.id
    }
    /// Returns the data in this frame
    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl<I: Clone> Frame<I> {
    /// Returns the timestamp when this frame was received (for incoming frames)
    /// or the transmission deadline (for outgoing frames)
    #[inline]
    pub fn timestamp(&self) -> I {
        self.timestamp.clone()
    }
}
