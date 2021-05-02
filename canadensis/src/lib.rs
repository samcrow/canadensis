// std only for debugging
// #![no_std]

extern crate alloc;
extern crate fallible_collections;
extern crate hash32;
extern crate heapless;

extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_encoding;

mod hash;

// Reexports from other canadensis crates
pub use canadensis_can::*;
pub use canadensis_core::transfer;
pub use canadensis_core::*;
pub use canadensis_encoding::*;

pub mod anonymous;
mod publisher;
mod requester;

use alloc::vec::Vec;
use core::iter;

use crate::hash::TrivialIndexMap;
use crate::publisher::Publisher;
use crate::requester::Requester;
use canadensis_core::time::Instant;
use canadensis_core::transfer::*;
use canadensis_encoding::{DeserializeError, Serialize, WriteCursor};
use fallible_collections::FallibleVec;
use std::marker::PhantomData;

/// Payloads above this size (in bytes) will use a dynamically allocated buffer
const STACK_THRESHOLD: usize = 64;

/// Serializes a payload into a buffer and passes the buffer to a closure
fn do_serialize<T, F>(payload: &T, operation: F) -> Result<(), OutOfMemoryError>
where
    T: Serialize,
    F: FnOnce(&[u8]) -> Result<(), OutOfMemoryError>,
{
    let payload_bytes = (payload.size_bits() + 7) / 8;
    if payload_bytes > STACK_THRESHOLD {
        let mut bytes: Vec<u8> = FallibleVec::try_with_capacity(payload_bytes)?;
        bytes.extend(iter::repeat(0).take(payload_bytes));
        payload.serialize(&mut WriteCursor::new(&mut bytes));
        operation(&bytes)
    } else {
        let mut bytes = [0u8; STACK_THRESHOLD];
        let bytes = &mut bytes[..payload_bytes];
        payload.serialize(&mut WriteCursor::new(bytes));
        operation(bytes)
    }
}

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
pub trait TransferHandler<C: Clock, const P: usize, const R: usize> {
    /// Potentially handles an incoming message transfer
    fn handle_message(
        &mut self,
        node: &mut Node<C, P, R>,
        transfer: MessageTransfer<Vec<u8>, C::Instant>,
    );

    /// Potentially handles an incoming service request
    fn handle_request(
        &mut self,
        node: &mut Node<C, P, R>,
        token: ResponseToken,
        transfer: ServiceTransfer<Vec<u8>, C::Instant>,
    );

    /// Potentially handles an incoming service response
    fn handle_response(
        &mut self,
        node: &mut Node<C, P, R>,
        transfer: ServiceTransfer<Vec<u8>, C::Instant>,
    );
}

/// A high-level interface with UAVCAN node functionality
///
/// Type parameters:
/// * `C`: The clock used to get the current time
/// * `P`: The maximum number of topics that can be published
/// * `R`: The maximum number of services for which requests can be sent
///
pub struct Node<C, const P: usize, const R: usize>
where
    C: Clock,
{
    clock: C,
    transmitter: Transmitter<C::Instant>,
    receiver: Receiver<C::Instant>,
    node_id: NodeId,
    publishers: TrivialIndexMap<SubjectId, Publisher<C::Instant>, P>,
    requesters: TrivialIndexMap<ServiceId, Requester<C::Instant>, R>,
}

