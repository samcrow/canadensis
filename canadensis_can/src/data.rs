//!
//! Common UAVCAN data types
//!

use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt::{self, Debug};
use heapless::consts::U64;

use canadensis_core::InvalidValue;
use embedded_time::{Clock, Instant};

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

/// CAN or CAN FD data frame with up to 64 bytes of data and an extended 29-bit ID
///
/// RTR/Error frames are not used and therefore not modeled here.
/// CAN frames with 11-bit ID are not used by UAVCAN/CAN and so they are not supported by the library.
///
/// Frames with the same ID will compare as equal.
pub struct Frame<C: Clock> {
    /// For RX frames: reception timestamp (time when this frame was received)
    /// For TX frames: transmission deadline (frame must be discarded if not transmitted by this time)
    /// The time system may be arbitrary as long as the clock is monotonic (steady).
    timestamp: Instant<C>,
    /// 29-bit extended ID
    id: CanId,
    /// The frame data
    data: heapless::Vec<u8, U64>,
}

impl<C> Debug for Frame<C>
where
    C: Clock,
    Instant<C>: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frame")
            .field("timestamp", &self.timestamp)
            .field("id", &self.id)
            .field("data", &self.data)
            .finish()
    }
}

impl<C: Clock> Frame<C> {
    /// Creates a frame
    ///
    /// # Panics
    /// This function will panic if the length of data is greater than 64.
    pub fn new(timestamp: Instant<C>, id: CanId, data: &[u8]) -> Self {
        Frame {
            timestamp,
            id,
            data: heapless::Vec::from_slice(data).expect("Data to large for a CAN FD frame"),
        }
    }

    /// Returns the timestamp when this frame was received (for incoming frames)
    /// or the transmission deadline (for outgoing frames)
    #[inline]
    pub fn timestamp(&self) -> Instant<C> {
        self.timestamp
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

impl<C: Clock> PartialOrd for Frame<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<C: Clock> Ord for Frame<C> {
    /// Compare by CAN ID, ignore other fields
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<C: Clock> PartialEq for Frame<C> {
    /// Compare by CAN ID, ignore other fields
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<C: Clock> Eq for Frame<C> {}
