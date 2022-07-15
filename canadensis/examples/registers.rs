//! Runs a Cyphal node that sends Heartbeat messages, responds to node information requests,
//! sends port list messages, and allows access to some registers
//!
//! Usage: `registers [SocketCAN interface name] [Node ID]`
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
//! registers vcan0 [node ID]
//! ```
//!
//! ## Interact with the node using Yakut
//!
//! To subscribe and print out Heartbeat messages:
//! `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" subscribe uavcan.node.Heartbeat.1.0`
//!
//! To send a NodeInfo request:
//! `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" call [Node ID of registers node] uavcan.node.GetInfo.1.0 {}`
//!
//! To get the name of register 0:
//! `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" call [Node ID of registers node] uavcan.register.List.1.0 "{ index: 0 }"`
//!
//! To read the node ID register:
//! `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" call [Node ID of registers node] uavcan.register.Access.1.0 "{ name: { name: \"uavcan.node.id\" } }"`
//!
//! To write the node ID register:
//! `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" call [Node ID of registers node] uavcan.register.Access.1.0 "{ name: { name: \"uavcan.node.id\" }, value: { natural16: { value: [value to write] }  }  }"`
//!
//! To write a 256-character-long node description:
//! `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" call [Node ID of registers node] uavcan.register.Access.1.0 "{ name: { name: \"uavcan.node.description\" }, value: { string: { value: \"We're no strangers to love\nYou know the rules and so do I\nA full commitment's what I'm thinking of\nYou wouldn't get this from any other guy\nI just wanna tell you how I'm feeling\nGotta make you understand\nNever gonna give you up\nNever gonna let you down\nNev\" }  }  }"`
//!
//! In the above two commands, 8 is the MTU of standard CAN and 42 is the node ID of the Yakut node.

extern crate canadensis;
extern crate canadensis_linux;
extern crate rand;
extern crate socketcan;

use std::convert::TryFrom;
use std::env;
use std::time::Duration;

use socketcan::CANSocket;

use canadensis::core::time::{Clock, Microseconds64};
use canadensis::core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis::node::{BasicNode, CoreNode};
use canadensis::register::basic::{RegisterString, SimpleRegister};
use canadensis::register::{RegisterBlock, RegisterHandler};
use canadensis::requester::TransferIdFixedMap;
use canadensis::{Node, ResponseToken, TransferHandler, TransferHandlerChain};
use canadensis_can::queue::{ArrayQueue, SingleQueueDriver};
use canadensis_can::{CanNodeId, CanReceiver, CanTransmitter, CanTransport, Error, Mtu};
use canadensis_data_types::uavcan::node::get_info_1_0::GetInfoResponse;
use canadensis_data_types::uavcan::node::version_1_0::Version;
use canadensis_linux::{LinuxCan, SystemClock};
use std::io::ErrorKind;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let can_interface = args.next().expect("Expected CAN interface name");
    let node_id = CanNodeId::try_from(
        args.next()
            .expect("Expected node ID")
            .parse::<u8>()
            .expect("Invalid node ID format"),
    )
    .expect("Node ID too large");

    let can = CANSocket::open(&can_interface).expect("Failed to open CAN interface");
    can.set_read_timeout(Duration::from_millis(500))?;
    can.set_write_timeout(Duration::from_millis(500))?;
    let can = LinuxCan::new(can);

    // Set up information about this node
    let node_info = GetInfoResponse {
        protocol_version: Version { major: 1, minor: 0 },
        hardware_version: Version { major: 0, minor: 0 },
        software_version: Version { major: 0, minor: 1 },
        software_vcs_revision_id: 0,
        unique_id: rand::random(),
        name: heapless::Vec::from_slice(b"org.samcrow.register_node").unwrap(),
        software_image_crc: heapless::Vec::new(),
        certificate_of_authenticity: Default::default(),
    };

    // Create a node with capacity for 82 publishers and 2 requesters
    type Queue = SingleQueueDriver<ArrayQueue<Microseconds64, 64>, LinuxCan>;
    const TRANSFER_IDS: usize = 1;
    const PUBLISHERS: usize = 2;
    const REQUESTERS: usize = 2;

    let queue = Queue::new(ArrayQueue::new(), can);
    let transmitter = CanTransmitter::new(Mtu::Can8);
    let receiver = CanReceiver::new(node_id, Mtu::Can8);
    let core_node: CoreNode<
        SystemClock,
        CanTransmitter<Microseconds64, Queue>,
        CanReceiver<Microseconds64, Queue>,
        TransferIdFixedMap<CanTransport, TRANSFER_IDS>,
        Queue,
        PUBLISHERS,
        REQUESTERS,
    > = CoreNode::new(SystemClock::new(), node_id, transmitter, receiver, queue);
    let mut node = BasicNode::new(core_node, node_info).unwrap();

    // Define the registers that can be accessed
    #[derive(RegisterBlock)]
    struct Registers {
        node_id: SimpleRegister<u16>,
        description: SimpleRegister<RegisterString>,
    }
    let register_block = Registers {
        node_id: SimpleRegister::with_value("uavcan.node.id", true, false, u16::MAX),
        description: SimpleRegister::new("uavcan.node.description", true, false),
    };
    let registers = RegisterHandler::new(register_block);
    RegisterHandler::<Registers>::subscribe_requests(&mut node).unwrap();

    let mut handler: TransferHandlerChain<RegisterHandler<Registers>, EmptyHandler> =
        registers.chain(EmptyHandler);

    let start_time = std::time::Instant::now();
    let mut prev_seconds = 0;
    loop {
        match node.receive(&mut handler) {
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

impl TransferHandler<<SystemClock as Clock>::Instant, CanTransport> for EmptyHandler {
    fn handle_message<N>(
        &mut self,
        _node: &mut N,
        transfer: &MessageTransfer<Vec<u8>, <SystemClock as Clock>::Instant, CanTransport>,
    ) -> bool
    where
        N: Node<Instant = <SystemClock as Clock>::Instant, Transport = CanTransport>,
    {
        println!("Got message {:?}", transfer);
        false
    }

    fn handle_request<N>(
        &mut self,
        _node: &mut N,
        _token: ResponseToken<CanTransport>,
        transfer: &ServiceTransfer<Vec<u8>, <SystemClock as Clock>::Instant, CanTransport>,
    ) -> bool
    where
        N: Node<Instant = <SystemClock as Clock>::Instant, Transport = CanTransport>,
    {
        println!("Got request {:?}", transfer);
        false
    }

    fn handle_response<N>(
        &mut self,
        _node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, <SystemClock as Clock>::Instant, CanTransport>,
    ) -> bool
    where
        N: Node<Instant = <SystemClock as Clock>::Instant, Transport = CanTransport>,
    {
        println!("Got response {:?}", transfer);
        false
    }
}
