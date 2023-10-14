use heapless::Vec;
use canadensis_core::SubjectId;
use canadensis_encoding::{DeserializeError, ReadCursor, WriteCursor};

const SUBJECT_ID_SPARSE_LIST_DISCRIMINANT: u8 = 1;

/// A fixed-capacity list of subject IDs that is compatible with
/// `uavcan.node.port.SubjectIDList.1.0`
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SubjectIdList<const N: usize>(pub Vec<SubjectId, N>);

impl<const N: usize> SubjectIdList<N> {
    // N must be less than 256 as defined in the DSDL
    const _ASSERT_SIZE: usize = 256 - N - 1;

    pub fn new() -> Self {
        SubjectIdList(Vec::new())
    }
}

impl<const N: usize> canadensis_encoding::DataType for SubjectIdList<N> {
    const EXTENT_BYTES: Option<u32> = crate::uavcan::node::port::subject_id_list_1_0::SubjectIDList::EXTENT_BYTES;
}
impl<const N: usize> canadensis_encoding::Message for SubjectIdList<N> {}
impl<const N: usize> canadensis_encoding::Serialize for SubjectIdList<N> {
    fn size_bits(&self) -> usize {
        // 8 bits of union discriminant, 8 bits of length, 16 bits for each ID
        8 + 8 + self.0.len() * 16
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        // Discriminant
        cursor.write_aligned_u8(SUBJECT_ID_SPARSE_LIST_DISCRIMINANT);
        // Length
        cursor.write_aligned_u8(self.0.len() as u8);
        for &value in &self.0 {
            cursor.write_aligned_u16(value.into())
        }
    }
}
impl<const N: usize> canadensis_encoding::Deserialize for SubjectIdList<N> {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError> where Self: Sized {
        let discriminant = cursor.read_aligned_u8();
        if discriminant != SUBJECT_ID_SPARSE_LIST_DISCRIMINANT {
            // Not a sparse list
            return Err(DeserializeError::UnionTag)
        }
        let length = cursor.read_aligned_u8();
        if usize::from(length) > N {
            return Err(DeserializeError::ArrayLength)
        }
        let mut ids = Vec::new();
        for _ in 0..length {
            // Push can't fail because of the above length check
            let _ = ids.push(SubjectId::from_truncating(cursor.read_aligned_u16()));
        }
        Ok(SubjectIdList(ids))
    }
}
