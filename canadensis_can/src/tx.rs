//!
//! Cyphal message transmission
//!

use core::convert::TryFrom;
use core::iter;
use core::marker::PhantomData;

use canadensis_core::nb;
use canadensis_core::time::{Clock, Microseconds32};
use canadensis_core::transfer::{Header, ServiceHeader, Transfer};
use canadensis_core::transport::Transmitter;

use crate::crc::TransferCrc;
use crate::data::Frame;
use crate::driver::TransmitDriver;
use crate::tx::breakdown::Breakdown;
use crate::types::{CanNodeId, CanTransport, Error};
use crate::{CanId, Mtu};

mod breakdown;
#[cfg(test)]
mod tx_test;

/// Splits outgoing transfers into frames
pub struct CanTransmitter<C, D> {
    /// Transport MTU (including the tail byte)
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
    _clock: PhantomData<C>,
    _driver: PhantomData<D>,
}

impl<C, D> Transmitter<C> for CanTransmitter<C, D>
where
    C: Clock,
    D: TransmitDriver<C>,
{
    type Transport = CanTransport;
    type Driver = D;
    type Error = Error<D::Error>;

    /// Breaks a transfer into frames
    ///
    /// The frames can be retrieved and sent using the peek() and pop() functions.
    ///
    /// This function returns an error if the queue does not have enough space to hold all
    /// the required frames.
    fn push<A>(
        &mut self,
        transfer: Transfer<A, CanTransport>,
        clock: &mut C,
        driver: &mut D,
    ) -> nb::Result<(), Self::Error>
    where
        A: AsRef<[u8]>,
    {
        // Convert the transfer payload into borrowed form
        let transfer = Transfer {
            header: transfer.header,
            loopback: transfer.loopback,
            payload: transfer.payload.as_ref(),
        };

        match self.push_inner(transfer, clock, driver) {
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

    fn flush(&mut self, clock: &mut C, driver: &mut D) -> nb::Result<(), Self::Error> {
        driver.flush(clock).map_err(|e| e.map(Error::Driver))
    }

    fn mtu(&self) -> usize {
        // Subtract 1 for the tail byte
        self.mtu - 1
    }
}

impl<C, D> CanTransmitter<C, D>
where
    C: Clock,
    D: TransmitDriver<C>,
{
    /// Creates a transmitter
    ///
    /// mtu: The maximum number of bytes in a frame
    pub fn new(mtu: Mtu) -> Self {
        CanTransmitter {
            mtu: mtu as usize,
            transfer_count: 0,
            error_count: 0,
            _clock: PhantomData,
            _driver: PhantomData,
        }
    }

    /// Sets the MTU
    ///
    /// This will take effect on the next call to push().
    pub fn set_mtu(&mut self, mtu: Mtu) {
        self.mtu = mtu as usize;
    }

    fn push_inner(
        &mut self,
        transfer: Transfer<&[u8], CanTransport>,
        clock: &mut C,
        driver: &mut D,
    ) -> nb::Result<(), Error<D::Error>> {
        let frame_stats = crate::calculate_frame_stats(transfer.payload.len(), self.mtu);
        // Check that enough space is available in the queue for all the frames.
        // Return an error if space is not available.
        driver
            .try_reserve(frame_stats.frames)
            .map_err(|oom| nb::Error::Other(Error::Memory(oom)))?;

        // Make an iterator over the payload bytes and padding. Run the CRC on that.
        let mut crc = TransferCrc::new();
        let payload_and_padding = transfer
            .payload
            .iter()
            .cloned()
            .chain(iter::repeat(0).take(frame_stats.last_frame_padding))
            .inspect(|byte| crc.add(*byte));
        // Break into frames
        let can_id = make_can_id(&transfer.header, transfer.payload);
        let mut breakdown = Breakdown::new(self.mtu, *transfer.header.transfer_id());
        let mut frames = 0;
        // Do the non-last frames
        for byte in payload_and_padding {
            if let Some(frame_data) = breakdown.add(byte) {
                // Filled up a frame
                self.push_frame(
                    transfer.header.timestamp(),
                    transfer.loopback,
                    can_id,
                    &frame_data,
                    driver,
                    clock,
                )
                .map_err(|e| e.map(Error::Driver))?;
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
                    self.push_frame(
                        transfer.header.timestamp(),
                        transfer.loopback,
                        can_id,
                        &frame_data,
                        driver,
                        clock,
                    )
                    .map_err(|e| e.map(Error::Driver))?;
                }
            }
        }
        let last_frame_data = breakdown.finish();
        self.push_frame(
            transfer.header.timestamp(),
            transfer.loopback,
            can_id,
            &last_frame_data,
            driver,
            clock,
        )
        .map_err(|e| e.map(Error::Driver))?;
        Ok(())
    }

    /// Creates a frame and sends it to the driver to be transmitted
    ///
    /// If the driver returns a removed lower-priority frame, this function discards it.
    fn push_frame(
        &mut self,
        timestamp: Microseconds32,
        loopback: bool,
        id: CanId,
        data: &[u8],
        driver: &mut D,
        clock: &mut C,
    ) -> nb::Result<(), D::Error> {
        let mut frame = Frame::new(timestamp, id, data);
        frame.set_loopback(loopback);
        // If a lower-priority frame was removed, drop it
        driver.transmit(frame, clock).map(drop)
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

fn make_can_id(header: &Header<CanTransport>, payload: &[u8]) -> CanId {
    let mut bits = 0u32;

    // Common fields for all transfer types
    bits |= (*header.priority() as u32) << 26;
    let source_node = header
        .source()
        .cloned()
        .unwrap_or_else(|| make_pseudo_id(payload));
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
fn encode_common_service_fields(header: &ServiceHeader<CanTransport>) -> u32 {
    // Service ID
    (u32::from(u16::from(header.service)) << 14)
        // Destination node ID
        | (u32::from(u8::from(header.destination)) << 7)
        // Set bit 25 to indicate service
        | (1 << 25)
}

/// Generates a non-reserved node pseudo-ID based on the provided transfer payload
fn make_pseudo_id(payload: &[u8]) -> CanNodeId {
    // Just XOR the payload
    let bits = payload
        .iter()
        .fold(0x55u8, |state, payload_byte| state ^ *payload_byte);
    let mut id = CanNodeId::from_truncating(bits);
    while id.is_diagnostic_reserved() {
        id = CanNodeId::from_truncating(u8::from(id) - 1);
    }
    id
}
