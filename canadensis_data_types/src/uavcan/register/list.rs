use crate::uavcan::register::name::Name;
use canadensis_core::ServiceId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Request, Response, Serialize, WriteCursor,
};

/// uavcan.register.List 1.0 request
#[derive(Debug, Clone, Default)]
pub struct ListRequest {
    pub index: u16,
}

impl ListRequest {
    pub const SERVICE: ServiceId = ServiceId::from_truncating(385);
}

impl Request for ListRequest {}

impl DataType for ListRequest {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for ListRequest {
    fn size_bits(&self) -> usize {
        16
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u16(self.index);
    }
}

impl Deserialize for ListRequest {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 16
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.index = cursor.read_aligned_u16();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        Ok(ListRequest {
            index: cursor.read_aligned_u16(),
        })
    }
}

/// uavcan.register.List version 1.0 response
#[derive(Debug, Clone, Default)]
pub struct ListResponse {
    pub name: Name,
}

impl ListResponse {
    pub const SERVICE: ServiceId = ListRequest::SERVICE;
}

impl Response for ListResponse {}

impl DataType for ListResponse {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for ListResponse {
    fn size_bits(&self) -> usize {
        self.name.size_bits()
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_composite(&self.name);
    }
}

impl Deserialize for ListResponse {
    fn in_bit_length_set(bit_length: usize) -> bool {
        Name::in_bit_length_set(bit_length)
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.name = cursor.read_composite()?;
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let name = cursor.read_composite()?;
        Ok(ListResponse { name })
    }
}
