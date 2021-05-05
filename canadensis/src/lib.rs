#![no_std]

extern crate alloc;
extern crate fallible_collections;
extern crate hash32;
extern crate heapless;

extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_encoding;

mod core_node;
mod hash;

pub mod anonymous;
mod publisher;
mod requester;

pub use crate::core_node::CoreNode;

use alloc::vec::Vec;
use core::marker::PhantomData;

use canadensis_can::{Frame, OutOfMemoryError};
use canadensis_core::time::{Clock, Instant};
use canadensis_core::transfer::*;
use canadensis_core::{NodeId, Priority, ServiceId, SubjectId, TransferId};
use canadensis_encoding::{Message, Request, Response, Serialize};

/// A token from a request that is needed to send a response
#[derive(Debug)]
pub struct ResponseToken {
    /// ID of the service that this is a response for
    service: ServiceId,
    /// ID of the node that sent the request
    client: NodeId,
    /// Transfer ID of the request transfer (and also the response transfer)
    transfer: TransferId,
    /// Priority of the request transfer (and also the response transfer)
    priority: Priority,
}

/// Something that may be able to handle incoming transfers
pub trait TransferHandler<N: Node + ?Sized> {
    /// Potentially handles an incoming message transfer
    fn handle_message(
        &mut self,
        node: &mut N,
        transfer: MessageTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    );

    /// Potentially handles an incoming service request
    fn handle_request(
        &mut self,
        node: &mut N,
        token: ResponseToken,
        transfer: ServiceTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    );

    /// Potentially handles an incoming service response
    fn handle_response(
        &mut self,
        node: &mut N,
        transfer: ServiceTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    );
}

pub trait Node {
    /// The clock that this node uses
    type Clock: Clock;
    /// The queue of outgoing frames that this node uses
    type FrameQueue;

    fn accept_frame<H>(
        &mut self,
        frame: Frame<<Self::Clock as Clock>::Instant>,
        handler: &mut H,
    ) -> Result<(), OutOfMemoryError>
    where
        H: TransferHandler<Self>;

    fn start_publishing<T>(
        &mut self,
        subject: SubjectId,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
        priority: Priority,
    ) -> Result<PublishToken<T>, CapacityError>
    where
        T: Message;

    fn publish<T>(&mut self, token: &PublishToken<T>, payload: &T) -> Result<(), OutOfMemoryError>
    where
        T: Message + Serialize;

    /// Sets up to send requests for a service
    ///
    /// This also subscribes to the corresponding responses.
    fn start_sending_requests<T>(
        &mut self,
        service: ServiceId,
        receive_timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
        response_payload_size_max: usize,
        priority: Priority,
    ) -> Result<ServiceToken<T>, CapacityOrMemoryError>
    where
        T: Request;

    fn send_request<T>(
        &mut self,
        token: &ServiceToken<T>,
        payload: &T,
        destination: NodeId,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Request + Serialize;

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError>;

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError>;

    fn send_response<T>(
        &mut self,
        token: ResponseToken,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
        payload: &T,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Response + Serialize;

    // Component access

    /// Returns a reference to the enclosed clock
    fn clock(&self) -> &Self::Clock;
    /// Returns a mutable reference to the enclosed clock
    fn clock_mut(&mut self) -> &mut Self::Clock;

    fn frame_queue(&self) -> &Self::FrameQueue;

    fn frame_queue_mut(&mut self) -> &mut Self::FrameQueue;

    /// Returns the identifier of this node
    fn node_id(&self) -> NodeId;
}

/// A token returned from start_publishing that can be used to a publish a transfer using the
/// associated subject ID
///
/// The type parameter `T` constrains the type of message sent.
pub struct PublishToken<T>(SubjectId, PhantomData<*mut T>);

/// A token returned from start_sending_requests that can be used to a request a service using the
/// associated service ID
///
/// The type parameter `T` constrains the type of request sent.
pub struct ServiceToken<T>(ServiceId, PhantomData<*mut T>);

/// An error indicating that an operation ran out of space in a fixed-capacity data structure
#[derive(Debug)]
pub struct CapacityError(());

#[derive(Debug)]
pub enum CapacityOrMemoryError {
    Capacity(CapacityError),
    OutOfMemory(OutOfMemoryError),
}

impl From<CapacityError> for CapacityOrMemoryError {
    fn from(inner: CapacityError) -> Self {
        CapacityOrMemoryError::Capacity(inner)
    }
}
impl From<OutOfMemoryError> for CapacityOrMemoryError {
    fn from(inner: OutOfMemoryError) -> Self {
        CapacityOrMemoryError::OutOfMemory(inner)
    }
}
