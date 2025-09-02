use alloc::vec::Vec;
use core::marker::PhantomData;

use core::net::SocketAddrV4;

use canadensis_core::nb;
use canadensis_core::time::{Clock, Microseconds32};
use canadensis_core::transfer::{Header, Transfer};
use canadensis_core::transport::Transmitter;
use canadensis_header::DataSpecifier;

use crate::address::Address;
use crate::tx::breakdown::{Breakdown, HeaderBase};
use crate::TRANSFER_CRC_SIZE;
use crate::{Error, UdpTransport};

mod breakdown;

pub struct UdpTransmitter<S, const MTU: usize> {
    destination_port: u16,
    _socket: PhantomData<S>,
}
impl<S, const MTU: usize> UdpTransmitter<S, MTU>
where
    S: crate::driver::UdpSocket,
{
    /// Creates a transmitter
    ///
    /// # Panics
    ///
    /// This function panics if `MTU` is less than 28. 28 bytes is the minimum MTU required to
    /// contain a header and transfer CRC in each frame.
    pub fn new(destination_port: u16) -> Self {
        // MTU must be big enough for the header and transfer CRC
        assert!(
            MTU > canadensis_header::SIZE + TRANSFER_CRC_SIZE,
            "MTU is too small"
        );

        UdpTransmitter {
            destination_port,
            _socket: PhantomData,
        }
    }

    fn push_inner<C>(
        &mut self,
        header_base: HeaderBase,
        dest: SocketAddrV4,
        deadline: Microseconds32,
        payload: &[u8],
        clock: &mut C,
        socket: &mut S,
    ) -> Result<(), S::Error>
    where
        C: Clock,
    {
        let breakdown = Breakdown::new(header_base, deadline, payload.iter().copied(), MTU);
        self.send_frames(breakdown, dest, clock, socket)
    }

    fn send_frames<B, C>(
        &mut self,
        breakdown: B,
        destination_address: SocketAddrV4,
        clock: &mut C,
        socket: &mut S,
    ) -> Result<(), S::Error>
    where
        B: IntoIterator<Item = UdpFrame>,
        C: Clock,
    {
        for frame in breakdown {
            if frame.deadline > clock.now() {
                socket.send_to(&frame.data, destination_address)?;
            } else {
                log::trace!("Discarding outgoing frame because its deadline has passed");
            }
        }
        Ok(())
    }
}

impl<C, S, const MTU: usize> Transmitter<C> for UdpTransmitter<S, MTU>
where
    C: Clock,
    S: crate::driver::UdpSocket,
{
    type Transport = UdpTransport;
    type Driver = S;
    type Error = Error<S::Error>;

    fn push<A>(
        &mut self,
        transfer: Transfer<A, Self::Transport>,
        clock: &mut C,
        socket: &mut S,
    ) -> nb::Result<(), Self::Error>
    where
        A: AsRef<[u8]>,
    {
        let deadline = transfer.header.timestamp();
        let (header_base, dest_address) = match transfer.header {
            Header::Message(header) => {
                let multicast_addr = Address::Multicast(header.subject);
                let header_base = HeaderBase {
                    data_specifier: DataSpecifier::Subject {
                        from: header.source,
                        subject: header.subject,
                    },
                    transfer_id: header.transfer_id,
                    priority: header.priority,
                    data: 0,
                };
                (header_base, multicast_addr)
            }
            Header::Request(header) => {
                let dest_addr = Address::Node(header.destination);
                let header_base = HeaderBase {
                    data_specifier: DataSpecifier::ServiceRequest {
                        from: header.source,
                        to: header.destination,
                        service: header.service,
                    },
                    transfer_id: header.transfer_id,
                    priority: header.priority,
                    data: 0,
                };
                (header_base, dest_addr)
            }
            Header::Response(header) => {
                let dest_addr = Address::Node(header.destination);
                let header_base = HeaderBase {
                    data_specifier: DataSpecifier::ServiceResponse {
                        from: header.source,
                        to: header.destination,
                        service: header.service,
                    },
                    transfer_id: header.transfer_id,
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
            socket,
        )
        .map_err(Error::Socket)
        .map_err(nb::Error::Other)
    }

    fn flush(
        &mut self,
        _clock: &mut C,
        _socket: &mut S,
    ) -> canadensis_core::nb::Result<(), Self::Error> {
        // Because the push() function blocks until everything has been transmitted, nothing is
        // needed here.
        Ok(())
    }

    fn mtu(&self) -> usize {
        // Subtract to get the maximum number of payload bytes per frame
        MTU - canadensis_header::SIZE - TRANSFER_CRC_SIZE
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct UdpFrame {
    deadline: Microseconds32,
    data: Vec<u8>,
}
