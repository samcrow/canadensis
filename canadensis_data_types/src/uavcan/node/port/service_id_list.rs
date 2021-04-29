use crate::bits::BitSet;
use crate::uavcan::node::port::service_id::ServiceId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor,
};

/// uavcan.node.port.ServiceIDList version 0.1
#[derive(Debug, Clone)]
pub struct ServiceIdList {
    pub mask: BitSet<{ (ServiceIdList::CAPACITY as usize + 7) / 8 }>,
}

impl Default for ServiceIdList {
    fn default() -> Self {
        ServiceIdList {
            mask: BitSet::new(usize::from(ServiceIdList::CAPACITY)),
        }
    }
}

impl ServiceIdList {
    const CAPACITY: u16 = ServiceId::MAX + 1;
}

impl DataType for ServiceIdList {
    const EXTENT_BYTES: Option<u32> = Some(128);
}

impl Serialize for ServiceIdList {
    fn size_bits(&self) -> usize {
        usize::from(ServiceIdList::CAPACITY)
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        self.mask.serialize(cursor);
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
            mask: BitSet::deserialize(usize::from(ServiceIdList::CAPACITY), cursor),
        })
    }
}
