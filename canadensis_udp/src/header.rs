use crate::{header_crc, UdpNodeId, UdpTransferId, NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST};
use canadensis_core::{InvalidValue, Priority, ServiceId, SubjectId};
use core::mem;
use std::convert::TryFrom;
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
    pub source_node_id: u16,
    /// Node ID of destination, or 0xffff if broadcast
    pub destination_node_id: u16,
    /// Subject or service ID
    pub data_specifier: u16,
    /// Transfer ID
    pub transfer_id: u64,
    /// Index of this frame within the transfer, and the end of transfer flag
    pub frame_index_eot: u32,
    /// Vendor-specific data
    pub data: u16,
    /// Checksum of the header
    pub header_checksum: u16,
}

impl UdpHeader {
    /// Returns true if this is the last frame in a transfer
    pub fn is_last_frame(&self) -> bool {
        (self.frame_index_eot & LAST_FRAME) != 0
    }
    /// Returns the index of this frame in a transfer
    pub fn frame_index(&self) -> u32 {
        self.frame_index_eot & !LAST_FRAME
    }
    /// Returns true if this header's checksum is correct
    pub fn checksum_valid(&self) -> bool {
        let expected_crc = self.header_checksum;
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
    /// ID of the source node, or None if anonymous
    pub source_node_id: Option<UdpNodeId>,
    /// ID of the destination node, or None if broadcast
    pub destination_node_id: Option<UdpNodeId>,
    /// Subject or service ID
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

    fn try_from(header: UdpHeader) -> Result<Self, Self::Error> {
        if header.version != VERSION {
            return Err(InvalidValue);
        }
        if !header.checksum_valid() {
            return Err(InvalidValue);
        }
        let priority = Priority::try_from(header.priority)?;
        let source_node_id = check_node_id(header.source_node_id)?;
        let destination_node_id = check_node_id(header.destination_node_id)?;

        let data_specifier = if (header.data_specifier & DATA_SPEC_SERVICE_NOT_MESSAGE)
            == DATA_SPEC_SERVICE_NOT_MESSAGE
        {
            let service_id =
                ServiceId::try_from(header.data_specifier & DATA_SPEC_SERVICE_ID_MASK)?;
            if (header.data_specifier & DATA_SPEC_REQUEST_NOT_RESPONSE)
                == DATA_SPEC_REQUEST_NOT_RESPONSE
            {
                DataSpecifier::ServiceRequest(service_id)
            } else {
                DataSpecifier::ServiceResponse(service_id)
            }
        } else {
            DataSpecifier::Subject(SubjectId::try_from(
                header.data_specifier & DATA_SPEC_SUBJECT_ID_MASK,
            )?)
        };

        Ok(ValidatedUdpHeader {
            priority,
            source_node_id,
            destination_node_id,
            data_specifier,
            frame_index: header.frame_index(),
            last_frame: header.is_last_frame(),
            transfer_id: header.transfer_id.into(),
            data: header.data,
        })
    }
}

#[derive(Debug, Clone)]
pub enum DataSpecifier {
    Subject(SubjectId),
    ServiceRequest(ServiceId),
    ServiceResponse(ServiceId),
}

fn check_node_id(id: u16) -> Result<Option<UdpNodeId>, InvalidValue> {
    if id == NODE_ID_RESERVED_ANONYMOUS_OR_BROADCAST {
        Ok(None)
    } else {
        UdpNodeId::try_from(id).map(Some)
    }
}
