#![no_std]

extern crate canadensis_core;
extern crate crc_any;
extern crate zerocopy;

use canadensis_core::time::Microseconds32;
use canadensis_core::transfer::{MessageHeader, ServiceHeader};
use canadensis_core::transport::{TransferId, Transport};
use canadensis_core::{InvalidValue, Priority, ServiceId, SubjectId};
use core::convert::TryFrom;
use core::mem;
use crc_any::CRCu16;
use defmt::Format;
use zerocopy::byteorder::{U16, U32, U64};
use zerocopy::{AsBytes, BigEndian, FromBytes, LittleEndian};

pub const SIZE: usize = mem::size_of::<RawHeader>();

/// A header, with memory layout matching the on-wire representation
///
/// [This post](https://forum.opencyphal.org/t/cyphal-udp-architectural-issues-caused-by-the-dependency-between-the-nodes-ip-address-and-its-identity/1765/60)
/// specifies the header format.
#[derive(AsBytes, FromBytes, Debug)]
#[repr(C)]
pub struct RawHeader {
    /// 4 bits of version, upper 4 bits reserved
    version: u8,
    /// 3 bits of priority, upper 5 bits reserved
    priority: u8,
    /// Node ID of source, or 0xffff if anonymous
    source_node_id: U16<LittleEndian>,
    /// Node ID of destination, or 0xffff if broadcast
    destination_node_id: U16<LittleEndian>,
    /// Subject or service ID
    data_specifier: U16<LittleEndian>,
    /// Transfer ID
    transfer_id: U64<LittleEndian>,
    /// Index of this frame within the transfer, and the end of transfer flag
    frame_index_eot: U32<LittleEndian>,
    /// Vendor-specific data
    data: U16<LittleEndian>,
    /// Checksum of the header
    header_checksum: U16<BigEndian>,
}

impl RawHeader {
    /// Returns true if this is the last frame in a transfer
    pub fn is_last_frame(&self) -> bool {
        (self.frame_index_eot.get() & LAST_FRAME) != 0
    }
    /// Returns the index of this frame in a transfer
    pub fn frame_index(&self) -> u32 {
        self.frame_index_eot.get() & !LAST_FRAME
    }
    /// Returns true if this header's checksum is correct
    pub fn checksum_valid(&self) -> bool {
        let header_bytes = self.as_bytes();
        let mut crc = header_crc();
        crc.digest(header_bytes);
        crc.get_crc() == 0
    }
}

/// Value to assign to the version field
const VERSION: u8 = 1;
/// Bit set in frame_index_eot if this is the last frame in the transfer
pub const LAST_FRAME: u32 = 0x8000_0000;

const DATA_SPEC_SERVICE_NOT_MESSAGE: u16 = 0x8000;
const DATA_SPEC_REQUEST_NOT_RESPONSE: u16 = 0x4000;
/// Bits available for the subject ID in a data specifier
const DATA_SPEC_SUBJECT_ID_MASK: u16 = !DATA_SPEC_SERVICE_NOT_MESSAGE;
/// Bits available for the service ID in a data specifier
const DATA_SPEC_SERVICE_ID_MASK: u16 =
    !(DATA_SPEC_SERVICE_NOT_MESSAGE | DATA_SPEC_REQUEST_NOT_RESPONSE);

/// A frame header with a convenient representation
///
/// [This post](https://forum.opencyphal.org/t/cyphal-udp-architectural-issues-caused-by-the-dependency-between-the-nodes-ip-address-and-its-identity/1765/60)
/// specifies the header format.
#[derive(Debug)]
pub struct Header {
    /// Priority level
    pub priority: Priority,
    /// Subject or service ID and source/destination node IDs
    pub data_specifier: DataSpecifier,
    /// Transfer ID
    pub transfer_id: TransferId64,
    /// Index of this frame within the transfer
    pub frame_index: u32,
    /// True if this is the last frame in the transfer
    pub last_frame: bool,
    /// Vendor-specific data
    pub data: u16,
}

impl TryFrom<RawHeader> for Header {
    type Error = InvalidValue;

