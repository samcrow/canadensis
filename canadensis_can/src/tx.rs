//!
//! UAVCAN transmission
//!

mod breakdown;
#[cfg(test)]
mod tx_test;

use crate::crc::TransferCrc;
use crate::data::Frame;
use crate::error::OutOfMemoryError;
use crate::heap::{Heap, Transaction};
use crate::tx::breakdown::Breakdown;
use crate::{CanId, FrameById, Mtu};
use canadensis_core::transfer::{ServiceHeader, Transfer, TransferHeader, TransferKindHeader};
use core::convert::TryFrom;
use core::iter;
use fallible_collections::TryReserveError;

/// Splits outgoing transfers into frames
pub struct Transmitter<I> {
    /// Queue of frames waiting to be sent
    frame_queue: Heap<FrameById<I>>,
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

impl<I: Clone> Transmitter<I> {
    /// Creates a transmitter
    ///
    /// mtu: The maximum number of bytes in a frame
    pub fn new(mtu: Mtu) -> Self {
        Transmitter {
            frame_queue: Heap::new(),
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
    pub fn push<P>(&mut self, transfer: Transfer<P, I>) -> Result<(), OutOfMemoryError>
    where
        P: AsRef<[u8]>,
    {
        // Convert the transfer payload into borrowed form
        let transfer = Transfer {
            timestamp: transfer.timestamp,
            header: transfer.header,
            transfer_id: transfer.transfer_id,
            payload: transfer.payload.as_ref(),
        };

        // Use a heap transaction to prevent having some frames left over in the queue after
        // running out of memory
        let mut transaction = self.frame_queue.transaction();
        match Self::try_push(&mut transaction, transfer, self.mtu) {
            Ok(()) => {
                transaction.commit();
                self.transfer_count = self.transfer_count.wrapping_add(1);
                Ok(())
            }
            Err(_) => {
                transaction.rollback();
                // Try to reduce memory pressure by shrinking the queue
                self.frame_queue.shrink_to_fit();
                self.error_count = self.error_count.wrapping_add(1);
                Err(OutOfMemoryError(()))
            }
        }
    }

    /// Breaks a transfer into frames and stores the frames
    ///
    /// If an out-of-memory condition occurs, this function returns an error. There may be frames
    /// remaining in the transaction that need to be cleaned up.
    fn try_push(
        transaction: &mut Transaction<'_, FrameById<I>>,
        transfer: Transfer<&'_ [u8], I>,
        mtu: usize,
    ) -> Result<(), OutOfMemoryError> {
        let padding = calculate_padding(transfer.payload.len(), mtu);

        // Make an iterator over the payload bytes and padding. Run the CRC on that.
        let mut crc = TransferCrc::new();
        let payload_and_padding = transfer
            .payload
            .iter()
            .cloned()
            .chain(iter::repeat(0).take(padding))
            .inspect(|byte| crc.add(*byte));
        // Break into frames
        let can_id = make_can_id(transfer.header);
        let mut breakdown = Breakdown::new(mtu, transfer.transfer_id);
        let mut frames = 0;
        // Do the non-last frames
        for byte in payload_and_padding {
            if let Some(frame_data) = breakdown.add(byte) {
                // Filled up a frame
                Self::push_frame(transaction, transfer.timestamp.clone(), can_id, &frame_data)?;
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
                    Self::push_frame(transaction, transfer.timestamp.clone(), can_id, &frame_data)?;
                }
            }
        }
        let last_frame_data = breakdown.finish();
        Self::push_frame(transaction, transfer.timestamp, can_id, &last_frame_data)?;
        Ok(())
    }

    /// Creates a frame and adds it to a transaction
    fn push_frame(
        transaction: &mut Transaction<'_, FrameById<I>>,
        timestamp: I,
        id: CanId,
        data: &[u8],
    ) -> core::result::Result<(), TryReserveError> {
        let frame = Frame::new(timestamp, id, data);
        transaction.push(FrameById(frame))
    }

    /// Returns a reference to the next frame waiting to be sent, if any exists
    pub fn peek(&self) -> Option<&Frame<I>> {
        self.frame_queue
            .peek()
            .map(|compare_wrapper| &compare_wrapper.0)
    }

    /// Removes and returns the next frame waiting to be sent, if any exists
    pub fn pop(&mut self) -> Option<Frame<I>> {
        self.frame_queue
            .pop()
            .map(|compare_wrapper| compare_wrapper.0)
    }

    /// Returns a frame that has not been sent and queues it to be sent later
    pub fn return_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        let mut transaction = self.frame_queue.transaction();
        transaction.push(FrameById(frame))?;
        transaction.commit();
        Ok(())
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
fn calculate_padding(payload_length: usize, mtu: usize) -> usize {
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
    // Divide and round up
    let tail_bytes = (payload_length + crc_length + (mtu_without_tail - 1)) / mtu_without_tail;
    let total_length = payload_length + crc_length + tail_bytes;

    // Get the number of bytes in the last frame (may be 0)
    let last_frame_length = total_length % mtu;
    let last_frame_rounded_length = round_up_frame_length(last_frame_length);
    last_frame_rounded_length - last_frame_length
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

fn make_can_id(header: TransferHeader) -> CanId {
    let mut bits = 0u32;

    // Common fields for all transfer types
    bits |= (header.priority as u32) << 26;
    bits |= u32::from(u8::from(header.source));

    match header.kind {
        TransferKindHeader::Message(header) => {
            // Subject ID
            bits |= u32::from(u16::from(header.subject)) << 8;
            // Set bits 21 and 22
            bits |= (1 << 21) | (1 << 22);
            // Anonymous
            if header.anonymous {
                bits |= 1 << 24;
            }
        }
        TransferKindHeader::Request(header) => {
            bits |= encode_common_service_fields(header);
            // Set bit 24 to indicate request
            bits |= 1 << 24;
        }
        TransferKindHeader::Response(header) => {
            bits |= encode_common_service_fields(header);
            // Leave bit 24 clear
        }
    }

    CanId::try_from(bits).expect("Generated CAN ID does not fit into 29 bits")
}

/// Encodes the service ID, destination ID, and service flag into a 29-bit CAN ID, and returns
/// it
fn encode_common_service_fields(header: ServiceHeader) -> u32 {
    // Service ID
    (u32::from(u16::from(header.service)) << 14)
        // Destination node ID
        | (u32::from(u8::from(header.destination)) << 7)
        // Set bit 25 to indicate service
        | (1 << 25)
}
