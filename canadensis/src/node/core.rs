use alloc::vec::Vec;
use core::marker::PhantomData;
use heapless::FnvIndexMap;

use canadensis_core::time::{Clock, Instant};
use canadensis_core::transfer::{
    Header, MessageTransfer, ServiceHeader, ServiceTransfer, Transfer,
};
use canadensis_core::transport::{Receiver, Transmitter, Transport};
use canadensis_core::{nb, OutOfMemoryError, ServiceId, ServiceSubscribeError, SubjectId};
use canadensis_encoding::{Message, Request, Response, Serialize};

use crate::publisher::Publisher;
use crate::requester::{Requester, TransferIdTracker};
use crate::serialize::do_serialize;
use crate::{Node, PublishToken, ResponseToken, ServiceToken, StartSendError, TransferHandler};

/// Basic Cyphal node functionality
///
/// Type parameters:
/// * `C`: The clock used to get the current time
/// * `T`: The transmitter used to send transfers
/// * `U`: The receiver used to receive transfers
/// * `TR`: The transfer ID tracker used to manage transfer IDs for outgoing transfers
/// * `D`: The driver used to send and receive frames
/// * `P`: The maximum number of topics that can be published
///   This must be greater than 0, or the code will fail to compile. It also must be a power of
///   two, or the software may behave incorrectly.
/// * `R`: The maximum number of services for which requests can be sent
///   This must be greater than 0, or the code will fail to compile. It also must be a power of
///   two, or the software may behave incorrectly.
///
#[derive(Debug)]
pub struct CoreNode<C, T, U, TR, D, const P: usize, const R: usize>
where
    C: Clock,
    U: Receiver<C>,
    T: Transmitter<C>,
{
    clock: C,
    transmitter: T,
    receiver: U,
    driver: D,
    node_id: <T::Transport as Transport>::NodeId,
    publishers: FnvIndexMap<SubjectId, Publisher<C, T>, P>,
    requesters: FnvIndexMap<ServiceId, Requester<C, T, TR>, R>,
}

impl<C, T, U, N, TR, D, const P: usize, const R: usize> CoreNode<C, T, U, TR, D, P, R>
where
    C: Clock,
    N: Transport,
    U: Receiver<C, Transport = N, Driver = D>,
    T: Transmitter<C, Transport = N, Driver = D>,
    TR: TransferIdTracker<N>,
{
    /// Creates a node
    ///
    /// * `clock`: A clock to use for frame deadlines and timeouts
    /// * `node_id`: The ID of this node
    /// * `transmitter`: A transport transmitter
    /// * `receiver`: A transport receiver
    /// * `driver`: A driver compatible with `receiver` and `transmitter`
    pub fn new(
        clock: C,
        node_id: <T::Transport as Transport>::NodeId,
        transmitter: T,
        receiver: U,
        driver: D,
    ) -> Self {
        CoreNode {
            clock,
            transmitter,
            receiver,
            driver,
            node_id,
            publishers: FnvIndexMap::new(),
            requesters: FnvIndexMap::new(),
        }
    }

    /// Returns a reference to the enclosed driver
    pub fn driver(&self) -> &D {
        &self.driver
    }
    /// Returns a mutable reference to the enclosed driver
    pub fn driver_mut(&mut self) -> &mut D {
        &mut self.driver
    }

    /// Categorizes a transfer as a message, request, response, or loopback,
    /// and calls the corresponding method of the handler
    fn handle_incoming_transfer<H>(
        &mut self,
        transfer: Transfer<Vec<u8>, C::Instant, U::Transport>,
        handler: &mut H,
    ) where
        H: TransferHandler<<Self as Node>::Instant, U::Transport>,
    {
        if transfer.loopback {
            handler.handle_loopback(self, &transfer);
        } else {
            match transfer.header {
                Header::Message(message_header) => {
                    let message_transfer = MessageTransfer {
                        header: message_header,
                        loopback: transfer.loopback,
                        payload: transfer.payload,
                    };
                    handler.handle_message(self, &message_transfer);
                }
                Header::Request(service_header) => {
                    let token = ResponseToken {
                        service: service_header.service,
                        client: service_header.source.clone(),
                        transfer: service_header.transfer_id.clone(),
                        priority: service_header.priority.clone(),
                    };
                    let service_transfer = ServiceTransfer {
                        header: service_header,
                        loopback: transfer.loopback,
                        payload: transfer.payload,
                    };
                    handler.handle_request(self, token, &service_transfer);
                }
                Header::Response(service_header) => {
                    let service_transfer = ServiceTransfer {
                        header: service_header,
                        loopback: transfer.loopback,
                        payload: transfer.payload,
                    };
                    handler.handle_response(self, &service_transfer);
                }
            }
        }
    }

    fn send_response_payload(
        &mut self,
        token: ResponseToken<T::Transport>,
        deadline: C::Instant,
        payload: &[u8],
    ) -> nb::Result<(), T::Error> {
        let transfer_out = Transfer {
            header: Header::Response(ServiceHeader {
                timestamp: deadline,
                transfer_id: token.transfer,
                priority: token.priority,
                service: token.service,
                source: self.node_id.clone(),
                destination: token.client,
            }),
            loopback: false,
            payload,
        };
        self.transmitter
            .push(transfer_out, &mut self.clock, &mut self.driver)
    }
}

