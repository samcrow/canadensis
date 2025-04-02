#![no_std]
#![deny(missing_docs)]

//!
//! # Canadensis: An implementation of Cyphal
//!
//! This library (`canadensis`) provides all the basic Cyphal functionality, with some re-exports
//! from other canadensis crates.
//!

extern crate alloc;
extern crate fallible_collections;
extern crate heapless;

extern crate canadensis_core;
extern crate canadensis_encoding;

// Re-exports from other crates
pub mod core {
    //! Basic Cyphal types
    pub use canadensis_core::*;
}
pub mod encoding {
    //! Data type serialization and deserialization
    pub use canadensis_encoding::*;
}
pub use canadensis_core::nb;

pub mod anonymous;
pub mod node;
mod publisher;
pub mod register;
pub mod requester;
mod serialize;
pub mod service;

use ::core::fmt::{Debug, Formatter};
use ::core::marker::PhantomData;
use alloc::vec::Vec;
use canadensis_core::OutOfMemoryError;

use crate::core::transport::Transport;
use canadensis_core::time::{Clock, MicrosecondDuration32};
use canadensis_core::transfer::*;
use canadensis_core::transport::{Receiver, Transmitter};
use canadensis_core::{ServiceId, SubjectId};
use canadensis_encoding::{Message, Request, Response, Serialize};

/// A token from a request that is needed to send a response
pub struct ResponseToken<T: Transport> {
    /// ID of the service that this is a response for
    service: ServiceId,
    /// ID of the node that sent the request
    client: T::NodeId,
    /// Transfer ID of the request transfer (and also the response transfer)
    transfer: T::TransferId,
    /// Priority of the request transfer (and also the response transfer)
    priority: T::Priority,
}

impl<T: Transport> Clone for ResponseToken<T>
where
    T::NodeId: Clone,
    T::TransferId: Clone,
    T::Priority: Clone,
{
    fn clone(&self) -> Self {
        ResponseToken {
            service: self.service,
            client: self.client.clone(),
            transfer: self.transfer.clone(),
            priority: self.priority.clone(),
        }
    }
}
impl<T: Transport> Debug for ResponseToken<T>
where
    T::NodeId: Debug,
    T::TransferId: Debug,
    T::Priority: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ResponseToken")
            .field("service", &self.service)
            .field("client", &self.client)
            .field("transfer", &self.transfer)
            .field("priority", &self.priority)
            .finish()
    }
}

/// Something that may be able to handle incoming transfers
pub trait TransferHandler<T: Transport> {
    /// Potentially handles an incoming message transfer
    ///
    /// This function does not handle any loopback transfers.
    ///
    /// This function returns true if the message was handled and should not be sent on to other
    /// handlers.
    ///
    /// The default implementation does nothing and returns false.
    fn handle_message<N: Node<Transport = T>>(
        &mut self,
        _node: &mut N,
        _transfer: &MessageTransfer<Vec<u8>, T>,
    ) -> bool {
        false
    }

    /// Potentially handles an incoming service request
    ///
    /// This function does not handle any loopback transfers.
    ///
    /// This function returns true if the request was handled and should not be sent on to other
    /// handlers.
    ///
    /// The default implementation does nothing and returns false.
    fn handle_request<N: Node<Transport = T>>(
        &mut self,
        _node: &mut N,
        _token: ResponseToken<T>,
        _transfer: &ServiceTransfer<Vec<u8>, T>,
    ) -> bool {
        false
    }

    /// Potentially handles an incoming service response
    ///
    /// This function does not handle any loopback transfers.
    ///
    /// This function returns true if the response was handled and should not be sent on to other
    /// handlers.
    ///
    /// The default implementation does nothing and returns false.
    fn handle_response<N: Node<Transport = T>>(
        &mut self,
        _node: &mut N,
        _transfer: &ServiceTransfer<Vec<u8>, T>,
    ) -> bool {
        false
    }

    /// Potentially handles a loopback transfer sent from this node
    ///
    /// All loopback transfers (message, request, and response) are handled here.
    ///
    /// This function returns true if the response was handled and should not be sent on to other
    /// handlers.
    ///
    /// The default implementation does nothing and returns false.
    fn handle_loopback<N: Node<Transport = T>>(
        &mut self,
        _node: &mut N,
        _transfer: &Transfer<Vec<u8>, T>,
    ) -> bool {
        false
    }

    /// Chains another handler after this handler and returns the combined handler
    ///
    /// For each incoming transfer, this handler will be given the transfer before the next handler.
    fn chain<H>(self, next: H) -> TransferHandlerChain<Self, H>
    where
        Self: Sized,
        H: TransferHandler<T>,
    {
        TransferHandlerChain::new(self, next)
    }
}

