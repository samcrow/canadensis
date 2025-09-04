use alloc::vec::Vec;
use core::iter::Peekable;
use core::mem;

use zerocopy::IntoBytes;

use canadensis_core::crc::Crc32c;
use canadensis_core::time::Microseconds32;
use canadensis_core::Priority;
use canadensis_header::{DataSpecifier, Header, RawHeader, LAST_FRAME};

use crate::tx::UdpFrame;
use crate::UdpTransferId;

/// An iterator that breaks a transfer into UDP frames and adds a CRC to each frame
pub(crate) struct Breakdown<P: Iterator<Item = u8>> {
    /// Basic header information to apply to all frames
    header_base: HeaderBase,
    /// The transmit deadline for this transfer
    deadline: Microseconds32,
    /// The payload and transfer CRC iterator
    payload: Peekable<AddCrc<P>>,
    /// The index of the frame currently being assembled
    frame_index: u32,
    /// If the last frame has already been produced
    done: bool,
    /// The payload in the frame currently being assembled
    ///
    /// Before the frame is returned, the first header::SIZE bytes are empty. The header and CRC
    /// are filled in when the frame is full.
    current_frame: Vec<u8>,
    /// Transport MTU (including the Cyphal header and transfer CRC)
    mtu: usize,
}

/// The parts of a header that are needed to create a Breakdown
pub(crate) struct HeaderBase {
    pub data_specifier: DataSpecifier,
    /// The ID of this transfer
    pub transfer_id: UdpTransferId,
    /// The priority of this transfer
    pub priority: Priority,
    /// Vendor-specific data to put in every header
    pub data: u16,
}