impl<C, T, U, N, TR, D, const P: usize, const R: usize> Node for CoreNode<C, T, U, TR, D, P, R>
where
    C: Clock,
    N: Transport,
    T: Transmitter<C, Transport = N, Driver = D>,
    U: Receiver<C, Transport = N, Driver = D>,
    TR: TransferIdTracker<N>,
{
    type Clock = C;
    type Instant = <C as Clock>::Instant;
    type Transport = N;
    type Transmitter = T;
    type Receiver = U;

    fn receive<H>(&mut self, handler: &mut H) -> Result<(), U::Error>
    where
        H: TransferHandler<Self::Instant, Self::Transport>,
    {
        if let Some(transfer) = self.receiver.receive(&mut self.clock, &mut self.driver)? {
            self.handle_incoming_transfer(transfer, handler)
        }
        Ok(())
    }

    fn start_publishing<M>(
        &mut self,
        subject: SubjectId,
        timeout: <C::Instant as Instant>::Duration,
        priority: N::Priority,
    ) -> Result<PublishToken<M>, StartSendError<T::Error>>
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

    fn publish<M>(&mut self, token: &PublishToken<M>, payload: &M) -> nb::Result<(), T::Error>
    where
        M: Message + Serialize,
    {
        let publisher = self
            .publishers
            .get_mut(&token.0)
            .expect("Bug: Token exists but no publisher");
        publisher.publish(
            &mut self.clock,
            token.0,
            payload,
            &mut self.transmitter,
            &mut self.driver,
        )
    }

    fn publish_loopback<M>(
        &mut self,
        token: &PublishToken<M>,
        payload: &M,
    ) -> nb::Result<(), T::Error>
    where
        M: Message + Serialize,
    {
        let publisher = self
            .publishers
            .get_mut(&token.0)
            .expect("Bug: Token exists but no publisher");
        publisher.publish_loopback(
            &mut self.clock,
            token.0,
            payload,
            &mut self.transmitter,
            &mut self.driver,
        )
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
    ) -> Result<ServiceToken<M>, StartSendError<U::Error>>
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
                &mut self.driver,
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
    ) -> nb::Result<N::TransferId, T::Error>
    where
        M: Request + Serialize,
    {
        let requester = self
            .requesters
            .get_mut(&token.0)
            .expect("Bug: No requester for token");
        requester.send(
            &mut self.clock,
            token.0,
            payload,
            destination,
            &mut self.transmitter,
            &mut self.driver,
        )
    }

    fn send_request_loopback<M>(
        &mut self,
        token: &ServiceToken<M>,
        payload: &M,
        destination: <Self::Transport as Transport>::NodeId,
    ) -> nb::Result<
        <Self::Transport as Transport>::TransferId,
        <Self::Transmitter as Transmitter<Self::Clock>>::Error,
    >
    where
        M: Request + Serialize,
    {
        let requester = self
            .requesters
            .get_mut(&token.0)
            .expect("Bug: No requester for token");
        requester.send_loopback(
            &mut self.clock,
            token.0,
            payload,
            destination,
            &mut self.transmitter,
            &mut self.driver,
        )
    }

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <C::Instant as Instant>::Duration,
    ) -> Result<(), U::Error> {
        self.receiver
            .subscribe_message(subject, payload_size_max, timeout, &mut self.driver)
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <C::Instant as Instant>::Duration,
    ) -> Result<(), U::Error> {
        let status =
            self.receiver
                .subscribe_request(service, payload_size_max, timeout, &mut self.driver);
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
    ) -> nb::Result<(), T::Error>
    where
        M: Response + Serialize,
    {
        let now = self.clock.now();
        let deadline = timeout + now;
        do_serialize(payload, |payload| {
            self.send_response_payload(token, deadline, payload)
        })
    }

    fn flush(&mut self) -> canadensis_core::nb::Result<(), T::Error> {
        self.transmitter.flush(&mut self.clock, &mut self.driver)
    }

    /// Returns a reference to the enclosed clock
    fn clock(&self) -> &C {
        &self.clock
    }
    /// Returns a mutable reference to the enclosed clock
    fn clock_mut(&mut self) -> &mut C {
        &mut self.clock
    }

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
