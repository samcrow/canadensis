use crate::core::transport::Transmitter;
use crate::node::{MinimalNode, NodeError};
use crate::{Node, PublishToken, ResponseToken, ServiceToken, StartSendError, TransferHandler};
use alloc::vec::Vec;
use canadensis_core::time::{milliseconds, Clock, Instant};
use canadensis_core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis_core::transport::{Receiver, Transport};
use canadensis_core::{nb, Priority, ServiceId, SubjectId};
use canadensis_data_types::uavcan::node::get_info_1_0::{self, GetInfoResponse};
use canadensis_data_types::uavcan::node::health_1_0::Health;
use canadensis_data_types::uavcan::node::heartbeat_1_0;
use canadensis_data_types::uavcan::node::mode_1_0::Mode;
use canadensis_data_types::uavcan::node::port::list_0_1::{self, List};
use canadensis_data_types::uavcan::node::port::service_id_list_0_1::ServiceIDList;
use canadensis_data_types::uavcan::node::port::subject_id_1_0;
use canadensis_data_types::uavcan::node::port::subject_id_list_0_1::SubjectIDList;
use canadensis_encoding::bits::BitArray;
use canadensis_encoding::{Message, Request, Response, Serialize};

/// A node that provides all basic application-layer functionality
///
/// This node performs the following functions:
///
/// * Sending a `uavcan.node.Heartbeat` every second
/// * Responding to `uavcan.node.GetInfo` requests
/// * Sending a `uavcan.node.port.List` message every 10 seconds
///
/// A BasicNode uses up two publisher slots in the underlying node.
///
/// The underlying node type `N` is usually a [`CoreNode`](crate::node::CoreNode).
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
    /// Creates a new basic node
    ///
    /// * `node`: The underlying node (this is usually a [`CoreNode`](crate::node::CoreNode))
    /// * `node_info`: The information that should be returned when handling node information requests
    pub fn new(
        mut node: N,
        node_info: GetInfoResponse,
    ) -> Result<
        Self,
        NodeError<
            StartSendError<<N::Transmitter as Transmitter<N::Instant>>::Error>,
            <N::Receiver as Receiver<N::Instant>>::Error,
        >,
    > {
        // The MinimalNode takes care of heartbeats.
        // Do node info and port list here.

        node.subscribe_request(get_info_1_0::SERVICE, 0, milliseconds(1000))
            .map_err(NodeError::Receiver)?;
        let port_list_token = node
            .start_publishing(
                list_0_1::SUBJECT,
                milliseconds(1000),
                Priority::Optional.into(),
            )
            .map_err(NodeError::Transmitter)?;

        let minimal = MinimalNode::new(node).map_err(NodeError::Transmitter)?;

        // Initialize the port list with the Heartbeat publisher, GetInfo responder, and List publisher
        let port_list = List {
            publishers: SubjectIDList::SparseList({
                let mut published_topics = heapless::Vec::new();
                published_topics
                    .push(subject_id_1_0::SubjectID {
                        value: heartbeat_1_0::SUBJECT.into(),
                    })
                    .ok()
                    .unwrap();
                published_topics
                    .push(subject_id_1_0::SubjectID {
                        value: list_0_1::SUBJECT.into(),
                    })
                    .ok()
                    .unwrap();
                published_topics
            }),
            subscribers: SubjectIDList::SparseList(heapless::Vec::new()),
            clients: ServiceIDList {
                mask: BitArray::new(512),
            },
            servers: {
                let mut servers = BitArray::new(512);
                servers.set(get_info_1_0::SERVICE.into(), true);
                ServiceIDList { mask: servers }
            },
        };

        Ok(BasicNode {
            node: minimal,
            port_list_token,
            port_list,
            node_info,
            seconds_since_port_list_published: 0,
        })
    }

    /// This function must be called once per second to send heartbeat and port list messages
    pub fn run_per_second_tasks(
        &mut self,
    ) -> nb::Result<(), <N::Transmitter as Transmitter<N::Instant>>::Error> {
        self.node.run_per_second_tasks()?;
        if self.seconds_since_port_list_published == 10 {
            self.seconds_since_port_list_published = 1;
            self.publish_port_list()?;
        } else {
            self.seconds_since_port_list_published += 1;
        }
        Ok(())
    }

    fn publish_port_list(
        &mut self,
    ) -> nb::Result<(), <N::Transmitter as Transmitter<N::Instant>>::Error> {
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
    type Transport = N::Transport;
    type Transmitter = N::Transmitter;
    type Receiver = N::Receiver;

    fn receive<H>(
        &mut self,
        handler: &mut H,
    ) -> Result<(), <N::Receiver as Receiver<N::Instant>>::Error>
    where
        H: TransferHandler<Self::Instant, Self::Transport, Self::Transmitter, Self::Receiver>,
    {
        let mut chained_handler = NodeInfoHandler {
            response: &self.node_info,
        }
        .chain(handler);
        self.node.node_mut().receive(&mut chained_handler)
    }

    fn start_publishing<T>(
        &mut self,
        subject: SubjectId,
        timeout: <<N::Clock as Clock>::Instant as Instant>::Duration,
        priority: <Self::Transport as Transport>::Priority,
    ) -> Result<PublishToken<T>, StartSendError<<N::Transmitter as Transmitter<N::Instant>>::Error>>
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

    fn publish<T>(
        &mut self,
        token: &PublishToken<T>,
        payload: &T,
    ) -> nb::Result<(), <N::Transmitter as Transmitter<N::Instant>>::Error>
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
        priority: <Self::Transport as Transport>::Priority,
    ) -> Result<ServiceToken<T>, StartSendError<<N::Receiver as Receiver<N::Instant>>::Error>>
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
        destination: <Self::Transport as Transport>::NodeId,
    ) -> nb::Result<
        <Self::Transport as Transport>::TransferId,
        <N::Transmitter as Transmitter<N::Instant>>::Error,
    >
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
    ) -> Result<(), <N::Receiver as Receiver<N::Instant>>::Error> {
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
    ) -> Result<(), <N::Receiver as Receiver<N::Instant>>::Error> {
        self.node
            .node_mut()
            .subscribe_request(service, payload_size_max, timeout)?;

        // Record that this node provides the service
        self.port_list.servers.mask.set(service.into(), true);

        Ok(())
    }

    fn send_response<T>(
        &mut self,
        token: ResponseToken<Self::Transport>,
        timeout: <<N::Clock as Clock>::Instant as Instant>::Duration,
        payload: &T,
    ) -> nb::Result<(), <N::Transmitter as Transmitter<N::Instant>>::Error>
    where
        T: Response + Serialize,
    {
        self.node.node_mut().send_response(token, timeout, payload)
    }

    fn flush(
        &mut self,
    ) -> canadensis_core::nb::Result<(), <N::Transmitter as Transmitter<N::Instant>>::Error> {
        self.node.node_mut().flush()
    }

    fn clock(&self) -> &Self::Clock {
        self.node.node().clock()
    }

    fn clock_mut(&mut self) -> &mut Self::Clock {
        self.node.node_mut().clock_mut()
    }

    fn transmitter(&self) -> &Self::Transmitter {
        self.node.node().transmitter()
    }

    fn transmitter_mut(&mut self) -> &mut Self::Transmitter {
        self.node.node_mut().transmitter_mut()
    }

    fn receiver(&self) -> &Self::Receiver {
        self.node.node().receiver()
    }

    fn receiver_mut(&mut self) -> &mut Self::Receiver {
        self.node.node_mut().receiver_mut()
    }

    fn node_id(&self) -> <Self::Transport as Transport>::NodeId {
        self.node.node().node_id()
    }
}

