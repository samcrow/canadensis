use crate::{SerialNodeId, SerialTransferId, SerialTransport};
use canadensis_core::transfer::{Header, MessageHeader, ServiceHeader};
use canadensis_core::{Priority, ServiceId, SubjectId};
use core::convert::TryFrom;
use core::mem;
use crc_any::CRCu32;
use zerocopy::{AsBytes, FromBytes};

/// The header of a serial transfer
#[derive(AsBytes, FromBytes, Debug)]
#[repr(C)]
pub struct SerialHeader {
    pub version: u8,
    pub priority: u8,
    pub source_node: u16,
    pub destination_node: u16,
    pub data_specifier: u16,
    pub _padding: u64,
    pub transfer_id: u64,
    pub frame_index_eot: u32,
    pub header_crc: u32,
}

impl SerialHeader {
    /// The expected version number
    const VERSION: u8 = 0;
    /// Source node for anonymous transfers
    const ANONYMOUS_SOURCE: u16 = 0xffff;
    /// Destination for broadcast transfers
    const BROADCAST_DESTINATION: u16 = 0xffff;
    /// Frame index/EOT for single-frame transfers
    const SINGLE_FRAME_INDEX: u32 = 0x8000_0000;

    /// Returns the bytes of self, excluding the CRC field
    fn bytes_to_crc(&self) -> &[u8] {
        &self.as_bytes()[..mem::size_of::<SerialHeader>() - mem::size_of::<u32>()]
    }

    /// Fills in `header_crc` based on the values of the other fields
    fn update_crc(&mut self) {
        let mut crc = CRCu32::crc32c();
        crc.digest(self.bytes_to_crc());
        self.header_crc = crc.get_crc();
    }

    /// Returns true if the CRC is correct
    fn check_crc(&self) -> bool {
        let mut crc = CRCu32::crc32c();
        crc.digest(self.bytes_to_crc());
        crc.get_crc() == self.header_crc
    }

    /// Parses this serial header into a transfer header
    pub fn into_header<I, E>(
        self,
        timestamp: I,
    ) -> Result<Header<I, SerialTransport<E>>, HeaderParseError> {
        if !self.check_crc() {
            return Err(HeaderParseError::Crc);
        }
        if self.version != SerialHeader::VERSION {
            return Err(HeaderParseError::Version);
        }
        if self.frame_index_eot != SerialHeader::SINGLE_FRAME_INDEX {
            return Err(HeaderParseError::NotSingleFrame);
        }
        let priority = Priority::try_from(self.priority).map_err(|_| HeaderParseError::Priority)?;
        let transfer_id: SerialTransferId = self.transfer_id.into();

        if data_specifier_is_service(self.data_specifier) {
            let source = SerialNodeId::try_from(self.source_node)
                .map_err(|_| HeaderParseError::AnonymousService)?;
            let destination = SerialNodeId::try_from(self.destination_node)
                .map_err(|_| HeaderParseError::AnonymousService)?;
            let service = ServiceId::from_truncating(self.data_specifier);
            let service_header = ServiceHeader {
                timestamp,
                transfer_id,
                priority,
                service,
                source,
                destination,
            };
            if data_specifier_is_response(self.data_specifier) {
                // Service response
                Ok(Header::Response(service_header))
            } else {
                // Service request
                Ok(Header::Request(service_header))
            }
        } else {
            // Message
            Ok(Header::Message(MessageHeader {
                timestamp,
                transfer_id,
                priority,
                subject: SubjectId::from_truncating(self.data_specifier),
                // A try_from error means that the source is anonymous
                source: SerialNodeId::try_from(self.source_node).ok(),
            }))
        }
    }
}

impl<I, E> From<Header<I, SerialTransport<E>>> for SerialHeader {
    /// Converts a transfer header into a serial header
    fn from(header: Header<I, SerialTransport<E>>) -> Self {
        let mut serial_header = match header {
            Header::Message(header) => SerialHeader {
                version: SerialHeader::VERSION,
                // Integer value of canadensis_core::Priority
                priority: header.priority as u8,
                source_node: header
                    .source
                    .map(u16::from)
                    .unwrap_or(SerialHeader::ANONYMOUS_SOURCE),
                destination_node: SerialHeader::BROADCAST_DESTINATION,
                data_specifier: header.subject.into(),
                _padding: 0,
                transfer_id: header.transfer_id.into(),
                frame_index_eot: SerialHeader::SINGLE_FRAME_INDEX,
                // Will be filled in later
                header_crc: 0,
            },
            Header::Request(header) => SerialHeader {
                version: SerialHeader::VERSION,
                // Integer value of canadensis_core::Priority
                priority: header.priority as u8,
                source_node: header.source.into(),
                destination_node: header.destination.into(),
                data_specifier: request_data_specifier(header.service),
                _padding: 0,
                transfer_id: header.transfer_id.into(),
                frame_index_eot: SerialHeader::SINGLE_FRAME_INDEX,
                // Will be filled in later
                header_crc: 0,
            },
            Header::Response(header) => SerialHeader {
                version: SerialHeader::VERSION,
                // Integer value of canadensis_core::Priority
                priority: header.priority as u8,
                source_node: header.source.into(),
                destination_node: header.destination.into(),
                data_specifier: response_data_specifier(header.service),
                _padding: 0,
                transfer_id: header.transfer_id.into(),
                frame_index_eot: SerialHeader::SINGLE_FRAME_INDEX,
                // Will be filled in later
                header_crc: 0,
            },
        };
        serial_header.update_crc();
        serial_header
    }
}

/// Errors that may occur when parsing a header
#[derive(Debug)]
pub enum HeaderParseError {
    /// An invalid CRC
    Crc,
    /// An invalid version field
    Version,
    /// An invalid priority value
    Priority,
    /// A transfer that does not fit into a single frame
    NotSingleFrame,
    /// A service from an anonymous source or with a broadcast destination
    AnonymousService,
}

fn request_data_specifier(service: ServiceId) -> u16 {
    // Set bit 15
    0x8000 | u16::from(service)
}
fn response_data_specifier(service: ServiceId) -> u16 {
    // Set bits 14 and 15
    0xc000 | u16::from(service)
}
fn data_specifier_is_service(specifier: u16) -> bool {
    // Bit 15 is set
    (specifier & 0x8000) != 0
}
fn data_specifier_is_response(specifier: u16) -> bool {
    // Bit 14 is set
    (specifier & 0x4000) != 0
}
