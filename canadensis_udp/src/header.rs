use crate::{header_crc, UdpNodeId, UdpTransferId, NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST};
use canadensis_core::{InvalidValue, Priority, ServiceId, SubjectId};
use core::convert::TryFrom;
use core::mem;
use zerocopy::byteorder::{LE, U16, U32, U64};
use zerocopy::{AsBytes, FromBytes};

pub const SIZE: usize = mem::size_of::<UdpHeader>();

/// The Cyphal/UDP header placed in each UDP packet
#[derive(AsBytes, FromBytes, Debug)]
#[repr(C)]
pub struct UdpHeader {
    /// 4 bits of version, upper 4 bits reserved
    pub version: u8,
    /// 3 bits of priority, upper 5 bits reserved
    pub priority: u8,
    /// Node ID of source, or 0xffff if anonymous
    pub source_node_id: U16<LE>,
    /// Node ID of destination, or 0xffff if broadcast
    pub destination_node_id: U16<LE>,
    /// Subject or service ID
    pub data_specifier: U16<LE>,
    /// Transfer ID
    pub transfer_id: U64<LE>,
    /// Index of this frame within the transfer, and the end of transfer flag
    pub frame_index_eot: U32<LE>,
    /// Vendor-specific data
    pub data: U16<LE>,
    /// Checksum of the header
    pub header_checksum: U16<LE>,
}

impl UdpHeader {
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
        let expected_crc = self.header_checksum.get();
        let header_bytes = self.as_bytes();
        let header_bytes_to_crc = &header_bytes[..header_bytes.len() - 2];
        let mut crc = header_crc();
        crc.digest(header_bytes_to_crc);
        crc.get_crc() == expected_crc
    }
}

/// Value to assign to the version field
pub const VERSION: u8 = 1;
/// Bit set in frame_index_eot if this is the last frame in the transfer
pub const LAST_FRAME: u32 = 0x8000_0000;

pub const DATA_SPEC_SERVICE_NOT_MESSAGE: u16 = 0x8000;
pub const DATA_SPEC_REQUEST_NOT_RESPONSE: u16 = 0x4000;
/// Bits available for the subject ID in a data specifier
const DATA_SPEC_SUBJECT_ID_MASK: u16 = !DATA_SPEC_SERVICE_NOT_MESSAGE;
/// Bits available for the service ID in a data specifier
const DATA_SPEC_SERVICE_ID_MASK: u16 =
    !(DATA_SPEC_SERVICE_NOT_MESSAGE | DATA_SPEC_REQUEST_NOT_RESPONSE);

#[derive(Debug)]
pub struct ValidatedUdpHeader {
    /// Priority level
    pub priority: Priority,
    /// Subject or service ID and source/destination node IDs
    pub data_specifier: DataSpecifier,
    /// Transfer ID
    pub transfer_id: UdpTransferId,
    /// Index of this frame within the transfer
    pub frame_index: u32,
    /// True if this is the last frame in the transfer
    pub last_frame: bool,
    /// Vendor-specific data
    pub data: u16,
}

impl TryFrom<UdpHeader> for ValidatedUdpHeader {
    type Error = InvalidValue;

    /// Parses a validated header from a raw header
    fn try_from(header: UdpHeader) -> Result<Self, Self::Error> {
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

        Ok(ValidatedUdpHeader {
            priority,
            data_specifier,
            frame_index: header.frame_index(),
            last_frame: header.is_last_frame(),
            transfer_id: header.transfer_id.get().into(),
            data: header.data.get(),
        })
    }
}

impl From<ValidatedUdpHeader> for UdpHeader {
    /// Encodes a validated header into a raw header for transmissoin
    fn from(header: ValidatedUdpHeader) -> Self {
        let last_frame_flag = if header.last_frame { LAST_FRAME } else { 0 };

        let mut header = UdpHeader {
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

#[derive(Debug, Clone)]
pub enum DataSpecifier {
    Subject {
        from: Option<UdpNodeId>,
        subject: SubjectId,
    },
    ServiceRequest {
        from: UdpNodeId,
        to: UdpNodeId,
        service: ServiceId,
    },
    ServiceResponse {
        from: UdpNodeId,
        to: UdpNodeId,
        service: ServiceId,
    },
}

impl DataSpecifier {
    pub fn source_node_id(&self) -> Option<UdpNodeId> {
        match self {
            DataSpecifier::Subject { from, .. } => *from,
            DataSpecifier::ServiceRequest { from, .. } => Some(*from),
            DataSpecifier::ServiceResponse { from, .. } => Some(*from),
        }
    }
    pub fn destination_node_id(&self) -> Option<UdpNodeId> {
        match self {
            DataSpecifier::Subject { .. } => None,
            DataSpecifier::ServiceRequest { to, .. } => Some(*to),
            DataSpecifier::ServiceResponse { to, .. } => Some(*to),
        }
    }
}

fn check_node_id(id: u16) -> Result<Option<UdpNodeId>, InvalidValue> {
    if id == NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST {
        Ok(None)
    } else {
        UdpNodeId::try_from(id).map(Some)
    }
}
