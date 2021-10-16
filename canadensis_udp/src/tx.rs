mod breakdown;

use crate::address::{NodeAddress, UdpPort};
use crate::tx::breakdown::Breakdown;
use crate::{bind_socket, header};
use crate::{Error, UdpTransferId, UdpTransport};
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, Transfer};
use canadensis_core::transport::Transmitter;
use canadensis_core::Priority;
use crc_any::CRCu32;
use std::net::{SocketAddrV4, UdpSocket};

pub struct UdpTransmitter<const MTU: usize> {
    /// The socket used to send frames
    socket: UdpSocket,
    /// The address of this node
    address: NodeAddress,
}
impl<const MTU: usize> UdpTransmitter<MTU> {
    /// Creates a transmitter
    ///
    /// # Panics
    ///
    /// This function panics if `MTU` is less than 25. 25 bytes is the minimum MTU required to
    /// contain a header and one byte of payload in each frame.
    pub fn new(address: NodeAddress) -> Result<Self, Error> {
        assert!(MTU >= header::SIZE + 1, "MTU is too small");

        // Bind to an ephemeral port
        let socket = bind_socket(address.clone().into(), 0)?;

        Ok(UdpTransmitter { socket, address })
    }

    fn push_inner<I: Instant>(
        &mut self,
        dest: SocketAddrV4,
        deadline: I,
        transfer_id: UdpTransferId,
        priority: Priority,
        payload: &[u8],
    ) -> Result<(), Error> {
        if breakdown::fits_into_one_frame::<MTU>(payload.len()) {
            // No CRC
            let breakdown = Breakdown::<_, _, MTU>::new(
                dest,
                deadline,
                transfer_id.into(),
                priority,
                payload.iter().copied(),
            );
            self.send_frames(breakdown)
        } else {
            // Add CRC
            let mut crc = CRCu32::crc32c();
            crc.digest(payload);
            let crc_bytes = crc.get_crc().to_le_bytes();
            let payload_and_crc = payload.iter().copied().chain(crc_bytes.iter().copied());
            let breakdown = Breakdown::<_, _, MTU>::new(
                dest,
                deadline,
                transfer_id.into(),
                priority,
                payload_and_crc,
            );
            self.send_frames(breakdown)
        }
    }

    fn send_frames<I: Instant, B: IntoIterator<Item = UdpFrame<I, MTU>>>(
        &mut self,
        breakdown: B,
    ) -> Result<(), Error> {
        for frame in breakdown {
            self.socket.send_to(&frame.data, frame.remote_address)?;
        }
        Ok(())
    }
}

impl<I, const MTU: usize> Transmitter<I> for UdpTransmitter<MTU>
where
    I: Instant,
{
    type Transport = UdpTransport<I>;

    fn push<A>(&mut self, transfer: Transfer<A, I, Self::Transport>) -> Result<(), Error>
    where
        A: AsRef<[u8]>,
    {
        match transfer.header {
            Header::Message(header) => {
                let multicast_addr = self.address.multicast_address(header.subject);
                let dest_port: u16 = UdpPort::Message.into();
                self.push_inner(
                    SocketAddrV4::new(multicast_addr, dest_port),
                    header.timestamp,
                    header.transfer_id,
                    header.priority,
                    transfer.payload.as_ref(),
                )
            }
            Header::Request(header) => {
                let dest_addr = self.address.remote_node_address(header.destination);
                let dest_port: u16 = UdpPort::Request(header.service).into();
                self.push_inner(
                    SocketAddrV4::new(dest_addr, dest_port),
                    header.timestamp,
                    header.transfer_id,
                    header.priority,
                    transfer.payload.as_ref(),
                )
            }
            Header::Response(header) => {
                let dest_addr = self.address.remote_node_address(header.destination);
                let dest_port: u16 = UdpPort::Response(header.service).into();
                self.push_inner(
                    SocketAddrV4::new(dest_addr, dest_port),
                    header.timestamp,
                    header.transfer_id,
                    header.priority,
                    transfer.payload.as_ref(),
                )
            }
        }
    }

    fn mtu(&self) -> usize {
        // Subtract to get the maximum number of payload bytes per frame
        MTU - header::SIZE
    }
}

pub(crate) struct UdpFrame<I, const MTU: usize> {
    remote_address: SocketAddrV4,
    deadline: I,
    data: heapless::Vec<u8, MTU>,
}
