/// Cyphal plug-and-play client
pub mod client;

/// Cyphal plug-and-play server
pub mod server;

use crate::Node;
use canadensis_core::transport::{Receiver, Transmitter, Transport};
use canadensis_core::SubjectId;
use canadensis_data_types::uavcan::node::id_1_0;
use canadensis_data_types::uavcan::pnp::{
    node_id_allocation_data_1_0, node_id_allocation_data_2_0,
};
use canadensis_encoding::{Deserialize, Message, Serialize};
use core::convert::TryFrom;
use crc_any::CRCu64;

/// A node ID allocation message
pub trait AllocationMessage<T: Transport>: Message + Serialize + Deserialize {
    /// The fixed subject ID for this message
    const SUBJECT: SubjectId;

    /// The maximum payload size for this message
    const PAYLOAD_SIZE_MAX: usize;

    /// Creates a message with the provided unique ID and no allocated node ID
    ///
    /// The message must fit into one frame of the transport that is being used.
    fn with_unique_id(id: &[u8; 16]) -> Self;

    /// Determines if this message matches the provided unique ID
    fn matches_unique_id(&self, id: &[u8; 16]) -> bool;

    /// Returns the allocated node ID in this message, if one is specified
    fn node_id(&self) -> Option<T::NodeId>;

    /// Sets the allocated or requested node ID
    fn with_node_id(self, id: T::NodeId) -> Self;
}

impl<T: Transport> AllocationMessage<T> for node_id_allocation_data_1_0::NodeIDAllocationData
where
    T::NodeId: Into<u16>,
{
    const SUBJECT: SubjectId = node_id_allocation_data_1_0::SUBJECT;
    const PAYLOAD_SIZE_MAX: usize = 9;

    fn with_unique_id(id: &[u8; 16]) -> Self {
        let id_hash = crc_64we_48_bits(id);
        Self {
            unique_id_hash: id_hash,
            allocated_node_id: heapless::Vec::new(),
        }
    }

    fn matches_unique_id(&self, id: &[u8; 16]) -> bool {
        let id_hash = crc_64we_48_bits(id);
        self.unique_id_hash == id_hash
    }

    fn node_id(&self) -> Option<T::NodeId> {
        self.allocated_node_id.iter().next().and_then(|id| {
            // The message may allow a wider range of node IDs than the transport allows.
            // If the ID is too large, return None.
            T::NodeId::try_from(id.value).ok()
        })
    }

    fn with_node_id(self, id: T::NodeId) -> Self {
        Self {
            unique_id_hash: self.unique_id_hash,
            allocated_node_id: heapless::Vec::from_array([id_1_0::ID { value: id.into() }]),
        }
    }
}

/// Calculates a CRC-64WE hash of the provided ID and returns the less significant 48 bits of the
/// result
fn crc_64we_48_bits(id: &[u8; 16]) -> u64 {
    let mut crc = CRCu64::crc64we();
    crc.digest(id);
    let value = crc.get_crc();
    value & 0x0000_ffff_ffff_ffff
}

impl<T: Transport> AllocationMessage<T> for node_id_allocation_data_2_0::NodeIDAllocationData
where
    T::NodeId: Into<u16>,
{
    const SUBJECT: SubjectId = node_id_allocation_data_2_0::SUBJECT;
    const PAYLOAD_SIZE_MAX: usize = 18;

    fn with_unique_id(id: &[u8; 16]) -> Self {
        Self {
            unique_id: *id,
            node_id: id_1_0::ID { value: 0 },
        }
    }

    fn matches_unique_id(&self, id: &[u8; 16]) -> bool {
        self.unique_id == *id
    }

    fn node_id(&self) -> Option<<T as Transport>::NodeId> {
        // The message may allow a wider range of node IDs than the transport allows.
        // If the ID is too large, return None.
        T::NodeId::try_from(self.node_id.value).ok()
    }

    fn with_node_id(self, id: T::NodeId) -> Self {
        Self {
            unique_id: self.unique_id,
            node_id: id_1_0::ID { value: id.into() },
        }
    }
}

/// Error type returned by [`PnpClientService::new`].
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NewError<N: Node> {
    /// The client could not subscribe to the message subject due to a receiver error.
    Subscribe(<N::Receiver as Receiver<N::Clock>>::Error),
    /// The client could not allocate a publish token due to an out of memory error.
    OutOfMemory,
    /// The client could not allocate a publish token as the subject is already in use.
    Duplicate,
    /// The client could not allocate a publish token due to a transmitter error.
    Publish(<N::Transmitter as Transmitter<N::Clock>>::Error),
}
