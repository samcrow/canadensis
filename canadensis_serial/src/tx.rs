use alloc::vec::Vec;
use core::marker::PhantomData;

use fallible_collections::FallibleVec;
use heapless::Deque;
use zerocopy::AsBytes;

use canadensis_core::time::Clock;
use canadensis_core::transfer::Transfer;
use canadensis_core::transport::Transmitter;
use canadensis_core::{nb, OutOfMemoryError};
use canadensis_header::{Header, RawHeader};

use crate::driver::TransmitDriver;
use crate::SerialTransport;
use crate::{cobs, Error};

/// Number of bytes added for each frame for the header and payload CRC
const PER_FRAME_ESCAPED_OVERHEAD: usize = canadensis_header::SIZE + 4;
/// Number of non-escaped bytes added for each frame for the start and end delimiters
const PER_FRAME_UNESCAPED_OVERHEAD: usize = 1 + 1;
/// The frame delimiter character
const DELIMITER: u8 = 0x0;

/// A transmitter for the Cyphal/serial transport
///
/// C is the size of the transmit queue in bytes
pub struct SerialTransmitter<D, const C: usize> {
    /// Queue of outgoing bytes
    queue: TransmitQueue<C>,
    _driver: PhantomData<D>,
}

impl<D, const C: usize> SerialTransmitter<D, C> {
    pub fn new() -> Self {
        SerialTransmitter {
            queue: TransmitQueue::new(),
            _driver: PhantomData,
        }
    }
}

impl<D, const C: usize> Default for SerialTransmitter<D, C> {
    fn default() -> Self {
        SerialTransmitter::new()
    }
}

impl<L, D, const C: usize> Transmitter<L> for SerialTransmitter<D, C>
where
    L: Clock,
    D: TransmitDriver,
{
    type Transport = SerialTransport;
    type Driver = D;
    type Error = Error<D::Error>;

    fn push<A>(
        &mut self,
        transfer: Transfer<A, Self::Transport>,
        _clock: &mut L,
        _driver: &mut D,
    ) -> nb::Result<(), Self::Error>
    where
        A: AsRef<[u8]>,
    {
        // Check queue capacity with worst-case escaping
        let frame_length = transfer.payload.as_ref().len() + PER_FRAME_ESCAPED_OVERHEAD;
        let escaped_length = cobs::escaped_size(frame_length);
        let length_on_wire = escaped_length + PER_FRAME_UNESCAPED_OVERHEAD;

        if length_on_wire > (self.queue.capacity() - self.queue.len()) {
            return Err(nb::Error::Other(Error::Memory(OutOfMemoryError)));
        }
        let header = RawHeader::from(Header::from(transfer.header));
        let payload_crc = crate::make_payload_crc(transfer.payload.as_ref());
        // Escape the header, payload, and payload CRC into a temporary buffer
        let mut escape_buffer: Vec<u8> = FallibleVec::try_with_capacity(escaped_length)
            .map_err(|e| Error::Memory(OutOfMemoryError::from(e)))?;
        escape_buffer.resize(escaped_length, 0);

        let data_to_escape = header
            .as_bytes()
            .iter()
            .copied()
            .chain(transfer.payload.as_ref().iter().copied())
            .chain(payload_crc.as_bytes().iter().copied());
        let escaped_length = cobs::escape_from_iter(data_to_escape, &mut escape_buffer)
            .expect("Incorrect escaped length");
        // Calculate the required queue capacity based on the real escaped length
        let length_on_wire = escaped_length + PER_FRAME_UNESCAPED_OVERHEAD;
        if length_on_wire > (self.queue.capacity() - self.queue.len()) {
            return Err(nb::Error::Other(Error::Memory(OutOfMemoryError)));
        }

        // Put in the queue: delimiter, escaped data, delimiter
        self.queue.push_back(DELIMITER).unwrap();
        for &byte in &escape_buffer[..escaped_length] {
            self.queue.push_back(byte).unwrap();
        }
        self.queue.push_back(DELIMITER).unwrap();

        Ok(())
    }

    fn flush(&mut self, _clock: &mut L, driver: &mut D) -> nb::Result<(), Self::Error> {
        while let Some(byte) = self.queue.pop_front() {
            match driver.send_byte(byte) {
                Ok(()) => {}
                Err(e) => {
                    // Put the byte back to send later
                    // Because we just removed this byte, there must be space to put it back.
                    self.queue
                        .push_front(byte)
                        .expect("No space to return byte to queue");
                    return match e {
                        nb::Error::WouldBlock => Err(nb::Error::WouldBlock),
                        nb::Error::Other(e) => Err(nb::Error::Other(Error::Driver(e))),
                    };
                }
            }
        }
        Ok(())
    }

    fn mtu(&self) -> usize {
        // Virtually unlimited
        usize::MAX
    }
}

/// A queue of bytes to be transmitted
struct TransmitQueue<const C: usize>(Deque<u8, C>);

impl<const C: usize> TransmitQueue<C> {
    fn new() -> Self {
        TransmitQueue(Deque::new())
    }

    fn push_back(&mut self, item: u8) -> Result<(), OutOfMemoryError> {
        self.0.push_back(item).map_err(|_| OutOfMemoryError)
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the number of bytes in this queue
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Removes the byte from the front of the queue
    pub fn pop_front(&mut self) -> Option<u8> {
        self.0.pop_front()
    }

    pub fn push_front(&mut self, item: u8) -> Result<(), OutOfMemoryError> {
        self.0.push_front(item).map_err(|_| OutOfMemoryError)
    }
}
