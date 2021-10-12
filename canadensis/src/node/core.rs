use alloc::vec::Vec;
use core::marker::PhantomData;

use canadensis_core::time::{Clock, Instant};
use canadensis_core::transfer::{
    Header, MessageTransfer, ServiceHeader, ServiceTransfer, Transfer,
};
use canadensis_core::transport::{Receiver, Transmitter, Transport};
use canadensis_core::{OutOfMemoryError, ServiceId, ServiceSubscribeError, SubjectId};
use canadensis_encoding::{Message, Request, Response, Serialize};

use crate::hash::TrivialIndexMap;
use crate::publisher::Publisher;
use crate::requester::{Requester, TransferIdTracker};
use crate::serialize::do_serialize;
use crate::{Node, PublishToken, ResponseToken, ServiceToken, StartSendError, TransferHandler};

/// Basic UAVCAN node functionality
///
/// Type parameters:
/// * `C`: The clock used to get the current time
/// * `Q`: The queue type used to store outgoing frames
/// * `P`: The maximum number of topics that can be published
/// * `R`: The maximum number of services for which requests can be sent
///
pub struct CoreNode<C, T, U, TR, const P: usize, const R: usize>
where
    C: Clock,
    U: Receiver<C::Instant>,
    T: Transmitter<C::Instant>,
{
    clock: C,
    transmitter: T,
    receiver: U,
    node_id: <T::Transport as Transport>::NodeId,
    publishers: TrivialIndexMap<SubjectId, Publisher<C::Instant, T>, P>,
    requesters: TrivialIndexMap<ServiceId, Requester<C::Instant, T, TR>, R>,
}

impl<C, T, U, N, TR, const P: usize, const R: usize> CoreNode<C, T, U, TR, P, R>
where
    C: Clock,
    N: Transport,
    U: Receiver<C::Instant, Transport = N>,
    T: Transmitter<C::Instant, Transport = N>,
    TR: TransferIdTracker<N>,
{
    /// Creates a node
    ///
    /// * `clock`: A clock to use for frame deadlines and timeouts
    /// * `node_id`: The ID of this node
    /// * `transmitter`: A transport transmitter
    /// * `receiver`: A transport receiver
    pub fn new(
        clock: C,
        node_id: <T::Transport as Transport>::NodeId,
        transmitter: T,
        receiver: U,
    ) -> Self {
        CoreNode {
            clock,
            transmitter,
            receiver,
            node_id,
            publishers: TrivialIndexMap::new(),
            requesters: TrivialIndexMap::new(),
        }
    }

    fn handle_incoming_transfer<H>(
        &mut self,
        transfer: Transfer<Vec<u8>, C::Instant, U::Transport>,
        handler: &mut H,
    ) where
        H: TransferHandler<<Self as Node>::Instant, U::Transport>,
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
                    service: service_header.service.clone(),
                    client: service_header.source.clone(),
                    transfer: service_header.transfer_id.clone(),
                    priority: service_header.priority.clone(),
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
        token: ResponseToken<T::Transport>,
        deadline: C::Instant,
        payload: &[u8],
    ) -> Result<(), <T::Transport as Transport>::Error> {
        let transfer_out = Transfer {
            header: Header::Response(ServiceHeader {
                timestamp: deadline,
                transfer_id: token.transfer,
                priority: token.priority,
                service: token.service,
                source: self.node_id.clone(),
                destination: token.client,
            }),
            payload,
        };
        self.transmitter.push(transfer_out)
    }
}

