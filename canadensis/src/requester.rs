//! Sending of service requests

use heapless::index_map::FnvIndexMap;

use canadensis_core::time::{Clock, MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::{Header, ServiceHeader, Transfer};
use canadensis_core::transport::{TransferId, Transmitter, Transport};
use canadensis_core::{nb, OutOfMemoryError, ServiceId, TransferIdTracker};
use canadensis_encoding::{Request, Serialize};

use crate::serialize::do_serialize;

/// Assembles transfers and manages transfer IDs to send service requests
pub struct Requester<C: Clock, T: Transmitter<C>, R> {
    /// The priority of transfers from this transmitter
    priority: <T::Transport as Transport>::Priority,
    /// The timeout for sending transfers
    timeout: MicrosecondDuration32,
    /// The ID of the next transfer to send, for each destination node
    transfer_ids: R,
}

impl<C: Clock, T: Transmitter<C>, R: TransferIdTracker<T::Transport>> Requester<C, T, R> {
    /// Creates a service request transmitter
    ///
    /// priority: The priority to use for messages
    ///
    /// service: The service ID to request
    pub fn new(
        timeout: MicrosecondDuration32,
        priority: <T::Transport as Transport>::Priority,
    ) -> Self {
        Requester {
            priority,
            timeout,
            transfer_ids: R::default(),
        }
    }

    /// Sends a service request and returns its transfer ID
    pub fn send<Q>(
        &mut self,
        clock: &mut C,
        source: <T::Transport as Transport>::NodeId,
        service: ServiceId,
        payload: &Q,
        destination: <T::Transport as Transport>::NodeId,
        transmitter: &mut T,
        driver: &mut T::Driver,
    ) -> nb::Result<<T::Transport as Transport>::TransferId, T::Error>
    where
        Q: Serialize + Request,
    {
        // Part 1: Serialize
        let deadline = clock.now() + self.timeout;
        do_serialize(payload, |payload_bytes| {
            // Part 2: Split into frames and send
            self.send_payload(
                payload_bytes,
                source,
                service,
                destination,
                deadline,
                false,
                transmitter,
                clock,
                driver,
            )
        })
    }

    /// Sends a loopback service request and returns its transfer ID
    pub fn send_loopback<Q>(
        &mut self,
        clock: &mut C,
        source: <T::Transport as Transport>::NodeId,
        service: ServiceId,
        payload: &Q,
        destination: <T::Transport as Transport>::NodeId,
        transmitter: &mut T,
        driver: &mut T::Driver,
    ) -> nb::Result<<T::Transport as Transport>::TransferId, T::Error>
    where
        Q: Serialize + Request,
    {
        // Part 1: Serialize
        let deadline = clock.now() + self.timeout;
        do_serialize(payload, |payload_bytes| {
            // Part 2: Split into frames and send
            self.send_payload(
                payload_bytes,
                source,
                service,
                destination,
                deadline,
                true,
                transmitter,
                clock,
                driver,
            )
        })
    }

    fn send_payload(
        &mut self,
        payload: &[u8],
        source: <T::Transport as Transport>::NodeId,
        service: ServiceId,
        destination: <T::Transport as Transport>::NodeId,
        deadline: Microseconds32,
        loopback: bool,
        transmitter: &mut T,
        clock: &mut C,
        driver: &mut T::Driver,
    ) -> nb::Result<<T::Transport as Transport>::TransferId, T::Error> {
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
                source,
                destination,
            }),
            loopback,
            payload,
        };

        transmitter.push(transfer, clock, driver)?;
        Ok(transfer_id)
    }
}

/// A fixed-capacity map from destination node IDs to transfer IDs of the next transfer
///
/// This map has a limited capacity and will return an error if asked to keep track of transfer
/// IDs for too many nodes.
///
/// **C must be a power of two and be greater than one** Other values may cause incorrect behavior
/// or a compile-time error.
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

    use canadensis_core::time::Clock;
    use canadensis_core::transport::{Transmitter, Transport};

    use crate::requester::Requester;

    impl<C, T, R> Debug for Requester<C, T, R>
    where
        C: Clock,
        T: Transmitter<C>,
        <T::Transport as Transport>::TransferId: Debug,
        <T::Transport as Transport>::Priority: Debug,
        <T::Transport as Transport>::NodeId: Debug,
        R: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Requester")
                .field("priority", &self.priority)
                .field("timeout", &self.timeout)
                .field("transfer_ids", &self.transfer_ids)
                .finish()
        }
    }
}
