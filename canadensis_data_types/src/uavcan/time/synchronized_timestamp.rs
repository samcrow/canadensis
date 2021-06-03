use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.time.SynchronizedTimestamp version 1.0
#[derive(Clone, Default)]
pub struct SynchronizedTimestamp {
    // Really 56 bits
    pub microsecond: u64,
}

impl SynchronizedTimestamp {
    pub const UNKNOWN: SynchronizedTimestamp = SynchronizedTimestamp { microsecond: 0 };
}

impl Message for SynchronizedTimestamp {}

impl DataType for SynchronizedTimestamp {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for SynchronizedTimestamp {
    fn size_bits(&self) -> usize {
        56
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u56(self.microsecond);
    }
}

impl Deserialize for SynchronizedTimestamp {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 56
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.microsecond = cursor.read_u56();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        Ok(SynchronizedTimestamp {
            microsecond: cursor.read_u56(),
        })
    }
}
