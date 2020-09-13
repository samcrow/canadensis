extern crate rand;
extern crate socketcan;

extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_encoding;
extern crate canadensis_node;

use std::convert::TryFrom;
use std::env;
use std::io;
use std::time::{Duration, Instant};

use socketcan::CANSocket;

use canadensis_can::{CanId, Frame, Mtu, Receiver, Transmitter};
use canadensis_core::transfer::{
    MessageHeader, ServiceHeader, Transfer, TransferHeader, TransferKindHeader,
};
use canadensis_core::{Microseconds, NodeId, Priority, TransferId};
use canadensis_encoding::{Serialize, WriteCursor};
use canadensis_node::{Mode, Node, NodeInfo};

/// Runs a basic UAVCAN node, sending Heartbeat messages and responding to NodeInfo requests
///
/// Usage: `basic_node [SocketCAN interface name] [Node ID]`
///
/// # Testing with pyuavcan
///
/// To subscribe and print out Heartbeat messages:
/// `pyuavcan subscribe --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0',64),42)" uavcan.node.Heartbeat.1.0`
///
/// To send a NodeInfo request:
/// `pyuavcan call --transport "CAN(can.media.socketcan.SocketCANMedia('vcan0', 8),42)" [Node ID of basic_node] uavcan.node.GetInfo.1.0 {}`
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
    // The ID of the next heartbeat transfer to be sent
    let mut next_heartbeat_transfer = TransferId::default();

    // UAVCAN TX/RX
    let mut tx = Transmitter::new(Mtu::Can8);
    let mut rx = Receiver::new(node_id);

    // Subscribe to NodeInfo
    rx.subscribe_request(canadensis_node::INFO_SERVICE, 0, Microseconds(0))
        .expect("Failed to subscribe");

    let start_time = Instant::now();

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
                    Microseconds(0),
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

        if let Some(transfer) = transfer_in {
            if let Some(ServiceHeader {
                service: canadensis_node::INFO_SERVICE,
                ..
            }) = transfer.header.kind.service_header()
            {
                // Send a node information response
                let response = node.info().unwrap();
                let mut response_payload = vec![0u8; (response.size_bits() + 7) / 8];
                response.serialize(&mut WriteCursor::new(&mut response_payload));
                let response_transfer: Transfer<&[u8]> = Transfer {
                    timestamp: Microseconds(0),
                    header: TransferHeader {
                        source: node.id(),
                        priority: Priority::default(),
                        kind: TransferKindHeader::Response(ServiceHeader {
                            service: canadensis_node::INFO_SERVICE,
                            destination: transfer.header.source,
                        }),
                    },
                    transfer_id: transfer.transfer_id,
                    payload: &response_payload,
                };

                tx.push(response_transfer).expect("Out of memory");
            }
        }

        // Send heartbeat
        let heartbeat = node.heartbeat();
        let mut heartbeat_payload = vec![0u8; (heartbeat.size_bits() + 7) / 8];
        heartbeat.serialize(&mut WriteCursor::new(&mut heartbeat_payload));
        let heartbeat_transfer: Transfer<&[u8]> = Transfer {
            timestamp: Microseconds(0),
            header: TransferHeader {
                source: node.id(),
                priority: Default::default(),
                kind: TransferKindHeader::Message(MessageHeader {
                    anonymous: false,
                    subject: canadensis_node::HEARTBEAT_SUBJECT,
                }),
            },
            transfer_id: next_heartbeat_transfer,
            payload: &heartbeat_payload,
        };
        next_heartbeat_transfer = next_heartbeat_transfer.increment();
        tx.push(heartbeat_transfer).expect("Out of memory");

        // Send frames
        while let Some(out_frame) = tx.pop() {
            // Convert to SocketCAN frame format
            let out_frame =
                socketcan::CANFrame::new(out_frame.id().into(), out_frame.data(), false, false)?;
            can.write_frame(&out_frame)?;
        }
    }
}