    /// Parses a validated header from a raw header
    fn try_from(header: RawHeader) -> Result<Self, Self::Error> {
        if header.version != VERSION {
            return Err(InvalidValue);
        }
        if !header.checksum_valid() {
            return Err(InvalidValue);
        }
        let priority = Priority::try_from(header.priority)?;
        let source_node_id = check_node_id(header.source_node_id.get())?;
        let destination_node_id = check_node_id(header.destination_node_id.get())?;

        let data_specifier = if (header.data_specifier.get() & DATA_SPEC_SERVICE_NOT_MESSAGE)
            == DATA_SPEC_SERVICE_NOT_MESSAGE
        {
            let service =
                ServiceId::try_from(header.data_specifier.get() & DATA_SPEC_SERVICE_ID_MASK)?;
            // Service transfers must have source and destination node IDs
            let from = source_node_id.ok_or(InvalidValue)?;
            let to = destination_node_id.ok_or(InvalidValue)?;
            if (header.data_specifier.get() & DATA_SPEC_REQUEST_NOT_RESPONSE)
                == DATA_SPEC_REQUEST_NOT_RESPONSE
            {
                DataSpecifier::ServiceRequest { from, to, service }
            } else {
                DataSpecifier::ServiceResponse { from, to, service }
            }
        } else {
            let subject =
                SubjectId::try_from(header.data_specifier.get() & DATA_SPEC_SUBJECT_ID_MASK)?;
            DataSpecifier::Subject {
                from: source_node_id,
                subject,
            }
        };

        Ok(Header {
            priority,
            data_specifier,
            frame_index: header.frame_index(),
            last_frame: header.is_last_frame(),
            transfer_id: header.transfer_id.get().into(),
            data: header.data.get(),
        })
    }
}

impl From<Header> for RawHeader {
    /// Encodes a validated header into a raw header for transmission
    fn from(header: Header) -> Self {
        let last_frame_flag = if header.last_frame { LAST_FRAME } else { 0 };

        let mut header = RawHeader {
            version: VERSION,
            priority: header.priority.into(),
            source_node_id: header
                .data_specifier
                .source_node_id()
                .map(u16::from)
                .unwrap_or(NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST)
                .into(),
            destination_node_id: header
                .data_specifier
                .destination_node_id()
                .map(u16::from)
                .unwrap_or(NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST)
                .into(),
            data_specifier: match header.data_specifier {
                DataSpecifier::Subject { subject, .. } => zerocopy::U16::from(u16::from(subject)),
                DataSpecifier::ServiceRequest { service, .. } => zerocopy::U16::from(
                    DATA_SPEC_SERVICE_NOT_MESSAGE
                        | DATA_SPEC_REQUEST_NOT_RESPONSE
                        | u16::from(service),
                ),
                DataSpecifier::ServiceResponse { service, .. } => {
                    zerocopy::U16::from(DATA_SPEC_SERVICE_NOT_MESSAGE | u16::from(service))
                }
            },
            transfer_id: u64::from(header.transfer_id).into(),
            frame_index_eot: (header.frame_index | last_frame_flag).into(),
            data: header.data.into(),
            header_checksum: 0.into(),
        };
        // Calculate CRC for the header, excluding the CRC field
        header.header_checksum = {
            let bytes: &[u8] = header.as_bytes();
            let mut crc = header_crc();
            crc.digest(&bytes[..bytes.len() - 2]);
            crc.get_crc().into()
        };
        debug_assert!(header.checksum_valid());
        header
    }
}

impl Header {
    /// Creates a core transfer header that is a copy of this header
    ///
    /// timestamp: A timestamp to assign to the returned header
    pub fn as_core_header<T>(
        &self,
        timestamp: Microseconds32,
    ) -> canadensis_core::transfer::Header<T>
    where
        T: Transport<TransferId = TransferId64, NodeId = NodeId16>,
    {
        match self.data_specifier {
            DataSpecifier::Subject { from, subject } => {
                canadensis_core::transfer::Header::Message(MessageHeader {
                    timestamp,
                    transfer_id: self.transfer_id,
                    priority: self.priority.into(),
                    subject,
                    source: from,
                })
            }
            DataSpecifier::ServiceRequest { from, to, service } => {
                canadensis_core::transfer::Header::Request(ServiceHeader {
                    timestamp,
                    transfer_id: self.transfer_id,
                    priority: self.priority.into(),
                    service,
                    source: from,
                    destination: to,
                })
            }
            DataSpecifier::ServiceResponse { from, to, service } => {
                canadensis_core::transfer::Header::Response(ServiceHeader {
                    timestamp,
                    transfer_id: self.transfer_id,
                    priority: self.priority.into(),
                    service,
                    source: from,
                    destination: to,
                })
            }
        }
    }
}

