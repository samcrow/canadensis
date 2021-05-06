use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.register.Name version 1.0
#[derive(Debug, Clone, Default)]
pub struct Name {
    pub name: heapless::Vec<u8, 255>,
}

impl Message for Name {}

impl DataType for Name {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for Name {
    fn size_bits(&self) -> usize {
        8 + 8 * self.name.len()
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u8(self.name.len() as u8);
        cursor.write_aligned_bytes(&self.name);
    }
}

impl Deserialize for Name {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length % 8 == 0 && (1..=256).contains(&(bit_length / 8))
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.name.clear();
        let length = cursor.read_aligned_u8();
        if usize::from(length) <= self.name.capacity() {
            for _ in 0..length {
                self.name.push(cursor.read_aligned_u8()).unwrap();
            }
            Ok(())
        } else {
            Err(DeserializeError::ArrayLength)
        }
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = Name::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
