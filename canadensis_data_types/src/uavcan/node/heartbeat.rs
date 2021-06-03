use crate::uavcan::node::health::Health;
use crate::uavcan::node::mode::Mode;
use canadensis_core::SubjectId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.Heartbeat version 1.0
#[derive(Clone, Default)]
pub struct Heartbeat {
    pub uptime: u32,
    pub health: Health,
    pub mode: Mode,
    pub vendor_specific_status_code: u8,
}

impl Heartbeat {
    pub const SUBJECT: SubjectId = SubjectId::from_truncating(7509);
    pub const MAX_PUBLICATION_PERIOD: u16 = 1;
    pub const OFFLINE_TIMEOUT: u16 = 3;
}

impl DataType for Heartbeat {
    const EXTENT_BYTES: Option<u32> = Some(12);
}

impl Message for Heartbeat {}

impl Serialize for Heartbeat {
    fn size_bits(&self) -> usize {
        56
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u32(self.uptime);
        cursor.write_composite(&self.health);
        cursor.write_composite(&self.mode);
        cursor.write_aligned_u8(self.vendor_specific_status_code);
    }
}

impl Deserialize for Heartbeat {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 56
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.uptime = cursor.read_aligned_u32();
        self.health = cursor.read_composite()?;
        self.mode = cursor.read_composite()?;
        self.vendor_specific_status_code = cursor.read_aligned_u8();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = Heartbeat::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
