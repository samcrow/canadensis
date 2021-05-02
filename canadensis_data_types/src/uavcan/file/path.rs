use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.file.Path version 2.0
#[derive(Debug, Clone, Default)]
pub struct Path {
    path: heapless::Vec<u8, { Path::MAX_LENGTH as usize }>,
}

impl Path {
    pub const SEPARATOR: u8 = b'/';
    pub const MAX_LENGTH: u8 = 255;
}

impl DataType for Path {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Message for Path {}

impl Serialize for Path {
    fn size_bits(&self) -> usize {
        8 + self.path.len() * 8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u8(self.path.len() as u8);
        cursor.write_aligned_bytes(&self.path);
    }
}

impl Deserialize for Path {
    fn in_bit_length_set(bit_length: usize) -> bool {
        if bit_length < 8 {
            false
        } else {
            let path_bytes_length = bit_length - 8;
            path_bytes_length % 8 == 0 && path_bytes_length / 8 <= usize::from(Path::MAX_LENGTH)
        }
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        let length = cursor.read_aligned_u8();
        // Because the maximum length equals the maximum u8 value, no length check is needed.
        self.path.clear();
        for _ in 0..length {
            self.path.push(cursor.read_aligned_u8()).unwrap();
        }
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = Path::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