impl<'h, T, H> TransferHandler<T> for &'h mut H
where
    T: Transport,
    H: TransferHandler<T>,
{
    fn handle_message<N: Node<Transport = T>>(
        &mut self,
        node: &mut N,
        transfer: &MessageTransfer<Vec<u8>, T>,
    ) -> bool {
        <H as TransferHandler<T>>::handle_message(self, node, transfer)
    }

    fn handle_request<N: Node<Transport = T>>(
        &mut self,
        node: &mut N,
        token: ResponseToken<T>,
        transfer: &ServiceTransfer<Vec<u8>, T>,
    ) -> bool {
        <H as TransferHandler<T>>::handle_request(self, node, token, transfer)
    }

    fn handle_response<N: Node<Transport = T>>(
        &mut self,
        node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, T>,
    ) -> bool {
        <H as TransferHandler<T>>::handle_response(self, node, transfer)
    }

    fn handle_loopback<N: Node<Transport = T>>(
        &mut self,
        node: &mut N,
        transfer: &Transfer<Vec<u8>, T>,
    ) -> bool {
        <H as TransferHandler<T>>::handle_loopback(self, node, transfer)
    }

    fn chain<H1>(self, next: H1) -> TransferHandlerChain<Self, H1>
    where
        Self: Sized,
        H1: TransferHandler<T>,
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

    /// Returns a reference to the first handler in this chain
    pub fn first(&self) -> &H0 {
        &self.handler0
    }
    /// Returns a mutable reference to the first handler in this chain
    pub fn first_mut(&mut self) -> &mut H0 {
        &mut self.handler0
    }
    /// Returns a reference to the second handler in this chain
    pub fn second(&self) -> &H1 {
        &self.handler1
    }
    /// Returns a mutable reference to the second handler in this chain
    pub fn second_mut(&mut self) -> &mut H1 {
        &mut self.handler1
    }

    /// Splits this chain into its inner handlers
    pub fn into_inner(self) -> (H0, H1) {
        (self.handler0, self.handler1)
    }
}

impl<T, H0, H1> TransferHandler<T> for TransferHandlerChain<H0, H1>
where
    T: Transport,
    H0: TransferHandler<T>,
    H1: TransferHandler<T>,
{
    fn handle_message<N: Node<Transport = T>>(
        &mut self,
        node: &mut N,
        transfer: &MessageTransfer<Vec<u8>, T>,
    ) -> bool {
        let handled = self.handler0.handle_message(node, transfer);
        if handled {
            true
        } else {
            self.handler1.handle_message(node, transfer)
        }
    }

    fn handle_request<N: Node<Transport = T>>(
        &mut self,
        node: &mut N,
        token: ResponseToken<T>,
        transfer: &ServiceTransfer<Vec<u8>, T>,
    ) -> bool {
        let handled = self.handler0.handle_request(node, token.clone(), transfer);
        if handled {
            true
        } else {
            self.handler1.handle_request(node, token, transfer)
        }
    }

    fn handle_response<N: Node<Transport = T>>(
        &mut self,
        node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, T>,
    ) -> bool {
        let handled = self.handler0.handle_response(node, transfer);
        if handled {
            true
        } else {
            self.handler1.handle_response(node, transfer)
        }
    }

    fn handle_loopback<N: Node<Transport = T>>(
        &mut self,
        node: &mut N,
        transfer: &Transfer<Vec<u8>, T>,
    ) -> bool {
        let handled = self.handler0.handle_loopback(node, transfer);
        if handled {
            true
        } else {
            self.handler1.handle_loopback(node, transfer)
        }
    }
}

/// A Cyphal node
///
/// A node has a node ID (it is not anonymous), a clock, a queue of outgoing frames waiting to be
/// sent, and information about the subjects and services it is using.
pub trait Node {
    /// The clock that this node uses
    type Clock: Clock;
    /// The transport that this node uses
    type Transport: Transport;
    /// The transmitter that this node uses
    type Transmitter: Transmitter<Self::Clock, Transport = Self::Transport>;
    /// The receiver that this node uses
    type Receiver: Receiver<Self::Clock, Transport = Self::Transport>;