impl<C, T, U, N, TR, const P: usize, const R: usize> Node for CoreNode<C, T, U, TR, P, R>
where
    C: Clock,
    N: Transport,
    T: Transmitter<<C as Clock>::Instant, Transport = N>,
    U: Receiver<<C as Clock>::Instant, Transport = N>,
    TR: TransferIdTracker<N>,
{
    type Clock = C;
    type Instant = <C as Clock>::Instant;
    type Transport = N;
    type Transmitter = T;
    type Receiver = U;

    fn accept_frame<H>(
        &mut self,
        frame: N::Frame,
        handler: &mut H,
    ) -> Result<(), <Self::Transport as Transport>::Error>
    where
        H: TransferHandler<Self::Instant, Self::Transport>,
    {
        if let Some(transfer) = self.receiver.accept(frame)? {
            self.handle_incoming_transfer(transfer, handler)
        }
        Ok(())
    }

    fn start_publishing<M>(
        &mut self,
        subject: SubjectId,
        timeout: <C::Instant as Instant>::Duration,
        priority: N::Priority,
    ) -> Result<PublishToken<M>, StartSendError<N::Error>>
    where
        M: Message,
    {
        let token = PublishToken(subject, PhantomData);
        if self.publishers.contains_key(&subject) {
            Err(StartSendError::Duplicate)
        } else {
            self.publishers
                .insert(
                    subject,
                    Publisher::new(self.node_id.clone(), timeout, priority),
                )
                .map(|_| token)
                .map_err(|_| StartSendError::Memory(OutOfMemoryError))
        }
    }

    fn stop_publishing<M>(&mut self, token: PublishToken<M>)
    where
        M: Message,
    {
        self.publishers.remove(&token.0);
    }

    fn publish<M>(&mut self, token: &PublishToken<M>, payload: &M) -> Result<(), N::Error>
    where
        M: Message + Serialize,
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
    fn start_sending_requests<M>(
        &mut self,
        service: ServiceId,
        receive_timeout: <C::Instant as Instant>::Duration,
        response_payload_size_max: usize,
        priority: N::Priority,
    ) -> Result<ServiceToken<M>, StartSendError<N::Error>>
    where
        M: Request,
    {
        let token = ServiceToken(service, PhantomData);
        if self.requesters.contains_key(&service) {
            Err(StartSendError::Duplicate)
        } else {
            self.requesters
                .insert(
                    service,
                    Requester::new(self.node_id.clone(), receive_timeout, priority),
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
                        ServiceSubscribeError::Transport(e) => Err(StartSendError::Transport(e)),
                        ServiceSubscribeError::Anonymous => {
                            unreachable!("CoreNode is never anonymous")
                        }
                    }
                }
            }
        }
    }

    fn stop_sending_requests<M>(&mut self, token: ServiceToken<M>)
    where
        M: Request,
    {
        self.requesters.remove(&token.0);
    }

    fn send_request<M>(
        &mut self,
        token: &ServiceToken<M>,
        payload: &M,
        destination: N::NodeId,
    ) -> Result<N::TransferId, <N as Transport>::Error>
    where
        M: Request + Serialize,
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
    ) -> Result<(), <Self::Transport as Transport>::Error> {
        self.receiver
            .subscribe_message(subject, payload_size_max, timeout)
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <C::Instant as Instant>::Duration,
    ) -> Result<(), <Self::Transport as Transport>::Error> {
        let status = self
            .receiver
            .subscribe_request(service, payload_size_max, timeout);
        // Because a CoreNode can't be anonymous, the above function can't return an Anonymous error.
        status.map_err(|e| match e {
            ServiceSubscribeError::Transport(e) => e,
            ServiceSubscribeError::Anonymous => unreachable!("CoreNode is never anonymous"),
        })
    }

    fn send_response<M>(
        &mut self,
        token: ResponseToken<Self::Transport>,
        timeout: <C::Instant as Instant>::Duration,
        payload: &M,
    ) -> Result<(), <Self::Transport as Transport>::Error>
    where
        M: Response + Serialize,
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

    /// TODO: Move to CanReceiver (this is CAN-specific)
    // fn frame_filters(&self) -> Result<Vec<Filter>, OutOfMemoryError> {
    //     self.receiver.frame_filters()
    // }

    fn transmitter(&self) -> &Self::Transmitter {
        &self.transmitter
    }
    fn transmitter_mut(&mut self) -> &mut Self::Transmitter {
        &mut self.transmitter
    }

    fn receiver(&self) -> &Self::Receiver {
        &self.receiver
    }
    fn receiver_mut(&mut self) -> &mut Self::Receiver {
        &mut self.receiver
    }

    /// Returns the identifier of this node
    fn node_id(&self) -> <Self::Transport as Transport>::NodeId {
        self.node_id.clone()
    }
}

// TODO: Move this to CanTransmitter
// impl<C, T, U, const P: usize, const R: usize> CoreNode<C, T, U, P, R>
// where
//     C: Clock,
//     T: Transmitter<C::Instant>,
//     U: Receiver<C::Instant>,
// {
//     /// Removes an outgoing frame from the queue and returns it
//     pub fn pop_frame(&mut self) -> Option<Frame<C::Instant>> {
//         self.transmitter.frame_queue_mut().pop_frame()
//     }
//
//     /// Returns a reference to the next outgoing frame in the queue, and does not remove it
//     pub fn peek_frame(&mut self) -> Option<&Frame<C::Instant>> {
//         self.transmitter.frame_queue_mut().peek_frame()
//     }
//
//     /// Returns an outgoing frame to the queue so that it can be transmitted later
//     pub fn return_frame(&mut self, frame: Frame<C::Instant>) -> Result<(), OutOfMemoryError> {
//         self.transmitter.frame_queue_mut().return_frame(frame)
//     }
// }
