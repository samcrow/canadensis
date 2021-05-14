use canadensis_core::SubjectId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.pnp.NodeIDAllocationData 1.0
#[derive(Debug, Clone, Default)]
pub struct NodeAllocationData {
    // Really 48 bits
    pub unique_id_hash: u64,
    pub allocated_node_id: Option<u16>,
}

impl NodeAllocationData {
    pub const SUBJECT: SubjectId = SubjectId::from_truncating(8166);
}

impl Message for NodeAllocationData {}

impl DataType for NodeAllocationData {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for NodeAllocationData {
    fn size_bits(&self) -> usize {
        let node_id_bits = if self.allocated_node_id.is_some() {
            16
        } else {
            0
        };
        48 + 8 + node_id_bits
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u48(self.unique_id_hash);
        if let Some(node_id) = self.allocated_node_id {
            // Length = 1
            cursor.write_aligned_u8(1);
            cursor.write_aligned_u16(node_id);
        } else {
            // Length = 0
            cursor.write_aligned_u8(0);
        }
    }
}

impl Deserialize for NodeAllocationData {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 56 || bit_length == 72
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.unique_id_hash = cursor.read_u48();
        let node_id_length = cursor.read_aligned_u8();
        match node_id_length {
            0 => {
                // No node id
                self.allocated_node_id = None;
            }
            1 => {
                let node_id = cursor.read_aligned_u16();
                self.allocated_node_id = Some(node_id);
            }
            _ => return Err(DeserializeError::ArrayLength),
        }
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = NodeAllocationData::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
