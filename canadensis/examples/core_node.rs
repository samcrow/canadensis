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

use canadensis::core::time::{Clock, Duration, Instant, MicrosecondDuration64, Microseconds64};
use canadensis::core::transfer::ServiceTransfer;
use canadensis::core::transport::Transport;
use canadensis::core::Priority;
use canadensis::node::CoreNode;
use canadensis::requester::TransferIdArray;
use canadensis::{Node, ResponseToken, TransferHandler};
use canadensis_can::queue::{ArrayQueue, FrameQueueSource};
use canadensis_can::types::{CanNodeId, CanTransport};
use canadensis_can::{CanId, CanReceiver, CanTransmitter, Frame, Mtu};
use canadensis_data_types::uavcan::node::get_info_1_0::{self, GetInfoResponse};
use canadensis_data_types::uavcan::node::health_1_0::Health;
use canadensis_data_types::uavcan::node::heartbeat_1_0::{self, Heartbeat};
use canadensis_data_types::uavcan::node::mode_1_0::Mode;
use canadensis_data_types::uavcan::node::port;
use canadensis_data_types::uavcan::node::port::list_0_1::{self, List};
use canadensis_data_types::uavcan::node::port::service_id_list_0_1::ServiceIDList;
use canadensis_data_types::uavcan::node::port::subject_id_list_0_1::SubjectIDList;
use canadensis_data_types::uavcan::node::version_1_0::Version;

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
    let node_id = CanNodeId::try_from(
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

    let frame_queue = ArrayQueue::<Microseconds64, 64>::new();

    let transmitter = CanTransmitter::new(Mtu::Can8, frame_queue);
    let receiver = CanReceiver::new(node_id, Mtu::Can8);
    let mut uavcan: CoreNode<_, _, _, TransferIdArray<CanTransport<Microseconds64>>, 8, 8> =
        CoreNode::new(SystemClock::new(), node_id, transmitter, receiver);

    let heartbeat_token = uavcan
        .start_publishing(
            heartbeat_1_0::SUBJECT,
            MicrosecondDuration64::new(1_000_000),
            Priority::Low,
        )
        .expect("Couldn't start publishing");
    let port_list_token = uavcan
        .start_publishing(
            list_0_1::SUBJECT,
            MicrosecondDuration64::new(1_000_000),
            Priority::Optional,
        )
        .expect("Couldn't start publishing");
    uavcan
        .subscribe_request(get_info_1_0::SERVICE, 0, MicrosecondDuration64::new(0))
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
                health: Health {
                    value: Health::NOMINAL,
                },
                mode: Mode {
                    value: Mode::OPERATIONAL,
                },
                vendor_specific_status_code: 0,
            };
            uavcan
                .publish(&heartbeat_token, &heartbeat)
                .expect("Out of memory");

            if run_time_seconds % u32::from(List::MAX_PUBLICATION_PERIOD) == 0 {
                // Send port list every 10 seconds
                let publishers = SubjectIDList::SparseList({
                    let mut subject_ids = heapless::Vec::new();
                    subject_ids
                        .push(port::subject_id_1_0::SubjectID {
                            value: heartbeat_1_0::SUBJECT.into(),
                        })
                        .ok()
                        .unwrap();
                    subject_ids
                        .push(port::subject_id_1_0::SubjectID {
                            value: list_0_1::SUBJECT.into(),
                        })
                        .ok()
                        .unwrap();
                    subject_ids
                });
                let servers = {
                    let mut servers = ServiceIDList {
                        mask: Default::default(),
                    };
                    servers.mask.set(usize::from(get_info_1_0::SERVICE), true);
                    servers
                };
                let port_list = List {
                    publishers,
                    subscribers: SubjectIDList::Mask(Default::default()),
                    clients: ServiceIDList {
                        mask: Default::default(),
                    },
                    servers,
                };
                uavcan
                    .publish(&port_list_token, &port_list)
                    .expect("Out of memory");
            }
        }
        // Send frames
        while let Some(out_frame) = uavcan.transmitter_mut().frame_queue_mut().pop_frame() {
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

impl<T: Transport> TransferHandler<Microseconds64, T> for BasicTransferHandler {
    fn handle_request<N>(
        &mut self,
        node: &mut N,
        token: ResponseToken<T>,
        transfer: &ServiceTransfer<Vec<u8>, <N::Clock as Clock>::Instant, T>,
    ) -> bool
    where
        N: Node<Instant = Microseconds64, Transport = T>,
    {
        println!("Handling request {:?}", transfer);
        if transfer.header.service == get_info_1_0::SERVICE {
            // Send a node information response
            let response = GetInfoResponse {
                protocol_version: Version { major: 1, minor: 0 },
                hardware_version: Version { major: 0, minor: 0 },
                software_version: Version { major: 0, minor: 1 },
                software_vcs_revision_id: 0,
                unique_id: self.unique_id.clone(),
                name: heapless::Vec::from_iter(b"org.samcrow.basic_node".iter().cloned()),
                software_image_crc: heapless::Vec::new(),
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
