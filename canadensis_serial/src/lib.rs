// std for testing only
// #![cfg_attr(not(test), no_std)]

extern crate alloc;
extern crate canadensis_core;
extern crate crc_any;
extern crate fallible_collections;
extern crate hash32;
extern crate hash32_derive;
extern crate heapless;
extern crate log;
extern crate zerocopy;

use core::convert::TryFrom;

use crc_any::CRCu32;
use hash32_derive::Hash32;

use canadensis_core::transport::{NodeId, TransferId, Transport};
use canadensis_core::{InvalidValue, OutOfMemoryError, Priority};

pub use crate::rx::SerialReceiver;
pub use crate::tx::{SerialTransmitter, TransmitQueue};

pub(crate) mod cobs;
pub(crate) mod header;
pub(crate) mod header_collector;
mod rx;
mod tx;

pub struct SerialTransport;

impl Transport for SerialTransport {
    type NodeId = SerialNodeId;
    type TransferId = SerialTransferId;
    type Priority = Priority;
    type Frame = u8;
    type Error = OutOfMemoryError;
}

/// A serial node identifier, which allows the values 0..=65534
///
/// 65535 is reserved as a broadcast address
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash32)]
pub struct SerialNodeId(u16);

const MAX_NODE_ID: u16 = 65534;

impl From<SerialNodeId> for u16 {
    fn from(id: SerialNodeId) -> Self {
        id.0
    }
}
impl From<SerialNodeId> for usize {
    fn from(id: SerialNodeId) -> Self {
        id.0.into()
    }
}

impl NodeId<SerialTransferId> for SerialNodeId {
    type TransferIds = SerialTransferIds;
}

pub struct SerialTransferIds([SerialTransferId; MAX_NODE_ID as usize + 1]);

impl Default for SerialTransferIds {
    fn default() -> Self {
        SerialTransferIds([SerialTransferId::default(); MAX_NODE_ID as usize + 1])
    }
}

impl AsMut<[SerialTransferId]> for SerialTransferIds {
    fn as_mut(&mut self) -> &mut [SerialTransferId] {
        &mut self.0
    }
}

impl TryFrom<u16> for SerialNodeId {
    type Error = InvalidValue;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= MAX_NODE_ID {
            Ok(SerialNodeId(value))
        } else {
            Err(InvalidValue)
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct SerialTransferId(u64);

impl TransferId for SerialTransferId {
    fn increment(self) -> Self {
        SerialTransferId(self.0.wrapping_add(1))
    }
}

impl From<SerialTransferId> for u64 {
    fn from(id: SerialTransferId) -> Self {
        id.0
    }
}

impl From<u64> for SerialTransferId {
    fn from(value: u64) -> Self {
        SerialTransferId(value)
    }
}

/// Calculates the CRC of a payload
fn make_payload_crc(payload: &[u8]) -> u32 {
    let mut crc = CRCu32::crc32c();
    crc.digest(payload);
    crc.get_crc()
}
