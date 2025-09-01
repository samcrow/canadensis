//! Runs a minimal Cyphal node, sending Heartbeat messages (and doing nothing else)
//!
//! Usage: `minimal_node [SocketCAN interface name] [Node ID]`
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
//! minimal_node vcan0 [node ID]
//! ```
//!
//! ## Interact with the node using Yakut
//!
//! To subscribe and print out Heartbeat messages:
//! `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" subscribe uavcan.node.Heartbeat.1.0`
//!
//! In the above command, 8 is the MTU of standard CAN and 42 is the node ID of the Yakut node.

extern crate canadensis;
extern crate canadensis_linux;
extern crate socketcan;

use std::convert::TryFrom;
use std::env;
use std::thread;
use std::time::Duration;

use socketcan::{CanSocket, Socket};

use canadensis::node::{CoreNode, MinimalNode};
use canadensis::requester::TransferIdFixedMap;
use canadensis::Node;
use canadensis_can::{CanNodeId, CanReceiver, CanTransmitter, CanTransport, Mtu};
use canadensis_linux::{LinuxCan, SystemClock};

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

    let can = CanSocket::open(&can_interface).expect("Failed to open CAN interface");
    can.set_read_timeout(Duration::from_millis(500))?;
    can.set_write_timeout(Duration::from_millis(500))?;
    let can = LinuxCan::new(can);

    // Create a node with capacity for 1 publisher and 1 requester
    let transmitter = CanTransmitter::new(Mtu::Can8);
    let receiver = CanReceiver::new(node_id);

    const TRANSFER_IDS: usize = 2;
    const PUBLISHERS: usize = 2;
    const REQUESTERS: usize = 2;
    let core_node: CoreNode<
        SystemClock,
        CanTransmitter<SystemClock, LinuxCan>,
        CanReceiver<SystemClock, LinuxCan>,
        TransferIdFixedMap<CanTransport, TRANSFER_IDS>,
        LinuxCan,
        PUBLISHERS,
        REQUESTERS,
    > = CoreNode::new(SystemClock::new(), node_id, transmitter, receiver, can);
    let mut node = MinimalNode::new(core_node).unwrap();

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
            node.node_mut().flush().unwrap();
        }

        thread::sleep(Duration::from_millis(500));
    }
}
