use crate::bits::BitArray;
use crate::uavcan::node::port::service_id::ServiceId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, Message, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.port.ServiceIDList version 0.1
#[derive(Clone)]
pub struct ServiceIdList {
    pub mask: BitArray<{ (ServiceIdList::CAPACITY as usize + 7) / 8 }>,
}

impl Default for ServiceIdList {
    fn default() -> Self {
        ServiceIdList {
            mask: BitArray::new(usize::from(ServiceIdList::CAPACITY)),
        }
    }
}

impl ServiceIdList {
    const CAPACITY: u16 = ServiceId::MAX + 1;
}

impl DataType for ServiceIdList {
    const EXTENT_BYTES: Option<u32> = Some(128);
}

impl Message for ServiceIdList {}

impl Serialize for ServiceIdList {
    fn size_bits(&self) -> usize {
        usize::from(ServiceIdList::CAPACITY)
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        let start_bits = cursor.bits_written();
        self.mask.serialize(cursor);
        let bits_written = cursor.bits_written() - start_bits;
        debug_assert_eq!(bits_written, self.size_bits());
    }
}

impl Deserialize for ServiceIdList {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == usize::from(ServiceIdList::CAPACITY)
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.mask.deserialize_in_place(cursor);
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        Ok(ServiceIdList {
            mask: BitArray::deserialize(usize::from(ServiceIdList::CAPACITY), cursor),
        })
    }
}
