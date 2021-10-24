use crate::header::{self, UdpHeader};
use crate::tx::UdpFrame;
use canadensis_core::Priority;
use core::mem;
use std::net::SocketAddrV4;
use zerocopy::AsBytes;

/// An iterator that breaks a transfer into UDP frames
///
/// If the payload requires more than one frame, it should already have a transfer CRC.
pub(crate) struct Breakdown<P, I, const MTU: usize> {
    /// The destination address for all frames
    dest_address: SocketAddrV4,
    /// The transmit deadline for this transfer
    deadline: I,
    /// The ID of this transfer
    transfer_id: u64,
    /// The priority of this transfer
    priority: Priority,
    /// The payload iterator
    payload: P,
    /// The index of the frame currently being assembled
    frame_index: u32,
    /// If the last frame has already been produced
    done: bool,
    /// The payload in the frame currently being assembled
    ///
    /// Before the frame is returned, the first header::SIZE bytes are empty. The header is filled
    /// in when the frame is full.
    current_frame: heapless::Vec<u8, MTU>,
}

impl<P, I, const MTU: usize> Breakdown<P, I, MTU>
where
    I: Clone,
{
    pub fn new(
        dest_address: SocketAddrV4,
        deadline: I,
        transfer_id: u64,
        priority: Priority,
        payload: P,
    ) -> Self {
        Breakdown {
            dest_address,
            deadline,
            transfer_id,
            priority,
            payload,
            frame_index: 0,
            done: false,
            // Initialize the current frame with empty space for the header. The payload will follow.
            current_frame: heapless::Vec::from_slice(&[0; header::SIZE]).unwrap(),
        }
    }

    /// Fills in self.current_frame with the provided header, clears self.current_frame, and returns
    /// a frame containing those bytes
    ///
    /// This function also re-initializes self.current_frame with header::SIZE zero bytes
    /// so that payload bytes can be added.
    fn take_frame(&mut self, header: &UdpHeader) -> UdpFrame<I, MTU> {
        // Copy the header into the current frame
        self.current_frame[..header::SIZE].copy_from_slice(header.as_bytes());
        let frame = UdpFrame {
            deadline: self.deadline.clone(),
            remote_address: self.dest_address,
            data: mem::take(&mut self.current_frame),
        };
        // Add space in the new current frame for the header
        self.current_frame
            .extend_from_slice(&[0; header::SIZE])
            .unwrap();
        frame
    }
}

impl<P, I, const MTU: usize> Iterator for Breakdown<P, I, MTU>
where
    P: Iterator<Item = u8>,
    I: Clone,
{
    type Item = UdpFrame<I, MTU>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        loop {
            match self.payload.next() {
                Some(byte) => {
                    self.current_frame.push(byte).unwrap();
                    if self.current_frame.is_full() {
                        let header = UdpHeader {
                            version: header::VERSION,
                            priority: self.priority.into(),
                            _padding0: header::PADDING0,
                            frame_index_eot: self.frame_index,
                            transfer_id: self.transfer_id,
                            _padding1: 0,
                        };
                        let frame = self.take_frame(&header);
                        self.frame_index += 1;
                        break Some(frame);
                    }
                }
                None => {
                    // End of data, return a frame with the last frame bit set
                    let header = UdpHeader {
                        version: header::VERSION,
                        priority: self.priority.into(),
                        _padding0: header::PADDING0,
                        frame_index_eot: header::LAST_FRAME | self.frame_index,
                        transfer_id: self.transfer_id,
                        _padding1: 0,
                    };
                    let frame = self.take_frame(&header);
                    self.done = true;
                    break Some(frame);
                }
            }
        }
    }
}

/// Returns true if a payload with the provided size can fit into a single frame
pub fn fits_into_one_frame<const MTU: usize>(payload_bytes: usize) -> bool {
    payload_bytes + header::SIZE <= MTU
}
