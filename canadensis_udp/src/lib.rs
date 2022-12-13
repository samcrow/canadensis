//!
//! # Cyphal/UDP transport
//!
//! The current version of the transport is documented in [a forum post from 2022-12-02](https://forum.opencyphal.org/t/cyphal-udp-architectural-issues-caused-by-the-dependency-between-the-nodes-ip-address-and-its-identity/1765/60).
//!
//! This implementation requires the `std` library for sockets.
//!
//! ## How sockets work
//!
//! ### Sending
//!
//! A transport can use one socket to send all outgoing message and service transfers.
//! This socket binds to and an ephemeral UDP port on one network interface.
//!
//! Outgoing transfers get sent to a multicast address based on the port ID and a fixed
//! UDP port.
//!
//! ### Receiving transfers
//!
//! All transfers are received through one socket, which joins any multicast groups required to
//! receive the correct frames.
//!

extern crate alloc;
extern crate canadensis_core;
extern crate crc_any;
extern crate fallible_collections;
extern crate hash32;
extern crate hash32_derive;
extern crate heapless;
extern crate log;
extern crate socket2;
extern crate zerocopy;

use core::fmt::Debug;
use std::convert::TryFrom;
use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use crc_any::{CRCu16, CRCu32};
use hash32_derive::Hash32;
use socket2::{Domain, Protocol, SockAddr, SockRef, Socket, Type};

use canadensis_core::transport::{TransferId, Transport};
use canadensis_core::{InvalidValue, OutOfMemoryError, Priority};

pub use crate::rx::{UdpReceiver, UdpSessionData};
pub use crate::tx::UdpTransmitter;

mod address;
mod header;
mod rx;
mod tx;

/// Size of the transfer CRC in bytes
///
/// This is added to the end of every frame
const TRANSFER_CRC_SIZE: usize = 4;

/// The minimum size of a Cyphal/UDP packet
///
/// This is also the minimum MTU. It includes the Cyphal/UDP header, 1 byte of data, and the
/// transfer CRC.
pub const MIN_PACKET_SIZE: usize = header::SIZE + 1 + TRANSFER_CRC_SIZE;

/// The default UDP port used for communication
pub const DEFAULT_PORT: u16 = 9382;

/// The Cyphal/UDP transport
///
/// This matches [the standard described on the forum on 2022-12-02](https://forum.opencyphal.org/t/cyphal-udp-architectural-issues-caused-by-the-dependency-between-the-nodes-ip-address-and-its-identity/1765/60).
pub struct UdpTransport(());

impl Transport for UdpTransport {
    type NodeId = UdpNodeId;
    type TransferId = UdpTransferId;
    type Priority = Priority;
}

/// A UDP node ID
///
/// This allows all u16 values except 65535, which is reserved for anonymous transfers
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash32)]
pub struct UdpNodeId(u16);

const NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST: u16 = 0xffff;

impl TryFrom<u16> for UdpNodeId {
    type Error = InvalidValue;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST {
            Err(InvalidValue)
        } else {
            Ok(UdpNodeId(value))
        }
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
impl From<std::io::Error> for Error {
    fn from(inner: std::io::Error) -> Self {
        Error::Socket(inner)
    }
}

const DEFAULT_TTL: u32 = 16;

/// Creates a socket, enables non-blocking mode, binds to the provided
/// address and port, and returns the socket
fn bind_receive_socket(address: Ipv4Addr, port: u16) -> Result<Socket, io::Error> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    // socket.set_nonblocking(true)?;
    socket.set_reuse_address(true)?;
    socket.bind(&SockAddr::from(SocketAddrV4::new(address, port)))?;
    Ok(socket)
}
/// Creates a socket, sets the TTL to DEFAULT_TTL, binds to the provided
/// address and port, and returns the socket
fn bind_transmit_socket(address: Ipv4Addr, port: u16) -> Result<UdpSocket, io::Error> {
    let socket = UdpSocket::bind((address, port))?;
    socket.set_multicast_ttl_v4(DEFAULT_TTL)?;
    Ok(socket)
}

/// Returns a CRC calculator used for headers
fn header_crc() -> CRCu16 {
    CRCu16::crc16ccitt_false()
}

/// Returns a CRC calculator used for data
fn data_crc() -> CRCu32 {
    CRCu32::crc32c()
}
