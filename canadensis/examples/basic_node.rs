//! Runs a basic Cyphal node that sends Heartbeat messages, responds to node information requests,
//! and sends port list messages
//!
//! Usage: `basic_node [SocketCAN interface name] [Node ID]`
//!
//! # Testing
//!
//! ## Create a virtual CAN device
//!
//! ```
//! sudo modprobe vcan
//! sudo ip link add dev vcan0 type vcan
//! sudo ip link set up vcan0
//! ```
//!
//! ## Start the node
//!
//! ```
//! basic_node vcan0 [node ID]
//! ```
//!
//! ## Interact with the node using Yakut
//!
//! To subscribe and print out Heartbeat messages:
//! `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" subscribe uavcan.node.Heartbeat.1.0`
//!
//! To send a NodeInfo request:
//! `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" call [Node ID of basic_node] uavcan.node.GetInfo.1.0 {}`
//!
//! In the above two commands, 8 is the MTU of standard CAN and 42 is the node ID of the Yakut node.

extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_linux;
extern crate rand;
extern crate simplelog;
extern crate socketcan;

use std::convert::TryFrom;
use std::env;
use std::io::ErrorKind;
use std::time::Duration;

use socketcan::{CanSocket, Socket};

use canadensis::core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis::core::transport::Transport;
use canadensis::node::{BasicNode, CoreNode};
use canadensis::requester::TransferIdFixedMap;
use canadensis::{Node, ResponseToken, TransferHandler};
use canadensis_can::queue::{ArrayQueue, SingleQueueDriver};
use canadensis_can::{CanNodeId, CanReceiver, CanTransmitter, CanTransport, Error, Mtu};
use canadensis_data_types::uavcan::node::get_info_1_0::GetInfoResponse;
use canadensis_data_types::uavcan::node::version_1_0::Version;
use canadensis_linux::{LinuxCan, SystemClock};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Warn,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let mut args = env::args().skip(1);
    let can_interface = args.next().expect("Expected CAN interface name");
    let node_id = CanNodeId::try_from(
        args.next()
            .expect("Expected node ID")
            .parse::<u8>()
            .expect("Invalid node ID format"),
    )
    .expect("Node ID too large");

    println!(
        "Port list size: {} bytes",
        std::mem::size_of::<canadensis_data_types::uavcan::node::port::list_1_0::List>()
    );

    let can = CanSocket::open(&can_interface).expect("Failed to open CAN interface");
    can.set_read_timeout(Duration::from_millis(100))?;
    can.set_write_timeout(Duration::from_millis(100))?;
    let can = LinuxCan::new(can);

    // Set up information about this node
    let node_info = GetInfoResponse {
        protocol_version: Version { major: 1, minor: 0 },
        hardware_version: Version { major: 0, minor: 0 },
        software_version: Version { major: 0, minor: 1 },
        software_vcs_revision_id: 0,
        unique_id: rand::random(),
        name: heapless::Vec::from_slice(b"org.samcrow.basic_node").unwrap(),
        software_image_crc: heapless::Vec::new(),
        certificate_of_authenticity: Default::default(),
    };

    const QUEUE_CAPACITY: usize = 1210;
    type Queue = SingleQueueDriver<SystemClock, ArrayQueue<QUEUE_CAPACITY>, LinuxCan<CanSocket>>;
    let queue_driver: Queue = SingleQueueDriver::new(ArrayQueue::new(), can);

    // Create a node with capacity for 8 publishers and 8 requesters
    let transmitter = CanTransmitter::new(Mtu::Can8);
    let receiver = CanReceiver::new(node_id);

    const TRANSFER_IDS: usize = 8;
    const PUBLISHERS: usize = 8;
    const REQUESTERS: usize = 8;

    let core_node: CoreNode<
        SystemClock,
        CanTransmitter<SystemClock, Queue>,
        CanReceiver<SystemClock, Queue>,
        TransferIdFixedMap<CanTransport, TRANSFER_IDS>,
        Queue,
        PUBLISHERS,
        REQUESTERS,
    > = CoreNode::new(
        SystemClock::new(),
        node_id,
        transmitter,
        receiver,
        queue_driver,
    );
    let mut node = BasicNode::new(core_node, node_info).unwrap();

    let start_time = std::time::Instant::now();
    let mut prev_seconds = 0;
    loop {
        match node.receive(&mut EmptyHandler) {
            Ok(_) => {}
            Err(Error::Driver(e)) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => panic!("{:?}", e),
        }

        let seconds = std::time::Instant::now()
            .duration_since(start_time)
            .as_secs();
        if seconds != prev_seconds {
            prev_seconds = seconds;
            node.run_per_second_tasks().unwrap();
            node.flush().unwrap();
        }
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
