//!
//! UAVCAN transmission
//!

mod breakdown;
#[cfg(test)]
mod tx_test;

use crate::crc::TransferCrc;
use crate::data::Frame;
use crate::error::OutOfMemoryError;
use crate::queue::FrameSink;
use crate::tx::breakdown::Breakdown;
use crate::{CanId, Mtu};
use canadensis_core::transfer::{Header, ServiceHeader, Transfer};
use canadensis_core::NodeId;
use core::convert::TryFrom;
use core::{cmp, iter};

/// Splits outgoing transfers into frames
pub struct Transmitter<Q> {
    /// Queue of frames waiting to be sent
    frame_queue: Q,
    /// Transport MTU
    mtu: usize,
    /// Number of transfers successfully transmitted
    ///
    /// Success means that the frames were placed into the frame queue successfully. CAN bus errors
    /// are ignored.
    transfer_count: u64,
    /// Number of transfers that could not be transmitted
    ///
    /// A failure to allocate memory is considered an error. CAN bus errors are ignored.
    error_count: u64,
}

impl<Q> Transmitter<Q> {
    /// Creates a transmitter
    ///
    /// mtu: The maximum number of bytes in a frame
    pub fn new(mtu: Mtu, frame_queue: Q) -> Self {
        Transmitter {
            frame_queue,
            mtu: mtu as usize,
            transfer_count: 0,
            error_count: 0,
        }
    }

    /// Sets the MTU
    ///
    /// This will take effect on the next call to push().
    pub fn set_mtu(&mut self, mtu: Mtu) {
        self.mtu = mtu as usize;
    }

    /// Breaks a transfer into frames
    ///
    /// The frames can be retrieved and sent using the peek() and pop() functions.
    ///
    /// This function returns an error if the queue does not have enough space to hold all
    /// the required frames.
    pub fn push<P, I>(&mut self, transfer: Transfer<P, I>) -> Result<(), OutOfMemoryError>
    where
        P: AsRef<[u8]>,
        Q: FrameSink<I>,
        I: Clone,
    {
        // Convert the transfer payload into borrowed form
        let transfer = Transfer {
            header: transfer.header,
            payload: transfer.payload.as_ref(),
        };

        match self.push_inner(transfer) {
            Ok(()) => {
                self.transfer_count = self.transfer_count.wrapping_add(1);
                Ok(())
            }
            Err(e) => {
                self.error_count = self.error_count.wrapping_add(1);
                Err(e)
            }
        }
    }

    fn push_inner<I>(&mut self, transfer: Transfer<&[u8], I>) -> Result<(), OutOfMemoryError>
    where
        Q: FrameSink<I>,
        I: Clone,
    {
        let frame_stats = calculate_frame_stats(transfer.payload.len(), self.mtu);
        // Check that enough space is available in the queue for all the frames.
        // Return an error if space is not available.
        self.frame_queue.try_reserve(frame_stats.frames)?;

        // Make an iterator over the payload bytes and padding. Run the CRC on that.
        let mut crc = TransferCrc::new();
        let payload_and_padding = transfer
            .payload
            .iter()
            .cloned()
            .chain(iter::repeat(0).take(frame_stats.last_frame_padding))
            .inspect(|byte| crc.add(*byte));
        // Break into frames
        let can_id = make_can_id(&transfer.header, &transfer.payload);
        let mut breakdown = Breakdown::new(self.mtu, transfer.header.transfer_id());
        let mut frames = 0;
        // Do the non-last frames
        for byte in payload_and_padding {
            if let Some(frame_data) = breakdown.add(byte) {
                // Filled up a frame
                self.push_frame(transfer.header.timestamp(), can_id, &frame_data)?;
                frames += 1;
            }
        }
        if frames != 0 {
            // The payload + padding was split across at least one non-last frame (handled above)
            // and the last frame (still in the Breakdown). It needs a CRC.
            let crc_value = crc.get();
            // Add the CRC value, most significant byte first
            let crc_bytes = [(crc_value >> 8) as u8, crc_value as u8];
            for &byte in crc_bytes.iter() {
                if let Some(frame_data) = breakdown.add(byte) {
                    // Filled up a frame
                    self.push_frame(transfer.header.timestamp(), can_id, &frame_data)?;
                }
            }
        }
        let last_frame_data = breakdown.finish();
        self.push_frame(transfer.header.timestamp(), can_id, &last_frame_data)?;
        Ok(())
    }

    /// Creates a frame and adds it to a transaction
    fn push_frame<I>(
        &mut self,
        timestamp: I,
        id: CanId,
        data: &[u8],
    ) -> core::result::Result<(), OutOfMemoryError>
    where
        Q: FrameSink<I>,
        I: Clone,
    {
        let frame = Frame::new(timestamp, id, data);
        self.frame_queue.push_frame(frame)
    }

