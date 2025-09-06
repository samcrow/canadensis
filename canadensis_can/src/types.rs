//! Data types used for Cyphal/CAN

use canadensis_core::transport::{TransferId, Transport};
use canadensis_core::{InvalidValue, OutOfMemoryError, Priority};
use core::convert::TryFrom;
use core::ops::RangeInclusive;

/// The Cyphal/CAN transport
pub struct CanTransport(());

impl Transport for CanTransport {
    type NodeId = CanNodeId;
    type TransferId = CanTransferId;
    type Priority = Priority;
}

/// Convenience type alias for a transfer header
pub type Header = canadensis_core::transfer::Header<CanTransport>;
/// Convenience type alias for a transfer
pub type Transfer<A> = canadensis_core::transfer::Transfer<A, CanTransport>;

const VALID_NODE_IDS: RangeInclusive<u8> = 0..=127;

/// Node ID
///
/// Valid node IDs are in the range 0..=127 (7 bits). IDs 126 and 127 are reserved for diagnostic
/// and debugging tools.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CanNodeId(u8);

impl CanNodeId {
    /// The smallest allowed node ID (0)
    pub const MIN: CanNodeId = CanNodeId(*VALID_NODE_IDS.start());
    /// The largest allowed node ID (127)
    pub const MAX: CanNodeId = CanNodeId(*VALID_NODE_IDS.end());

    /// Returns the integer value of this node ID
    pub const fn to_u8(self) -> u8 {
        self.0
    }

    /// Creates a valid NodeID from a u8, truncating values that are out of range
    pub const fn from_truncating(value: u8) -> Self {
        CanNodeId(value & *VALID_NODE_IDS.end())
    }

    /// Returns true if this node ID is one of the two highest values, which are reserved for
    /// diagnostic and debugging tools
    pub fn is_diagnostic_reserved(self) -> bool {
        self.0 >= *VALID_NODE_IDS.end() - 1
    }
}

/// A wrapper for an array of `CanTransferIds` that implements Default
pub struct CanTransferIds([CanTransferId; *VALID_NODE_IDS.end() as usize + 1]);

impl AsMut<[CanTransferId]> for CanTransferIds {
    fn as_mut(&mut self) -> &mut [CanTransferId] {
        self.0.as_mut()
    }
}
impl Default for CanTransferIds {
    fn default() -> Self {
        CanTransferIds([CanTransferId::default(); *VALID_NODE_IDS.end() as usize + 1])
    }
}

impl TryFrom<u8> for CanNodeId {
    type Error = InvalidValue;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        if VALID_NODE_IDS.contains(&bits) {
            Ok(CanNodeId(bits))
        } else {
            Err(InvalidValue)
        }
    }
}

impl TryFrom<u16> for CanNodeId {
    type Error = InvalidValue;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= u16::from(*VALID_NODE_IDS.end()) {
            Ok(CanNodeId(value as u8))
        } else {
            Err(InvalidValue)
        }
    }
}

impl From<CanNodeId> for u8 {
    #[inline]
    fn from(id: CanNodeId) -> Self {
        id.0
    }
}
impl From<CanNodeId> for u16 {
    #[inline]
    fn from(id: CanNodeId) -> Self {
        id.0 as u16
    }
}
impl From<CanNodeId> for u32 {
    #[inline]
    fn from(id: CanNodeId) -> Self {
        id.0 as u32
    }
}
impl From<CanNodeId> for usize {
    #[inline]
    fn from(id: CanNodeId) -> Self {
        id.0 as usize
    }
}

const VALID_TRANSFER_IDS: RangeInclusive<u8> = 0..=31;

/// Transfer ID, 5 bits, in range 0..=31
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CanTransferId(u8);

impl CanTransferId {
    /// The largest allowed transfer ID
    pub const MAX: CanTransferId = CanTransferId(*VALID_TRANSFER_IDS.end());
    /// Returns the default transfer ID (0). This is equivalent to Default::default(), but it can
    /// be called in a constant expression.
    pub const fn const_default() -> Self {
        CanTransferId(0)
    }
    /// Returns this transfer ID as a u8
    ///
    /// Unlike `Into::into`, this is const.
    pub const fn to_u8(self) -> u8 {
        self.0
    }
}

impl TransferId for CanTransferId {
    fn increment(self) -> Self {
        if self.0 == *VALID_TRANSFER_IDS.end() {
            // Wrap around to 0
            CanTransferId(0)
        } else {
            CanTransferId(self.0 + 1)
        }
    }
}

impl TryFrom<u8> for CanTransferId {
    type Error = InvalidValue;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        if VALID_TRANSFER_IDS.contains(&value) {
            Ok(CanTransferId(value))
        } else {
            Err(InvalidValue)
        }
    }
}

impl From<CanTransferId> for u8 {
    #[inline]
    fn from(id: CanTransferId) -> Self {
        id.0
    }
}
impl From<CanTransferId> for u16 {
    #[inline]
    fn from(id: CanTransferId) -> Self {
        id.0 as u16
    }
}
impl From<CanTransferId> for u32 {
    #[inline]
    fn from(id: CanTransferId) -> Self {
        id.0 as u32
    }
}
impl From<CanTransferId> for usize {
    #[inline]
    fn from(id: CanTransferId) -> Self {
        id.0 as usize
    }
}

impl Default for CanTransferId {
    /// Returns a transfer ID of 0
    fn default() -> Self {
        CanTransferId::const_default()
    }
}

/// CAN transport errors
#[derive(Debug, PartialEq)]
pub enum Error<E> {
    /// Memory allocation failed
    Memory(OutOfMemoryError),
    /// The driver returned an error
    Driver(E),
}

impl<E> From<OutOfMemoryError> for Error<E> {
    fn from(oom: OutOfMemoryError) -> Self {
        Error::Memory(oom)
    }
}

mod fmt_impl {
    use super::{CanNodeId, CanTransferId};
    use core::fmt::{Debug, Display, Formatter, Result};
    impl Display for CanNodeId {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            Display::fmt(&self.0, f)
        }
    }
    impl Debug for CanNodeId {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            Debug::fmt(&self.0, f)
        }
    }
    impl Display for CanTransferId {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            Display::fmt(&self.0, f)
        }
    }
    impl Debug for CanTransferId {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            Debug::fmt(&self.0, f)
        }
    }
}
