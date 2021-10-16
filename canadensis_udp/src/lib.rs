//!
//! # UAVCAN/UDP transport
//!
//! The current version of the transport is documented at <https://pyuavcan.readthedocs.io/en/latest/api/pyuavcan.transport.udp.html>.
//!
//! This implementation requires the `std` library for sockets.
//!
//! ## How sockets work
//!
//! ### Sending
//!
//! A transport can use one socket to send all outgoing message and service transfers.
//! This socket gets bound to a normal address derived from the local node ID and an ephemeral
//! UDP port.
//!
//! Outgoing message transfers get sent to a multicast address based on the port ID and the fixed
//! UDP port 16383.
//!
//! Outgoing request transfers get sent to the address of the destination node with a UDP port
//! number based on the service ID.
//!
//! ### Receiving message transfers
//!
//! Each subscription requires its own socket. The socket gets bound to the multicast address
//! derived from the subject ID and the fixed UDP port 16383.
//!
//! When the transport receives a packet, it knows the subject ID (associated with the socket)
//! and extracts the source node ID from the source IP address.
//!
//! ### Receiving service transfers
//!
//! Each subscription requires its own socket. The socket gets bound to a normal address derived
//! from the local node ID and a UDP port based on the service ID.
//!
//! When the transport receives a packet, it knows the service ID (associated with the socket)
//! and extracts the source node ID from the source IP address.
//!

extern crate alloc;
extern crate canadensis_core;
extern crate crc_any;
extern crate fallible_collections;
extern crate hash32;
extern crate hash32_derive;
extern crate heapless;
extern crate log;
extern crate zerocopy;

use core::fmt::Debug;
use fallible_collections::TryReserveError;
use std::io;
use std::marker::PhantomData;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

use hash32_derive::Hash32;

use canadensis_core::time::Instant;
use canadensis_core::transport::{NodeId, TransferId, Transport};
use canadensis_core::{OutOfMemoryError, Priority};

pub use crate::address::NodeAddress;
pub use crate::rx::{UdpReceiver, UdpSessionData};
pub use crate::tx::UdpTransmitter;

mod address;
mod header;
mod rx;
mod tx;

/// The UAVCAN/UDP transport
///
/// This matches [the pyuavcan implementation](https://pyuavcan.readthedocs.io/en/latest/api/pyuavcan.transport.udp.html#pyuavcan.transport.udp.UDPTransport).
///
/// `I` is a clock instant. `MTU` is the maximum number of payload bytes per packet.
///
/// The MTU must be at least 25 bytes so that each frame can hold a header (24 bytes) and one byte
/// of payload.
pub struct UdpTransport<I>(PhantomData<I>);

impl<I> Transport for UdpTransport<I>
where
    I: Instant,
{
    type NodeId = UdpNodeId;
    type TransferId = UdpTransferId;
    type Priority = Priority;
    /// Because each subscription has its own socket, this can't use the normal interface.
    type Frame = I;
    type Error = Error;
}

/// A UDP node ID
///
/// This allows all u16 values (0..=65535)
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash32)]
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
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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
pub enum Error {
    Memory(OutOfMemoryError),
    Socket(std::io::Error),
}

impl From<OutOfMemoryError> for Error {
    fn from(oom: OutOfMemoryError) -> Self {
        Error::Memory(oom)
    }
}
impl From<TryReserveError> for Error {
    fn from(inner: TryReserveError) -> Self {
        Error::Memory(OutOfMemoryError::from(inner))
    }
}
impl From<std::io::Error> for Error {
    fn from(inner: std::io::Error) -> Self {
        Error::Socket(inner)
    }
}

/// Creates a socket, enables port and address reuse, enables non-blocking mode, binds to the provided
/// address and port, and returns the socket
fn bind_socket(address: Ipv4Addr, port: u16) -> Result<UdpSocket, io::Error> {
    let socket = socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, None)?;
    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;
    socket.set_nonblocking(true)?;
    // Bind to the multicast address and fixed message port
    socket.bind(&SocketAddr::V4(SocketAddrV4::new(address, port)).into())?;
    Ok(socket.into())
}
