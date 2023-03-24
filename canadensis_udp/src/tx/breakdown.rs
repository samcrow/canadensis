use alloc::vec::Vec;
use core::iter::Peekable;
use core::mem;

use crc_any::CRCu32;
use zerocopy::AsBytes;

use canadensis_core::Priority;

use crate::tx::UdpFrame;
use crate::{data_crc, UdpTransferId, TRANSFER_CRC_SIZE};
use canadensis_header::{DataSpecifier, Header, RawHeader, LAST_FRAME};

/// An iterator that breaks a transfer into UDP frames and adds a CRC to each frame
pub(crate) struct Breakdown<P: Iterator<Item = u8>, I> {
    /// Basic header information to apply to all frames
    header_base: HeaderBase,
    /// The transmit deadline for this transfer
    deadline: I,
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

impl<P: Iterator<Item = u8>, I: Clone> Breakdown<P, I> {
    pub fn new(header_base: HeaderBase, deadline: I, payload: P, mtu: usize) -> Self {
        Breakdown {
            header_base,
            deadline,
            payload: payload.peekable(),
            frame_index: 0,
            done: false,
            transfer_crc: data_crc(),
            // Initialize the current frame with empty space for the header. The payload will follow.
            current_frame: {
                let mut frame: Vec<u8> = Vec::with_capacity(mtu - TRANSFER_CRC_SIZE);
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
    fn take_frame(&mut self, header: Header, crc: u32) -> UdpFrame<I> {
        // Copy the header into the current frame
        self.current_frame[..canadensis_header::SIZE]
            .copy_from_slice(RawHeader::from(header).as_bytes());
        // Add CRC
        self.current_frame.extend_from_slice(&crc.to_le_bytes());
        let frame = UdpFrame {
            deadline: self.deadline.clone(),
            data: mem::take(&mut self.current_frame),
        };
        // Add space in the new current frame for the header
        self.current_frame
            .reserve_exact(self.mtu - TRANSFER_CRC_SIZE);
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
        .into()
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
                            crc.digest(
                                &self.current_frame
                                    [canadensis_header::SIZE..self.current_frame.len()],
                            );
                            crc.get_crc()
                        };

                        let frame = self.take_frame(header, data_crc);
                        self.frame_index += 1;
                        assert_eq!(self.frame_index & LAST_FRAME, 0, "Frame index too large");
                        break Some(frame);
                    }
                }
                None => {
                    if self.current_frame.len() != canadensis_header::SIZE {
                        // End of data, return a frame with the last frame bit set
                        // and with a CRC covering all the data
                        let header = self.make_header(true);
                        let transfer_crc = self.transfer_crc.get_crc();
                        let frame = self.take_frame(header, transfer_crc);
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
mod tests {
    use std::convert::TryFrom;
    use std::iter;

    use canadensis_core::time::Microseconds64;
    use canadensis_core::{Priority, ServiceId, SubjectId};

    use crate::{data_crc, UdpNodeId, TRANSFER_CRC_SIZE};
    use canadensis_header::{header_crc, DataSpecifier};

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
        // No payload, should produce no frame
        let mut breakdown =
            Breakdown::new(header_base, Microseconds64::new(0), iter::empty(), 1472);
        assert!(breakdown.next().is_none(), "Unexpected frame");
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
            Microseconds64::new(0),
            IntoIterator::into_iter(payload),
            1472,
        );
        let frame = breakdown.next().expect("No frame");
        assert!(breakdown.next().is_none(), "Extra frame at end");

        assert_eq!(
            frame.data.len(),
            canadensis_header::SIZE + payload.len() + TRANSFER_CRC_SIZE
        );
        // Check everything before the header CRC
        let expected_bytes = [
            0x1, // Version
            0x4, // Priority
            0x30, 0xde, // Source node
            0xff, 0xff, // Destination node
            0xff, 0x1f, // Subject
            0x08, 0x07, 0x06, 05, 0x04, 0x03, 0x02, 0x01, // Transfer ID
            0x00, 0x00, 0x00, 0x80, // Frame index and last frame flag
            0xfe, 0x39, // Data
        ];
        assert_eq!(
            HexDebug(&frame.data[..canadensis_header::SIZE - 2]),
            HexDebug(expected_bytes.as_slice())
        );
        check_header_crc(&frame.data[..canadensis_header::SIZE]);
        assert_eq!(
            frame.data[canadensis_header::SIZE],
            payload[0],
            "Incorrect payload"
        );
        check_single_frame_data_crc(&frame.data[canadensis_header::SIZE..]);
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
            Microseconds64::new(0),
            IntoIterator::into_iter(payload),
            mtu,
        );
        let frame = breakdown.next().expect("No frame");
        assert!(breakdown.next().is_none(), "Extra frame at end");

        assert_eq!(
            frame.data.len(),
            canadensis_header::SIZE + payload.len() + TRANSFER_CRC_SIZE
        );
        // Check everything before the header CRC
        let expected_bytes = [
            0x1, // Version
            0x2, // Priority
            0x30, 0xde, // Source node
            0x03, 0x00, // Destination node
            0x20, 0xc0, // Service bit | request bit | service ID
            0x24, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Transfer ID
            0x00, 0x00, 0x00, 0x80, // Frame index and last frame flag
            0x21, 0x10, // Data
        ];
        assert_eq!(
            HexDebug(&frame.data[..canadensis_header::SIZE - 2]),
            HexDebug(expected_bytes.as_slice())
        );
        check_header_crc(&frame.data[..canadensis_header::SIZE]);
        assert_eq!(
            frame.data[canadensis_header::SIZE..][..payload.len()],
            payload,
            "Incorrect payload"
        );
        check_single_frame_data_crc(&frame.data[canadensis_header::SIZE..]);
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
            Microseconds64::new(0),
            IntoIterator::into_iter(payload),
            mtu,
        );
        // First frame
        {
            let frame = breakdown.next().expect("No frame");
            assert_eq!(frame.data.len(), mtu);
            // Check everything before the header CRC
            let expected_bytes = [
                0x1, // Version
                0x5, // Priority
                0x09, 0x00, // Source node
                0x83, 0x00, // Destination node
                0x1f, 0x80, // Service bit | service ID
                0xe3, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Transfer ID
                0x00, 0x00, 0x00, 0x00, // Frame index, not last frame
                0xfe, 0xff, // Data
            ];
            assert_eq!(
                HexDebug(&frame.data[..canadensis_header::SIZE - 2]),
                HexDebug(expected_bytes.as_slice())
            );
            check_header_crc(&frame.data[..canadensis_header::SIZE]);
            assert_eq!(
                frame.data[canadensis_header::SIZE..][..4],
                payload[..4],
                "Incorrect payload"
            );
            check_single_frame_data_crc(&frame.data[canadensis_header::SIZE..]);
        }
        // Second frame
        {
            let frame = breakdown.next().expect("No frame");
            assert_eq!(
                frame.data.len(),
                canadensis_header::SIZE + 1 + TRANSFER_CRC_SIZE
            );
            // Check everything before the header CRC
            let expected_bytes = [
                0x1, // Version
                0x5, // Priority
                0x09, 0x00, // Source node
                0x83, 0x00, // Destination node
                0x1f, 0x80, // Service bit | service ID
                0xe3, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Transfer ID
                0x01, 0x00, 0x00, 0x80, // Frame index, last frame
                0xfe, 0xff, // Data
            ];
            assert_eq!(
                HexDebug(&frame.data[..canadensis_header::SIZE - 2]),
                HexDebug(expected_bytes.as_slice())
            );
            check_header_crc(&frame.data[..canadensis_header::SIZE]);
            assert_eq!(
                frame.data[canadensis_header::SIZE],
                payload[4],
                "Incorrect payload"
            );
            // Because this is the last frame of a multi-frame transfer, the CRC should cover
            // the complete payload reassembled from all the frames.
            check_full_payload_crc(
                &payload,
                &frame.data[frame.data.len() - TRANSFER_CRC_SIZE..],
            );
        }
        assert!(breakdown.next().is_none(), "Extra frame at end");
    }

    fn check_header_crc(header_bytes: &[u8]) {
        assert_eq!(header_bytes.len(), canadensis_header::SIZE);
        let mut crc = header_crc();
        crc.digest(header_bytes);
        assert_eq!(crc.get_crc(), 0);
    }

    fn check_single_frame_data_crc(payload_and_crc: &[u8]) {
        let (payload, crc_bytes) =
            payload_and_crc.split_at(payload_and_crc.len() - TRANSFER_CRC_SIZE);
        let actual_crc =
            u32::from_le_bytes([crc_bytes[0], crc_bytes[1], crc_bytes[2], crc_bytes[3]]);
        let mut crc = data_crc();
        crc.digest(payload);
        assert_eq!(crc.get_crc(), actual_crc);
    }

    fn check_full_payload_crc(payload: &[u8], actual_crc_bytes: &[u8]) {
        let actual_crc = u32::from_le_bytes([
            actual_crc_bytes[0],
            actual_crc_bytes[1],
            actual_crc_bytes[2],
            actual_crc_bytes[3],
        ]);
        let mut crc = data_crc();
        crc.digest(payload);
        assert_eq!(crc.get_crc(), actual_crc);
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
