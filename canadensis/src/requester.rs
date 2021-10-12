//! Sending of service requests

use crate::hash::TrivialIndexMap;
use crate::serialize::do_serialize;
use canadensis_core::time::Instant;
use canadensis_core::transfer::{Header, ServiceHeader, Transfer};
use canadensis_core::transport::{NodeId, TransferId, Transmitter, Transport};
use canadensis_core::{OutOfMemoryError, ServiceId};
use canadensis_encoding::{Request, Serialize};

/// Assembles transfers and manages transfer IDs to send service requests
pub struct Requester<
    I: Instant,
    T: Transmitter<I>,
    R = TransferIdArray<<T as Transmitter<I>>::Transport>,
> {
    /// The ID of this node
    this_node: <T::Transport as Transport>::NodeId,
    /// The priority of transfers from this transmitter
    priority: <T::Transport as Transport>::Priority,
    /// The timeout for sending transfers
    timeout: I::Duration,
    /// The ID of the next transfer to send, for each destination node
    transfer_ids: R,
}

impl<I: Instant, T: Transmitter<I>, R: TransferIdTracker<T::Transport>> Requester<I, T, R> {
    /// Creates a service request transmitter
    ///
    /// this_node: The ID of this node
    ///
    /// priority: The priority to use for messages
    ///
    /// service: The service ID to request
    pub fn new(
        this_node: <T::Transport as Transport>::NodeId,
        timeout: I::Duration,
        priority: <T::Transport as Transport>::Priority,
    ) -> Self {
        Requester {
            this_node,
            priority,
            timeout,
            transfer_ids: R::default(),
        }
    }

    /// Sends a service request and returns its transfer ID
    pub fn send<Q>(
        &mut self,
        now: I,
        service: ServiceId,
        payload: &Q,
        destination: <T::Transport as Transport>::NodeId,
        transmitter: &mut T,
    ) -> Result<<T::Transport as Transport>::TransferId, <T::Transport as Transport>::Error>
    where
        Q: Serialize + Request,
    {
        // Part 1: Serialize
        let deadline = self.timeout + now;
        do_serialize(payload, |payload_bytes| {
            // Part 2: Split into frames and send
            self.send_payload(payload_bytes, service, destination, deadline, transmitter)
        })
    }

    fn send_payload(
        &mut self,
        payload: &[u8],
        service: ServiceId,
        destination: <T::Transport as Transport>::NodeId,
        deadline: I,
        transmitter: &mut T,
    ) -> Result<<T::Transport as Transport>::TransferId, <T::Transport as Transport>::Error> {
        // Assemble the transfer
        let transfer_id = self.transfer_ids.next_transfer_id(destination.clone())?;
        let transfer = Transfer {
            header: Header::Request(ServiceHeader {
                timestamp: deadline,
                transfer_id: transfer_id.clone(),
                priority: self.priority.clone(),
                service,
                source: self.this_node.clone(),
                destination,
            }),
            payload,
        };

        transmitter.push(transfer)?;
        Ok(transfer_id)
    }
}

/// Something that can keep track of the next transfer ID to use for each destination node
pub trait TransferIdTracker<T: Transport>: Default {
    /// Returns the next transfer ID for the provided node, and increments the stored ID
    fn next_transfer_id(
        &mut self,
        destination: T::NodeId,
    ) -> Result<T::TransferId, OutOfMemoryError>;
}

/// A fixed-capacity map from destination node IDs to transfer IDs of the next transfer
///
/// This map has a limited capacity and will return an error if asked to keep track of transfer
/// IDs for too many nodes.
pub struct TransferIdFixedMap<T: Transport, const C: usize> {
    ids: TrivialIndexMap<T::NodeId, T::TransferId, C>,
}

impl<T: Transport, const C: usize> Default for TransferIdFixedMap<T, C> {
    fn default() -> Self {
        TransferIdFixedMap {
            ids: TrivialIndexMap::default(),
        }
    }
}
impl<T: Transport, const C: usize> TransferIdTracker<T> for TransferIdFixedMap<T, C> {
    fn next_transfer_id(
        &mut self,
        destination: T::NodeId,
    ) -> Result<T::TransferId, OutOfMemoryError> {
        match self.ids.get_mut(&destination) {
            Some(entry) => {
                let current = entry.clone();
                *entry = entry.clone().increment();
                Ok(current)
            }
            None => {
                let current = T::TransferId::default();
                let next = current.clone().increment();
                // Try to store the next transfer ID
                match self.ids.insert(destination, next) {
                    Ok(_) => Ok(current),
                    Err(_) => Err(OutOfMemoryError),
                }
            }
        }
    }
}

/// A map from destination node IDs to transfer IDs of the next transfer
///
/// This implementation contains a fixed-size array with one transfer ID for every possible
/// node ID. With transports that allow a large range of node IDs, it may be too large.
pub struct TransferIdArray<T: Transport> {
    ids: <T::NodeId as NodeId<T::TransferId>>::TransferIds,
}

impl<T: Transport> Default for TransferIdArray<T> {
    fn default() -> Self {
        TransferIdArray {
            ids: Default::default(),
        }
    }
}

impl<T: Transport> TransferIdTracker<T> for TransferIdArray<T> {
    fn next_transfer_id(
        &mut self,
        destination: T::NodeId,
    ) -> Result<T::TransferId, OutOfMemoryError> {
        let entry = &mut self.ids.as_mut()[destination.into()];
        let current = entry.clone();
        *entry = entry.clone().increment();
        Ok(current)
    }
}
