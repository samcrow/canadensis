use crate::uavcan::register::name::Name;
use crate::uavcan::register::value::Value;
use crate::uavcan::time::synchronized_timestamp::SynchronizedTimestamp;
use canadensis_core::ServiceId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Request, Response, Serialize, WriteCursor,
};

/// uavcan.register.Access version 1.0 request
#[derive(Debug, Clone, Default)]
pub struct AccessRequest {
    pub name: Name,
    pub value: Value,
}

impl AccessRequest {
    pub const SERVICE: ServiceId = ServiceId::from_truncating(384);
}

impl Request for AccessRequest {}

impl DataType for AccessRequest {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for AccessRequest {
    fn size_bits(&self) -> usize {
        self.name.size_bits() + self.value.size_bits()
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_composite(&self.name);
        cursor.write_composite(&self.value);
    }
}

impl Deserialize for AccessRequest {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length % 8 == 0 && (2..=515).contains(&(bit_length / 8))
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.name = cursor.read_composite()?;
        self.value = cursor.read_composite()?;
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let name = cursor.read_composite()?;
        let value = cursor.read_composite()?;
        Ok(AccessRequest { name, value })
    }
}

/// uavcan.register.Access version 1.0 response
#[derive(Debug, Clone, Default)]
pub struct AccessResponse {
    pub timestamp: SynchronizedTimestamp,
    pub mutable: bool,
    pub persistent: bool,
    pub value: Value,
}

impl AccessResponse {
    pub const SERVICE: ServiceId = AccessRequest::SERVICE;
}

impl Response for AccessResponse {}

impl DataType for AccessResponse {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for AccessResponse {
    fn size_bits(&self) -> usize {
        // 8 bits for the two flags and 8 padding bits
        self.timestamp.size_bits() + 8 + self.value.size_bits()
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_composite(&self.timestamp);
        cursor.write_bool(self.mutable);
        cursor.write_bool(self.persistent);
        cursor.skip_6();
        cursor.write_composite(&self.value);
    }
}

impl Deserialize for AccessResponse {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length % 8 == 0 && (9..=267).contains(&(bit_length / 8))
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.timestamp = cursor.read_composite()?;
        self.mutable = cursor.read_bool();
        self.persistent = cursor.read_bool();
        cursor.skip_6();
        self.value = cursor.read_composite()?;
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = AccessResponse::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
