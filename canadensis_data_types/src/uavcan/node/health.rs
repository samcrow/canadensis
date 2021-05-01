use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.Health version 1.0
#[derive(Debug, Clone)]
pub enum Health {
    Nominal = 0,
    Advisory = 1,
    Caution = 2,
    Warning = 3,
}

impl Default for Health {
    fn default() -> Self {
        Health::Nominal
    }
}

impl DataType for Health {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for Health {
    fn size_bits(&self) -> usize {
        // Size gets rounded up to 8 because this is a composite type
        8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u2(self.clone() as u8);
    }
}

impl Deserialize for Health {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 8
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        *self = Health::deserialize(cursor)?;
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let health = match cursor.read_u2() {
            0 => Health::Nominal,
            1 => Health::Advisory,
            2 => Health::Caution,
            3 => Health::Warning,
            _ => unreachable!("A 2-bit integer can't be greater than 3"),
        };
        Ok(health)
    }
}
