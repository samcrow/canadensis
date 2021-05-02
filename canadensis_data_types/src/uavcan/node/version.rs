use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.Version version 1.0
#[derive(Debug, Clone, Default)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl DataType for Version {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Message for Version {}

impl Serialize for Version {
    fn size_bits(&self) -> usize {
        16
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u8(self.major);
        cursor.write_aligned_u8(self.minor);
    }
}

impl Deserialize for Version {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 16
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        *self = Version::deserialize(cursor)?;
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let major = cursor.read_aligned_u8();
        let minor = cursor.read_aligned_u8();
        Ok(Version { major, minor })
    }
}
