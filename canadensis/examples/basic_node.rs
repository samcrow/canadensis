//!
//! Runs a simple UAVCAN node using the canadensis library
//!

extern crate canadensis;
extern crate rand;
extern crate socketcan;

use std::convert::TryFrom;
use std::env;
use std::io;
use std::iter::FromIterator;
use std::time::Duration as StdDuration;
use std::time::Instant as StdInstant;

use socketcan::CANSocket;

use canadensis::{Node, ResponseToken, TransferHandler};
use canadensis_can::queue::ArrayQueue;
use canadensis_can::{CanId, Frame, Mtu};
use canadensis_core::time::{Clock, Duration, Instant, MicrosecondDuration64, Microseconds64};
use canadensis_core::transfer::ServiceTransfer;
use canadensis_core::{NodeId, Priority};
use canadensis_data_types::uavcan::node::get_info::{GetInfoRequest, GetInfoResponse};
use canadensis_data_types::uavcan::node::health::Health;
use canadensis_data_types::uavcan::node::heartbeat::Heartbeat;
use canadensis_data_types::uavcan::node::mode::Mode;
use canadensis_data_types::uavcan::node::port;
use canadensis_data_types::uavcan::node::port::list::List;
use canadensis_data_types::uavcan::node::port::service_id_list::ServiceIdList;
use canadensis_data_types::uavcan::node::port::subject_id_list::SubjectIdList;
use canadensis_data_types::uavcan::node::version::Version;

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
/// In the above two commands, 8 is the MTU of standard CAN and 42 is the node ID of the Yakut node.
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
    can.set_read_timeout(StdDuration::from_millis(100))?;
    can.set_write_timeout(StdDuration::from_millis(100))?;

    // Generate a random unique ID
    let unique_id: [u8; 16] = rand::random();
    let mut transfer_handler = BasicTransferHandler { unique_id };
    let mut clock = SystemClock::new();

    let frame_queue = ArrayQueue::<_, 64>::new();

    let mut uavcan: canadensis::CoreNode<_, _, 4, 4> =
        canadensis::CoreNode::new(clock.clone(), node_id, Mtu::Can8, frame_queue);

    let heartbeat_token = uavcan
        .start_publishing(
            Heartbeat::SUBJECT,
            MicrosecondDuration64::new(1_000_000),
            Priority::Low,
        )
        .expect("Couldn't start publishing");
    let port_list_token = uavcan
        .start_publishing(
            List::SUBJECT,
            MicrosecondDuration64::new(1_000_000),
            Priority::Optional,
        )
        .expect("Couldn't start publishing");
    uavcan
        .subscribe_request(GetInfoRequest::SERVICE, 0, MicrosecondDuration64::new(0))
        .expect("Out of memory");

    let mut last_run_time_seconds = 0u64;
    loop {
        let run_time = StdInstant::now().duration_since(clock.start_time);
        let run_time_seconds = run_time.as_secs();
        let new_second = run_time_seconds != last_run_time_seconds;
        last_run_time_seconds = run_time_seconds;
        let run_time_seconds = if run_time_seconds > u64::from(u32::MAX) {
            u32::MAX
        } else {
            run_time_seconds as u32
        };

        let rx_status = can.read_frame();
        match rx_status {
            Ok(frame) => {
                // Convert frame from socketcan to canadensis_can format
                let frame = Frame::new(
                    clock.now(),
                    CanId::try_from(frame.id()).unwrap(),
                    frame.data(),
                );

                uavcan
                    .accept_frame(frame, &mut transfer_handler)
                    .expect("Out of memory");
            }
            Err(e) => match e.kind() {
                io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut => {}
                _ => return Err(Box::new(e)),
            },
        };

        if new_second {
            // Publish heartbeat
            let heartbeat = Heartbeat {
                uptime: run_time_seconds,
                health: Health::Nominal,
                mode: Mode::Operational,
                vendor_specific_status_code: 0,
            };
            uavcan
                .publish(&heartbeat_token, &heartbeat)
                .expect("Out of memory");

            if run_time_seconds % u32::from(List::MAX_PUBLICATION_PERIOD) == 0 {
                // Send port list every 10 seconds
                let publishers = SubjectIdList::SparseList({
                    let mut subject_ids = heapless::Vec::new();
                    subject_ids
                        .push(port::subject_id::SubjectId {
                            value: Heartbeat::SUBJECT.into(),
                        })
                        .unwrap();
                    subject_ids
                        .push(port::subject_id::SubjectId {
                            value: List::SUBJECT.into(),
                        })
                        .unwrap();
                    subject_ids
                });
                let servers = {
                    let mut servers = ServiceIdList::default();
                    servers.mask.set(usize::from(GetInfoRequest::SERVICE), true);
                    servers
                };
                let port_list = List {
                    publishers,
                    subscribers: Default::default(),
                    clients: Default::default(),
                    servers,
                };
                uavcan
                    .publish(&port_list_token, &port_list)
                    .expect("Out of memory");
            }
        }
        // Send frames
        while let Some(out_frame) = uavcan.pop_frame() {
            // Convert to SocketCAN frame format
            let out_frame =
                socketcan::CANFrame::new(out_frame.id().into(), out_frame.data(), false, false)?;
            can.write_frame(&out_frame)?;
        }
    }
}

struct BasicTransferHandler {
    unique_id: [u8; 16],
}

impl TransferHandler<Microseconds64> for BasicTransferHandler {
    fn handle_request<N>(
        &mut self,
        node: &mut N,
        token: ResponseToken,
        transfer: &ServiceTransfer<Vec<u8>, <N::Clock as Clock>::Instant>,
    ) -> bool
    where
        N: Node<Instant = Microseconds64>,
    {
        println!("Handling request {:?}", transfer);
        if transfer.header.service == GetInfoRequest::SERVICE {
            // Send a node information response
            let response = GetInfoResponse {
                protocol_version: Version { major: 1, minor: 0 },
                hardware_version: Version { major: 0, minor: 0 },
                software_version: Version { major: 0, minor: 1 },
                software_vcs_revision_id: 0,
                unique_id: self.unique_id.clone(),
                name: heapless::Vec::from_iter(b"org.samcrow.basic_node".iter().cloned()),
                software_image_crc: None,
                certificate_of_authenticity: heapless::Vec::new(),
            };
            let timeout =
                <<N::Clock as Clock>::Instant as Instant>::Duration::from_millis(1000).unwrap();
            node.send_response(token, timeout, &response)
                .expect("Out of memory");
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
struct SystemClock {
    start_time: StdInstant,
}

impl SystemClock {
    pub fn new() -> Self {
        SystemClock {
            start_time: StdInstant::now(),
        }
    }
}

impl Clock for SystemClock {
    type Instant = Microseconds64;

    fn now(&mut self) -> Self::Instant {
        let since_start = StdInstant::now().duration_since(self.start_time);
        let microseconds = since_start.as_micros();
        Microseconds64::new(microseconds as u64)
    }
}
