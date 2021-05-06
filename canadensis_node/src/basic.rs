use crate::MinimalNode;
use alloc::prelude::v1::Vec;
use canadensis::{
    CapacityError, CapacityOrMemoryError, Node, PublishToken, ResponseToken, ServiceToken,
    TransferHandler,
};
use canadensis_can::{Frame, OutOfMemoryError};
use canadensis_core::time::{milliseconds, Clock, Duration, Instant};
use canadensis_core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis_core::{NodeId, Priority, ServiceId, SubjectId};
use canadensis_data_types::uavcan::node::get_info::{GetInfoRequest, GetInfoResponse};
use canadensis_data_types::uavcan::node::heartbeat::Heartbeat;
use canadensis_data_types::uavcan::node::port::list::List;
use canadensis_data_types::uavcan::node::port::subject_id;
use canadensis_data_types::uavcan::node::port::subject_id_list::SubjectIdList;
use canadensis_encoding::{Message, Request, Response, Serialize};

/// A node that provides all basic application-layer functionality
///
/// This node performs the following functions:
///
/// * Sending a `uavcan.node.Heartbeat` every second
/// * Responding to `uavcan.node.GetInfo` requests
/// * Sending a `uavcan.node.port.List` message every 10 seconds
///
pub struct BasicNode<N>
where
    N: Node,
{
    node: MinimalNode<N>,
    port_list_token: PublishToken<List>,
    last_port_list_seconds: u64,
    port_list: List,
}

impl<N> BasicNode<N>
where
    N: Node,
{
    pub fn new(mut node: N) -> Result<Self, CapacityOrMemoryError> {
        // The MinimalNode takes care of heartbeats.
        // Do node info and port list here.

        node.subscribe_request(GetInfoRequest::SERVICE, 0, milliseconds(1000))?;
        let port_list_token =
            node.start_publishing(List::SUBJECT, milliseconds(1000), Priority::Optional)?;

        let minimal = MinimalNode::new(node)?;

        // Initialize the port list with the Heartbeat publisher, GetInfo responder, and List publisher
        let mut port_list = List::default();
        port_list
            .servers
            .mask
            .set(GetInfoRequest::SERVICE.into(), true);
        port_list.publishers = SubjectIdList::SparseList({
            let mut published_topics = heapless::Vec::new();
            published_topics
                .push(subject_id::SubjectId {
                    value: Heartbeat::SUBJECT.into(),
                })
                .unwrap();
            published_topics
                .push(subject_id::SubjectId {
                    value: List::SUBJECT.into(),
                })
                .unwrap();
            published_topics
        });

        Ok(BasicNode {
            node: minimal,
            port_list_token,
            last_port_list_seconds: 0,
            port_list,
        })
    }

    /// This function must be called once per second (or more frequently) to send heartbeat
    /// messages
    pub fn run_periodic_tasks(&mut self) -> Result<(), OutOfMemoryError> {
        self.node.run_periodic_tasks()?;

        let now = self.node.node_mut().clock_mut().now();
        let since_start = now.duration_since(&self.node.start_time());
        let seconds_since_start = since_start.as_secs();
        if seconds_since_start >= self.last_port_list_seconds + 10 {
            self.last_port_list_seconds = seconds_since_start;
            self.publish_port_list()?;
        }

        Ok(())
    }

    fn publish_port_list(&mut self) -> Result<(), OutOfMemoryError> {
        self.node
            .node_mut()
            .publish(&self.port_list_token, &self.port_list)
    }
}

impl<N> Node for BasicNode<N>
where
    N: Node,
{
    type Clock = N::Clock;
    type FrameQueue = N::FrameQueue;

    fn accept_frame<H>(
        &mut self,
        frame: Frame<<N::Clock as Clock>::Instant>,
        handler: &mut H,
    ) -> Result<(), OutOfMemoryError>
    where
        H: TransferHandler<Self>,
    {
        let mut adapter = HandlerAdapter {
            basic_node: self,
            handler,
        };
        self.node.node_mut().accept_frame(frame, &mut adapter)
    }

    fn start_publishing<T>(
        &mut self,
        subject: SubjectId,
        timeout: <<N::Clock as Clock>::Instant as Instant>::Duration,
        priority: Priority,
    ) -> Result<PublishToken<T>, CapacityError>
    where
        T: Message,
    {
        // TODO: Record publishing
        self.node
            .node_mut()
            .start_publishing(subject, timeout, priority)
    }

    fn publish<T>(&mut self, token: &PublishToken<T>, payload: &T) -> Result<(), OutOfMemoryError>
    where
        T: Message + Serialize,
    {
        self.node.node_mut().publish(token, payload)
    }

    fn start_sending_requests<T>(
        &mut self,
        service: ServiceId,
        receive_timeout: <<N::Clock as Clock>::Instant as Instant>::Duration,
        response_payload_size_max: usize,
        priority: Priority,
    ) -> Result<ServiceToken<T>, CapacityOrMemoryError>
    where
        T: Request,
    {
        // TODO: Record client-ing
        self.node.node_mut().start_sending_requests(
            service,
            receive_timeout,
            response_payload_size_max,
            priority,
        )
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
        self.node
            .node_mut()
            .send_request(token, payload, destination)
    }

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <<N::Clock as Clock>::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError> {
        // TODO: Record subscription
        self.node
            .node_mut()
            .subscribe_message(subject, payload_size_max, timeout)
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <<N::Clock as Clock>::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError> {
        // TODO: Record subscription/serve
        self.node
            .node_mut()
            .subscribe_request(service, payload_size_max, timeout)
    }

    fn send_response<T>(
        &mut self,
        token: ResponseToken,
        timeout: <<N::Clock as Clock>::Instant as Instant>::Duration,
        payload: &T,
    ) -> Result<(), OutOfMemoryError>
    where
        T: Response + Serialize,
    {
        self.node.node_mut().send_response(token, timeout, payload)
    }

    fn clock(&self) -> &Self::Clock {
        self.node.node().clock()
    }

    fn clock_mut(&mut self) -> &mut Self::Clock {
        self.node.node_mut().clock_mut()
    }

    fn frame_queue(&self) -> &Self::FrameQueue {
        self.node.node().frame_queue()
    }

    fn frame_queue_mut(&mut self) -> &mut Self::FrameQueue {
        self.node.node_mut().frame_queue_mut()
    }

    fn node_id(&self) -> NodeId {
        self.node.node().node_id()
    }
}

/// Adapts a TransferHandler<BasicNode<N>> to a TransferHandler<N>
struct HandlerAdapter<'b, 'h, N, H>
where
    N: Node,
{
    basic_node: &'b mut BasicNode<N>,
    handler: &'h mut H,
}

impl<'b, 'h, N, H> TransferHandler<N> for HandlerAdapter<'b, 'h, N, H>
where
    H: TransferHandler<BasicNode<N>>,
    N: Node,
{
    fn handle_message(
        &mut self,
        _node: &mut N,
        transfer: MessageTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    ) {
        self.handler.handle_message(self.basic_node, transfer);
    }

    fn handle_request(
        &mut self,
        _node: &mut N,
        token: ResponseToken,
        transfer: ServiceTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    ) {
        self.handler
            .handle_request(self.basic_node, token, transfer);
        // TODO: Handle NodeInfo requests
    }

    fn handle_response(
        &mut self,
        _node: &mut N,
        transfer: ServiceTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    ) {
        self.handler.handle_response(self.basic_node, transfer);
    }
}
