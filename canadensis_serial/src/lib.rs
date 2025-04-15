#![cfg_attr(not(test), no_std)]

extern crate alloc;
extern crate canadensis_core;
extern crate canadensis_header;
extern crate crc_any;
extern crate fallible_collections;
extern crate heapless;
extern crate zerocopy;

use crc_any::CRCu32;

use canadensis_core::transport::Transport;
use canadensis_core::{OutOfMemoryError, Priority};
use canadensis_header::{NodeId16, TransferId64};

pub use crate::rx::{SerialReceiver, Subscription};
pub use crate::tx::SerialTransmitter;

pub(crate) mod cobs;
pub mod driver;
pub(crate) mod header_collector;
mod rx;
mod tx;

/// The Cyphal/Serial transport
///
/// This matches [the pycyphal implementation](https://pycyphal.readthedocs.io/en/latest/api/pycyphal.transport.serial.html).
pub struct SerialTransport(());

impl Transport for SerialTransport {
    type NodeId = SerialNodeId;
    type TransferId = SerialTransferId;
    type Priority = Priority;
}

/// A serial node identifier, which allows the values 0..=65534
///
/// 65535 is reserved as a broadcast address
pub type SerialNodeId = NodeId16;

/// A serial transfer identifier
///
/// This is just a `u64`.
pub type SerialTransferId = TransferId64;

/// Calculates the CRC of a payload
fn make_payload_crc(payload: &[u8]) -> u32 {
    let mut crc = CRCu32::crc32c();
    crc.digest(payload);
    crc.get_crc()
}

/// Serial transport errors
#[derive(Debug)]
pub enum Error<E> {
    /// Memory allocation failed
    Memory(OutOfMemoryError),
    /// The serial driver reported an error
    Driver(E),
}

impl<E> From<OutOfMemoryError> for Error<E> {
    fn from(oom: OutOfMemoryError) -> Self {
        Error::Memory(oom)
    }
}