impl<P: Iterator<Item = u8>> Breakdown<P> {
    pub fn new(header_base: HeaderBase, deadline: Microseconds32, payload: P, mtu: usize) -> Self {
        Breakdown {
            header_base,
            deadline,
            payload: AddCrc::new(payload).peekable(),
            frame_index: 0,
            done: false,
            // Initialize the current frame with empty space for the header. The payload will follow.
            current_frame: {
                let mut frame: Vec<u8> = Vec::with_capacity(mtu);
                frame.extend_from_slice(&[0; canadensis_header::SIZE]);
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
    fn take_frame(&mut self, header: Header) -> UdpFrame {
        // Copy the header into the current frame
        self.current_frame[..canadensis_header::SIZE]
            .copy_from_slice(RawHeader::from(header).as_bytes());
        let frame = UdpFrame {
            deadline: self.deadline,
            data: mem::take(&mut self.current_frame),
        };
        // Add space in the new current frame for the header
        self.current_frame.reserve(self.mtu);
        self.current_frame
            .extend_from_slice(&[0; canadensis_header::SIZE]);
        frame
    }

    /// Generates and returns a Cyphal/UDP header, including the CRC
    fn make_header(&self, last_frame: bool) -> Header {
        Header {
            priority: self.header_base.priority,
            data_specifier: self.header_base.data_specifier.clone(),
            transfer_id: self.header_base.transfer_id,
            frame_index: self.frame_index,
            last_frame,
            data: self.header_base.data,
        }
    }
}

impl<P> Iterator for Breakdown<P>
where
    P: Iterator<Item = u8>,
{
    type Item = UdpFrame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        loop {
            match self.payload.next() {
                Some(byte) => {
                    self.current_frame.push(byte);

                    if self.current_frame.len() == self.current_frame.capacity() {
                        let more_payload_coming = self.payload.peek().is_some();
                        let header = self.make_header(!more_payload_coming);
                        let frame = self.take_frame(header);
                        self.frame_index += 1;
                        assert_eq!(self.frame_index & LAST_FRAME, 0, "Frame index too large");
                        break Some(frame);
                    }
                }
                None => {
                    if self.current_frame.len() != canadensis_header::SIZE {
                        // End of data, return a frame with the last frame bit set
                        let header = self.make_header(true);
                        let frame = self.take_frame(header);
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

#[cfg(test)]
mod breakdown_tests {
    use std::convert::TryFrom;
    use std::iter;

    use canadensis_core::time::Microseconds32;
    use canadensis_core::{Priority, ServiceId, SubjectId};

    use crate::tx::UdpFrame;
    use crate::UdpNodeId;
    use canadensis_header::DataSpecifier;

    use super::{Breakdown, HeaderBase};

    #[test]
    fn test_empty() {
        let header_base = HeaderBase {
            data_specifier: DataSpecifier::Subject {
                from: Some(UdpNodeId::try_from(0xde30).unwrap()),
                subject: SubjectId::try_from(8191).unwrap(),
            },
            transfer_id: 0x0102_0304_0506_0708.into(),
            priority: Default::default(),
            data: 0x39fe,
        };
        // One frame with header and CRC only
        let mut breakdown = Breakdown::new(
            header_base,
            Microseconds32::from_ticks(0),
            iter::empty(),
            1472,
        );
        assert_eq!(
            Some(UdpFrame {
                deadline: Microseconds32::from_ticks(0),
                data: vec![
                    0x01, // Version
                    0x04, // Priority
                    0x30, 0xde, // Source node
                    0xff, 0xff, // Destination node
                    0xff, 0x1f, // Subject
                    0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // Transfer ID
                    0x00, 0x00, 0x00, 0x80, // Frame index and last frame flag
                    0xfe, 0x39, // User data
                    0x4b, 0x59, // Header CRC
                    // (No data)
                    0x00, 0x00, 0x00, 0x00, // Transfer CRC
                ]
            }),
            breakdown.next()
        );
    }

    #[test]
    fn test_one_byte_payload() {
        let header_base = HeaderBase {
            data_specifier: DataSpecifier::Subject {
                from: Some(UdpNodeId::try_from(0xde30).unwrap()),
                subject: SubjectId::try_from(8191).unwrap(),
            },
            transfer_id: 0x0102_0304_0506_0708.into(),
            priority: Default::default(),
            data: 0x39fe,
        };
        let payload: [u8; 1] = [0xf2];
        let mut breakdown = Breakdown::new(
            header_base,
            Microseconds32::from_ticks(0),
            IntoIterator::into_iter(payload),
            1472,
        );
        let frame = breakdown.next().expect("No frame");
        assert!(breakdown.next().is_none(), "Extra frame at end");
        let expected_bytes = [
            0x1, // Version
            0x4, // Priority
            0x30, 0xde, // Source node
            0xff, 0xff, // Destination node
            0xff, 0x1f, // Subject
            0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // Transfer ID
            0x00, 0x00, 0x00, 0x80, // Frame index and last frame flag
            0xfe, 0x39, // User data
            0x4b, 0x59, // Header CRC
            0xf2, // Payload
            0xd3, 0x4c, 0x28, 0x40, // Transfer CRC
        ];
        assert_eq!(HexDebug(&frame.data), HexDebug(expected_bytes.as_slice()));
    }
    #[test]
    fn test_one_full_frame() {
        let header_base = HeaderBase {
            data_specifier: DataSpecifier::ServiceRequest {
                from: UdpNodeId::try_from(0xde30).unwrap(),
                to: UdpNodeId::try_from(0x3).unwrap(),
                service: ServiceId::try_from(32).unwrap(),
            },
            transfer_id: 0x1024.into(),
            priority: Priority::Fast,
            data: 0x1021,
        };
        let payload: [u8; 4] = [0xf2, 0x93, 0x01, 0xfd];
        // MTU: 24 bytes header + 4 bytes payload + 4 bytes transfer CRC
        let mtu = 32;
        let mut breakdown = Breakdown::new(
            header_base,
            Microseconds32::from_ticks(0),
            IntoIterator::into_iter(payload),
            mtu,
        );
        let frame = breakdown.next().expect("No frame");
        assert!(breakdown.next().is_none(), "Extra frame at end");
        let expected_bytes = [
            0x1, // Version
            0x2, // Priority
            0x30, 0xde, // Source node
            0x03, 0x00, // Destination node
            0x20, 0xc0, // Service bit | request bit | service ID
            0x24, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Transfer ID
            0x00, 0x00, 0x00, 0x80, // Frame index and last frame flag
            0x21, 0x10, // User data
            0xb1, 0x95, // Header CRC
            0xf2, 0x93, 0x01, 0xfd, // Payload
            0x43, 0x2a, 0x08, 0x7e, // Transfer CRC
        ];
        assert_eq!(HexDebug(&frame.data), HexDebug(expected_bytes.as_slice()));
    }

    #[test]
    fn test_one_full_frame_plus_one_byte() {
        let header_base = HeaderBase {
            data_specifier: DataSpecifier::ServiceResponse {
                from: UdpNodeId::try_from(0x9).unwrap(),
                to: UdpNodeId::try_from(0x83).unwrap(),
                service: ServiceId::try_from(0x1f).unwrap(),
            },
            transfer_id: 0x0fe3.into(),
            priority: Priority::Low,
            data: 0xfffe,
        };
        let payload: [u8; 5] = [0xf2, 0x93, 0x01, 0xfd, 0xe6];
        // MTU: 24 bytes header + 4 bytes payload + 4 bytes transfer CRC
        let mtu = 32;
        let mut breakdown = Breakdown::new(
            header_base,
            Microseconds32::from_ticks(0),
            IntoIterator::into_iter(payload),
            mtu,
        );
        // First frame
        {
            let frame = breakdown.next().expect("No frame");
            let expected_bytes: [u8; 32] = [
                0x1, // Version
                0x5, // Priority
                0x09, 0x00, // Source node
                0x83, 0x00, // Destination node
                0x1f, 0x80, // Service bit | service ID
                0xe3, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Transfer ID
                0x00, 0x00, 0x00, 0x00, // Frame index, not last frame
                0xfe, 0xff, // User data
                0xa2, 0x85, // Header CRC
                0xf2, 0x93, 0x01, 0xfd, 0xe6, // Payload
                0xc1, 0xfa, 0xb9, // First three bytes of transfer CRC
            ];
            assert_eq!(HexDebug(&frame.data), HexDebug(expected_bytes.as_slice()));
        }
        // Second frame
        {
            let frame = breakdown.next().expect("No frame");
            let expected_bytes = [
                0x1, // Version
                0x5, // Priority
                0x09, 0x00, // Source node
                0x83, 0x00, // Destination node
                0x1f, 0x80, // Service bit | service ID
                0xe3, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Transfer ID
                0x01, 0x00, 0x00, 0x80, // Frame index, last frame
                0xfe, 0xff, // User data
                0xdc, 0x7f, // Header CRC
                0xc5, // Last byte of transfer CRC
            ];
            assert_eq!(HexDebug(&frame.data), HexDebug(expected_bytes.as_slice()));
        }
        assert!(breakdown.next().is_none(), "Extra frame at end");
    }

    /// Wraps a slice and implements Debug in a way that always uses hexadecimal numbers
    #[derive(PartialEq)]
    struct HexDebug<'a>(&'a [u8]);
    impl std::fmt::Debug for HexDebug<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list()
                .entries(self.0.iter().copied().map(HexDebugU8))
                .finish()
        }
    }

    /// Wraps a u8 and implements Debug in a way that always uses hexadecimal numbers
    struct HexDebugU8(u8);
    impl std::fmt::Debug for HexDebugU8 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:#04x}", self.0)
        }
    }
}

/// A byte stream adapter that adds a little-endian CRC32 of the previous bytes
struct AddCrc<I> {
    inner: I,
    crc: Crc32c,
    crc_bytes_provided: u8,
}

impl<I> AddCrc<I> {
    pub fn new(inner: I) -> AddCrc<I> {
        AddCrc {
            inner,
            crc: Crc32c::new(),
            crc_bytes_provided: 0,
        }
    }
}

impl<I> Iterator for AddCrc<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.crc_bytes_provided {
            0 => match self.inner.next() {
                Some(byte) => {
                    self.crc.digest(byte);
                    Some(byte)
                }
                None => {
                    self.crc_bytes_provided = 1;
                    Some(self.crc.get_crc() as u8)
                }
            },
            1 => {
                self.crc_bytes_provided = 2;
                Some((self.crc.get_crc() >> 8) as u8)
            }
            2 => {
                self.crc_bytes_provided = 3;
                Some((self.crc.get_crc() >> 16) as u8)
            }
            3 => {
                self.crc_bytes_provided = 4;
                Some((self.crc.get_crc() >> 24) as u8)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod add_crc_tests {
    use super::AddCrc;

    #[test]
    fn add_crc_empty() {
        let expected: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
        assert_eq!(
            expected,
            AddCrc::new(core::iter::empty()).collect::<Vec<u8>>()
        );
    }
    #[test]
    fn add_crc_short() {
        let expected: Vec<u8> = vec![0x6f, 0x77, 0x6f, 0x9f, 0x30, 0x05, 0xf0];
        assert_eq!(
            expected,
            AddCrc::new(IntoIterator::into_iter([0x6f, 0x77, 0x6f])).collect::<Vec<u8>>()
        );
    }
}
