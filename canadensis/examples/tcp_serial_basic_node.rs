//! Runs a basic Cyphal node that sends Heartbeat messages, responds to node information requests,
//! and sends port list messages
//!
//! This node connects to a TCP server and uses the serial transport.
//!
//! Usage: `tcp_serial_basic_node [address:port] [Node ID]`
//!
//! # Testing
//!
//! ## Start a server
//!
//! ```
//! ncat --broker -l -p [port]
//! ```
//!
//! ## Start the node
//!
//! ```
//! tcp_serial_basic_node 127.0.0.1:[port] [Node ID]
//! ```
//!
//! ## Interact with the node using Yakut
//!
//! ```
//! yakut --transport "SerialTransport('socket://127.0.0.1:[port]', local_node_id=128)" monitor
//! ```
//!
//! In the above two commands, 8 is the MTU of standard CAN and 42 is the node ID of the Yakut node.

extern crate canadensis;
extern crate canadensis_serial;
extern crate rand;

use core::slice;
use std::convert::TryFrom;
use std::time::Duration;
use std::{env, io};

use canadensis::core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis::core::transport::Transport;
use canadensis::node::{BasicNode, CoreNode};
use canadensis::requester::TransferIdFixedMap;
use canadensis::{Node, ResponseToken, TransferHandler};
use canadensis_core::nb;
use canadensis_core::subscription::DynamicSubscriptionManager;
use canadensis_data_types::uavcan::node::get_info_1_0::GetInfoResponse;
use canadensis_data_types::uavcan::node::version_1_0::Version;
use canadensis_linux::SystemClock;
use canadensis_serial::driver::{ReceiveDriver, TransmitDriver};
use canadensis_serial::{
    Error, SerialNodeId, SerialReceiver, SerialTransmitter, SerialTransport, Subscription,
};
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let server_address = args.next().expect("Expected server address and port");
    let node_id = SerialNodeId::try_from(
        args.next()
            .expect("Expected node ID")
            .parse::<u16>()
            .expect("Invalid node ID format"),
    )
    .expect("Node ID too large");

    let socket = TcpStream::connect(server_address)?;
    socket.set_read_timeout(Some(Duration::from_millis(500)))?;
    socket.set_write_timeout(Some(Duration::from_millis(500)))?;
    let driver = SocketDriver(socket);

    // Set up information about this node
    let node_info = GetInfoResponse {
        protocol_version: Version { major: 1, minor: 0 },
        hardware_version: Version { major: 0, minor: 0 },
        software_version: Version { major: 0, minor: 1 },
        software_vcs_revision_id: 0,
        unique_id: rand::random(),
        name: heapless::Vec::from_slice(b"org.samcrow.tcp_serial_basic_node").unwrap(),
        software_image_crc: heapless::Vec::new(),
        certificate_of_authenticity: Default::default(),
    };

    // Create a node with capacity for 8 publishers and 8 requesters
    const TRANSFER_IDS: usize = 1;
    const PUBLISHERS: usize = 8;
    const REQUESTERS: usize = 8;

    let transmitter = SerialTransmitter::<_, 256>::new();
    let receiver = SerialReceiver::new(node_id);
    let core_node: CoreNode<
        SystemClock,
        SerialTransmitter<SocketDriver, 256>,
        SerialReceiver<SystemClock, SocketDriver, DynamicSubscriptionManager<Subscription>>,
        TransferIdFixedMap<SerialTransport, TRANSFER_IDS>,
        SocketDriver,
        PUBLISHERS,
        REQUESTERS,
    > = CoreNode::new(SystemClock::new(), node_id, transmitter, receiver, driver);
    let mut node = BasicNode::new(core_node, node_info).unwrap();

    let start_time = std::time::Instant::now();
    let mut prev_seconds = 0;
    loop {
        match node.receive(&mut EmptyHandler) {
            Ok(_) => {}
            Err(Error::Driver(e)) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => panic!("{:?}", e),
        };

        let seconds = std::time::Instant::now()
            .duration_since(start_time)
            .as_secs();
        if seconds != prev_seconds {
            prev_seconds = seconds;
            node.run_per_second_tasks().unwrap();
        }

        node.flush().unwrap();
    }
}

struct EmptyHandler;

impl<T: Transport> TransferHandler<T> for EmptyHandler {
    fn handle_message<N>(&mut self, _node: &mut N, transfer: &MessageTransfer<Vec<u8>, T>) -> bool
    where
        N: Node<Transport = T>,
    {
        println!("Got message {:?}", transfer);
        false
    }

    fn handle_request<N>(
        &mut self,
        _node: &mut N,
        _token: ResponseToken<T>,
        transfer: &ServiceTransfer<Vec<u8>, T>,
    ) -> bool
    where
        N: Node<Transport = T>,
    {
        println!("Got request {:?}", transfer);
        false
    }

    fn handle_response<N>(&mut self, _node: &mut N, transfer: &ServiceTransfer<Vec<u8>, T>) -> bool
    where
        N: Node<Transport = T>,
    {
        println!("Got response {:?}", transfer);
        false
    }
}

/// A serial driver that uses a TCP socket
struct SocketDriver(TcpStream);

impl TransmitDriver for SocketDriver {
    type Error = io::Error;

    fn send_byte(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        match self.0.write_all(&[byte]) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == ErrorKind::WouldBlock => Err(nb::Error::WouldBlock),
            Err(e) => Err(nb::Error::Other(e)),
        }
    }
}

impl ReceiveDriver for SocketDriver {
    type Error = io::Error;

    fn receive_byte(&mut self) -> nb::Result<u8, Self::Error> {
        let mut byte = 0;
        match self.0.read(slice::from_mut(&mut byte)) {
            Ok(_) => Ok(byte),
            Err(e) if e.kind() == ErrorKind::WouldBlock => Err(nb::Error::WouldBlock),
            Err(e) => Err(nb::Error::Other(e)),
        }
    }
}
