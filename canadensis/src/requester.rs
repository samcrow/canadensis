//! Sending of service requests

use heapless::FnvIndexMap;

use canadensis_core::time::{Clock, Instant};
use canadensis_core::transfer::{Header, ServiceHeader, Transfer};
use canadensis_core::transport::{TransferId, Transmitter, Transport};
use canadensis_core::{nb, OutOfMemoryError, ServiceId};
use canadensis_encoding::{Request, Serialize};

use crate::serialize::do_serialize;

/// Assembles transfers and manages transfer IDs to send service requests
pub struct Requester<I: Instant, T: Transmitter<I>, R> {
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
    pub fn send<Q, C>(
        &mut self,
        clock: &mut C,
        service: ServiceId,
        payload: &Q,
        destination: <T::Transport as Transport>::NodeId,
        transmitter: &mut T,
        driver: &mut T::Driver,
    ) -> nb::Result<<T::Transport as Transport>::TransferId, T::Error>
    where
        Q: Serialize + Request,
        C: Clock<Instant = I>,
    {
        // Part 1: Serialize
        let deadline = self.timeout + clock.now();
        do_serialize(payload, |payload_bytes| {
            // Part 2: Split into frames and send
            self.send_payload(
                payload_bytes,
                service,
                destination,
                deadline,
                transmitter,
                clock,
                driver,
            )
        })
    }

    fn send_payload<C>(
        &mut self,
        payload: &[u8],
        service: ServiceId,
        destination: <T::Transport as Transport>::NodeId,
        deadline: I,
        transmitter: &mut T,
        clock: &mut C,
        driver: &mut T::Driver,
    ) -> nb::Result<<T::Transport as Transport>::TransferId, T::Error>
    where
        C: Clock<Instant = I>,
    {
        // Assemble the transfer
        let transfer_id = self
            .transfer_ids
            .next_transfer_id(destination.clone())
            .map_err(|oom| nb::Error::Other(oom.into()))?;
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

        transmitter.push(transfer, clock, driver)?;
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
    ids: FnvIndexMap<T::NodeId, T::TransferId, C>,
}

impl<T: Transport, const C: usize> Default for TransferIdFixedMap<T, C> {
    fn default() -> Self {
        TransferIdFixedMap {
            ids: FnvIndexMap::default(),
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

mod fmt_impl {
    use core::fmt::{Debug, Formatter, Result};

    use canadensis_core::time::Instant;
    use canadensis_core::transport::{Transmitter, Transport};

    use crate::requester::Requester;

    impl<I, T, R> Debug for Requester<I, T, R>
    where
        I: Instant,
        T: Transmitter<I>,
        I::Duration: Debug,
        <T::Transport as Transport>::TransferId: Debug,
        <T::Transport as Transport>::Priority: Debug,
        <T::Transport as Transport>::NodeId: Debug,
        R: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Requester")
                .field("this_node", &self.this_node)
                .field("priority", &self.priority)
                .field("timeout", &self.timeout)
                .field("transfer_ids", &self.transfer_ids)
                .finish()
        }
    }
}
