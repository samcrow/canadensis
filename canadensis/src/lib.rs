#![no_std]

extern crate alloc;
extern crate embedded_time;
extern crate fallible_collections;

extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_encoding;
extern crate canadensis_node;

// Reexports from other canadensis crates
pub use canadensis_can::*;
pub use canadensis_core::transfer;
pub use canadensis_core::*;
pub use canadensis_encoding::*;
pub mod node {
    ///! Basic node functionality
    pub use canadensis_node::*;
}

use alloc::vec::Vec;
use core::iter;

use canadensis_core::transfer::*;
use canadensis_encoding::{Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor};
use embedded_time::{Clock, Instant};
use fallible_collections::FallibleVec;

/// Payloads above this size (in bytes) will use a dynamically allocated buffer
const STACK_THRESHOLD: usize = 64;

/// Assembles transfers and manages transfer IDs to send messages
pub struct Publisher {
    /// The ID of this node
    source: NodeId,
    /// The priority of transfers from this transmitter
    priority: Priority,
    /// The subject to transmit on
    subject: SubjectId,
    /// The ID of the next transfer sent
    next_transfer_id: TransferId,
}

impl Publisher {
    /// Creates a message transmitter
    ///
    /// node: The ID of this node
    /// priority: The priority to use for messages
    /// subject: The subject ID to publish to
    pub const fn new(node: NodeId, priority: Priority, subject: SubjectId) -> Self {
        Publisher {
            source: node,
            priority,
            subject,
            next_transfer_id: TransferId::const_default(),
        }
    }

    pub fn send<T, C>(
        &mut self,
        payload: &T,
        deadline: Instant<C>,
        transmitter: &mut Transmitter<C>,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Serialize,
        C: Clock,
    {
        // Part 1: Serialize
        do_serialize(payload, |payload_bytes| {
            self.send_payload(payload_bytes, deadline, transmitter)
        })
    }

    pub fn send_payload<C>(
        &mut self,
        payload: &[u8],
        deadline: Instant<C>,
        transmitter: &mut Transmitter<C>,
    ) -> Result<(), OutOfMemoryError>
    where
        C: Clock,
    {
        // Assemble the transfer
        let transfer: Transfer<&[u8], C> = Transfer {
            timestamp: deadline,
            header: TransferHeader {
                source: self.source,
                priority: self.priority,
                kind: TransferKindHeader::Message(MessageHeader {
                    anonymous: false,
                    subject: self.subject,
                }),
            },
            transfer_id: self.next_transfer_id,
            payload,
        };
        self.next_transfer_id = self.next_transfer_id.increment();

        transmitter.push(transfer)
    }
}

/// A transmitter that sends anonymous messages and does not require a node ID
pub struct AnonymousPublisher {
    /// The priority of transfers from this transmitter
    priority: Priority,
    /// The subject to transmit on
    subject: SubjectId,
    /// The ID of the next transfer sent
    next_transfer_id: TransferId,
}

impl AnonymousPublisher {
    /// Creates an anonymous message transmitter
    ///
    /// priority: The priority to use for messages
    /// subject: The subject ID to publish to
    pub fn new(priority: Priority, subject: SubjectId) -> Self {
        AnonymousPublisher {
            priority,
            subject,
            next_transfer_id: TransferId::const_default(),
        }
    }

    pub fn send<T, C>(
        &mut self,
        payload: &T,
        deadline: Instant<C>,
        transmitter: &mut Transmitter<C>,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Serialize,
        C: Clock,
    {
        // Part 1: Serialize
        do_serialize(payload, |payload_bytes| {
            self.send_payload(payload_bytes, deadline, transmitter)
        })
    }

    pub fn send_payload<C>(
        &mut self,
        payload: &[u8],
        deadline: Instant<C>,
        transmitter: &mut Transmitter<C>,
    ) -> Result<(), OutOfMemoryError>
    where
        C: Clock,
    {
        // Assemble the transfer
        let transfer: Transfer<&[u8], C> = Transfer {
            timestamp: deadline,
            header: TransferHeader {
                source: make_pseudo_id(payload),
                priority: self.priority,
                kind: TransferKindHeader::Message(MessageHeader {
                    anonymous: false,
                    subject: self.subject,
                }),
            },
            transfer_id: self.next_transfer_id,
            payload,
        };
        self.next_transfer_id = self.next_transfer_id.increment();

        transmitter.push(transfer)
    }
}

/// Assembles transfers and manages transfer IDs to send service requests
pub struct Requester {
    /// The ID of this node
    this_node: NodeId,
    /// The priority of transfers from this transmitter
    priority: Priority,
    /// The service ID to transmit on
    service: ServiceId,
    /// The ID of the next transfer sent
    next_transfer_id: TransferId,
}

impl Requester {
    /// Creates a service request transmitter
    ///
    /// this_node: The ID of this node
    /// priority: The priority to use for messages
    /// service: The service ID to request
    pub fn new(this_node: NodeId, priority: Priority, service: ServiceId) -> Self {
        Requester {
            this_node,
            priority,
            service,
            next_transfer_id: TransferId::const_default(),
        }
    }