/// A transfer handler that responds to node information requests
struct NodeInfoResponder<'r, 'h, H> {
    /// The response to send
    info: &'r GetInfoResponse,
    /// The inner handler that will process any other incoming transfers
    inner: &'h mut H,
}

impl<'r, 'h, I, T, TX, RX, H> TransferHandler<I, T, TX, RX> for NodeInfoResponder<'r, 'h, H>
where
    I: Instant,
    H: TransferHandler<I, T, TX, RX>,
    T: Transport,
    TX: Transmitter<I>,
    RX: Receiver<I>,
{
    fn handle_message<N>(&mut self, node: &mut N, transfer: &MessageTransfer<Vec<u8>, I, T>) -> bool
    where
        N: Node<Instant = I, Transport = T, Transmitter = TX, Receiver = RX>,
    {
        // Forward to inner handler
        self.inner.handle_message(node, transfer)
    }

    fn handle_request<N>(
        &mut self,
        node: &mut N,
        token: ResponseToken<T>,
        transfer: &ServiceTransfer<Vec<u8>, I, T>,
    ) -> bool
    where
        N: Node<Instant = I, Transport = T, Transmitter = TX, Receiver = RX>,
    {
        if transfer.header.service == get_info_1_0::SERVICE {
            // Ignore out-of-memory errors
            let _ = node.send_response(token, milliseconds(1000), self.info);
            // Request handled
            true
        } else {
            // Forward to inner handler
            self.inner.handle_request(node, token, transfer)
        }
    }

    fn handle_response<N>(
        &mut self,
        node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, I, T>,
    ) -> bool
    where
        N: Node<Instant = I, Transport = T, Transmitter = TX, Receiver = RX>,
    {
        // Forward to inner handler
        self.inner.handle_response(node, transfer)
    }
}

