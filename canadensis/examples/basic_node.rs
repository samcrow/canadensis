//!
//! Runs a simple UAVCAN node using the canadensis library
//!

extern crate canadensis;
extern crate embedded_time;
extern crate rand;
extern crate socketcan;

use std::convert::{Infallible, TryFrom};
use std::env;
use std::io;
use std::time::{Duration, Instant};

use socketcan::CANSocket;

use canadensis::node::{Mode, Node, NodeInfo, NodeInfoRequest};
use canadensis::{CanId, Frame, Mtu, NodeId, Publisher, Receiver, Responder, Transmitter};
use canadensis_core::Priority;
use embedded_time::duration::{Fraction, Microseconds};
use embedded_time::Clock;

/// Runs a basic UAVCAN node, sending Heartbeat messages and responding to NodeInfo requests
///
/// Usage: `basic_node [SocketCAN interface name] [Node ID]`
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
/// basic_node vcan0 [node ID]
/// ```
///
/// ## Interact with the node using Yakut
///
/// To subscribe and print out Heartbeat messages:
/// `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" subscribe uavcan.node.Heartbeat.1.0`
///
/// To send a NodeInfo request:
/// `yakut --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',8),42)" call [Node ID of basic_node] uavcan.node.GetInfo.1.0 {}`
///
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let can_interface = args.next().expect("Expected CAN interface name");
    let node_id = NodeId::try_from(
        args.next()
            .expect("Expected node ID")
            .parse::<u8>()
            .expect("Invalid node ID format"),
    )
    .expect("Node ID too large");

    let can = CANSocket::open(&can_interface).expect("Failed to open CAN interface");
    can.set_read_timeout(Duration::from_millis(500))?;
    can.set_write_timeout(Duration::from_millis(500))?;

    // Generate a random unique ID
    let unique_id: [u8; 16] = rand::random();
    let mut node = Node::with_info(
        node_id,
        NodeInfo::new(unique_id, "org.samcrow.canadensis.basic_node"),
    );
    node.set_mode(Mode::Operational);

    // UAVCAN TX/RX
    let mut tx = Transmitter::new(Mtu::Can8);
    let mut rx = Receiver::new(node_id);

    // Subscribe to NodeInfo
    rx.subscribe_request(canadensis_node::INFO_SERVICE, 0, Microseconds(10_000_u32))
        .expect("Failed to subscribe");

    // Presentation layer
    let mut heartbeat_publisher = Publisher::new(
        node.id(),
        Priority::default(),
        canadensis::node::HEARTBEAT_SUBJECT,
    );
    let mut node_info_responder = Responder::new(node.id(), canadensis::node::INFO_SERVICE);

    let start_time = Instant::now();

    let embedded_clock = SystemClock::new();

    loop {
        let run_time = Instant::now().duration_since(start_time);
        let run_time_seconds = run_time.as_secs();
        let run_time_seconds = if run_time_seconds > u64::from(u32::MAX) {
            u32::MAX
        } else {
            run_time_seconds as u32
        };
        node.set_uptime(run_time_seconds);

        let rx_status = can.read_frame();
        let transfer_in = match rx_status {
            Ok(frame) => {
                // Convert frame from socketcan to canadensis_can format
                let frame = Frame::new(
                    embedded_clock.try_now().unwrap(),
                    CanId::try_from(frame.id()).unwrap(),
                    frame.data(),
                );
                println!("Handling frame {:#?}", frame);
                rx.accept(frame).expect("Out of memory")
            }
            Err(e) => match e.kind() {
                io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut => {
                    // Didn't get a frame, but OK
                    None
                }
                _ => return Err(Box::new(e)),
            },
        };

        // Handle transfers
        if let Some(transfer_in) = transfer_in {
            if node_info_responder.interested(&transfer_in.header) {
                node_info_responder
                    .handle_request::<NodeInfoRequest, _, _, Infallible, _>(
                        transfer_in,
                        embedded_clock.try_now().unwrap() + Microseconds(10_000_u32),
                        &mut tx,
                        |_: NodeInfoRequest| Ok(node.info().unwrap().clone()),
                    )
                    .expect("Respond failed");
            }
        }
        // Publish heartbeat
        heartbeat_publisher
            .send(
                &node.heartbeat(),
                embedded_clock.try_now().unwrap() + Microseconds(10_000_u32),
                &mut tx,
            )
            .expect("Out of memory");

        // Send frames
        while let Some(out_frame) = tx.pop() {
            // Convert to SocketCAN frame format
            let out_frame =
                socketcan::CANFrame::new(out_frame.id().into(), out_frame.data(), false, false)?;
            can.write_frame(&out_frame)?;
        }
    }
}

/// A clock with microsecond precision
#[derive(Debug)]
struct SystemClock {
    start_time: Instant,
}

impl SystemClock {
    pub fn new() -> Self {
        SystemClock {
            start_time: Instant::now(),
        }
    }
}

impl Clock for SystemClock {
    type T = u64;
    /// 1 tick = 1 microsecond
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000);

    fn try_now(&self) -> Result<embedded_time::Instant<Self>, embedded_time::clock::Error> {
        let now = Instant::now();
        let since_start = now.duration_since(self.start_time);
        // Truncate microseconds to 64 bits
        Ok(embedded_time::Instant::new(since_start.as_micros() as u64))
    }
}
