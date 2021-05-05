use alloc::vec::Vec;

use canadensis_core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis_core::{NodeId, Priority, ServiceId, SubjectId};

use crate::{
    CapacityError, CapacityOrMemoryError, Node, PublishToken, ResponseToken, ServiceToken,
    TransferHandler,
};
use canadensis_can::{Frame, OutOfMemoryError};
use canadensis_core::time::{Clock, Instant};
use canadensis_encoding::{Message, Request, Response, Serialize};

/// A middleware entity that can respond to various node events
pub trait Middleware {
    //noinspection RsDropRef
    fn periodic_tasks<N: Node>(&mut self, node: &mut N) {
        drop(node);
    }

    fn handle_subscribe_message<N: Node>(&mut self, node: &mut N, subject: SubjectId) {
        drop((node, subject));
    }
    fn handle_subscribe_request<N: Node>(&mut self, node: &mut N, service: ServiceId) {
        drop((node, service));
    }
    fn handle_subscribe_response<N: Node>(&mut self, node: &mut N, service: ServiceId) {
        drop((node, service));
    }

    fn handle_start_publishing<N: Node>(&mut self, node: &mut N, subject: SubjectId) {
        drop((node, subject));
    }
    fn handle_start_sending_requests<N: Node>(&mut self, node: &mut N, service: ServiceId) {
        drop((node, service));
    }
    // TODO: Unsubscribe events

    /// Potentially handles an incoming message
    ///
    /// Returns true if this message has been handled, or false to let another handler process
    /// the message
    ///
    /// The default implementation does nothing and returns false.
    fn handle_message<N: Node>(
        &mut self,
        node: &mut N,
        transfer: &MessageTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    ) -> bool {
        drop((node, transfer));
        false
    }

    /// Potentially handles an incoming service request
    ///
    /// Returns true if this request has been handled, or false to let another handler process
    /// the request
    ///
    /// The default implementation does nothing and returns false.
    fn handle_request<N: Node>(
        &mut self,
        node: &mut N,
        token: ResponseToken,
        transfer: &ServiceTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    ) -> bool {
        drop((node, token, transfer));
        false
    }

    /// Potentially handles an incoming service response
    ///
    /// Returns true if this response has been handled, or false to let another handler process
    /// the response
    ///
    /// The default implementation does nothing and returns false.
    fn handle_response<N: Node>(
        &mut self,
        node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    ) -> bool {
        drop((node, transfer));
        false
    }
}

pub struct MiddlewareAdapter<N, M> {
    pub(crate) node: N,
    pub(crate) middleware: M,
}

