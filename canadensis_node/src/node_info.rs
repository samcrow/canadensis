//! Definitions used to handle uavcan.node.GetInfo requests

use alloc::vec::Vec;
use core::iter;

use fallible_collections::FallibleVec;

use crate::Node;
use canadensis_core::transfer::Transfer;
use canadensis_core::ServiceId;
use canadensis_encoding::{EncodeCursor, DataType, ZeroCopy};

/// Node information service ID
const INFO_SERVICE: ServiceId = ServiceId::from_truncating(430);

/// Version is UAVCAN 1.0
const PROTOCOL_VERSION: Version = Version { major: 1, minor: 0};

/// A version, containing major and minor fields but no patch version field
#[repr(C, packed)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

unsafe impl ZeroCopy for Version {}

/// Content of a GetInfo response
///
/// Full details: https://github.com/UAVCAN/public_regulated_data_types/blob/master/uavcan/node/430.GetInfo.1.0.uavcan
pub struct NodeInfo<'info> {
    // Protocol version is not here. It's hard-coded.
    hardware_version: Version,
    software_version: Version,
    software_vcs_revision_id: u64,
    unique_id: [u8; 16],
    name: &'info str,
    software_image_crc: Option<u64>,
    certificate_of_authenticity: Option<&'info [u8]>,
}

impl DataType for NodeInfo<'_> {
    fn size_bits(&self) -> usize {
        30 * 8 // fixed-size section
        + 6 // name length
        + self.name.len() * 8 // name
        + 1 // software_image_crc length
        + self.software_image_crc.map(|_| 64).unwrap_or(0)
            + 8 // certificate_of_authenticity length
        + self.certificate_of_authenticity.map(|coa| coa.len() * 8).unwrap_or(0)
    }

    fn encode(&self, cursor: &mut EncodeCursor<'_>) {
        PROTOCOL_VERSION.encode(cursor);
        self.hardware_version.encode(cursor);
        self.software_version.encode(cursor);
        cursor.write_u64(self.software_vcs_revision_id);
        cursor.write_bytes(&self.unique_id);
        cursor.write_variable_array(50, self.name.as_bytes());
        if let Some(crc) = self.software_image_crc {
            cursor.write_u1(1);
            cursor.write_u64(crc);
        } else {
            cursor.write_u1(0);
        }
        cursor.write_variable_array(222, self.certificate_of_authenticity.unwrap_or(&[]));
    }

    fn decode_in_place(&mut self, bytes: &[u8]) {
        unimplemented!()
    }

    fn decode(bytes: &[u8]) -> Self where
        Self: Sized {
        unimplemented!()
    }
}

