use crate::uavcan::node::port::io_statistics::IoStatistics;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor,
};
use core::cmp;

/// uavcan.node.GetTransportStatistics version 0.1 request
#[derive(Debug, Clone, Default)]
pub struct GetTransportStatisticsRequest;

impl DataType for GetTransportStatisticsRequest {
    // Sealed type
    const EXTENT_BYTES: Option<u32> = None;
}

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
    /// Allowed range 0..3
    // TODO: Make this less error-prone. Use a regular Vec or heapless::Vec?
    pub network_interface_statistics_length: u8,
    pub network_interface_statistics:
        [IoStatistics; GetTransportStatisticsResponse::MAX_NETWORK_INTERFACES as usize],
}

impl GetTransportStatisticsResponse {
    const MAX_NETWORK_INTERFACES: u8 = 3;

    fn real_length(&self) -> u8 {
        cmp::min(self.network_interface_statistics_length, 2)
    }
}

impl DataType for GetTransportStatisticsResponse {
    const EXTENT_BYTES: Option<u32> = Some(192);
}

impl Serialize for GetTransportStatisticsResponse {
    fn size_bits(&self) -> usize {
        120 + usize::from(self.real_length()) * 120
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_composite(&self.transfer_statistics);
        let real_length = self.real_length();
        cursor.write_aligned_u8(real_length);
        for interface_stats in &self.network_interface_statistics[..usize::from(real_length)] {
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
        let length_read = cursor.read_aligned_u8();
        if length_read <= GetTransportStatisticsResponse::MAX_NETWORK_INTERFACES {
            self.network_interface_statistics_length = length_read;
            for slot in &mut self.network_interface_statistics[..usize::from(length_read)] {
                *slot = cursor.read_composite()?;
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
