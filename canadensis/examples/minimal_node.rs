extern crate canadensis;
extern crate canadensis_linux;
extern crate socketcan;

use std::convert::TryFrom;
use std::env;
use std::thread;
use std::time::Duration;

use socketcan::CANSocket;

use canadensis::core::time::Microseconds64;
use canadensis::node::{CoreNode, MinimalNode};
use canadensis::Node;
use canadensis_can::queue::{ArrayQueue, FrameQueueSource};
use canadensis_can::types::CanNodeId;
use canadensis_can::{CanReceiver, CanTransmitter, Mtu};
use canadensis_linux::{LinuxCan, SystemClock};

/// Runs a minimal UAVCAN node, sending Heartbeat messages (and doing nothing else)
///
/// Usage: `minimal_node [SocketCAN interface name] [Node ID]`
///
/// # Testing
///
/// ## Create a virtual CAN device
///
/// ```
/// sudo modprobe vcan
/// sudo ip link add dev vcan0 type vcan
/// sudo ip link set up vcan0
/// ```
///
/// ## Start the node
///
/// ```
/// minimal_node vcan0 [node ID]
/// ```
///
/// ## Interact with the node using Yakut
///
/// To subscribe and print out Heartbeat messages:
/// `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" subscribe uavcan.node.Heartbeat.1.0`
///
/// In the above command, 8 is the MTU of standard CAN and 42 is the node ID of the Yakut node.
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
    let mut can = LinuxCan::new(can);

    // Create a node with capacity for 1 publisher and 1 requester
    let frame_queue = ArrayQueue::<Microseconds64, 64>::new();
    let transmitter = CanTransmitter::new(Mtu::Can8, frame_queue);
    let receiver = CanReceiver::new(node_id, Mtu::Can8);
    let core_node: CoreNode<_, _, _, 1, 1> =
        CoreNode::new(SystemClock::new(), node_id, transmitter, receiver);
    let mut node = MinimalNode::new(core_node).unwrap();

    // Now that the node has subscribed to everything it wants, set up the frame acceptance filters
    let frame_filters = node.node().receiver().frame_filters().unwrap();
    println!("Filters: {:?}", frame_filters);
    can.set_filters(&frame_filters)?;

    let start_time = std::time::Instant::now();
    let mut prev_seconds = 0;
    loop {
        // Don't need to check for incoming frames because this node does not receive anything.

        let seconds = std::time::Instant::now()
            .duration_since(start_time)
            .as_secs();
        if seconds != prev_seconds {
            prev_seconds = seconds;
            node.run_per_second_tasks().unwrap();
        }

        while let Some(frame_out) = node
            .node_mut()
            .transmitter_mut()
            .frame_queue_mut()
            .pop_frame()
        {
            can.send(frame_out)?;
        }

        thread::sleep(Duration::from_millis(500));
    }
}
