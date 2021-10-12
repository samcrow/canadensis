use core::mem;

use fallible_collections::FallibleVec;
use heapless::Deque;
use zerocopy::AsBytes;

use canadensis_core::time::Instant;
use canadensis_core::transfer::Transfer;
use canadensis_core::transport::{Transmitter, Transport};
use canadensis_core::OutOfMemoryError;

use crate::cobs;
use crate::header::SerialHeader;
use crate::SerialTransport;

/// Number of bytes added for each frame for the header and payload CRC
const PER_FRAME_ESCAPED_OVERHEAD: usize = mem::size_of::<SerialHeader>() + 4;
/// Number of non-escaped bytes added for each frame for the start and end delimiters
const PER_FRAME_UNESCAPED_OVERHEAD: usize = 1 + 1;
/// The frame delimiter character
const DELIMITER: u8 = 0x0;

pub struct SerialTransmitter<const C: usize> {
    /// Queue of outgoing bytes
    queue: TransmitQueue<C>,
}

impl<const C: usize> SerialTransmitter<C> {
    pub fn new() -> Self {
        SerialTransmitter {
            queue: TransmitQueue::new(),
        }
    }

    pub fn queue(&self) -> &TransmitQueue<C> {
        &self.queue
    }
    pub fn queue_mut(&mut self) -> &mut TransmitQueue<C> {
        &mut self.queue
    }
}

impl<I, const C: usize> Transmitter<I> for SerialTransmitter<C>
where
    I: Instant,
{
    type Transport = SerialTransport;

    fn push<A>(
        &mut self,
        transfer: Transfer<A, I, Self::Transport>,
    ) -> Result<(), <Self::Transport as Transport>::Error>
    where
        A: AsRef<[u8]>,
    {
        // Check queue capacity with worst-case escaping
        let frame_length = transfer.payload.as_ref().len() + PER_FRAME_ESCAPED_OVERHEAD;
        let escaped_length = cobs::escaped_size(frame_length);
        let length_on_wire = escaped_length + PER_FRAME_UNESCAPED_OVERHEAD;
        println!("Length on wire {} bytes", length_on_wire);
        if length_on_wire > (self.queue.capacity() - self.queue.len()) {
            return Err(OutOfMemoryError);
        }
        let header = SerialHeader::from(transfer.header);
        let payload_crc = crate::make_payload_crc(transfer.payload.as_ref());
        // Escape the header, payload, and payload CRC into a temporary buffer
        let mut escape_buffer: Vec<u8> = FallibleVec::try_with_capacity(escaped_length)?;
        for _ in 0..escaped_length {
            escape_buffer.push(0);
        }
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
            return Err(OutOfMemoryError);
        }

        // Put in the queue: delimiter, escaped data, delimiter
        self.queue.push_back(DELIMITER).unwrap();
        for &byte in &escape_buffer[..escaped_length] {
            self.queue.push_back(byte).unwrap();
        }
        self.queue.push_back(DELIMITER).unwrap();

        Ok(())
    }

    fn mtu(&self) -> usize {
        // Virtually unlimited
        usize::MAX
    }
}

/// A queue of bytes to be transmitted
pub struct TransmitQueue<const C: usize>(Deque<u8, C>);

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

    /// Returns a reference to the byte at the front of the queue
    pub fn peek_front(&self) -> Option<&u8> {
        self.0.front()
    }
    /// Removes the byte from the front of the queue
    pub fn pop_front(&mut self) -> Option<u8> {
        self.0.pop_front()
    }

    /// Returns an iterator over the bytes in this queue
    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        self.0.iter()
    }
}
