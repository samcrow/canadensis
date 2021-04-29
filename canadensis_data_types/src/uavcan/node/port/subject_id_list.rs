use crate::bits::BitSet;
use crate::uavcan::node::port::subject_id::SubjectId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.port.SubjectIDList version 0.1
#[derive(Debug, Clone)]
pub enum SubjectIdList {
    Mask(BitSet<{ (SubjectIdList::CAPACITY as usize + 7) / 8 }>),
    SparseList(heapless::Vec<SubjectId, 255>),
    Total,
}

impl Default for SubjectIdList {
    fn default() -> Self {
        SubjectIdList::Total
    }
}

impl SubjectIdList {
    const CAPACITY: u16 = SubjectId::MAX + 1;
}

impl DataType for SubjectIdList {
    const EXTENT_BYTES: Option<u32> = Some(4097);
}

impl Serialize for SubjectIdList {
    fn size_bits(&self) -> usize {
        let tag_bits = 8;
        let variant_bits = match self {
            SubjectIdList::Mask(_) => usize::from(SubjectIdList::CAPACITY),
            SubjectIdList::SparseList(items) => {
                items.iter().map(Serialize::size_bits).sum::<usize>() + 8
            }
            SubjectIdList::Total => 0,
        };
        tag_bits + variant_bits
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        match self {
            SubjectIdList::Mask(mask) => {
                // Tag
                cursor.write_aligned_u8(0);
                mask.serialize(cursor);
            }
            SubjectIdList::SparseList(items) => {
                // Tag
                cursor.write_aligned_u8(items.len() as u8);
                for item in items {
                    cursor.write_composite(item);
                }
            }
            SubjectIdList::Total => {
                // Tag
                cursor.write_aligned_u8(2);
            }
        }
    }
}

fn in_sparse_list_bit_length_set(bit_length: usize) -> bool {
    // 8 + 8 + 16 * n where 0 <= n <= 255
    if bit_length < 16 {
        false
    } else {
        let array_element_bits = bit_length - 16;
        array_element_bits / 16 <= 255 && array_element_bits % 16 == 0
    }
}

impl Deserialize for SubjectIdList {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 8 + usize::from(SubjectIdList::CAPACITY)
            || in_sparse_list_bit_length_set(bit_length)
            || bit_length == 8
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
        let tag = cursor.read_aligned_u8();
        match tag {
            0 => Ok(SubjectIdList::Mask(BitSet::deserialize(
                usize::from(SubjectIdList::CAPACITY),
                cursor,
            ))),
            1 => {
                let length = cursor.read_aligned_u8();
                let mut ids = heapless::Vec::new();
                for _ in 0..length {
                    ids.push(cursor.read_composite()?).unwrap();
                }
                Ok(SubjectIdList::SparseList(ids))
            }
            2 => Ok(SubjectIdList::Total),
            _ => Err(DeserializeError::UnionTag),
        }
    }
}