    /// Receives any available incoming frames and attempts ot reassemble them into a transfer
    ///
    /// If the frame completes a transfer, the transfer is passed to the provided handler.
    ///
    /// This function returns an error if memory for the received transfer could not be allocated.
    /// Other types of errors, like an invalid frame format or an incorrect transfer CRC,
    /// may cause transfers to be lost but are not reported as errors here.
    fn receive<H>(
        &mut self,
        handler: &mut H,
    ) -> Result<(), <Self::Receiver as Receiver<Self::Clock>>::Error>
    where
        H: TransferHandler<Self::Transport>;

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
        timeout: MicrosecondDuration32,
        priority: <Self::Transport as Transport>::Priority,
    ) -> Result<
        PublishToken<T>,
        StartSendError<<Self::Transmitter as Transmitter<Self::Clock>>::Error>,
    >
    where
        T: Message;

    /// Stops publishing messages on a subject
    fn stop_publishing<T>(&mut self, token: PublishToken<T>)
    where
        T: Message;

    /// Publishes a message
    ///
    /// A token can be created by calling [`start_publishing`](#tymethod.start_publishing).
    fn publish<T>(
        &mut self,
        token: &PublishToken<T>,
        payload: &T,
    ) -> nb::Result<(), <Self::Transmitter as Transmitter<Self::Clock>>::Error>
    where
        T: Message + Serialize;

    /// Publishes a message with the loopback flag set to true
    ///
    /// A token can be created by calling [`start_publishing`](#tymethod.start_publishing).
    fn publish_loopback<T>(
        &mut self,
        token: &PublishToken<T>,
        payload: &T,
    ) -> nb::Result<(), <Self::Transmitter as Transmitter<Self::Clock>>::Error>
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
        receive_timeout: MicrosecondDuration32,
        response_payload_size_max: usize,
        priority: <Self::Transport as Transport>::Priority,
    ) -> Result<ServiceToken<T>, StartSendError<<Self::Receiver as Receiver<Self::Clock>>::Error>>
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
        destination: <Self::Transport as Transport>::NodeId,
    ) -> nb::Result<
        <Self::Transport as Transport>::TransferId,
        <Self::Transmitter as Transmitter<Self::Clock>>::Error,
    >
    where
        T: Request + Serialize;

    /// Sends a service request to another node, with the loopback flag set to true
    ///
    /// On success, this function returns the transfer ID of the request.
    fn send_request_loopback<T>(
        &mut self,
        token: &ServiceToken<T>,
        payload: &T,
        destination: <Self::Transport as Transport>::NodeId,
    ) -> nb::Result<
        <Self::Transport as Transport>::TransferId,
        <Self::Transmitter as Transmitter<Self::Clock>>::Error,
    >
    where
        T: Request + Serialize;

    /// Subscribes to messages on a topic
    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
    ) -> Result<(), <Self::Receiver as Receiver<Self::Clock>>::Error>;

    /// Unsubscribes from messages on a topic
    fn unsubscribe_message(&mut self, subject: SubjectId);

    /// Subscribes to requests for a service
    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: MicrosecondDuration32,
    ) -> Result<(), <Self::Receiver as Receiver<Self::Clock>>::Error>;

    /// Unsubscribes from requests for a service
    fn unsubscribe_request(&mut self, service: ServiceId);

    /// Responds to a service request
    ///
    /// This function requires a response token to match this response to its corresponding
    /// request. The token is passed to a transfer handler along with a request, so that the handler
    /// can send a response.
    ///
    /// The response has its loopback flag set to false.
    ///
    /// # Panics
    ///
    /// Some implementations may panic if this function is called on an anonymous node though this
    /// situation should never occur in practice as anonymous nodes will not produce response tokens.
    fn send_response<T>(
        &mut self,
        token: ResponseToken<Self::Transport>,
        timeout: MicrosecondDuration32,
        payload: &T,
    ) -> nb::Result<(), <Self::Transmitter as Transmitter<Self::Clock>>::Error>
    where
        T: Response + Serialize;

    /// Attempts to flush all outgoing frames
    fn flush(&mut self) -> nb::Result<(), <Self::Transmitter as Transmitter<Self::Clock>>::Error>;

    // Component access

    /// Returns a reference to the enclosed clock
    fn clock(&self) -> &Self::Clock;
    /// Returns a mutable reference to the enclosed clock
    fn clock_mut(&mut self) -> &mut Self::Clock;

    /// Returns a reference to the transport transmitter
    fn transmitter(&self) -> &Self::Transmitter;
    /// Returns a mutable reference to the transport transmitter
    fn transmitter_mut(&mut self) -> &mut Self::Transmitter;

    /// Returns a reference to the transport receiver
    fn receiver(&self) -> &Self::Receiver;
    /// Returns a mutable reference to the transport receiver
    fn receiver_mut(&mut self) -> &mut Self::Receiver;

    /// Returns the identifier of this node
    ///
    /// If the node is anonymous, this function returns `None`.
    fn node_id(&self) -> Option<<Self::Transport as Transport>::NodeId>;

    /// Sets the identifier of this node
    fn set_node_id(&mut self, node_id: <Self::Transport as Transport>::NodeId);
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

mod fmt_impl {
    use super::PublishToken;
    use core::fmt::{Debug, Formatter, Result};

    impl<T> Debug for PublishToken<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_tuple("PublishToken").field(&self.0).finish()
        }
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
pub enum StartSendError<E> {
    /// Memory to store the publisher was not available
    Memory(OutOfMemoryError),
    /// Tne transport returned an error
    Transport(E),
    /// The provided subject ID or service ID is already in use
    Duplicate,
    /// The node or transmitter is anonymous and cannot send requests
    AnonymousRequest,
}

impl<E> From<E> for StartSendError<E> {
    fn from(inner: E) -> Self {
        StartSendError::Transport(inner)
    }
}
