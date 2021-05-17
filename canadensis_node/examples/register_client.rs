extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_linux;
extern crate canadensis_node;
extern crate rand;
extern crate socketcan;

use std::convert::TryFrom;
use std::env;
use std::str;
use std::time::Duration;

use socketcan::CANSocket;

use canadensis::{CoreNode, Node, ServiceToken, TransferHandler};
use canadensis_can::queue::{ArrayQueue, FrameQueueSource};
use canadensis_can::Mtu;
use canadensis_core::time::{milliseconds, Clock, Microseconds64};
use canadensis_core::transfer::ServiceTransfer;
use canadensis_core::{NodeId, Priority, TransferId};
use canadensis_data_types::uavcan::node::get_info::GetInfoResponse;
use canadensis_data_types::uavcan::node::version::Version;
use canadensis_data_types::uavcan::register::access::{AccessRequest, AccessResponse};
use canadensis_data_types::uavcan::register::list::{ListRequest, ListResponse};
use canadensis_data_types::uavcan::register::value::Value;
use canadensis_encoding::Deserialize;
use canadensis_linux::{LinuxCan, SystemClock};
use canadensis_node::BasicNode;
use std::collections::BTreeMap;
use std::io::ErrorKind;

/// Runs a UAVCAN node that connects to another node and gets information about its registers
///
/// Usage: `register_client [SocketCAN interface name] [Local node ID] [Target node ID]`
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
/// register_client [SocketCAN interface name] [Local node ID] [Target node ID]
/// ```
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
    let target_node_id = NodeId::try_from(
        args.next()
            .expect("Expected target node ID")
            .parse::<u8>()
            .expect("Invalid node ID format"),
    )
    .expect("Node ID too large");

    let can = CANSocket::open(&can_interface).expect("Failed to open CAN interface");
    can.set_read_timeout(Duration::from_millis(5))?;
    can.set_write_timeout(Duration::from_millis(500))?;
    let mut can = LinuxCan::new(can);

    // Set up information about this node
    let node_info = GetInfoResponse {
        protocol_version: Version { major: 1, minor: 0 },
        hardware_version: Version { major: 0, minor: 0 },
        software_version: Version { major: 0, minor: 1 },
        software_vcs_revision_id: 0,
        unique_id: rand::random(),
        name: heapless::Vec::from_slice(b"org.samcrow.register_client").unwrap(),
        software_image_crc: None,
        certificate_of_authenticity: Default::default(),
    };

    // Create a node with capacity for 8 publishers and 8 requesters
    let core_node: CoreNode<_, _, 8, 8> = CoreNode::new(
        SystemClock::new(),
        node_id,
        Mtu::Can8,
        ArrayQueue::<Microseconds64, 128>::new(),
    );
    let mut node = BasicNode::new(core_node, node_info).unwrap();
    let list_request_token: ServiceToken<ListRequest> = node
        .start_sending_requests(ListRequest::SERVICE, milliseconds(1000), 256, Priority::Low)
        .unwrap();
    let access_token = node
        .start_sending_requests(
            AccessRequest::SERVICE,
            milliseconds(1000),
            267,
            Priority::Low,
        )
        .unwrap();

    // Now that the node has subscribed to everything it wants, set up the frame acceptance filters
    let frame_filters = node.frame_filters().unwrap();
    can.set_filters(&frame_filters)?;

    // Send a register list request for the register at index 0
    node.send_request(
        &list_request_token,
        &ListRequest { index: 0 },
        target_node_id,
    )
    .unwrap();

    let mut handler = RegisterHandler {
        target_node_id,
        next_register_index: 1,
        registers: BTreeMap::new(),
        list_request_token,
        access_token,
        all_registers_listed: false,
        done: false,
        timeout: std::time::Instant::now() + std::time::Duration::from_secs(20),
    };

    while !handler.done && handler.timeout > std::time::Instant::now() {
        match can.receive() {
            Ok(frame) => {
                node.accept_frame(frame, &mut handler).unwrap();
            }
            Err(e) => match e.kind() {
                ErrorKind::WouldBlock => {}
                _ => return Err(e.into()),
            },
        };

        node.run_periodic_tasks().unwrap();
        while let Some(frame_out) = node.frame_queue_mut().pop_frame() {
            can.send(frame_out)?;
        }
    }
    // Either finished or timed out
    if handler.done {
        // Print register information
        for (name, state) in handler.registers {
            if let RegisterState::Done(value) = state {
                println!("{}: {:?}", name, DebugValue(&value));
            }
        }
        std::process::exit(0);
    } else {
        println!("Timed out, registers found:");
        for (name, state) in handler.registers {
            if let RegisterState::Done(value) = state {
                println!("{}: {:?}", name, DebugValue(&value));
            } else {
                println!("{}: <unknown>", name);
            }
        }
        std::process::exit(1);
    }
}