impl<C, const P: usize, const R: usize> Node<C, P, R>
where
    C: Clock,
{
    pub fn new(clock: C, node_id: NodeId, mtu: Mtu) -> Self {
        Node {
            clock,
            transmitter: Transmitter::new(mtu),
            receiver: Receiver::new(node_id),
            node_id,
            publishers: TrivialIndexMap::new(),
            requesters: TrivialIndexMap::new(),
        }
    }

    pub fn accept_frame<H>(
        &mut self,
        frame: Frame<C::Instant>,
        handler: &mut H,
    ) -> Result<(), OutOfMemoryError>
    where
        H: TransferHandler<C, P, R>,
    {
        match self.receiver.accept(frame)? {
            Some(transfer) => {
                self.handle_incoming_transfer(transfer, handler);
            }
            None => {}
        }
        Ok(())
    }

    fn handle_incoming_transfer<H>(
        &mut self,
        transfer: Transfer<Vec<u8>, C::Instant>,
        handler: &mut H,
    ) where
        H: TransferHandler<C, P, R>,
    {
        match transfer.header.kind {
            TransferKindHeader::Message(message_header) => {
                let message_transfer = MessageTransfer {
                    timestamp: transfer.timestamp,
                    header: MessageOnlyHeader {
                        source: transfer.header.source,
                        priority: transfer.header.priority,
                        message: message_header,
                    },
                    transfer_id: transfer.transfer_id,
                    payload: transfer.payload,
                };
                handler.handle_message(self, message_transfer);
            }
            TransferKindHeader::Request(service_header) => {
                let token = ResponseToken {
                    service: service_header.service,
                    client: transfer.header.source,
                    transfer: transfer.transfer_id,
                    priority: transfer.header.priority,
                };
                let service_transfer = ServiceTransfer {
                    timestamp: transfer.timestamp,
                    header: ServiceOnlyHeader {
                        source: transfer.header.source,
                        priority: transfer.header.priority,
                        service: service_header,
                    },
                    transfer_id: transfer.transfer_id,
                    payload: transfer.payload,
                };
                handler.handle_request(self, token, service_transfer);
            }
            TransferKindHeader::Response(service_header) => {
                let service_transfer = ServiceTransfer {
                    timestamp: transfer.timestamp,
                    header: ServiceOnlyHeader {
                        source: transfer.header.source,
                        priority: transfer.header.priority,
                        service: service_header,
                    },
                    transfer_id: transfer.transfer_id,
                    payload: transfer.payload,
                };
                handler.handle_response(self, service_transfer);
            }
        }
    }

    pub fn start_publishing_topic<T>(
        &mut self,
        subject: SubjectId,
        timeout: <C::Instant as Instant>::Duration,
        priority: Priority,
    ) -> Result<SubscriptionToken<T>, CapacityError>
    where
        T: Message,
    {
        let token = SubscriptionToken(subject.clone(), PhantomData);
        self.publishers
            .insert(subject, Publisher::new(self.node_id, timeout, priority))
            .map(|_| token)
            .map_err(|_| CapacityError(()))
    }

    pub fn publish_to_topic<T>(
        &mut self,
        token: &SubscriptionToken<T>,
        payload: &T,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Message + Serialize,
    {
        let publisher = self
            .publishers
            .get_mut(&token.0)
            .expect("Bug: Token exists but no subscriber");
        publisher.publish(self.clock.now(), token.0, payload, &mut self.transmitter)
    }

    /// Sets up to send requests for a service
    ///
    /// This also subscribes to the corresponding responses.
    pub fn start_sending_requests<T>(
        &mut self,
        service: ServiceId,
        receive_timeout: <C::Instant as Instant>::Duration,
        response_payload_size_max: usize,
        priority: Priority,
    ) -> Result<ServiceToken<T>, CapacityOrMemoryError>
    where
        T: Request,
    {
        let token = ServiceToken(service, PhantomData);
        self.requesters
            .insert(
                service,
                Requester::new(self.node_id, receive_timeout.clone(), priority),
            )
            .map_err(|_| CapacityError(()))?;
        match self
            .receiver
            .subscribe_response(service, response_payload_size_max, receive_timeout)
        {
            Ok(()) => Ok(token),
            Err(e) => {
                // Clean up requester
                self.requesters.remove(&service);
                Err(e.into())
            }
        }
    }

    pub fn send_request<T>(
        &mut self,
        token: &ServiceToken<T>,
        payload: &T,
        destination: NodeId,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Request + Serialize,
    {
        let requester = self
            .requesters
            .get_mut(&token.0)
            .expect("Bug: No requester for token");
        requester.send(
            self.clock.now(),
            token.0,
            payload,
            destination,
            &mut self.transmitter,
        )
    }

    pub fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <C::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError> {
        self.receiver
            .subscribe_message(subject, payload_size_max, timeout)
    }

    pub fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <C::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError> {
        self.receiver
            .subscribe_request(service, payload_size_max, timeout)
    }

    pub fn send_response<T>(
        &mut self,
        token: ResponseToken,
        timeout: <C::Instant as Instant>::Duration,
        payload: &T,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Response + Serialize,
    {
        let now = self.clock.now();
        let deadline = timeout + now;
        do_serialize(payload, |payload| {
            self.send_response_payload(token, deadline, payload)
        })
    }

    fn send_response_payload(
        &mut self,
        token: ResponseToken,
        deadline: C::Instant,
        payload: &[u8],
    ) -> Result<(), OutOfMemoryError> {
        let transfer_out = Transfer {
            timestamp: deadline,
            header: TransferHeader {
                source: self.node_id,
                priority: token.priority,
                kind: TransferKindHeader::Response(ServiceHeader {
                    service: token.service,
                    destination: token.client,
                }),
            },
            transfer_id: token.transfer,
            payload,
        };
        self.transmitter.push(transfer_out)
    }

    /// Removes an outgoing frame from the queue and returns it
    pub fn pop_frame(&mut self) -> Option<Frame<C::Instant>> {
        self.transmitter.pop()
    }

    /// Returns a reference to the next outgoing frame in the queue, and does not remove it
    pub fn peek_frame(&mut self) -> Option<&Frame<C::Instant>> {
        self.transmitter.peek()
    }

    /// Returns an outgoing frame to the queue so that it can be transmitted later
    pub fn return_frame(&mut self, frame: Frame<C::Instant>) -> Result<(), OutOfMemoryError> {
        self.transmitter.return_frame(frame)
    }
}

/// A token returned from start_publishing_topic that can be used to a publish a transfer using the
/// associated subject ID
///
/// The type parameter `T` constrains the type of message sent.
pub struct SubscriptionToken<T>(SubjectId, PhantomData<*mut T>);

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

pub trait Clock {
    type Instant: Instant;
    fn now(&mut self) -> Self::Instant;
}

/// Errors that may occur when responding to a request
#[derive(Debug)]
pub enum RespondError<E> {
    /// The request could not be deserialized
    Deserialize(DeserializeError),
    /// Memory was not available
    OutOfMemory(OutOfMemoryError),
    /// The request handler returned an error
    Handler(E),
}

impl<E> From<DeserializeError> for RespondError<E> {
    fn from(deserialize: DeserializeError) -> Self {
        RespondError::Deserialize(deserialize)
    }
}

impl<E> From<OutOfMemoryError> for RespondError<E> {
    fn from(oom: OutOfMemoryError) -> Self {
        RespondError::OutOfMemory(oom)
    }
}
