//! Definitions used to handle uavcan.node.GetInfo requests

use core::slice;

use canadensis_core::ServiceId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor, ZeroCopy,
};

/// Node information service ID
pub const INFO_SERVICE: ServiceId = ServiceId::from_truncating(430);

/// Version is UAVCAN 1.0
const PROTOCOL_VERSION: Version = Version { major: 1, minor: 0 };

/// A version, containing major and minor fields but no patch version field
#[repr(C, packed)]
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8) -> Self {
        Version { major, minor }
    }
}

unsafe impl ZeroCopy for Version {}

/// Content of a GetInfo response
///
/// Full details: https://github.com/UAVCAN/public_regulated_data_types/blob/master/uavcan/node/430.GetInfo.1.0.uavcan
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
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

impl<'info> NodeInfo<'info> {
    /// Creates a new node information object
    ///
    /// name must be a string up to 50 bytes long.
    ///
    /// This function will panic if the length of unique_id is not 16 or the length of name
    /// is greater than 50 bytes.
    pub fn new(unique_id: [u8; 16], name: &'info str) -> Self {
        assert!(name.len() <= 50, "Name is longer than 50 bytes");
        NodeInfo {
            hardware_version: Version { major: 0, minor: 0 },
            software_version: Version { major: 0, minor: 0 },
            software_vcs_revision_id: 0,
            unique_id,
            name,
            software_image_crc: None,
            certificate_of_authenticity: None,
        }
    }
    /// Sets the version of the hardware this node is running on
    pub fn hardware_version(self, version: Version) -> Self {
        NodeInfo {
            hardware_version: version,
            ..self
        }
    }
    /// Sets the version of the software running on this device
    pub fn software_version(self, version: Version) -> Self {
        NodeInfo {
            software_version: version,
            ..self
        }
    }
    /// Sets the version control revision code of the software
    pub fn revision(self, revision: u64) -> Self {
        NodeInfo {
            software_vcs_revision_id: revision,
            ..self
        }
    }
    /// Sets the hash of the software image
    pub fn software_crc(self, crc: u64) -> Self {
        NodeInfo {
            software_image_crc: Some(crc),
            ..self
        }
    }
    /// Sets the certificate of authenticity
    ///
    /// This function panics if the certificate of authenticity is more than 222 bytes long.
    pub fn certificate_of_authenticity(self, coa: &'info [u8]) -> Self {
        assert!(coa.len() <= 222);
        NodeInfo {
            certificate_of_authenticity: Some(coa),
            ..self
        }
    }
}

impl DataType for NodeInfo<'_> {}

impl Serialize for NodeInfo<'_> {
    fn size_bits(&self) -> usize {
        30 * 8 // fixed-size section
        + 8 // name length
        + self.name.len() * 8 // name
        + 8 // software_image_crc length
        + self.software_image_crc.map(|_| 64).unwrap_or(0)
            + 8 // certificate_of_authenticity length
        + self.certificate_of_authenticity.map(|coa| coa.len() * 8).unwrap_or(0)
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        PROTOCOL_VERSION.serialize(cursor);
        self.hardware_version.serialize(cursor);
        self.software_version.serialize(cursor);
        cursor.write_u64(self.software_vcs_revision_id);
        cursor.write_bytes(&self.unique_id);
        cursor.write_variable_array(50, self.name.as_bytes());
        let crc_slice: &[u64] = self
            .software_image_crc
            .as_ref()
            .map(slice::from_ref)
            .unwrap_or(&[]);
        cursor.write_variable_array(1, crc_slice);
        cursor.write_variable_array(222, self.certificate_of_authenticity.unwrap_or(&[]));
    }
}

/// An empty node information request
pub struct NodeInfoRequest;

impl DataType for NodeInfoRequest {}

impl Deserialize for NodeInfoRequest {
    fn deserialize_in_place(
        &mut self,
        _cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        Ok(())
    }

    fn deserialize(_cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        Ok(NodeInfoRequest)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_serialize() {
        let info = NodeInfo::new(
            [
                0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10,
            ],
            "",
        );
        check_serialize(
            &info,
            &[
                0x01, 0x00, // Protocol version
                0x00, 0x00, // Hardware version
                0x00, 0x00, // Software version
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // VCS revision ID
                0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
                0x10, // Unique ID (16 bytes)
                0x00, // Name length
                0x00, // Software image CRC length
                0x00, // Certificate of authenticity length
            ],
        );
        // Replace with new info, with a 50-character name
        let info = NodeInfo::new(
            [
                0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10,
            ],
            "com.mycompany.extremely_long_name.more_long_things",
        );
        check_serialize(
            &info,
            &[
                0x01, 0x00, // Protocol version
                0x00, 0x00, // Hardware version
                0x00, 0x00, // Software version
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // VCS revision ID
                0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
                0x10, // Unique ID (16 bytes)
                0x32, // Name length
                0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x6e, 0x79, 0x2e,
                0x65, 0x78, 0x74, 0x72, 0x65, 0x6d, 0x65, 0x6c, 0x79, 0x5f, 0x6c, 0x6f, 0x6e, 0x67,
                0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x6d, 0x6f, 0x72, 0x65, 0x5f, 0x6c, 0x6f, 0x6e,
                0x67, 0x5f, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x73, // Name (50 bytes)
                0x00, // Software image CRC length
                0x00, // Certificate of authenticity length
            ],
        );
        // Fill in optional fields
        let info = info
            .hardware_version(Version { major: 3, minor: 1 })
            .software_version(Version { major: 9, minor: 7 })
            .revision(0xff92_b621_380c_9dd8)
            .software_crc(0x8123_cdef_300b_18ba)
            .certificate_of_authenticity(&[0xae, 0x92, 0xcd, 0xfe]);
        check_serialize(
            &info,
            &[
                0x01, 0x00, // Protocol version
                0x03, 0x01, // Hardware version
                0x09, 0x07, // Software version
                0xd8, 0x9d, 0x0c, 0x38, 0x21, 0xb6, 0x92, 0xff, // VCS revision ID
                0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
                0x10, // Unique ID (16 bytes)
                0x32, // Name length
                0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x79, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x6e, 0x79, 0x2e,
                0x65, 0x78, 0x74, 0x72, 0x65, 0x6d, 0x65, 0x6c, 0x79, 0x5f, 0x6c, 0x6f, 0x6e, 0x67,
                0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x6d, 0x6f, 0x72, 0x65, 0x5f, 0x6c, 0x6f, 0x6e,
                0x67, 0x5f, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x73, // Name (50 bytes)
                0x01, // Software image CRC length
                0xba, 0x18, 0x0b, 0x30, 0xef, 0xcd, 0x23, 0x81, // Software image CRC
                0x04, // Certificate of authenticity length
                0xae, 0x92, 0xcd, 0xfe,
            ],
        );
    }

    fn check_serialize(info: &NodeInfo, expected_bytes: &[u8]) {
        assert_eq!(info.size_bits(), expected_bytes.len() * 8);
        let mut bytes = vec![0u8; expected_bytes.len()];
        info.serialize(&mut WriteCursor::new(&mut bytes));
        assert_eq!(&*bytes, expected_bytes);
    }
}
