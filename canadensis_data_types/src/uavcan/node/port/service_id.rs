use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.port.ServiceID version 1.0
#[derive(Clone)]
pub struct ServiceId {
    // Really u9
    pub value: u16,
}

impl ServiceId {
    pub const MAX: u16 = 511;
}

impl DataType for ServiceId {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Message for ServiceId {}

impl Serialize for ServiceId {
    fn size_bits(&self) -> usize {
        16
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u9(self.value);
    }
}

impl Deserialize for ServiceId {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 16
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.value = cursor.read_u9();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        Ok(ServiceId {
            value: cursor.read_u9(),
        })
    }
}