fn insert_into_list(subject_list: &mut SubjectIDList, subject: SubjectId) {
    match subject_list {
        SubjectIDList::Mask(mask) => {
            mask.set(subject.into(), true);
        }
        SubjectIDList::SparseList(list) => {
            // Check that this subject is not already in the list
            if !list.iter().any(|in_list| in_list.value == subject.into()) {
                match list.push(subject_id_1_0::SubjectID {
                    value: subject.into(),
                }) {
                    Ok(_) => {}
                    Err(_) => {
                        // The list is full, need to switch to the mask representation
                        let mut mask = BitArray::new(SubjectIDList::CAPACITY as usize);
                        for port in list.iter() {
                            mask.set(port.value.into(), true);
                        }
                        // Set the bit for the topic that's now subscribed
                        mask.set(subject.into(), true);
                        *subject_list = SubjectIDList::Mask(mask);
                    }
                }
            }
        }
        SubjectIDList::Total(_) => { /* All subject IDs in use, can't add */ }
    };
}

fn remove_from_list(subject_list: &mut SubjectIDList, subject: SubjectId) {
    match subject_list {
        SubjectIDList::Mask(mask) => {
            mask.set(subject.into(), false);
        }
        SubjectIDList::SparseList(list) => {
            if let Some(position) = list
                .iter()
                .position(|id_in_list| id_in_list.value == u16::from(subject))
            {
                list.swap_remove(position);
            }
        }
        SubjectIDList::Total(_) => {
            // Convert from total into a mask with everything except subject set to 1
            let mut mask = BitArray::new(SubjectIDList::CAPACITY.into());
            mask.fill(true);
            mask.set(subject.into(), false);
            *subject_list = SubjectIDList::Mask(mask);
        }
    }
}

/// Responds to NodeInfo requests with the provided response
struct NodeInfoHandler<'r> {
    response: &'r GetInfoResponse,
}

impl<'r, I, T, TX, RX> TransferHandler<I, T, TX, RX> for NodeInfoHandler<'r>
where
    I: Instant,
    T: Transport,
    TX: Transmitter<I>,
    RX: Receiver<I>,
{
    fn handle_request<N: Node<Instant = I, Transport = T, Transmitter = TX, Receiver = RX>>(
        &mut self,
        node: &mut N,
        token: ResponseToken<T>,
        transfer: &ServiceTransfer<Vec<u8>, I, T>,
    ) -> bool {
        if transfer.header.service == get_info_1_0::SERVICE {
            let _ = node.send_response(token, milliseconds(1000), self.response);
            true
        } else {
            false
        }
    }
}
