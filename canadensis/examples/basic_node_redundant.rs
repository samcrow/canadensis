extern crate canadensis;
extern crate canadensis_linux;
extern crate rand;
extern crate socketcan;

use std::convert::TryFrom;
use std::env;
use std::io;
use std::time::Duration;

use socketcan::CANSocket;

use canadensis::can::queue::{ArrayQueue, FrameQueueSource};
use canadensis::can::redundant::{Deduplicator, RedundantQueue};
use canadensis::can::Mtu;
use canadensis::core::time::{milliseconds, Instant, Microseconds64};
use canadensis::core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis::core::NodeId;
use canadensis::node::{BasicNode, CoreNode};
use canadensis::{Node, ResponseToken, TransferHandler};
use canadensis_data_types::uavcan::node::get_info::GetInfoResponse;
use canadensis_data_types::uavcan::node::version::Version;
use canadensis_linux::{LinuxCan, SystemClock};
use std::io::ErrorKind;

/// Runs a basic UAVCAN node that sends Heartbeat messages, responds to node information requests,
/// and sends port list messages with two redundant transports
///
/// Usage: `basic_node_redundant [SocketCAN interface name 0] [SocketCAN interface name 1] [Node ID]`
///
/// This node sends a copy of each outgoing frame on each interface. For incoming frames, it uses
/// the interface that receives the first frame and ignores the other interface. If the active
/// interface has not received any frames for one second, the node switches to the next interface
/// that receives a frame.
///
/// # Testing
///
/// ## Create a virtual CAN device
///
/// ```
/// sudo modprobe vcan
/// sudo ip link add dev vcan0 type vcan
/// sudo ip link set up vcan0
/// sudo ip link add dev vcan1 type vcan
/// sudo ip link set up vcan1
/// ```
///
/// ## Start the node
///
/// ```
/// basic_node_redundant vcan0 vcan1 [node ID]
/// ```
///
/// ## Interact with the node using Yakut
///
/// These commands currently use only one CAN interface. There may be a way to get Yakut to use
/// both.
///
/// To subscribe and print out Heartbeat messages:
/// `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" subscribe uavcan.node.Heartbeat.1.0`
///
/// To send a NodeInfo request:
/// `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" call [Node ID of basic_node] uavcan.node.GetInfo.1.0 {}`
///
/// In the above two commands, 8 is the MTU of standard CAN and 42 is the node ID of the Yakut node.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let can_interface_0 = args.next().expect("Expected CAN interface name");
    let can_interface_1 = args.next().expect("Expected CAN interface name");
    let node_id = NodeId::try_from(
        args.next()
            .expect("Expected node ID")
            .parse::<u8>()
            .expect("Invalid node ID format"),
    )
    .expect("Node ID too large");

    let mut cans = [open_can(&can_interface_0)?, open_can(&can_interface_1)?];

    // Set up information about this node
    let node_info = GetInfoResponse {
        protocol_version: Version { major: 1, minor: 0 },
        hardware_version: Version { major: 0, minor: 0 },
        software_version: Version { major: 0, minor: 1 },
        software_vcs_revision_id: 0,
        unique_id: rand::random(),
        name: heapless::Vec::from_slice(b"org.samcrow.basic_node_redundant").unwrap(),
        software_image_crc: None,
        certificate_of_authenticity: Default::default(),
    };

    // Redundant transport utilities
    let transmit_queue = RedundantQueue::new(
        ArrayQueue::<Microseconds64, 128>::new(),
        ArrayQueue::<Microseconds64, 128>::new(),
    );
    let mut deduplicator = Deduplicator::<_, 2>::new(milliseconds(1000));

    // Create a node with capacity for 8 publishers and 8 requesters
    let core_node: CoreNode<_, _, 8, 8> =
        CoreNode::new(SystemClock::new(), node_id, Mtu::Can8, transmit_queue);
    let mut node = BasicNode::new(core_node, node_info).unwrap();

    // Now that the node has subscribed to everything it wants, set up the frame acceptance filters
    let frame_filters = node.frame_filters().unwrap();
    println!("Filters: {:?}", frame_filters);
    cans[0].set_filters(&frame_filters)?;
    cans[1].set_filters(&frame_filters)?;

    let start_time = std::time::Instant::now();
    let mut prev_seconds = 0;
    loop {
        for (i, can) in cans.iter_mut().enumerate() {
            match can.receive() {
                Ok(frame) => {
                    println!("RX {}: {:?}", i, frame);
                    if deduplicator.accept(&frame, i) {
                        node.accept_frame(frame, &mut EmptyHandler).unwrap();
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::WouldBlock => {}
                    _ => return Err(e.into()),
                },
            };
        }

        let seconds = std::time::Instant::now()
            .duration_since(start_time)
            .as_secs();
        if seconds != prev_seconds {
            prev_seconds = seconds;
            node.run_per_second_tasks().unwrap();
        }

        while let Some(frame_out) = node.frame_queue_mut().queue_0_mut().pop_frame() {
            cans[0].send(frame_out)?;
        }
        while let Some(frame_out) = node.frame_queue_mut().queue_1_mut().pop_frame() {
            cans[1].send(frame_out)?;
        }
    }
}

fn open_can(name: &str) -> io::Result<LinuxCan> {
    let can = CANSocket::open(&name).expect("Failed to open CAN interface");
    can.set_read_timeout(Duration::from_millis(10))?;
    can.set_write_timeout(Duration::from_millis(10))?;
    Ok(LinuxCan::new(can))
}

struct EmptyHandler;

impl<I: Instant> TransferHandler<I> for EmptyHandler {
    fn handle_message<N>(&mut self, _node: &mut N, transfer: &MessageTransfer<Vec<u8>, I>) -> bool
    where
        N: Node<Instant = I>,
    {
        println!("Got message {:?}", transfer);
        false
    }

    fn handle_request<N>(
        &mut self,
        _node: &mut N,
        _token: ResponseToken,
        transfer: &ServiceTransfer<Vec<u8>, I>,
    ) -> bool
    where
        N: Node<Instant = I>,
    {
        println!("Got request {:?}", transfer);
        false
    }

    fn handle_response<N>(&mut self, _node: &mut N, transfer: &ServiceTransfer<Vec<u8>, I>) -> bool
    where
        N: Node<Instant = I>,
    {
        println!("Got response {:?}", transfer);
        false
    }
}
