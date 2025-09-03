//!
//! # Cyphal/UDP transport
//!
//! The current version of the transport is documented in [a forum post from 2022-12-02](https://forum.opencyphal.org/t/cyphal-udp-architectural-issues-caused-by-the-dependency-between-the-nodes-ip-address-and-its-identity/1765/60).
//!
//! If the `std` feature is enabled, this implementation requires the `std` library for sockets.
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

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate canadensis_core;
extern crate canadensis_header;
extern crate fallible_collections;
extern crate heapless;
extern crate log;
extern crate nb;
#[cfg(feature = "std")]
extern crate socket2;
extern crate zerocopy;

use canadensis_core::transport::Transport;
use canadensis_core::{OutOfMemoryError, Priority};
use canadensis_header::{NodeId16, TransferId64};
use core::fmt::Debug;

pub use crate::rx::{UdpReceiver, UdpSessionData};
pub use crate::tx::UdpTransmitter;

mod address;
pub mod driver;
mod rx;
mod tx;

/// Size of the transfer CRC in bytes
///
/// This is added to the end of every frame
const TRANSFER_CRC_SIZE: usize = 4;

/// The minimum size of a Cyphal/UDP packet
///
/// This is also the minimum MTU. It includes the Cyphal/UDP header and the transfer CRC.
pub const MIN_PACKET_SIZE: usize = canadensis_header::SIZE + TRANSFER_CRC_SIZE;

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
pub type UdpNodeId = NodeId16;

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
pub type UdpTransferId = TransferId64;

#[derive(Debug)]
pub enum Error<S> {
    Memory(OutOfMemoryError),
    Socket(S),
}

impl<S> From<OutOfMemoryError> for Error<S> {
    fn from(oom: OutOfMemoryError) -> Self {
        Error::Memory(oom)
    }
}
