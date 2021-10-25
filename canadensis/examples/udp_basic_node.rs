extern crate canadensis;
extern crate canadensis_udp;
extern crate rand;
extern crate socketcan;

use std::convert::TryFrom;
use std::net::Ipv4Addr;
use std::time::Duration;
use std::{env, thread};

use canadensis::core::time::Instant;
use canadensis::core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis::core::transport::Transport;
use canadensis::node::{BasicNode, CoreNode};
use canadensis::requester::TransferIdFixedMap;
use canadensis::{Node, ResponseToken, TransferHandler};
use canadensis_core::session::SessionDynamicMap;
use canadensis_core::time::Microseconds64;
use canadensis_data_types::uavcan::node::get_info_1_0::GetInfoResponse;
use canadensis_data_types::uavcan::node::version_1_0::Version;
use canadensis_linux::SystemClock;
use canadensis_udp::{
    NodeAddress, UdpNodeId, UdpReceiver, UdpSessionData, UdpTransferId, UdpTransmitter,
    UdpTransport,
};

/// Runs a basic UAVCAN node that sends Heartbeat messages, responds to node information requests,
/// and sends port list messages
///
/// This node connects uses a UDP transport.
///
/// Usage: `tcp_serial_basic_node [node IP address]`
///
/// The node ID is automatically derived from the 16 least significant bits of the node IP address.
///
/// # Testing
///
/// ## Start the node
///
/// ```
/// udp_basic_node 127.0.0.[node ID]
/// ```
///
/// ## Interact with the node using Yakut
///
/// ```
/// yakut --transport "UDPTransport('127.0.0.127')" monitor
/// ```
///
/// In the above two commands, 8 is the MTU of standard CAN and 42 is the node ID of the Yakut node.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let local_address: Ipv4Addr = args
        .next()
        .expect("No local IP address")
        .parse()
        .expect("Invalid IP address");
    let local_address =
        NodeAddress::try_from(local_address).expect("IP address is not a valid node address");
    let node_id = local_address.node_id();

    // Set up information about this node
    let node_info = GetInfoResponse {
        protocol_version: Version { major: 1, minor: 0 },
        hardware_version: Version { major: 0, minor: 0 },
        software_version: Version { major: 0, minor: 1 },
        software_vcs_revision_id: 0,
        unique_id: rand::random(),
        name: heapless::Vec::from_slice(b"org.samcrow.udp_basic_node").unwrap(),
        software_image_crc: heapless::Vec::new(),
        certificate_of_authenticity: Default::default(),
    };

    // Create a node with capacity for 8 publishers and 8 requesters
    const TRANSFER_IDS: usize = 1;
    const PUBLISHERS: usize = 8;
    const REQUESTERS: usize = 8;
    const MTU: usize = 1200;

    let transmitter = UdpTransmitter::<MTU>::new(local_address.clone()).unwrap();
    let receiver = UdpReceiver::new(local_address);
    let core_node: CoreNode<
        SystemClock,
        UdpTransmitter<MTU>,
        UdpReceiver<
            Microseconds64,
            SessionDynamicMap<Microseconds64, UdpNodeId, UdpTransferId, UdpSessionData>,
            MTU,
        >,
        TransferIdFixedMap<UdpTransport, TRANSFER_IDS>,
        (),
        PUBLISHERS,
        REQUESTERS,
    > = CoreNode::new(SystemClock::new(), node_id, transmitter, receiver, ());
    let mut node = BasicNode::new(core_node, node_info).unwrap();

    let start_time = std::time::Instant::now();
    let mut prev_seconds = 0;
    loop {
        match node.receive(&mut EmptyHandler) {
            Ok(_) => {}
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
        thread::sleep(Duration::from_millis(100));
    }
}

struct EmptyHandler;

impl<I: Instant, T: Transport> TransferHandler<I, T> for EmptyHandler {
    fn handle_message<N>(
        &mut self,
        _node: &mut N,
        transfer: &MessageTransfer<Vec<u8>, I, T>,
    ) -> bool
    where
        N: Node<Instant = I, Transport = T>,
    {
        println!("Got message {:?}", transfer);
        false
    }

    fn handle_request<N>(
        &mut self,
        _node: &mut N,
        _token: ResponseToken<T>,
        transfer: &ServiceTransfer<Vec<u8>, I, T>,
    ) -> bool
    where
        N: Node<Instant = I, Transport = T>,
    {
        println!("Got request {:?}", transfer);
        false
    }

    fn handle_response<N>(
        &mut self,
        _node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, I, T>,
    ) -> bool
    where
        N: Node<Instant = I, Transport = T>,
    {
        println!("Got response {:?}", transfer);
        false
    }
}
