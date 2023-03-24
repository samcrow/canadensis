use canadensis_header::RawHeader;
use zerocopy::FromBytes;

const HEADER_BYTES: usize = canadensis_header::SIZE;

pub struct HeaderCollector {
    /// The bytes that represent the header
    ///
    /// Note: This doesn't have to be aligned to use `FromBytes::read_from`.
    bytes: [u8; HEADER_BYTES],
    /// The number of bytes that have been written
    len: u8,
}

impl HeaderCollector {
    pub fn new() -> Self {
        HeaderCollector {
            bytes: [0; HEADER_BYTES],
            len: 0,
        }
    }
    /// Returns true if a full header has been collected
    pub fn is_done(&self) -> bool {
        usize::from(self.len) == HEADER_BYTES
    }
    /// Appends a byte to the header
    ///
    /// # Panics
    ///
    /// This function panics if all the bytes required for a header have already been collected.
    pub fn push(&mut self, byte: u8) {
        self.bytes[usize::from(self.len)] = byte;
        self.len += 1;
    }

    /// Interprets the bytes as a header
    pub fn as_header(&self) -> RawHeader {
        RawHeader::read_from(&self.bytes[..]).expect("Incorrect byte length or alignment")
    }
}
