use std::cmp::Ordering;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use canadensis_core::nb;
use canadensis_core::time::{Clock, Instant};
use canadensis_core::transfer::{Header, Transfer};
use canadensis_core::transport::Transmitter;

use crate::address::Address;
use crate::header::DataSpecifier;
use crate::tx::breakdown::{Breakdown, HeaderBase};
use crate::{header, UdpNodeId, TRANSFER_CRC_SIZE};
use crate::{Error, UdpTransport};

mod breakdown;

pub struct UdpTransmitter<const MTU: usize> {
    /// The socket used to send frames
    socket: UdpSocket,
    /// The address of this node
    local_id: Option<UdpNodeId>,
    destination_port: u16,
}
impl<const MTU: usize> UdpTransmitter<MTU> {
    /// Creates a transmitter
    ///
    /// # Panics
    ///
    /// This function panics if `MTU` is less than 29. 29 bytes is the minimum MTU required to
    /// contain a header, transfer CRC, and one byte of payload in each frame.
    pub fn new(
        local_id: Option<UdpNodeId>,
        bind_address: Ipv4Addr,
        destination_port: u16,
    ) -> Result<Self, Error> {
        // MTU must be big enough for the header, transfer CRC, and at least 1 byte of data
        assert!(
            MTU > header::SIZE + TRANSFER_CRC_SIZE + 1,
            "MTU is too small"
        );

        // Bind to an ephemeral port
        let socket = bind_transmit_socket(bind_address, 0)?;

        Ok(UdpTransmitter {
            socket,
            local_id,
            destination_port,
        })
    }

    fn push_inner<I, C>(
        &mut self,
        header_base: HeaderBase,
        dest: SocketAddrV4,
        deadline: I,
        payload: &[u8],
        clock: &mut C,
    ) -> Result<(), Error>
    where
        I: Instant,
        C: Clock<Instant = I>,
    {
        let breakdown = Breakdown::new(header_base, deadline, payload.iter().copied(), MTU);
        self.send_frames(breakdown, dest, clock)
    }

    fn send_frames<I, B, C>(
        &mut self,
        breakdown: B,
        destination_address: SocketAddrV4,
        clock: &mut C,
    ) -> Result<(), Error>
    where
        I: Instant,
        B: IntoIterator<Item = UdpFrame<I>>,
        C: Clock<Instant = I>,
    {
        for frame in breakdown {
            if frame.deadline.overflow_safe_compare(&clock.now()) == Ordering::Greater {
                self.socket.send_to(&frame.data, destination_address)?;
            }
        }
        Ok(())
    }
}

impl<I, const MTU: usize> Transmitter<I> for UdpTransmitter<MTU>
where
    I: Instant,
{
    type Transport = UdpTransport;
    /// The UDP transport uses an internal socket instead of a separate driver.
    type Driver = ();
    type Error = Error;

    fn push<A, C>(
        &mut self,
        transfer: Transfer<A, I, Self::Transport>,
        clock: &mut C,
        _driver: &mut (),
    ) -> nb::Result<(), Error>
    where
        A: AsRef<[u8]>,
        C: Clock<Instant = I>,
    {
        let deadline = transfer.header.timestamp();
        let (header_base, dest_address) = match transfer.header {
            Header::Message(header) => {
                let multicast_addr = Address::Multicast(header.subject);
                let header_base = HeaderBase {
                    source_node: self.local_id,
                    destination_node: None,
                    data_specifier: DataSpecifier::Subject(header.subject),
                    transfer_id: header.transfer_id.into(),
                    priority: header.priority,
                    data: 0,
                };
                (header_base, multicast_addr)
            }
            Header::Request(header) => {
                let dest_addr = Address::Node(header.destination);
                let header_base = HeaderBase {
                    source_node: self.local_id,
                    destination_node: Some(header.destination),
                    data_specifier: DataSpecifier::ServiceRequest(header.service),
                    transfer_id: header.transfer_id.into(),
                    priority: header.priority,
                    data: 0,
                };
                (header_base, dest_addr)
            }
            Header::Response(header) => {
                let dest_addr = Address::Node(header.destination);
                let header_base = HeaderBase {
                    source_node: self.local_id,
                    destination_node: Some(header.destination),
                    data_specifier: DataSpecifier::ServiceResponse(header.service),
                    transfer_id: header.transfer_id.into(),
                    priority: header.priority,
                    data: 0,
                };
                (header_base, dest_addr)
            }
        };
        self.push_inner(
            header_base,
            SocketAddrV4::new(dest_address.into(), self.destination_port),
            deadline,
            transfer.payload.as_ref(),
            clock,
        )
        .map_err(nb::Error::Other)
    }

    fn flush<C>(
        &mut self,
        _clock: &mut C,
        _driver: &mut (),
    ) -> canadensis_core::nb::Result<(), Self::Error>
    where
        C: Clock<Instant = I>,
    {
        // Because the push() function blocks until everything has been transmitted, nothing is
        // needed here.
        Ok(())
    }

    fn mtu(&self) -> usize {
        // Subtract to get the maximum number of payload bytes per frame
        MTU - header::SIZE - TRANSFER_CRC_SIZE
    }
}

pub(crate) struct UdpFrame<I> {
    deadline: I,
    data: Vec<u8>,
}

/// Creates a socket, sets the TTL to DEFAULT_TTL, binds to the provided
/// address and port, and returns the socket
fn bind_transmit_socket(address: Ipv4Addr, port: u16) -> Result<UdpSocket, std::io::Error> {
    let socket = UdpSocket::bind((address, port))?;
    socket.set_multicast_ttl_v4(crate::DEFAULT_TTL)?;
    Ok(socket)
}
