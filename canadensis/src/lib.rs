#![no_std]

extern crate alloc;
extern crate fallible_collections;
extern crate hash32;
extern crate heapless;

extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_encoding;
extern crate canadensis_filter_config;

// Re-exports from other crates
pub mod can {
    /// Re-exports the `canadensis_can` crate
    pub use canadensis_can::*;
}
pub mod core {
    /// Re-exports the `canadensis_core` crate
    pub use canadensis_core::*;
}
pub mod encoding {
    /// Re-exports the `canadensis_encoding` crate
    pub use canadensis_encoding::*;
}
pub mod filter {
    /// Re-exports the `canadensis_filter_config` crate
    pub use canadensis_filter_config::*;
}

mod core_node;
mod hash;

pub mod anonymous;
mod basic;
mod minimal;
mod publisher;
pub mod register;
mod requester;

pub use crate::basic::BasicNode;
pub use crate::core_node::CoreNode;
pub use crate::minimal::MinimalNode;

use ::core::marker::PhantomData;
use alloc::vec::Vec;

use canadensis_can::{Frame, OutOfMemoryError};
use canadensis_core::time::{Clock, Instant};
use canadensis_core::transfer::*;
use canadensis_core::{NodeId, Priority, ServiceId, SubjectId, TransferId};
use canadensis_encoding::{Message, Request, Response, Serialize};
use canadensis_filter_config::Filter;

/// A token from a request that is needed to send a response
#[derive(Debug, Clone)]
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
pub trait TransferHandler<I: Instant> {
    /// Potentially handles an incoming message transfer
    ///
    /// This function returns true if the message was handled and should not be sent on to other
    /// handlers.
    ///
    /// The default implementation does nothing and returns false.
    fn handle_message<N: Node<Instant = I>>(
        &mut self,
        node: &mut N,
        transfer: &MessageTransfer<Vec<u8>, I>,
    ) -> bool {
        drop((node, transfer));
        false
    }

    /// Potentially handles an incoming service request
    ///
    /// This function returns true if the request was handled and should not be sent on to other
    /// handlers.
    ///
    /// The default implementation does nothing and returns false.
    fn handle_request<N: Node<Instant = I>>(
        &mut self,
        node: &mut N,
        token: ResponseToken,
        transfer: &ServiceTransfer<Vec<u8>, I>,
    ) -> bool {
        drop((node, token, transfer));
        false
    }

    /// Potentially handles an incoming service response
    ///
    /// This function returns true if the response was handled and should not be sent on to other
    /// handlers.
    ///
    /// The default implementation does nothing and returns false.
    fn handle_response<N: Node<Instant = I>>(
        &mut self,
        node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, I>,
    ) -> bool {
        drop((node, transfer));
        false
    }

    /// Chains another handler after this handler and returns the combined handler
    ///
    /// For each incoming transfer, this handler will be given the transfer before the next handler.
    fn chain<H>(self, next: H) -> TransferHandlerChain<Self, H>
    where
        Self: Sized,
        H: TransferHandler<I>,
    {
        TransferHandlerChain::new(self, next)
    }
}

/// Combines two transfer handlers
pub struct TransferHandlerChain<H0, H1> {
    handler0: H0,
    handler1: H1,
}

impl<H0, H1> TransferHandlerChain<H0, H1> {
    /// Creates a handler chain
    ///
    /// Each incoming transfer will be passed to handler 0 first. If handler 0 does not
    /// handle the transfer, the transfer will be passed to handler 1.
    pub fn new(handler0: H0, handler1: H1) -> Self {
        TransferHandlerChain { handler0, handler1 }
    }
}

impl<I, H0, H1> TransferHandler<I> for TransferHandlerChain<H0, H1>
where
    I: Instant,
    H0: TransferHandler<I>,
    H1: TransferHandler<I>,
{
    fn handle_message<N: Node<Instant = I>>(
        &mut self,
        node: &mut N,
        transfer: &MessageTransfer<Vec<u8>, I>,
    ) -> bool {
        let handled = self.handler0.handle_message(node, transfer);
        if handled {
            true
        } else {
            self.handler1.handle_message(node, transfer)
        }
    }

    fn handle_request<N: Node<Instant = I>>(
        &mut self,
        node: &mut N,
        token: ResponseToken,
        transfer: &ServiceTransfer<Vec<u8>, I>,
    ) -> bool {
        let handled = self.handler0.handle_request(node, token.clone(), transfer);
        if handled {
            true
        } else {
            self.handler1.handle_request(node, token, transfer)
        }
    }

    fn handle_response<N: Node<Instant = I>>(
        &mut self,
        node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, I>,
    ) -> bool {
        let handled = self.handler0.handle_response(node, transfer);
        if handled {
            true
        } else {
            self.handler1.handle_response(node, transfer)
        }
    }
}

