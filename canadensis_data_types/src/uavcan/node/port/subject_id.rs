use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.port.SubjectID version 1.0
#[derive(Clone, Eq, PartialEq)]
pub struct SubjectId {
    // Really u13
    pub value: u16,
}

impl SubjectId {
    pub const MAX: u16 = 8191;
}

impl DataType for SubjectId {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Message for SubjectId {}

impl Serialize for SubjectId {
    fn size_bits(&self) -> usize {
        16
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u13(self.value);
    }
}

impl Deserialize for SubjectId {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 16
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.value = cursor.read_u13();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        Ok(SubjectId {
            value: cursor.read_u13(),
        })
    }
}
