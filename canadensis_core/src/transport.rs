//! Transport layer traits

use crate::error::{OutOfMemoryError, ServiceSubscribeError};
use crate::time::Instant;
use crate::transfer::Transfer;
use crate::{ServiceId, SubjectId};
use alloc::vec::Vec;
use core::fmt::Debug;

/// Basic requirements for a transport that can be used to send and receive transfers
///
/// The [`Transmitter`](Transmitter) and [`Receiver`](Receiver) sub-traits add additional functions.
pub trait Transport {
    /// A node ID type that can hold the node ID values that this transport allows
    type NodeId: NodeId<Self::TransferId>;
    /// A transfer ID type that can hold all supported transfer ID values
    type TransferId: TransferId;
    /// A priority type that can hold all supported priority values
    type Priority: Clone + Debug + From<crate::Priority>;
    /// The frame type used in the underlying transport
    type Frame;
    /// An error type
    ///
    /// This type must have an out-of-memory variant that can hold an `OutOfMemoryError`.
    type Error: Debug + From<OutOfMemoryError>;
}

/// A transmitter that can send outgoing transfers
pub trait Transmitter<I>
where
    I: Instant,
{
    /// The transport that this transmitter works with
    type Transport: Transport;

    /// Starts the process of sending an outgoing transfer
    ///
    /// The transport implementation may block until the entire transfer is sent, or put frames in
    /// a queue to be sent separately.
    ///
    /// If this function returns an error, no part of the transfer may be transmitted.
    fn push<A>(
        &mut self,
        transfer: Transfer<A, I, Self::Transport>,
    ) -> Result<(), <Self::Transport as Transport>::Error>
    where
        A: AsRef<[u8]>;

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
    /// Handles an incoming frame
    ///
    /// If the frame completes a transfer, the transfer is returned.
    ///
    // TODO: If an error occurs, what is required of the transport state?
    fn accept(
        &mut self,
        frame: <Self::Transport as Transport>::Frame,
    ) -> Result<Option<Transfer<Vec<u8>, I, Self::Transport>>, <Self::Transport as Transport>::Error>;

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
    ) -> Result<(), <Self::Transport as Transport>::Error>;

    /// Unsubscribes from messages on a subject
    fn unsubscribe_message(&mut self, subject: SubjectId);

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
    ) -> Result<(), ServiceSubscribeError<<Self::Transport as Transport>::Error>>;

    /// Unsubscribes from requests for a service
    fn unsubscribe_request(&mut self, service: ServiceId);

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
    ) -> Result<(), ServiceSubscribeError<<Self::Transport as Transport>::Error>>;

    /// Unsubscribes from responses for a service
    fn unsubscribe_response(&mut self, service: ServiceId);
}

/// Required operations for a node ID
pub trait NodeId<T>: Debug + Clone + Into<usize> {
    /// An array of transfer IDs that contains a transfer ID for each possible node ID value
    ///
    /// This is normally `[T; the maximum node ID value + 1]`.
    type TransferIds: AsMut<[T]> + Default;
}

/// Required operations for a transfer ID
pub trait TransferId: Default + Debug + Clone {
    /// Increments the value of this transfer ID by 1
    ///
    /// If this transfer ID is the maximum allowed value, this function must wrap around to the
    /// minimum allowed value.
    fn increment(self) -> Self;
}
