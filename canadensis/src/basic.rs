use crate::MinimalNode;
use crate::{Node, PublishToken, ResponseToken, ServiceToken, StartSendError, TransferHandler};
use alloc::vec::Vec;
use canadensis_can::{Frame, OutOfMemoryError};
use canadensis_core::time::{milliseconds, Clock, Instant};
use canadensis_core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis_core::{NodeId, Priority, ServiceId, SubjectId, TransferId};
use canadensis_data_types::bits::BitArray;
use canadensis_data_types::uavcan::node::get_info::{GetInfoRequest, GetInfoResponse};
use canadensis_data_types::uavcan::node::health::Health;
use canadensis_data_types::uavcan::node::heartbeat::Heartbeat;
use canadensis_data_types::uavcan::node::mode::Mode;
use canadensis_data_types::uavcan::node::port::list::List;
use canadensis_data_types::uavcan::node::port::subject_id;
use canadensis_data_types::uavcan::node::port::subject_id_list::SubjectIdList;
use canadensis_encoding::{Message, Request, Response, Serialize};
use canadensis_filter_config::Filter;

/// A node that provides all basic application-layer functionality
///
/// This node performs the following functions:
///
/// * Sending a `uavcan.node.Heartbeat` every second
/// * Responding to `uavcan.node.GetInfo` requests
/// * Sending a `uavcan.node.port.List` message every 10 seconds
pub struct BasicNode<N>
where
    N: Node,
{
    node: MinimalNode<N>,
    port_list_token: PublishToken<List>,
    port_list: List,
    node_info: GetInfoResponse,
    seconds_since_port_list_published: u8,
}

impl<N> BasicNode<N>
where
    N: Node,
{
    pub fn new(mut node: N, node_info: GetInfoResponse) -> Result<Self, StartSendError> {
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
            port_list,
            node_info,
            seconds_since_port_list_published: 0,
        })
    }

    /// This function must be called once per second to send heartbeat and port list messages
    pub fn run_per_second_tasks(&mut self) -> Result<(), OutOfMemoryError> {
        self.node.run_per_second_tasks()?;
        if self.seconds_since_port_list_published == 10 {
            self.seconds_since_port_list_published = 1;
            self.publish_port_list()?;
        } else {
            self.seconds_since_port_list_published += 1;
        }
        Ok(())
    }

    fn publish_port_list(&mut self) -> Result<(), OutOfMemoryError> {
        self.node
            .node_mut()
            .publish(&self.port_list_token, &self.port_list)
    }

    /// Sets the operating mode that will be reported in the heartbeat messages
    pub fn set_mode(&mut self, mode: Mode) {
        self.node.set_mode(mode);
    }
    /// Sets the health status that will be reported in the heartbeat messages
    pub fn set_health(&mut self, health: Health) {
        self.node.set_health(health);
    }
    /// Sets the vendor-specific status code that will be reported in the heartbeat messages
    pub fn set_status_code(&mut self, status: u8) {
        self.node.set_status_code(status);
    }
}

