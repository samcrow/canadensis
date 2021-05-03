use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.diagnostic.Severity version 1.0
#[derive(Debug, Clone)]
pub enum Severity {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Notice = 3,
    Warning = 4,
    Error = 5,
    Critical = 6,
    Alert = 7,
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Notice
    }
}

impl DataType for Severity {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Message for Severity {}

impl Serialize for Severity {
    fn size_bits(&self) -> usize {
        // Rounded up to 8 because this is a composite type
        8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u8(self.clone() as u8);
    }
}

impl Deserialize for Severity {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 8
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        *self = Self::deserialize(cursor)?;
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        // Read a 3-bit integer, which covers all possible values
        let severity = match cursor.read_aligned_u8() & 0b111 {
            0 => Severity::Trace,
            1 => Severity::Debug,
            2 => Severity::Info,
            3 => Severity::Notice,
            4 => Severity::Warning,
            5 => Severity::Error,
            6 => Severity::Alert,
            7 => Severity::Critical,
            _ => unreachable!("Invalid value for 3-bit integer"),
        };
        Ok(severity)
    }
}