    /// Returns a reference to the frame queue, where outgoing frames are stored
    pub fn frame_queue(&self) -> &Q {
        &self.frame_queue
    }

    /// Returns a mutable reference to the frame queue, where outgoing frames are stored
    pub fn frame_queue_mut(&mut self) -> &mut Q {
        &mut self.frame_queue
    }

    /// Returns the number of transfers successfully transmitted
    ///
    /// Success means that the frames were placed into the frame queue successfully. CAN bus errors
    ///  are ignored.
    #[inline]
    pub fn transfer_count(&self) -> u64 {
        self.transfer_count
    }

    /// Returns the number of transfers that could not be transmitted
    ///
    /// A failure to allocate memory is considered an error. CAN bus errors are ignored.
    #[inline]
    pub fn error_count(&self) -> u64 {
        self.error_count
    }
}

/// Calculates the number of padding bytes to add to a payload so that all frames will have valid
/// length values for CAN FD
fn calculate_frame_stats(payload_length: usize, mtu: usize) -> FrameStats {
    assert!(mtu <= 64, "MTU too large for CAN FD");
    assert!(mtu > 1, "MTU too small");
    let mtu_without_tail = mtu - 1;

    // Calculate the length of the payload, CRC, and tail bytes
    let crc_length = if payload_length <= mtu_without_tail {
        // Fits into one frame, no need to add a transfer CRC
        0
    } else {
        // Add two bytes for the transfer CRC
        2
    };
    // Total length of all tail bytes
    // Divide and round up (minimum 1 tail byte)
    let tail_bytes = cmp::max(
        1,
        (payload_length + crc_length + (mtu_without_tail - 1)) / mtu_without_tail,
    );
    // Total length of the payloads of all frames, including CRC and tail bytes
    let total_length = payload_length + crc_length + tail_bytes;
    let frames = (total_length + mtu - 1) / mtu;

    // Get the number of bytes in the last frame (may be 0)
    let last_frame_length = total_length % mtu;
    let last_frame_rounded_length = round_up_frame_length(last_frame_length);
    let last_frame_padding = last_frame_rounded_length - last_frame_length;

    FrameStats {
        frames,
        last_frame_padding,
    }
}

/// Information about how to fit a transfer payload into frames
#[derive(Debug, Eq, PartialEq)]
struct FrameStats {
    /// The total number of frames
    frames: usize,
    /// The number of bytes that must be added to the last frame to give it a valid length
    /// for CAN FD
    last_frame_padding: usize,
}

/// Rounds up a frame length to a value that can be represented by a CAN FD data length code
fn round_up_frame_length(length: usize) -> usize {
    match length {
        0..=8 => length,
        9..=12 => 12,
        13..=16 => 16,
        17..=20 => 20,
        21..=24 => 24,
        25..=32 => 32,
        33..=48 => 48,
        49..=64 => 64,
        _ => panic!("MTU too large for CAN FD"),
    }
}

fn make_can_id<I>(header: &Header<I>, payload: &[u8]) -> CanId {
    let mut bits = 0u32;

    // Common fields for all transfer types
    bits |= (header.priority() as u32) << 26;
    let source_node = header.source().unwrap_or_else(|| make_pseudo_id(payload));
    bits |= u32::from(source_node);

    match header {
        Header::Message(message_header) => {
            // Subject ID
            bits |= u32::from(u16::from(message_header.subject)) << 8;
            // Set bits 21 and 22
            bits |= (1 << 21) | (1 << 22);
            // Anonymous
            if message_header.source.is_none() {
                bits |= 1 << 24;
            }
        }
        Header::Request(service_header) => {
            bits |= encode_common_service_fields(service_header);
            // Set bit 24 to indicate request
            bits |= 1 << 24;
        }
        Header::Response(service_header) => {
            bits |= encode_common_service_fields(service_header);
            // Leave bit 24 clear
        }
    }

    CanId::try_from(bits).expect("Generated CAN ID does not fit into 29 bits")
}

/// Encodes the service ID, destination ID, and service flag into a 29-bit CAN ID, and returns
/// it
fn encode_common_service_fields<I>(header: &ServiceHeader<I>) -> u32 {
    // Service ID
    (u32::from(u16::from(header.service)) << 14)
        // Destination node ID
        | (u32::from(u8::from(header.destination)) << 7)
        // Set bit 25 to indicate service
        | (1 << 25)
}

/// Generates a non-reserved node pseudo-ID based on the provided transfer payload
fn make_pseudo_id(payload: &[u8]) -> NodeId {
    // Just XOR the payload
    let bits = payload
        .iter()
        .fold(0x55u8, |state, payload_byte| state ^ *payload_byte);
    let mut id = NodeId::from_truncating(bits);
    while id.is_diagnostic_reserved() {
        id = NodeId::from_truncating(u8::from(id) - 1);
    }
    id
}