impl<N, M> Node for MiddlewareAdapter<N, M>
where
    N: Node,
    M: Middleware,
{
    type Clock = N::Clock;
    type FrameQueue = N::FrameQueue;

    fn accept_frame<H>(
        &mut self,
        frame: Frame<<Self::Clock as Clock>::Instant>,
        handler: &mut H,
    ) -> Result<(), OutOfMemoryError>
    where
        H: TransferHandler,
    {
        let mut adapter = MiddlewareHandlerAdapter::new(&mut self.middleware, handler);
        self.node.accept_frame(frame, &mut adapter)
    }

    fn start_publishing<T>(
        &mut self,
        subject: SubjectId,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
        priority: Priority,
    ) -> Result<PublishToken<T>, CapacityError>
    where
        T: Message,
    {
        let token = self.node.start_publishing(subject, timeout, priority)?;
        self.middleware
            .handle_start_publishing(&mut self.node, subject);
        Ok(token)
    }

    fn publish<T>(&mut self, token: &PublishToken<T>, payload: &T) -> Result<(), OutOfMemoryError>
    where
        T: Message + Serialize,
    {
        // No middleware callback
        self.node.publish(token, payload)
    }

    fn start_sending_requests<T>(
        &mut self,
        service: ServiceId,
        receive_timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
        response_payload_size_max: usize,
        priority: Priority,
    ) -> Result<ServiceToken<T>, CapacityOrMemoryError>
    where
        T: Request,
    {
        let token = self.node.start_sending_requests(
            service,
            receive_timeout,
            response_payload_size_max,
            priority,
        )?;
        // The real implementation of start_sending_requests also subscribes to the response,
        // so this calls both middleware handler functions.
        self.middleware
            .handle_start_sending_requests(&mut self.node, service);
        self.middleware
            .handle_subscribe_response(&mut self.node, service);
        Ok(token)
    }

    fn send_request<T>(
        &mut self,
        token: &ServiceToken<T>,
        payload: &T,
        destination: NodeId,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Request + Serialize,
    {
        // No middleware callback
        self.node.send_request(token, payload, destination)
    }

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError> {
        self.node
            .subscribe_message(subject, payload_size_max, timeout)?;
        self.middleware
            .handle_subscribe_message(&mut self.node, subject);
        Ok(())
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError> {
        self.node
            .subscribe_request(service, payload_size_max, timeout)?;
        self.middleware
            .handle_subscribe_request(&mut self.node, service);
        Ok(())
    }

    fn send_response<T>(
        &mut self,
        token: ResponseToken,
        timeout: <<<Self as Node>::Clock as Clock>::Instant as Instant>::Duration,
        payload: &T,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Response + Serialize,
    {
        // No middleware callback
        self.node.send_response(token, timeout, payload)
    }

    fn clock(&self) -> &Self::Clock {
        // No middleware callback
        self.node.clock()
    }

    fn clock_mut(&mut self) -> &mut Self::Clock {
        // No middleware callback
        self.node.clock_mut()
    }

    fn frame_queue(&self) -> &Self::FrameQueue {
        // No middleware callback
        self.node.frame_queue()
    }

    fn frame_queue_mut(&mut self) -> &mut Self::FrameQueue {
        // No middleware callback
        self.node.frame_queue_mut()
    }

    fn node_id(&self) -> NodeId {
        // No middleware callback
        self.node.node_id()
    }
}

/// Adapts a middleware and a standard handler to the handler interface
///
/// The middleware is given each transfer first. If the middleware does not handle a transfer,
/// the transfer is then given to the standard handler.
struct MiddlewareHandlerAdapter<'m, 'h, M, H> {
    middleware: &'m mut M,
    handler: &'h mut H,
}

impl<'m, 'h, M, H> MiddlewareHandlerAdapter<'m, 'h, M, H> {
    pub fn new(middleware: &'m mut M, handler: &'h mut H) -> Self {
        MiddlewareHandlerAdapter {
            middleware,
            handler,
        }
    }
}

impl<'m, 'h, M, H> TransferHandler for MiddlewareHandlerAdapter<'m, 'h, M, H>
where
    M: Middleware,
    H: TransferHandler,
{
    fn handle_message<N: Node>(
        &mut self,
        node: &mut N,
        transfer: MessageTransfer<Vec<u8>, <<N as Node>::Clock as Clock>::Instant>,
    ) {
        let handled_by_middleware = self.middleware.handle_message(node, &transfer);
        if !handled_by_middleware {
            self.handler.handle_message(node, transfer);
        }
    }

    fn handle_request<N: Node>(
        &mut self,
        node: &mut N,
        token: ResponseToken,
        transfer: ServiceTransfer<Vec<u8>, <<N as Node>::Clock as Clock>::Instant>,
    ) {
        let handled_by_middleware =
            self.middleware
                .handle_request(node, token.private_clone(), &transfer);
        if !handled_by_middleware {
            self.handler.handle_request(node, token, transfer);
        }
    }

    fn handle_response<N: Node>(
        &mut self,
        node: &mut N,
        transfer: ServiceTransfer<Vec<u8>, <<N as Node>::Clock as Clock>::Instant>,
    ) {
        let handled_by_middleware = self.middleware.handle_response(node, &transfer);
        if !handled_by_middleware {
            self.handler.handle_response(node, transfer);
        }
    }
}