impl<N> Node for BasicNode<N>
where
    N: Node,
{
    type Clock = N::Clock;
    type Instant = N::Instant;
    type FrameQueue = N::FrameQueue;

    fn accept_frame<H>(
        &mut self,
        frame: Frame<<N::Clock as Clock>::Instant>,
        handler: &mut H,
    ) -> Result<(), OutOfMemoryError>
    where
        H: TransferHandler<Self::Instant>,
    {
        let mut responder = NodeInfoResponder {
            info: &self.node_info,
            inner: handler,
        };

        self.node.node_mut().accept_frame(frame, &mut responder)
    }

    fn start_publishing<T>(
        &mut self,
        subject: SubjectId,
        timeout: <<N::Clock as Clock>::Instant as Instant>::Duration,
        priority: Priority,
    ) -> Result<PublishToken<T>, StartSendError>
    where
        T: Message,
    {
        let token = self
            .node
            .node_mut()
            .start_publishing(subject, timeout, priority)?;
        // Record that this port is in use
        insert_into_list(&mut self.port_list.publishers, subject);
        Ok(token)
    }

    fn stop_publishing<T>(&mut self, token: PublishToken<T>)
    where
        T: Message,
    {
        let subject = token.subject_id();
        self.node.node_mut().stop_publishing(token);
        remove_from_list(&mut self.port_list.publishers, subject);
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
    ) -> Result<ServiceToken<T>, StartSendError>
    where
        T: Request,
    {
        let token = self.node.node_mut().start_sending_requests(
            service,
            receive_timeout,
            response_payload_size_max,
            priority,
        )?;
        // Record that this node is a client for the service
        self.port_list.clients.mask.set(service.into(), true);

        Ok(token)
    }

    fn stop_sending_requests<T>(&mut self, token: ServiceToken<T>)
    where
        T: Request,
    {
        let service_id = token.service_id();
        self.node.node_mut().stop_sending_requests(token);
        self.port_list.clients.mask.set(service_id.into(), false);
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
        self.node
            .node_mut()
            .subscribe_message(subject, payload_size_max, timeout)?;

        // Record that this node is subscribed
        insert_into_list(&mut self.port_list.subscribers, subject);

        Ok(())
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <<N::Clock as Clock>::Instant as Instant>::Duration,
    ) -> Result<(), OutOfMemoryError> {
        self.node
            .node_mut()
            .subscribe_request(service, payload_size_max, timeout)?;

        // Record that this node provides the service
        self.port_list.servers.mask.set(service.into(), true);

        Ok(())
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

    fn frame_filters(&self) -> Result<Vec<Filter>, OutOfMemoryError> {
        self.node.node().frame_filters()
    }
}

/// A transfer handler that
struct NodeInfoResponder<'r, 'h, H> {
    /// The response to send
    info: &'r GetInfoResponse,
    inner: &'h mut H,
}

impl<'r, 'h, I, H> TransferHandler<I> for NodeInfoResponder<'r, 'h, H>
where
    I: Instant,
    H: TransferHandler<I>,
{
    fn handle_message<N>(&mut self, node: &mut N, transfer: &MessageTransfer<Vec<u8>, I>) -> bool
    where
        N: Node<Instant = I>,
    {
        // Forward to inner handler
        self.inner.handle_message(node, transfer)
    }

    fn handle_request<N>(
        &mut self,
        node: &mut N,
        token: ResponseToken,
        transfer: &ServiceTransfer<Vec<u8>, I>,
    ) -> bool
    where
        N: Node<Instant = I>,
    {
        if transfer.header.service == GetInfoResponse::SERVICE {
            // Ignore out-of-memory errors
            let _ = node.send_response(token, milliseconds(1000), self.info);
            // Request handled
            true
        } else {
            // Forward to inner handler
            self.inner.handle_request(node, token, transfer)
        }
    }

    fn handle_response<N>(&mut self, node: &mut N, transfer: &ServiceTransfer<Vec<u8>, I>) -> bool
    where
        N: Node<Instant = I>,
    {
        // Forward to inner handler
        self.inner.handle_response(node, transfer)
    }
}

fn insert_into_list(subject_list: &mut SubjectIdList, subject: SubjectId) {
    match subject_list {
        SubjectIdList::Mask(mask) => {
            mask.set(subject.into(), true);
        }
        SubjectIdList::SparseList(list) => {
            // Check that this subject is not already in the list
            let subject_id_message = subject_id::SubjectId {
                value: subject.into(),
            };
            if !list.contains(&subject_id_message) {
                match list.push(subject_id_message) {
                    Ok(_) => {}
                    Err(_) => {
                        // The list is full, need to switch to the mask representation
                        let mut mask = BitArray::new(SubjectIdList::CAPACITY as usize);
                        for port in list.iter() {
                            mask.set(port.value.into(), true);
                        }
                        // Set the bit for the topic that's now subscribed
                        mask.set(subject.into(), true);
                        *subject_list = SubjectIdList::Mask(mask);
                    }
                }
            }
        }
        SubjectIdList::Total => { /* All subject IDs in use, can't add */ }
    };
}

fn remove_from_list(subject_list: &mut SubjectIdList, subject: SubjectId) {
    match subject_list {
        SubjectIdList::Mask(mask) => {
            mask.set(subject.into(), false);
        }
        SubjectIdList::SparseList(list) => {
            if let Some(position) = list
                .iter()
                .position(|id_in_list| id_in_list.value == u16::from(subject))
            {
                list.swap_remove(position);
            }
        }
        SubjectIdList::Total => {
            // Convert from total into a mask with everything except subject set to 1
            let mut mask = BitArray::new(SubjectIdList::CAPACITY.into());
            mask.fill(true);
            mask.set(subject.into(), false);
            *subject_list = SubjectIdList::Mask(mask);
        }
    }
}
