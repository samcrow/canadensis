use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.Mode version 1.0
#[derive(Debug, Clone)]
pub enum Mode {
    Operational,
    Initialization,
    Maintenance,
    SoftwareUpdate,
    /// Some other mode (the enclosed value is really a 3-bit integer)
    Other(u8),
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Operational
    }
}

impl DataType for Mode {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Message for Mode {}

impl Serialize for Mode {
    fn size_bits(&self) -> usize {
        // Round up to 8 because this is a composite type
        8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        let bits = match self {
            Mode::Operational => 0,
            Mode::Initialization => 1,
            Mode::Maintenance => 2,
            Mode::SoftwareUpdate => 3,
            Mode::Other(other) => other & 0b111,
        };
        cursor.write_u3(bits);
    }
}

impl Deserialize for Mode {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 8
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        *self = Mode::deserialize(cursor)?;
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let bits = cursor.read_u3();
        let mode = match bits {
            0 => Mode::Operational,
            1 => Mode::Initialization,
            2 => Mode::Maintenance,
            3 => Mode::SoftwareUpdate,
            other => Mode::Other(other),
        };
        Ok(mode)
    }
}
