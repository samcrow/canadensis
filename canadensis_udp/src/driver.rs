use core::fmt::Debug;
use std::net::{Ipv4Addr, SocketAddrV4};

/// A driver that contains a UDP socket and can send and receive frames
pub trait UdpDriver {
    /// The error type that this driver can report
    type Error: Debug;

    type Socket: UdpSocket<Error = Self::Error>;

    fn bind(&mut self, address: Ipv4Addr, port: u16) -> Result<Self::Socket, Self::Error>;
}

pub trait UdpSocket {
    type Error: Debug;

    /// Sets the time to live for IPv4 multicast packets sent through this socket
    fn set_multicast_ttl_v4(&mut self, multicast_ttl_v4: u32) -> Result<(), Self::Error>;
    /// Joins an IPv4 multicast group
    ///
    /// multiaddr: The address of the group
    ///
    /// interface: The address of the network interface to operate on
    fn join_multicast_v4(
        &mut self,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> Result<(), Self::Error>;
    /// Leaves an IPv4 multicast group
    ///
    /// multiaddr: The address of the group
    ///
    /// interface: The address of the network interface to operate on
    fn leave_multicast_v4(
        &mut self,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> Result<(), Self::Error>;

    fn send_to(&mut self, data: &[u8], destination: SocketAddrV4) -> Result<usize, Self::Error>;

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
}

struct StdUdpDriver;

impl UdpDriver for StdUdpDriver {
    type Error = std::io::Error;
    type Socket = StdUdpSocket;

    fn bind(&mut self, address: Ipv4Addr, port: u16) -> Result<Self::Socket, Self::Error> {
        let socket = std::net::UdpSocket::bind((address, port))?;
        Ok(StdUdpSocket(socket))
    }
}

struct StdUdpSocket(std::net::UdpSocket);

impl UdpSocket for StdUdpSocket {
    type Error = std::io::Error;

    fn set_multicast_ttl_v4(&mut self, multicast_ttl_v4: u32) -> Result<(), Self::Error> {
        self.0.set_multicast_ttl_v4(multicast_ttl_v4)
    }

    fn join_multicast_v4(
        &mut self,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> Result<(), Self::Error> {
        self.0.join_multicast_v4(multiaddr, interface)
    }

    fn leave_multicast_v4(
        &mut self,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> Result<(), Self::Error> {
        self.0.leave_multicast_v4(multiaddr, interface)
    }

    fn send_to(&mut self, data: &[u8], destination: SocketAddrV4) -> Result<usize, Self::Error> {
        self.0.send_to(data, destination)
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.recv(buffer)
    }
}
