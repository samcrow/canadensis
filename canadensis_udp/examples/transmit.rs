extern crate canadensis_core;
extern crate canadensis_udp;

use canadensis_core::time::Microseconds32;
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::{TransferId, Transmitter};
use canadensis_core::{Priority, SubjectId};
use canadensis_udp::socket::{Socket, UnboundSocket};
use canadensis_udp::{NodeAddress, UdpNodeId, UdpTransferId, UdpTransmitter};
use embedded_nal::{nb, Ipv4Addr, SocketAddrV4};
use std::convert::TryFrom;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let node_id = UdpNodeId::from(4);
    let address = NodeAddress::new(127 << 1, 8, node_id.clone()).unwrap();
    println!(
        "This node's IP address: {}",
        Ipv4Addr::from(address.clone())
    );
    const MTU: usize = 1200;
    let mut transmitter = UdpTransmitter::<StdSocket, MTU>::new(StdUnboundSocket, address).unwrap();

    let mut transfer_id = UdpTransferId::default();
    loop {
        let transfer = Transfer {
            header: Header::Message(MessageHeader {
                timestamp: Microseconds32::new(0),
                transfer_id: transfer_id.clone(),
                priority: Priority::Nominal,
                subject: SubjectId::try_from(99u16).unwrap(),
                source: Some(node_id.clone()),
            }),
            // This payload is compatible with the uvcan.primitive.String.1.0 format.
            payload: b"\x1a\x00This is a UDP transmission",
        };

        transmitter.push(transfer).unwrap();

        transfer_id = transfer_id.increment();

        sleep(Duration::from_secs(1));
    }
}

struct StdUnboundSocket;
struct StdSocket(std::net::UdpSocket);

impl UnboundSocket for StdUnboundSocket {
    type Error = std::io::Error;
    type Bound = StdSocket;

    fn bind(self, address: Ipv4Addr) -> Result<Self::Bound, Self::Error> {
        let converted_address =
            std::net::SocketAddr::V4(std::net::SocketAddrV4::new(convert_addr(address), 0));
        let socket = UdpSocket::bind(converted_address)?;
        socket.set_nonblocking(true)?;
        Ok(StdSocket(socket))
    }
}

impl Socket for StdSocket {
    type Error = std::io::Error;
    type Unbound = StdUnboundSocket;

    fn join_multicast_v4(&mut self, address: Ipv4Addr) -> Result<(), Self::Error> {
        let local_address = match self.0.local_addr()? {
            std::net::SocketAddr::V4(v4) => v4.ip().clone(),
            std::net::SocketAddr::V6(_) => unreachable!("Socket bound to v6 address"),
        };
        let group_address = convert_addr(address);
        self.0.join_multicast_v4(&group_address, &local_address)
    }

    fn leave_multicast_v4(&mut self, address: Ipv4Addr) -> Result<(), Self::Error> {
        let local_address = match self.0.local_addr()? {
            std::net::SocketAddr::V4(v4) => v4.ip().clone(),
            std::net::SocketAddr::V6(_) => unreachable!("Socket bound to v6 address"),
        };
        let group_address = convert_addr(address);
        self.0.leave_multicast_v4(&group_address, &local_address)
    }

    fn send_to(&mut self, buf: &[u8], address: SocketAddrV4) -> nb::Result<(), Self::Error> {
        let address = convert_socket_addr(address);
        let bytes_sent = match self.0.send_to(buf, address) {
            Ok(bytes) => bytes,
            Err(e) => {
                return match e.kind() {
                    ErrorKind::WouldBlock => Err(nb::Error::WouldBlock),
                    _ => Err(nb::Error::Other(e)),
                }
            }
        };
        if bytes_sent == buf.len() {
            Ok(())
        } else {
            Err(nb::Error::Other(std::io::Error::new(
                ErrorKind::Interrupted,
                "Can't write full buffer",
            )))
        }
    }

    fn recv_from(&mut self, buf: &mut [u8]) -> nb::Result<(usize, SocketAddrV4), Self::Error> {
        loop {
            match self.0.recv_from(buf) {
                Ok((bytes, source)) => {
                    if let Some(source) = convert_socket_addr_back(source) {
                        break Ok((bytes, source));
                    } else {
                        // This was an IPv6 packet. Go back and try the next packet.
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::WouldBlock => break Err(nb::Error::WouldBlock),
                    _ => break Err(nb::Error::Other(e)),
                },
            }
        }
    }
}
/// Converts an embedded-nal address into a std::net address
fn convert_addr(address: Ipv4Addr) -> std::net::Ipv4Addr {
    std::net::Ipv4Addr::from(address.octets())
}
/// Converts an embedded-nal SocketAddrV4 into a std::net SocketAddr
fn convert_socket_addr(address: SocketAddrV4) -> std::net::SocketAddr {
    let ip = convert_addr(address.ip().clone());
    let port = address.port();
    std::net::SocketAddr::V4(std::net::SocketAddrV4::new(ip, port))
}
fn convert_socket_addr_back(address: std::net::SocketAddr) -> Option<SocketAddrV4> {
    match address {
        std::net::SocketAddr::V4(v4) => {
            let address = Ipv4Addr::from(v4.ip().octets());
            let port = v4.port();
            Some(SocketAddrV4::new(address, port))
        }
        std::net::SocketAddr::V6(_) => None,
    }
}
