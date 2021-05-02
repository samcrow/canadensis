use crate::uavcan::node::port::service_id::ServiceId;
use crate::uavcan::node::port::subject_id::SubjectId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.port.ID version 1.0
#[derive(Debug, Clone)]
pub enum Id {
    Subject(SubjectId),
    Service(ServiceId),
}

impl DataType for Id {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Message for Id {}

impl Serialize for Id {
    fn size_bits(&self) -> usize {
        24
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        match self {
            Id::Subject(inner) => {
                cursor.write_aligned_u8(0);
                cursor.write_composite(inner);
            }
            Id::Service(inner) => {
                cursor.write_aligned_u8(1);
                cursor.write_composite(inner);
            }
        }
    }
}

impl Deserialize for Id {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 24
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        let tag = cursor.read_aligned_u8();
        match tag {
            0 => {
                let inner = cursor.read_composite()?;
                *self = Id::Subject(inner);
                Ok(())
            }
            1 => {
                let inner = cursor.read_composite()?;
                *self = Id::Service(inner);
                Ok(())
            }
            _ => Err(DeserializeError::UnionTag),
        }
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let tag = cursor.read_aligned_u8();
        match tag {
            0 => {
                let inner = cursor.read_composite()?;
                Ok(Id::Subject(inner))
            }
            1 => {
                let inner = cursor.read_composite()?;
                Ok(Id::Service(inner))
            }
            _ => Err(DeserializeError::UnionTag),
        }
    }
}
