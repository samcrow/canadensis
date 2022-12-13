use crate::header::{
    self, DataSpecifier, UdpHeader, DATA_SPEC_REQUEST_NOT_RESPONSE, DATA_SPEC_SERVICE_NOT_MESSAGE,
    LAST_FRAME,
};
use crate::tx::UdpFrame;
use crate::{
    data_crc, header_crc, UdpNodeId, NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST, TRANSFER_CRC_SIZE,
};
use canadensis_core::Priority;
use core::mem;
use crc_any::CRCu32;
use std::iter::Peekable;
use std::net::SocketAddrV4;
use zerocopy::AsBytes;

/// An iterator that breaks a transfer into UDP frames and adds a CRC to each frame
pub(crate) struct Breakdown<P: Iterator<Item = u8>, I> {
    /// The destination address for all frames
    dest_address: SocketAddrV4,
    /// Source node ID, or None if anonymous
    source_node: Option<UdpNodeId>,
    /// Destination node ID, or None if message
    // TODO: Combine this with DataSpecifier
    destination_node: Option<UdpNodeId>,
    data_specifier: DataSpecifier,
    /// Vendor-specific data to put in every header
    data: u16,
    /// The transmit deadline for this transfer
    deadline: I,
    /// The ID of this transfer
    transfer_id: u64,
    /// The priority of this transfer
    priority: Priority,
    /// The payload iterator
    payload: Peekable<P>,
    /// The index of the frame currently being assembled
    frame_index: u32,
    /// If the last frame has already been produced
    done: bool,
    /// A transfer CRC that has processed the data in all packets produced so far
    transfer_crc: CRCu32,
    /// The payload in the frame currently being assembled
    ///
    /// Before the frame is returned, the first header::SIZE bytes are empty. The header and CRC
    /// are filled in when the frame is full.
    current_frame: Vec<u8>,
    mtu: usize,
}

impl<P: Iterator<Item = u8>, I: Clone> Breakdown<P, I> {
    pub fn new(
        dest_address: SocketAddrV4,
        source_node: Option<UdpNodeId>,
        destination_node: Option<UdpNodeId>,
        data_specifier: DataSpecifier,
        data: u16,
        deadline: I,
        transfer_id: u64,
        priority: Priority,
        payload: P,
        mtu: usize,
    ) -> Self {
        Breakdown {
            dest_address,
            source_node,
            destination_node,
            data_specifier,
            data,
            deadline,
            transfer_id,
            priority,
            payload: payload.peekable(),
            frame_index: 0,
            done: false,
            transfer_crc: data_crc(),
            // Initialize the current frame with empty space for the header. The payload will follow.
            current_frame: {
                let mut frame: Vec<u8> = Vec::with_capacity(mtu - TRANSFER_CRC_SIZE);
                frame.extend_from_slice(&[0; header::SIZE]);
                frame
            },
            mtu,
        }
    }

    /// Fills in self.current_frame with the provided header and CRC, clears self.current_frame,
    /// and returns a frame containing those bytes
    ///
    /// This function also re-initializes self.current_frame with header::SIZE zero bytes
    /// so that payload bytes can be added.
    fn take_frame(&mut self, header: &UdpHeader, crc: u32) -> UdpFrame<I> {
        // Copy the header into the current frame
        self.current_frame[..header::SIZE].copy_from_slice(header.as_bytes());
        // Add CRC
        self.current_frame.extend_from_slice(&crc.to_le_bytes());
        let frame = UdpFrame {
            deadline: self.deadline.clone(),
            remote_address: self.dest_address,
            data: mem::take(&mut self.current_frame),
        };
        // Add space in the new current frame for the header
        self.current_frame
            .reserve_exact(self.mtu - TRANSFER_CRC_SIZE);
        self.current_frame.extend_from_slice(&[0; header::SIZE]);
        frame
    }

    /// Generates and returns a Cyphal/UDP header, including the CRC
    fn make_header(&self, last_frame: bool) -> UdpHeader {
        let last_frame_flag = if last_frame { LAST_FRAME } else { 0 };

        let mut header = UdpHeader {
            version: header::VERSION,
            priority: self.priority.into(),
            source_node_id: self
                .source_node
                .clone()
                .map(|node| u16::from(node))
                .unwrap_or(NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST),
            destination_node_id: self
                .destination_node
                .clone()
                .map(|node| u16::from(node))
                .unwrap_or(NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST),
            data_specifier: match self.data_specifier.clone() {
                DataSpecifier::Subject(subject) => subject.into(),
                DataSpecifier::ServiceRequest(service) => {
                    DATA_SPEC_SERVICE_NOT_MESSAGE
                        | DATA_SPEC_REQUEST_NOT_RESPONSE
                        | u16::from(service)
                }
                DataSpecifier::ServiceResponse(service) => {
                    DATA_SPEC_SERVICE_NOT_MESSAGE | u16::from(service)
                }
            },
            transfer_id: self.transfer_id,
            frame_index_eot: self.frame_index | last_frame_flag,
            data: self.data,
            header_checksum: 0,
        };
        // Calculate CRC for the header, excluding the CRC field
        header.header_checksum = {
            let bytes: &[u8] = header.as_bytes();
            let mut crc = header_crc();
            crc.digest(&bytes[..bytes.len() - 2]);
            crc.get_crc()
        };
        debug_assert!(header.checksum_valid());
        header
    }
}

impl<P, I> Iterator for Breakdown<P, I>
where
    P: Iterator<Item = u8>,
    I: Clone,
{
    type Item = UdpFrame<I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        loop {
            match self.payload.next() {
                Some(byte) => {
                    self.current_frame.push(byte);
                    self.transfer_crc.digest(&[byte]);

                    if self.current_frame.len() == self.current_frame.capacity() {
                        let more_payload_coming = self.payload.peek().is_some();
                        let header = self.make_header(!more_payload_coming);

                        // This is not the last frame, so calculate the CRC over the data in this
                        // frame only.
                        // The CRC hasn't been added yet, so go all the way to the end.
                        let data_crc = {
                            let mut crc = data_crc();
                            crc.digest(&self.current_frame[header::SIZE..self.current_frame.len()]);
                            crc.get_crc()
                        };

                        let frame = self.take_frame(&header, data_crc);
                        self.frame_index += 1;
                        assert_eq!(self.frame_index & LAST_FRAME, 0, "Frame index too large");
                        break Some(frame);
                    }
                }
                None => {
                    if self.current_frame.len() != header::SIZE {
                        // End of data, return a frame with the last frame bit set
                        // and with a CRC covering all the data
                        let header = self.make_header(true);
                        let transfer_crc = self.transfer_crc.get_crc();
                        let frame = self.take_frame(&header, transfer_crc);
                        self.done = true;
                        break Some(frame);
                    } else {
                        // No data in the current frame
                        break None;
                    }
                }
            }
        }
    }
}
