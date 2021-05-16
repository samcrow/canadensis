use alloc::vec::Vec;
use core::iter;
use core::marker::PhantomData;

use fallible_collections::FallibleVec;

use canadensis_can::queue::{FrameQueueSource, FrameSink};
use canadensis_can::{Frame, Mtu, OutOfMemoryError, Receiver, ServiceSubscribeError, Transmitter};
use canadensis_core::time::{Clock, Instant};
use canadensis_core::transfer::{
    Header, MessageTransfer, ServiceHeader, ServiceTransfer, Transfer,
};
use canadensis_core::{NodeId, Priority, ServiceId, SubjectId, TransferId};
use canadensis_encoding::{Message, Request, Response, Serialize, WriteCursor};

use crate::hash::TrivialIndexMap;
use crate::publisher::Publisher;
use crate::requester::Requester;
use crate::{Node, PublishToken, ResponseToken, ServiceToken, StartSendError, TransferHandler};
use canadensis_filter_config::Filter;

/// A high-level interface with UAVCAN node functionality
///
/// Type parameters:
/// * `C`: The clock used to get the current time
/// * `Q`: The queue type used to store outgoing frames
/// * `P`: The maximum number of topics that can be published
/// * `R`: The maximum number of services for which requests can be sent
///
pub struct CoreNode<C, Q, const P: usize, const R: usize>
where
    C: Clock,
{
    clock: C,
    transmitter: Transmitter<Q>,
    receiver: Receiver<C::Instant>,
    node_id: NodeId,
    publishers: TrivialIndexMap<SubjectId, Publisher<C::Instant>, P>,
    requesters: TrivialIndexMap<ServiceId, Requester<C::Instant>, R>,
}

impl<C, Q, const P: usize, const R: usize> CoreNode<C, Q, P, R>
where
    C: Clock,
    Q: FrameSink<C::Instant>,
{
    pub fn new(clock: C, node_id: NodeId, mtu: Mtu, transmit_queue: Q) -> Self {
        CoreNode {
            clock,
            transmitter: Transmitter::new(mtu, transmit_queue),
            receiver: Receiver::new(node_id, mtu),
            node_id,
            publishers: TrivialIndexMap::new(),
            requesters: TrivialIndexMap::new(),
        }
    }

    fn handle_incoming_transfer<H>(
        &mut self,
        transfer: Transfer<Vec<u8>, C::Instant>,
        handler: &mut H,
    ) where
        H: TransferHandler<<Self as Node>::Instant>,
    {
        match transfer.header {
            Header::Message(message_header) => {
                let message_transfer = MessageTransfer {
                    header: message_header,
                    payload: transfer.payload,
                };
                handler.handle_message(self, &message_transfer);
            }
            Header::Request(service_header) => {
                let token = ResponseToken {
                    service: service_header.service,
                    client: service_header.source,
                    transfer: service_header.transfer_id,
                    priority: service_header.priority,
                };
                let service_transfer = ServiceTransfer {
                    header: service_header,
                    payload: transfer.payload,
                };
                handler.handle_request(self, token, &service_transfer);
            }
            Header::Response(service_header) => {
                let service_transfer = ServiceTransfer {
                    header: service_header,
                    payload: transfer.payload,
                };
                handler.handle_response(self, &service_transfer);
            }
        }
    }

    fn send_response_payload(
        &mut self,
        token: ResponseToken,
        deadline: C::Instant,
        payload: &[u8],
    ) -> Result<(), OutOfMemoryError> {
        let transfer_out = Transfer {
            header: Header::Response(ServiceHeader {
                timestamp: deadline,
                transfer_id: token.transfer,
                priority: token.priority,
                service: token.service,
                source: self.node_id,
                destination: token.client,
            }),
            payload,
        };
        self.transmitter.push(transfer_out)
    }
}