impl<'h, T> From<&'h canadensis_core::transfer::Header<T>> for Header
where
    T: Transport<NodeId = NodeId16, Priority = Priority, TransferId = TransferId64>,
{
    /// Converts a transfer header into a transport header
    ///
    /// The returned header has its frame index set to 0 and last frame set to true.
    fn from(header: &'h canadensis_core::transfer::Header<T>) -> Self {
        let data_specifier = match header {
            canadensis_core::transfer::Header::Message(message_header) => DataSpecifier::Subject {
                from: message_header.source,
                subject: message_header.subject,
            },
            canadensis_core::transfer::Header::Request(request_header) => {
                DataSpecifier::ServiceRequest {
                    from: request_header.source,
                    to: request_header.destination,
                    service: request_header.service,
                }
            }
            canadensis_core::transfer::Header::Response(response_header) => {
                DataSpecifier::ServiceResponse {
                    from: response_header.source,
                    to: response_header.destination,
                    service: response_header.service,
                }
            }
        };
        Header {
            priority: *header.priority(),
            data_specifier,
            transfer_id: *header.transfer_id(),
            frame_index: 0,
            last_frame: true,
            data: 0,
        }
    }
}

impl<T> From<canadensis_core::transfer::Header<T>> for Header
where
    T: Transport<NodeId = NodeId16, Priority = Priority, TransferId = TransferId64>,
{
    fn from(header: canadensis_core::transfer::Header<T>) -> Self {
        Header::from(&header)
    }
}

/// The message/service, subject ID/service ID, source, and destination of a frame
#[derive(Debug, Clone)]
pub enum DataSpecifier {
    Subject {
        from: Option<NodeId16>,
        subject: SubjectId,
    },
    ServiceRequest {
        from: NodeId16,
        to: NodeId16,
        service: ServiceId,
    },
    ServiceResponse {
        from: NodeId16,
        to: NodeId16,
        service: ServiceId,
    },
}

impl DataSpecifier {
    pub fn source_node_id(&self) -> Option<NodeId16> {
        match self {
            DataSpecifier::Subject { from, .. } => *from,
            DataSpecifier::ServiceRequest { from, .. } => Some(*from),
            DataSpecifier::ServiceResponse { from, .. } => Some(*from),
        }
    }
    pub fn destination_node_id(&self) -> Option<NodeId16> {
        match self {
            DataSpecifier::Subject { .. } => None,
            DataSpecifier::ServiceRequest { to, .. } => Some(*to),
            DataSpecifier::ServiceResponse { to, .. } => Some(*to),
        }
    }
}

fn check_node_id(id: u16) -> Result<Option<NodeId16>, InvalidValue> {
    if id == NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST {
        Ok(None)
    } else {
        NodeId16::try_from(id).map(Some)
    }
}

/// Returns a CRC calculator used for headers
pub fn header_crc() -> CRCu16 {
    CRCu16::crc16ccitt_false()
}

/// A 16-bit node ID
///
/// This allows all u16 values except 65535, which is reserved for anonymous transfers
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Format)]
pub struct NodeId16(u16);

const NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST: u16 = 0xffff;

impl TryFrom<u16> for NodeId16 {
    type Error = InvalidValue;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST {
            Err(InvalidValue)
        } else {
            Ok(NodeId16(value))
        }
    }
}
impl From<NodeId16> for u16 {
    fn from(id: NodeId16) -> Self {
        id.0
    }
}
impl From<NodeId16> for u32 {
    fn from(id: NodeId16) -> Self {
        id.0.into()
    }
}

impl From<NodeId16> for usize {
    fn from(id: NodeId16) -> Self {
        id.0.into()
    }
}

/// A transfer identifier that should not wrap
///
/// This is just a `u64`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct TransferId64(u64);

impl TransferId for TransferId64 {
    fn increment(self) -> Self {
        TransferId64(self.0.wrapping_add(1))
    }
}

impl From<TransferId64> for u64 {
    fn from(id: TransferId64) -> Self {
        id.0
    }
}

impl From<u64> for TransferId64 {
    fn from(value: u64) -> Self {
        TransferId64(value)
    }
}
