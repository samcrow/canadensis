//! Transport layer traits

use crate::error::{OutOfMemoryError, ServiceSubscribeError};
use crate::time::{Clock, Instant};
use crate::transfer::Transfer;
use crate::{ServiceId, SubjectId};
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::fmt::Debug;
use hash32::Hash;

/// Basic requirements for a transport that can be used to send and receive transfers
///
/// The [`Transmitter`](Transmitter) and [`Receiver`](Receiver) sub-traits add additional functions.
pub trait Transport {
    /// A node ID type that can hold the node ID values that this transport allows
    type NodeId: Debug + Clone + PartialEq + Eq + Hash + Into<usize> + TryFrom<u16>;
    /// A transfer ID type that can hold all supported transfer ID values
    type TransferId: TransferId;
    /// A priority type that can hold all supported priority values
    type Priority: Clone + Debug + From<crate::Priority>;
}

/// A transmitter that can send outgoing transfers
pub trait Transmitter<I>
where
    I: Instant,
{
    /// The transport that this transmitter works with
    type Transport: Transport;
    /// The driver type that this transmitter uses to send frames
    type Driver;
    /// An error type
    ///
    /// This type must have an out-of-memory variant that can hold an `OutOfMemoryError`.
    type Error: Debug + From<OutOfMemoryError>;

    /// Starts the process of sending an outgoing transfer
    ///
    /// The transport implementation may block until the entire transfer is sent, or put frames in
    /// a queue to be sent separately.
    fn push<A, C>(
        &mut self,
        transfer: Transfer<A, I, Self::Transport>,
        clock: &mut C,
        driver: &mut Self::Driver,
    ) -> nb::Result<(), Self::Error>
    where
        A: AsRef<[u8]>,
        C: Clock<Instant = I>;

    /// Attempts to send all queued outgoing frames
    ///
    /// If ths transport's `push` implementation blocks until all frames have been sent,
    /// this function may be empty.
    ///
    /// The transport implementation may block until all frames have been sent, or return
    /// `Err(nb::Error::WouldBlock)` if not all frames can be sent.
    ///
    /// Return values:
    /// * `Ok(())`: All frames were sent
    /// * `Err(nb::Error::WouldBlock)`: At least one frame could not be sent yet
    /// * `Err(nb::Error::Other(e))`: Some other error occurred
    fn flush<C>(&mut self, clock: &mut C, driver: &mut Self::Driver) -> nb::Result<(), Self::Error>
    where
        C: Clock<Instant = I>;

    /// Returns the maximum transmission unit of this transport, in bytes
    ///
    /// A message larger than this will need to be split into multiple frames.
    ///
    /// For example, UAVCAN/CAN over classic CAN can transfer up to 7 bytes per frame (the eighth
    /// byte is used up by the tail byte), so it would return 7.
    fn mtu(&self) -> usize;
}

/// A receiver that can assemble incoming frames into transfers
pub trait Receiver<I>
where
    I: Instant,
{
    /// The transport that this transmitter works with
    type Transport: Transport;
    /// The driver type that this transmitter uses to receive frames
    type Driver;
    /// An error type
    ///
    /// This type must have an out-of-memory variant that can hold an `OutOfMemoryError`.
    type Error: Debug + From<OutOfMemoryError>;

    /// Checks for incoming frames and processes them, possibly returning a transfer
    ///
    /// If the frame completes a transfer and the transfer matches an active subscription, the
    /// transfer is returned.
    ///
    /// This function must not block. If no frame can immediately be read, it should return `Ok(None)`.
    ///
    /// If the transport reads a frame and processes it, but the frame does not complete a transfer,
    /// this function must try again to read and process a frame. It must not return `Ok(None)`
    /// if there are incoming frames that remain to be processed.
    ///
    /// This function must not return any transfers for which the transport is not currently
    /// subscribed. It also must not return any service transfers not addressed to this node.
    ///
    /// The argument `now` should be the current time. This may be used to assign timestamps to
    /// incoming frames and delete sessions that have timed out.
    fn receive(
        &mut self,
        now: I,
        driver: &mut Self::Driver,
    ) -> Result<Option<Transfer<Vec<u8>, I, Self::Transport>>, Self::Error>;

    /// Subscribes to messages on a subject
    ///
    /// This will enable incoming transfers from all nodes on the specified subject ID.
    ///
    /// subject: The subject ID to subscribe to
    ///
    /// payload_size_max: The maximum number of payload bytes expected on this subject
    /// (longer transfers will be dropped)
    ///
    /// timeout: The maximum time between the first and last frames in a transfer (transfers that
    /// do not finish within this time will be dropped)
    ///
    /// If all transfers fit into one frame, the timeout has no meaning and may be zero.
    ///
    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: I::Duration,
        driver: &mut Self::Driver,
    ) -> Result<(), Self::Error>;

    /// Unsubscribes from messages on a subject
    fn unsubscribe_message(&mut self, subject: SubjectId, driver: &mut Self::Driver);

    /// Subscribes to requests for a service
    ///
    /// This will enable incoming service request transfers from all nodes on the specified service
    /// ID.
    ///
    /// service: The service ID to subscribe to
    ///
    /// payload_size_max: The maximum number of payload bytes expected on this subject
    /// (longer transfers will be dropped)
    ///
    /// timeout: The maximum time between the first and last frames in a transfer (transfers that
    /// do not finish within this time will be dropped)
    ///
    /// If all transfers fit into one frame, the timeout has no meaning and may be zero.
    ///
    /// This function returns an error if this node is anonymous or some other transport error
    /// occurs.
    ///
    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: I::Duration,
        driver: &mut Self::Driver,
    ) -> Result<(), ServiceSubscribeError<Self::Error>>;

    /// Unsubscribes from requests for a service
    fn unsubscribe_request(&mut self, service: ServiceId, driver: &mut Self::Driver);

    /// Subscribes to responses for a service
    ///
    /// This will enable incoming service response transfers from all nodes on the specified service
    /// ID.
    ///
    /// service: The service ID to subscribe to
    ///
    /// payload_size_max: The maximum number of payload bytes expected on this subject
    /// (longer transfers will be dropped)
    ///
    /// timeout: The maximum time between the first and last frames in a transfer (transfers that
    /// do not finish within this time will be dropped)
    ///
    /// If all transfers fit into one frame, the timeout has no meaning and may be zero.
    ///
    /// This function returns an error if this node is anonymous or some other transport error
    /// occurs.
    ///
    fn subscribe_response(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: I::Duration,
        driver: &mut Self::Driver,
    ) -> Result<(), ServiceSubscribeError<Self::Error>>;

    /// Unsubscribes from responses for a service
    fn unsubscribe_response(&mut self, service: ServiceId, driver: &mut Self::Driver);
}

/// Required operations for a transfer ID
pub trait TransferId: Default + Debug + Clone {
    /// Increments the value of this transfer ID by 1
    ///
    /// If this transfer ID is the maximum allowed value, this function must wrap around to the
    /// minimum allowed value.
    fn increment(self) -> Self;
}