/// A UAVCAN node
///
/// A node has a node ID (it is not anonymous), a clock, a queue of outgoing frames waiting to be
/// sent, and information about the subjects and services it is using.
pub trait Node {
    /// The clock that this node uses
    type Clock: Clock<Instant = Self::Instant>;
    /// The instant that this node's clock produces
    type Instant: Instant;
    /// The queue of outgoing frames that this node uses
    type FrameQueue;

    /// Handles an incoming frame
    ///
    /// If the frame completes a transfer, the transfer is passed to the provided handler.
    ///
    /// This function returns an error if memory for the received transfer could not be allocated.
    /// Other types of errors, like an invalid frame format or an incorrect transfer CRC,
    /// may cause transfers to be lost but are not reported as errors here.
    fn accept_frame<H>(
        &mut self,
        frame: Frame<<Self::Clock as Clock>::Instant>,
        handler: &mut H,
    ) -> Result<(), OutOfMemoryError>
    where
        H: TransferHandler<Self::Instant>;

    /// Starts publishing messages on subject
    ///
    /// The returned [`PublishToken`] can be used with the [`publish`](#tymethod.publish) function to
    /// send a message.
    ///
    /// This function returns an error if memory for the publishing data could not be allocated,
    /// or if the subject ID is already in use.
    fn start_publishing<T>(
        &mut self,
        subject: SubjectId,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
        priority: Priority,
    ) -> Result<PublishToken<T>, StartSendError>
    where
        T: Message;

    /// Stops publishing messages on a subject
    fn stop_publishing<T>(&mut self, token: PublishToken<T>)
    where
        T: Message;

    /// Publishes a message
    ///
    /// A token can be created by calling [`start_publishing`](#tymethod.start_publishing).
    fn publish<T>(&mut self, token: &PublishToken<T>, payload: &T) -> Result<(), OutOfMemoryError>
    where
        T: Message + Serialize;

    /// Sets up to send requests for a service
    ///
    /// This also subscribes to the corresponding responses.
    ///
    /// This function returns an error if memory could not be allocated,
    /// or if the subject ID is already in use.
    fn start_sending_requests<T>(
        &mut self,
        service: ServiceId,
        receive_timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
        response_payload_size_max: usize,
        priority: Priority,
    ) -> Result<ServiceToken<T>, StartSendError>
    where
        T: Request;

    /// Stops sending requests for a service
    fn stop_sending_requests<T>(&mut self, token: ServiceToken<T>)
    where
        T: Request;

    /// Sends a service request to another node
    ///
    /// On success, this function returns the transfer ID of the request.
    fn send_request<T>(
        &mut self,
        token: &ServiceToken<T>,
        payload: &T,
        destination: NodeId,
    ) -> Result<TransferId, OutOfMemoryError>
    where
        T: Request + Serialize;

    /// Subscribes to messages on a topic
    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError>;

    /// Subscribes to requests for a service
    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError>;

    /// Responds to a service request
    ///
    /// This function requires a response token to match this response to its corresponding
    /// request. The token is passed to a transfer handler along with a request, so that the handler
    /// can send a response.
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

    /// Returns a set of filters that accept the frames this node is subscribed to
    fn frame_filters(&self) -> Result<Vec<Filter>, OutOfMemoryError>;
}

/// A token returned from [`Node::start_publishing`](Node#tymethod.start_publishing) that can be
/// used to a publish a transfer using the associated subject ID
///
/// The type parameter `T` constrains the type of message sent.
pub struct PublishToken<T>(SubjectId, PhantomData<T>);

impl<T> PublishToken<T> {
    /// Returns the subject ID that this token is used to publish on
    pub fn subject_id(&self) -> SubjectId {
        self.0
    }
}

/// A token returned from [`Node::start_sending_requests`](Node#tymethod.start_sending_requests)
/// that can be used to a request a service using the associated service ID
///
/// The type parameter `T` constrains the type of request sent.
pub struct ServiceToken<T>(ServiceId, PhantomData<T>);

impl<T> ServiceToken<T> {
    /// returns the service ID that this token is used to send requests on
    pub fn service_id(&self) -> ServiceId {
        self.0
    }
}

/// Errors that may occur when starting to send messages or requests
#[derive(Debug)]
pub enum StartSendError {
    /// Memory could not be allocated
    Memory(OutOfMemoryError),
    /// The provided subject ID or service ID is already in use
    Duplicate,
}

impl From<OutOfMemoryError> for StartSendError {
    fn from(inner: OutOfMemoryError) -> Self {
        StartSendError::Memory(inner)
    }
}
