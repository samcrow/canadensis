use core::fmt::Debug;
use core::net::{Ipv4Addr, SocketAddrV4};

/// A socket that supports the basic operations required for Cyphal/UDP
///
/// # Setup requirements
///
/// Before a socket can be used, it needs to be bound to a local port and IPv4 address.
///
/// The time to live of outgoing multicast packets may also need to be changed.
///
pub trait UdpSocket {
    type Error: Debug;

    /// Returns the local address this socket is bound to
    fn local_addr(&self) -> Result<SocketAddrV4, Self::Error>;

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

    /// Sends a packet to the provided destination, and returns the number of bytes sent
    ///
    /// This function must block until the packet can be sent.
    fn send_to(&mut self, data: &[u8], destination: SocketAddrV4) -> Result<usize, Self::Error>;

    /// Tries to receive a packet and write it to the provided buffer, and returns the number
    /// of bytes read
    ///
    /// This function must not block.
    fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, nb::Error<Self::Error>>;
}

#[cfg(feature = "std")]
pub use self::std_socket::StdUdpSocket;

#[cfg(feature = "std")]
mod std_socket {
    use super::UdpSocket;
    use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    use socket2::{Domain, Protocol, SockAddr, Socket, Type};
    use std::time::Duration;

    /// A socket that uses the standard library UdpSocket implementation
    pub struct StdUdpSocket(std::net::UdpSocket);

    impl StdUdpSocket {
        /// Creates a socket and binds it to the provided IP address and port
        pub fn bind(interface_address: Ipv4Addr, local_port: u16) -> std::io::Result<Self> {
            let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
            // The reuse address option lets multiple processes get the same multicast packets
            socket.set_reuse_address(true)?;
            socket.set_multicast_ttl_v4(16)?;
            // Set a low read timeout to approximate non-blocking reads but keep writes blocking
            socket.set_read_timeout(Some(Duration::from_millis(1)))?;
            socket.bind(&SockAddr::from(SocketAddrV4::new(
                interface_address,
                local_port,
            )))?;
            Ok(StdUdpSocket(socket.into()))
        }
    }

    impl UdpSocket for StdUdpSocket {
        type Error = std::io::Error;

        fn local_addr(&self) -> Result<SocketAddrV4, Self::Error> {
            self.0.local_addr().map(|addr| match addr {
                SocketAddr::V4(addr) => addr,
                SocketAddr::V6(_) => unreachable!("IPv6 not supported"),
            })
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

        fn send_to(
            &mut self,
            data: &[u8],
            destination: SocketAddrV4,
        ) -> Result<usize, Self::Error> {
            self.0.send_to(data, destination)
        }

        fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, nb::Error<Self::Error>> {
            self.0.recv(buffer).map_err(|e| {
                // Convert would-block-type errors into nb::Error::WouldBlock
                use std::io::ErrorKind::*;
                match e.kind() {
                    WouldBlock | TimedOut => nb::Error::WouldBlock,
                    _ => nb::Error::Other(e),
                }
            })
        }
    }
}
