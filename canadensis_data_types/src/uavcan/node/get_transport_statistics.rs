use crate::uavcan::node::port::io_statistics::IoStatistics;
use canadensis_core::ServiceId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Request, Response, Serialize, WriteCursor,
};

/// uavcan.node.GetTransportStatistics version 0.1 request
#[derive(Debug, Clone, Default)]
pub struct GetTransportStatisticsRequest;

impl GetTransportStatisticsRequest {
    pub const SERVICE: ServiceId = ServiceId::from_truncating(434);
}

impl DataType for GetTransportStatisticsRequest {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

impl Request for GetTransportStatisticsRequest {}

impl Serialize for GetTransportStatisticsRequest {
    fn size_bits(&self) -> usize {
        0
    }

    fn serialize(&self, _cursor: &mut WriteCursor<'_>) {
        // Nothing to do
    }
}

impl Deserialize for GetTransportStatisticsRequest {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 0
    }

    fn deserialize_in_place(
        &mut self,
        _cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        // Nothing to do
        Ok(())
    }

    fn deserialize(_cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        // Nothing to do
        Ok(GetTransportStatisticsRequest)
    }
}

/// uavcan.node.GetTransportStatistics version 0.1 response
#[derive(Debug, Clone, Default)]
pub struct GetTransportStatisticsResponse {
    pub transfer_statistics: IoStatistics,
    pub network_interface_statistics: heapless::Vec<
        IoStatistics,
        { GetTransportStatisticsResponse::MAX_NETWORK_INTERFACES as usize },
    >,
}

impl GetTransportStatisticsResponse {
    pub const SERVICE: ServiceId = ServiceId::from_truncating(434);
    pub const MAX_NETWORK_INTERFACES: u8 = 3;
}

impl DataType for GetTransportStatisticsResponse {
    const EXTENT_BYTES: Option<u32> = Some(192);
}

impl Response for GetTransportStatisticsResponse {}

impl Serialize for GetTransportStatisticsResponse {
    fn size_bits(&self) -> usize {
        120 + self.network_interface_statistics.len() * 120
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_composite(&self.transfer_statistics);
        cursor.write_aligned_u8(self.network_interface_statistics.len() as u8);
        for interface_stats in &self.network_interface_statistics {
            cursor.write_composite(interface_stats);
        }
    }
}

impl Deserialize for GetTransportStatisticsResponse {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 120 + 8
            || bit_length == 120 + 8 + 120
            || bit_length == 120 + 8 + 120 * 2
            || bit_length == 120 + 8 + 120 * 3
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.transfer_statistics = cursor.read_composite()?;
        self.network_interface_statistics.clear();
        let length_read = cursor.read_aligned_u8();
        if length_read <= GetTransportStatisticsResponse::MAX_NETWORK_INTERFACES {
            for _ in 0..length_read {
                self.network_interface_statistics
                    .push(cursor.read_composite()?)
                    .expect("Array too long");
            }
            Ok(())
        } else {
            Err(DeserializeError::ArrayLength)
        }
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = GetTransportStatisticsResponse::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
