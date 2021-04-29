use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.IOStatistics version 0.1
#[derive(Debug, Clone, Default)]
pub struct IoStatistics {
    // Really u40
    pub num_emitted: u64,
    // Really u40
    pub num_received: u64,
    // Really u40
    pub num_errored: u64,
}

impl DataType for IoStatistics {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for IoStatistics {
    fn size_bits(&self) -> usize {
        120
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u40(self.num_emitted);
        cursor.write_u40(self.num_received);
        cursor.write_u40(self.num_errored);
    }
}

impl Deserialize for IoStatistics {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 120
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.num_emitted = cursor.read_u40();
        self.num_received = cursor.read_u40();
        self.num_errored = cursor.read_u40();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = IoStatistics {
            num_emitted: 0,
            num_received: 0,
            num_errored: 0,
        };
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
