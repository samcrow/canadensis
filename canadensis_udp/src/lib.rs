#![no_std]

extern crate alloc;
extern crate canadensis_core;
extern crate crc_any;
extern crate embedded_nal;
extern crate hash32;
extern crate hash32_derive;
extern crate heapless;
extern crate zerocopy;

mod address;
mod header;
mod rx;
pub mod socket;
mod tx;

pub use crate::address::NodeAddress;
pub use crate::rx::UdpReceiver;
pub use crate::tx::UdpTransmitter;

use canadensis_core::transport::{NodeId, TransferId, Transport};
use canadensis_core::{OutOfMemoryError, Priority};
use core::fmt::Debug;
use core::marker::PhantomData;
use embedded_nal::SocketAddrV4;
use hash32_derive::Hash32;

/// The UAVCAN/UDP transport
///
/// This matches [the pyuavcan implementation](https://pyuavcan.readthedocs.io/en/latest/api/pyuavcan.transport.udp.html#pyuavcan.transport.udp.UDPTransport).
///
/// `I` is a clock instant. `MTU` is the maximum number of payload bytes per packet.
///
/// The MTU must be at least 25 bytes so that each frame can hold a header (24 bytes) and one byte
/// of payload.
pub struct UdpTransport<I, E, const MTU: usize>(PhantomData<I>, PhantomData<E>);

impl<I, E: Debug, const MTU: usize> Transport for UdpTransport<I, E, MTU> {
    type NodeId = UdpNodeId;
    type TransferId = UdpTransferId;
    type Priority = Priority;
    type Frame = UdpFrame<I, MTU>;
    type Error = Error<E>;
}

/// A timestamped frame containing up to MTU bytes of data
#[derive(Debug)]
pub struct UdpFrame<I, const MTU: usize> {
    /// For incoming frames, this is the time when the frame was received from the network.
    ///
    /// For outgoing frames, this is the transmit deadline
    timestamp: I,
    /// For incoming frames, this is the source address.
    ///
    /// For outgoing frames, this is the destination address
    remote_address: SocketAddrV4,
    /// The data in the frame
    data: heapless::Vec<u8, MTU>,
}

impl<I, const MTU: usize> UdpFrame<I, MTU> {
    pub fn new_incoming(timestamp: I, from: SocketAddrV4, data: heapless::Vec<u8, MTU>) -> Self {
        UdpFrame {
            timestamp,
            remote_address: from,
            data,
        }
    }
    pub fn new_outgoing(timestamp: I, to: SocketAddrV4, data: heapless::Vec<u8, MTU>) -> Self {
        UdpFrame {
            timestamp,
            remote_address: to,
            data,
        }
    }

    pub fn timestamp(&self) -> &I {
        &self.timestamp
    }
    pub fn remote_address(&self) -> SocketAddrV4 {
        self.remote_address
    }
    pub fn data(&self) -> &heapless::Vec<u8, MTU> {
        &self.data
    }
    pub fn into_data(self) -> heapless::Vec<u8, MTU> {
        self.data
    }
}

/// A UDP node ID
///
/// This allows all u16 values (0..=65535)
#[derive(Debug, Clone, Eq, PartialEq, Hash32)]
pub struct UdpNodeId(u16);

impl From<u16> for UdpNodeId {
    fn from(value: u16) -> Self {
        UdpNodeId(value)
    }
}
impl From<UdpNodeId> for u16 {
    fn from(id: UdpNodeId) -> Self {
        id.0
    }
}
impl From<UdpNodeId> for u32 {
    fn from(id: UdpNodeId) -> Self {
        id.0.into()
    }
}

impl From<UdpNodeId> for usize {
    fn from(id: UdpNodeId) -> Self {
        id.0.into()
    }
}

impl NodeId<UdpTransferId> for UdpNodeId {
    type TransferIds = UdpTransferIds;
}

pub struct UdpTransferIds([UdpTransferId; u16::MAX as usize + 1]);

impl Default for UdpTransferIds {
    fn default() -> Self {
        UdpTransferIds([UdpTransferId::default(); u16::MAX as usize + 1])
    }
}

impl AsMut<[UdpTransferId]> for UdpTransferIds {
    fn as_mut(&mut self) -> &mut [UdpTransferId] {
        &mut self.0
    }
}

/// A UDP transfer identifier
///
/// This is just a `u64`.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct UdpTransferId(u64);

impl TransferId for UdpTransferId {
    fn increment(self) -> Self {
        UdpTransferId(self.0.wrapping_add(1))
    }
}

impl From<UdpTransferId> for u64 {
    fn from(id: UdpTransferId) -> Self {
        id.0
    }
}

impl From<u64> for UdpTransferId {
    fn from(value: u64) -> Self {
        UdpTransferId(value)
    }
}

#[derive(Debug)]
pub enum Error<E> {
    Memory(OutOfMemoryError),
    Socket(E),
}

impl<E> From<OutOfMemoryError> for Error<E> {
    fn from(oom: OutOfMemoryError) -> Self {
        Error::Memory(oom)
    }
}