    pub fn send<T, C>(
        &mut self,
        payload: &T,
        destination: NodeId,
        deadline: Instant<C>,
        transmitter: &mut Transmitter<C>,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Serialize,
        C: Clock,
    {
        // Part 1: Serialize
        do_serialize(payload, |payload_bytes| {
            self.send_payload(payload_bytes, destination, deadline, transmitter)
        })
    }

    pub fn send_payload<C>(
        &mut self,
        payload: &[u8],
        destination: NodeId,
        deadline: Instant<C>,
        transmitter: &mut Transmitter<C>,
    ) -> Result<(), OutOfMemoryError>
    where
        C: Clock,
    {
        // Assemble the transfer
        let transfer: Transfer<&[u8], C> = Transfer {
            timestamp: deadline,
            header: TransferHeader {
                source: self.this_node,
                priority: self.priority,
                kind: TransferKindHeader::Request(ServiceHeader {
                    service: self.service,
                    destination,
                }),
            },
            transfer_id: self.next_transfer_id,
            payload,
        };
        self.next_transfer_id = self.next_transfer_id.increment();

        transmitter.push(transfer)
    }
}

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

fn make_pseudo_id(payload: &[u8]) -> NodeId {
    // XOR some things. I don't know if this will actually work well.
    let mut id_bits = 37u8;
    for &byte in payload {
        id_bits ^= byte;
    }
    // Get a non-reserved ID
    loop {
        let id = NodeId::from_truncating(id_bits);
        if !id.is_diagnostic_reserved() {
            // Got a valid, non-diagnostic ID
            break id;
        }
        // This one is reserved. Try one lower.
        id_bits = id_bits.wrapping_sub(1);
    }
}

/// Handles incoming service requests and sends responses
pub struct Responder {
    /// The ID of this node
    this_node: NodeId,
    /// The service ID to handle
    service: ServiceId,
}

impl Responder {
    /// Creates a responder to handle requests for a specific service
    pub const fn new(this_node: NodeId, service: ServiceId) -> Self {
        Responder { this_node, service }
    }

    /// Returns true if this responder is interested in handling a request with the provided header
    pub fn interested(&self, header: &TransferHeader) -> bool {
        match header.kind {
            TransferKindHeader::Request(ServiceHeader {
                destination,
                service,
            }) if destination == self.this_node && service == self.service => true,
            _ => false,
        }
    }

    /// Handles an incoming request and sends a response
    ///
    /// transfer_in: An incoming transfer. This must be a service request transfer with a
    /// destination of this node and for a service matching this responder's service ID.
    /// This function will panic if any of those conditions is not satisfied. These are the same
    /// conditions that the interested() function checks.
    ///
    /// response_deadline: The time by which the response must finish sending
    ///
    /// transmitter: The transmitter to use when sending the response
    ///
    /// handler: A function that takes a request and returns a response
    pub fn handle_request<Q, R, H, E, C>(
        &mut self,
        transfer_in: Transfer<Vec<u8>, C>,
        response_deadline: Instant<C>,
        transmitter: &mut Transmitter<C>,
        handler: H,
    ) -> Result<(), RespondError<E>>
    where
        Q: Deserialize,
        R: Serialize,
        H: FnOnce(Q) -> Result<R, E>,
        C: Clock,
    {
        // Check that this is a service request and has the correct parameters
        assert!(
            self.interested(&transfer_in.header),
            "Can't handle an unrelated transfer"
        );
        let transfer_in_id = transfer_in.transfer_id;
        let transfer_in_source = transfer_in.header.source;
        let priority = transfer_in.header.priority;

        // Deserialize request
        let request = Q::deserialize(&mut ReadCursor::new(&transfer_in.payload))?;
        drop(transfer_in.payload);
        // Handle the request
        let response: R = handler(request).map_err(RespondError::Handler)?;
        do_serialize(&response, |response_bytes| {
            let transfer_out = Transfer {
                timestamp: response_deadline,
                header: TransferHeader {
                    source: self.this_node,
                    priority,
                    kind: TransferKindHeader::Response(ServiceHeader {
                        service: self.service,
                        destination: transfer_in_source,
                    }),
                },
                transfer_id: transfer_in_id,
                payload: response_bytes,
            };
            transmitter.push(transfer_out)
        })?;

        Ok(())
    }
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