impl<C, Q, const P: usize, const R: usize> Node for CoreNode<C, Q, P, R>
where
    C: Clock,
    Q: FrameSink<C::Instant>,
{
    type Clock = C;
    type Instant = <C as Clock>::Instant;
    type FrameQueue = Q;

    fn accept_frame<H>(
        &mut self,
        frame: Frame<C::Instant>,
        handler: &mut H,
    ) -> Result<(), OutOfMemoryError>
    where
        H: TransferHandler<Self::Instant>,
    {
        if let Some(transfer) = self.receiver.accept(frame)? {
            self.handle_incoming_transfer(transfer, handler)
        }
        Ok(())
    }

    fn start_publishing<T>(
        &mut self,
        subject: SubjectId,
        timeout: <C::Instant as Instant>::Duration,
        priority: Priority,
    ) -> Result<PublishToken<T>, StartSendError>
    where
        T: Message,
    {
        let token = PublishToken(subject, PhantomData);
        if self.publishers.contains_key(&subject) {
            Err(StartSendError::Duplicate)
        } else {
            self.publishers
                .insert(subject, Publisher::new(self.node_id, timeout, priority))
                .map(|_| token)
                .map_err(|_| StartSendError::Memory(OutOfMemoryError))
        }
    }

    fn stop_publishing<T>(&mut self, token: PublishToken<T>)
    where
        T: Message,
    {
        self.publishers.remove(&token.0);
    }

    fn publish<T>(&mut self, token: &PublishToken<T>, payload: &T) -> Result<(), OutOfMemoryError>
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
    fn start_sending_requests<T>(
        &mut self,
        service: ServiceId,
        receive_timeout: <C::Instant as Instant>::Duration,
        response_payload_size_max: usize,
        priority: Priority,
    ) -> Result<ServiceToken<T>, StartSendError>
    where
        T: Request,
    {
        let token = ServiceToken(service, PhantomData);
        if self.requesters.contains_key(&service) {
            Err(StartSendError::Duplicate)
        } else {
            self.requesters
                .insert(
                    service,
                    Requester::new(self.node_id, receive_timeout, priority),
                )
                .map_err(|_| StartSendError::Memory(OutOfMemoryError))?;
            match self.receiver.subscribe_response(
                service,
                response_payload_size_max,
                receive_timeout,
            ) {
                Ok(()) => Ok(token),
                Err(e) => {
                    // Clean up requester
                    self.requesters.remove(&service);
                    // Because a CoreNode can't be anonymous, the above function can't return an Anonymous error.
                    match e {
                        ServiceSubscribeError::Memory(e) => Err(e.into()),
                        ServiceSubscribeError::Anonymous => {
                            unreachable!("CoreNode is never anonymous")
                        }
                    }
                }
            }
        }
    }

    fn stop_sending_requests<T>(&mut self, token: ServiceToken<T>)
    where
        T: Request,
    {
        self.requesters.remove(&token.0);
    }

    fn send_request<T>(
        &mut self,
        token: &ServiceToken<T>,
        payload: &T,
        destination: NodeId,
    ) -> Result<TransferId, OutOfMemoryError>
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

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <C::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError> {
        self.receiver
            .subscribe_message(subject, payload_size_max, timeout)
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <C::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError> {
        let status = self
            .receiver
            .subscribe_request(service, payload_size_max, timeout);
        // Because a CoreNode can't be anonymous, the above function can't return an Anonymous error.
        status.map_err(|e| match e {
            ServiceSubscribeError::Memory(e) => e,
            ServiceSubscribeError::Anonymous => unreachable!("CoreNode is never anonymous"),
        })
    }

    fn send_response<T>(
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

    /// Returns a reference to the enclosed clock
    fn clock(&self) -> &C {
        &self.clock
    }
    /// Returns a mutable reference to the enclosed clock
    fn clock_mut(&mut self) -> &mut C {
        &mut self.clock
    }

    fn frame_queue(&self) -> &Self::FrameQueue {
        self.transmitter.frame_queue()
    }

    fn frame_queue_mut(&mut self) -> &mut Self::FrameQueue {
        self.transmitter.frame_queue_mut()
    }

    /// Returns the identifier of this node
    fn node_id(&self) -> NodeId {
        self.node_id
    }

    fn frame_filters(&self) -> Result<Vec<Filter>, OutOfMemoryError> {
        self.receiver.frame_filters()
    }
}

impl<C, Q, const P: usize, const R: usize> CoreNode<C, Q, P, R>
where
    C: Clock,
    Q: FrameQueueSource<C::Instant>,
{
    /// Removes an outgoing frame from the queue and returns it
    pub fn pop_frame(&mut self) -> Option<Frame<C::Instant>> {
        self.transmitter.frame_queue_mut().pop_frame()
    }

    /// Returns a reference to the next outgoing frame in the queue, and does not remove it
    pub fn peek_frame(&mut self) -> Option<&Frame<C::Instant>> {
        self.transmitter.frame_queue_mut().peek_frame()
    }

    /// Returns an outgoing frame to the queue so that it can be transmitted later
    pub fn return_frame(&mut self, frame: Frame<C::Instant>) -> Result<(), OutOfMemoryError> {
        self.transmitter.frame_queue_mut().return_frame(frame)
    }
}

/// Payloads above this size (in bytes) will use a dynamically allocated buffer
const STACK_THRESHOLD: usize = 64;

/// Serializes a payload into a buffer and passes the buffer to a closure
pub(crate) fn do_serialize<T, F, R>(payload: &T, operation: F) -> Result<R, OutOfMemoryError>
where
    T: Serialize,
    F: FnOnce(&[u8]) -> Result<R, OutOfMemoryError>,
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
