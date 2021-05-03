use crate::uavcan::diagnostic::severity::Severity;
use crate::uavcan::time::synchronized_timestamp::SynchronizedTimestamp;
use canadensis_core::SubjectId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.diagnostic.Record version 1.1
#[derive(Debug, Clone, Default)]
pub struct Record {
    pub timestamp: SynchronizedTimestamp,
    pub severity: Severity,
    pub text: heapless::Vec<u8, 255>,
}

impl Record {
    pub const SUBJECT: SubjectId = SubjectId::from_truncating(8184);
}

impl Message for Record {}

impl DataType for Record {
    const EXTENT_BYTES: Option<u32> = Some(300);
}

impl Serialize for Record {
    fn size_bits(&self) -> usize {
        self.timestamp.size_bits() + self.severity.size_bits() + 8 + self.text.len() * 8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_composite(&self.timestamp);
        cursor.write_composite(&self.severity);
        cursor.write_aligned_u8(self.text.len() as u8);
        for &byte in self.text.iter() {
            cursor.write_aligned_u8(byte);
        }
    }
}

impl Deserialize for Record {
    fn in_bit_length_set(bit_length: usize) -> bool {
        // 56 bits time + 8 bits severity + 8 bits text length = 72 bits minimum
        if bit_length < 72 {
            false
        } else {
            let text_bytes_bits = bit_length - 72;
            text_bytes_bits % 8 == 0 && text_bytes_bits / 8 <= 255
        }
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.timestamp = cursor.read_composite()?;
        self.severity = cursor.read_composite()?;
        self.text.clear();
        let text_length = cursor.read_aligned_u8();
        for _ in 0..text_length {
            self.text
                .push(cursor.read_aligned_u8())
                .expect("Array too long");
        }
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = Record::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