struct RegisterHandler {
    /// The ID of the node to query
    target_node_id: NodeId,
    /// The index of the next register to query
    next_register_index: u16,
    /// Each known register and its value
    registers: BTreeMap<String, RegisterState>,
    /// Token used to send register list requests
    list_request_token: ServiceToken<ListRequest>,
    /// Token used to send register access request
    access_token: ServiceToken<AccessRequest>,
    /// True if all register list responses have been received
    all_registers_listed: bool,
    /// True if all register values are known
    done: bool,
    /// The time when the read operation will time out
    timeout: std::time::Instant,
}

impl TransferHandler<<SystemClock as Clock>::Instant> for RegisterHandler {
    fn handle_response<N>(
        &mut self,
        node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, <SystemClock as Clock>::Instant>,
    ) -> bool
    where
        N: Node<Instant = <SystemClock as Clock>::Instant>,
    {
        match transfer.header.service {
            ListResponse::SERVICE => {
                if let Ok(list_response) = ListResponse::deserialize_from_bytes(&transfer.payload) {
                    match str::from_utf8(&list_response.name.name) {
                        Ok(register_name) => {
                            if register_name.is_empty() {
                                // No more registers
                                self.all_registers_listed = true;
                                self.check_if_done();
                            } else {
                                // Record information about this register and send a request to
                                // read its value
                                let read_transfer_id = node
                                    .send_request(
                                        &self.access_token,
                                        &AccessRequest {
                                            name: list_response.name.clone(),
                                            value: Default::default(),
                                        },
                                        self.target_node_id,
                                    )
                                    .unwrap();

                                self.registers.insert(
                                    register_name.to_owned(),
                                    RegisterState::Waiting(read_transfer_id),
                                );
                                // Send a request for the next register
                                node.send_request(
                                    &self.list_request_token,
                                    &ListRequest {
                                        index: self.next_register_index,
                                    },
                                    self.target_node_id,
                                )
                                .unwrap();
                                self.next_register_index += 1;
                            }
                        }
                        Err(_) => {
                            println!("Invalid UTF-8 in register name");
                            self.done = true;
                        }
                    }
                    // Transfer handled
                    true
                } else {
                    false
                }
            }
            AccessResponse::SERVICE => {
                if let Ok(response) = AccessResponse::deserialize_from_bytes(&transfer.payload) {
                    // Find the register name with the matching transfer ID
                    let register_entry =
                        self.registers
                            .iter_mut()
                            .find(|(_name, state)| match state {
                                RegisterState::Waiting(transfer_id) => {
                                    transfer_id == &transfer.header.transfer_id
                                }
                                RegisterState::Done(_) => false,
                            });
                    if let Some((_name, state)) = register_entry {
                        *state = RegisterState::Done(response.value);
                    } else {
                        eprintln!(
                            "Couldn't find a corresponding register for transfer ID {:?}",
                            transfer.header.transfer_id
                        );
                    }

                    self.check_if_done();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl RegisterHandler {
    fn check_if_done(&mut self) {
        // Done if all register values are known
        if self.all_registers_listed
            && self
                .registers
                .iter()
                .all(|(_name, state)| matches!(state, RegisterState::Done(_)))
        {
            self.done = true;
        }
    }
}

enum RegisterState {
    Waiting(TransferId),
    Done(Value),
}

struct DebugValue<'v>(&'v Value);

impl std::fmt::Debug for DebugValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Value::Empty => f.debug_struct("Empty").finish(),
            Value::String(bytes) => {
                let string = String::from_utf8_lossy(&bytes);
                f.debug_tuple("String").field(&string).finish()
            }
            Value::Unstructured(bytes) => f
                .debug_tuple("Unstructured")
                .field(&DebugHexBytes(&bytes))
                .finish(),
            Value::Bit(bits) => f.debug_tuple("Bit").field(&bits).finish(),
            Value::Integer64(values) => f.debug_tuple("Integer64").field(&values).finish(),
            Value::Integer32(values) => f.debug_tuple("Integer32").field(&values).finish(),
            Value::Integer16(values) => f.debug_tuple("Integer16").field(&values).finish(),
            Value::Integer8(values) => f.debug_tuple("Integer8").field(&values).finish(),
            Value::Natural64(values) => f.debug_tuple("Natural64").field(&values).finish(),
            Value::Natural32(values) => f.debug_tuple("Natural32").field(&values).finish(),
            Value::Natural16(values) => f.debug_tuple("Natural16").field(&values).finish(),
            Value::Natural8(values) => f.debug_tuple("Natural8").field(&values).finish(),
            Value::Real64(value) => f.debug_tuple("Real64").field(value).finish(),
            Value::Real32(value) => f.debug_tuple("Real32").field(value).finish(),
            Value::Real16(value) => f.debug_tuple("Real16").field(value).finish(),
        }
    }
}

struct DebugHexBytes<'a>(&'a [u8]);
impl std::fmt::Debug for DebugHexBytes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(|byte| format!("{:#04x}", *byte)))
            .finish()
    }
}
