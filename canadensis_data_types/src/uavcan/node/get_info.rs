use crate::uavcan::node::version::Version;
use canadensis_core::ServiceId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.GetInfo version 1.0 request
#[derive(Debug, Clone, Default)]
pub struct GetInfoRequest {}

impl GetInfoRequest {
    pub const SERVICE: ServiceId = ServiceId::from_truncating(430);
}

impl DataType for GetInfoRequest {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for GetInfoRequest {
    fn size_bits(&self) -> usize {
        0
    }

    fn serialize(&self, _cursor: &mut WriteCursor<'_>) {
        // Nothing to do
    }
}

impl Deserialize for GetInfoRequest {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 0
    }

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
        Ok(GetInfoRequest {})
    }
}

/// uavcan.node.GetInfo version 1.0 response
#[derive(Debug, Clone, Default)]
pub struct GetInfoResponse {
    pub protocol_version: Version,
    pub hardware_version: Version,
    pub software_version: Version,
    pub software_vcs_revision_id: u64,
    pub unique_id: [u8; 16],
    pub name: heapless::Vec<u8, 50>,
    pub software_image_crc: Option<u64>,
    pub certificate_of_authenticity: heapless::Vec<u8, 222>,
}

impl GetInfoResponse {
    pub const SERVICE: ServiceId = ServiceId::from_truncating(430);
}

impl DataType for GetInfoResponse {
    const EXTENT_BYTES: Option<u32> = Some(448);
}

impl Serialize for GetInfoResponse {
    fn size_bits(&self) -> usize {
        // Protocol version
        16
            // Hardware version
            + 16
            // Software version
            + 16
            // VCS revision ID
            + 64
            // Unique ID
            + 8 * 16
            // Name length and bytes
            + 8 + self.name.len() * 8
            // CRC length and value
            + 8 + if self.software_image_crc.is_some() { 64 } else { 0 }
            // Certificate of authenticity length and value
            + 8 + self.certificate_of_authenticity.len() * 8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_composite(&self.protocol_version);
        cursor.write_composite(&self.hardware_version);
        cursor.write_composite(&self.software_version);
        cursor.write_aligned_u64(self.software_vcs_revision_id);
        cursor.write_aligned_bytes(&self.unique_id);
        cursor.write_aligned_u8(self.name.len() as u8);
        cursor.write_aligned_bytes(&self.name);
        if let Some(crc) = self.software_image_crc {
            // Length 1 and value
            cursor.write_aligned_u8(1);
            cursor.write_aligned_u64(crc);
        } else {
            // Length 0
            cursor.write_aligned_u8(0);
        }
        cursor.write_aligned_u8(self.certificate_of_authenticity.len() as u8);
        cursor.write_aligned_bytes(&self.certificate_of_authenticity);
    }
}

impl Deserialize for GetInfoResponse {
    fn in_bit_length_set(bit_length: usize) -> bool {
        // This may be too permissive
        let bytes = bit_length / 8;
        bit_length % 8 == 0 && (33..=313).contains(&bytes)
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.protocol_version = cursor.read_composite()?;
        self.hardware_version = cursor.read_composite()?;
        self.software_version = cursor.read_composite()?;
        self.software_vcs_revision_id = cursor.read_aligned_u64();
        self.unique_id.fill_with(|| cursor.read_aligned_u8());
        self.name.clear();
        let name_length = cursor.read_aligned_u8();
        if name_length > 50 {
            return Err(DeserializeError::ArrayLength);
        }
        for _ in 0..name_length {
            self.name
                .push(cursor.read_aligned_u8())
                .expect("Array too long");
        }
        let crc_length = cursor.read_aligned_u8();
        match crc_length {
            0 => self.software_image_crc = None,
            1 => self.software_image_crc = Some(cursor.read_aligned_u64()),
            _ => return Err(DeserializeError::ArrayLength),
        }
        self.certificate_of_authenticity.clear();
        let coa_length = cursor.read_aligned_u8();
        if coa_length > 222 {
            return Err(DeserializeError::ArrayLength);
        }
        for _ in 0..coa_length {
            self.certificate_of_authenticity
                .push(cursor.read_aligned_u8())
                .expect("Array too long");
        }
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = GetInfoResponse::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
