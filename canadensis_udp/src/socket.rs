//! Socket traits for drivers that the UDP transport can use

use core::fmt::Debug;
use embedded_nal::{nb, Ipv4Addr, SocketAddrV4};

/// A UDP socket that is not yet bound to a local interface and port
pub trait UnboundSocket {
    /// The error type
    type Error: Debug;
    /// The version of this socket after binding
    type Bound: Socket<Error = Self::Error, Unbound = Self>;

    /// Binds this socket to a local network interface
    ///
    /// The port number should be unspecified so that the operating system (or similar) will
    /// automatically select an ephemeral port.
    fn bind(self, address: Ipv4Addr) -> Result<Self::Bound, Self::Error>;
}

/// A bound UDP socket that can be used to send and receive packets
pub trait Socket {
    type Error: Debug;
    /// The unbound version of this socket
    type Unbound: UnboundSocket<Error = Self::Error, Bound = Self>;

    /// Joins a multicast group so that this socket will receive packets sent to the given address
    /// on the network interface that it is bound to
    fn join_multicast_v4(&mut self, address: Ipv4Addr) -> Result<(), Self::Error>;
    /// Leaves a multicast group
    fn leave_multicast_v4(&mut self, address: Ipv4Addr) -> Result<(), Self::Error>;

    /// Sends a packet to a destination address
    ///
    /// This function must not block. It should return a `WouldBlock` error if the operation
    /// cannot be completed immediately.
    ///
    /// This function returns an error if the number of bytes actually sent is less than the
    /// length of `buf`.
    fn send_to(&mut self, buf: &[u8], address: SocketAddrV4) -> nb::Result<(), Self::Error>;

    /// Receives an incoming packet
    ///
    /// This function must not block. It should return a `WouldBlock` error if the operation
    /// cannot be completed immediately.
    ///
    /// On success, this function returns the number of bytes read and the source address.
    fn recv_from(&mut self, buf: &mut [u8]) -> nb::Result<(usize, SocketAddrV4), Self::Error>;
}
