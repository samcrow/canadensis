//! Runs a Cyphal node that connects to another node and gets information about its registers
//!
//! Usage: `register_client [SocketCAN interface name] [Local node ID] [Target node ID]`
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
//! register_client [SocketCAN interface name] [Local node ID] [Target node ID] [delay time after each message]
//! ```

extern crate canadensis;
extern crate canadensis_data_types;
extern crate canadensis_linux;
extern crate rand;
extern crate socketcan;

use std::convert::TryFrom;
use std::env;
use std::str;
use std::time::Duration;

use socketcan::CANSocket;

use canadensis::core::time::milliseconds;
use canadensis::core::transfer::ServiceTransfer;
use canadensis::core::Priority;
use canadensis::encoding::Deserialize;
use canadensis::node::{BasicNode, CoreNode};
use canadensis::requester::TransferIdFixedMap;
use canadensis::{Node, ServiceToken, TransferHandler};
use canadensis_can::queue::{ArrayQueue, SingleQueueDriver};
use canadensis_can::{
    CanNodeId, CanReceiver, CanTransferId, CanTransmitter, CanTransport, Error, Mtu,
};
use canadensis_data_types::uavcan::node::get_info_1_0::GetInfoResponse;
use canadensis_data_types::uavcan::node::version_1_0::Version;
use canadensis_data_types::uavcan::primitive::empty_1_0::Empty;
use canadensis_data_types::uavcan::register::access_1_0::{self, AccessRequest, AccessResponse};
use canadensis_data_types::uavcan::register::list_1_0::{self, ListRequest, ListResponse};
use canadensis_data_types::uavcan::register::name_1_0::Name;
use canadensis_data_types::uavcan::register::value_1_0::Value;
use canadensis_linux::{LinuxCan, SystemClock};
use std::collections::BTreeMap;
use std::io::ErrorKind;
use std::thread;

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
    let target_node_id = CanNodeId::try_from(
        args.next()
            .expect("Expected target node ID")
            .parse::<u8>()
            .expect("Invalid node ID format"),
    )
    .expect("Node ID too large");
    let delay_time_seconds: f32 = args.next().map(|s| s.parse().unwrap()).unwrap_or(0.0);
    let delay_time = Duration::new(
        delay_time_seconds.floor() as u64,
        (delay_time_seconds.fract() * 1e9) as u32,
    );

    let can = CANSocket::open(&can_interface).expect("Failed to open CAN interface");
    can.set_read_timeout(Duration::from_millis(5))?;
    can.set_write_timeout(Duration::from_millis(500))?;
    let can = LinuxCan::new(can);

    // Set up information about this node
    let node_info = GetInfoResponse {
        protocol_version: Version { major: 1, minor: 0 },
        hardware_version: Version { major: 0, minor: 0 },
        software_version: Version { major: 0, minor: 1 },
        software_vcs_revision_id: 0,
        unique_id: rand::random(),
        name: heapless::Vec::from_slice(b"org.samcrow.register_client").unwrap(),
        software_image_crc: heapless::Vec::new(),
        certificate_of_authenticity: Default::default(),
    };

    // Create a node with capacity for 2 publishers and 2 requesters
    type Queue = SingleQueueDriver<SystemClock, ArrayQueue<64>, LinuxCan>;
    // TRANSFER_IDS must be a power of two and greater than one
    const TRANSFER_IDS: usize = 2;
    const PUBLISHERS: usize = 2;
    const REQUESTERS: usize = 2;

    let queue = Queue::new(ArrayQueue::new(), can);
    let transmitter = CanTransmitter::new(Mtu::Can8);
    let receiver = CanReceiver::new(node_id, Mtu::Can8);
    let core_node: CoreNode<
        SystemClock,
        CanTransmitter<SystemClock, Queue>,
        CanReceiver<SystemClock, Queue>,
        TransferIdFixedMap<CanTransport, TRANSFER_IDS>,
        Queue,
        PUBLISHERS,
        REQUESTERS,
    > = CoreNode::new(SystemClock::new(), node_id, transmitter, receiver, queue);
    let mut node = BasicNode::new(core_node, node_info).unwrap();
    let list_request_token: ServiceToken<ListRequest> = node
        .start_sending_requests(list_1_0::SERVICE, milliseconds(1000), 256, Priority::Low)
        .unwrap();
    let access_token = node
        .start_sending_requests(access_1_0::SERVICE, milliseconds(1000), 267, Priority::Low)
        .unwrap();

    // Send a register list request for the register at index 0
    node.send_request(
        &list_request_token,
        &ListRequest { index: 0 },
        target_node_id,
    )
    .unwrap();
    node.flush().unwrap();

    let timeout_duration = std::time::Duration::from_secs(1);

    let mut handler = RegisterHandler {
        target_node_id,
        next_register_index: 1,
        registers: BTreeMap::new(),
        list_request_token,
        access_token,
        all_registers_listed: false,
        done: false,
        timeout: std::time::Instant::now() + timeout_duration,
        timeout_duration,
        delay_time,
    };

    let start_time = std::time::Instant::now();
    let mut prev_seconds = 0;
    while !handler.done && handler.timeout > std::time::Instant::now() {
        match node.receive(&mut handler) {
            Ok(_) => { /* Keep receiving */ }
            Err(Error::Driver(e)) if e.kind() == ErrorKind::WouldBlock => {
                // Keep receiving
            }
            Err(e) => panic!("{:?}", e),
        }

        let seconds = std::time::Instant::now()
            .duration_since(start_time)
            .as_secs();
        if seconds != prev_seconds {
            prev_seconds = seconds;
            node.run_per_second_tasks().unwrap();
        }
        node.flush().unwrap();
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
    target_node_id: CanNodeId,
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
    /// The timeout interval (used to update `timeout` after each successful receive)
    timeout_duration: Duration,
    /// The time to wait after sending each outgoing transfer
    delay_time: Duration,
}

impl TransferHandler<CanTransport> for RegisterHandler {
    fn handle_response<N>(
        &mut self,
        node: &mut N,
        transfer: &ServiceTransfer<Vec<u8>, CanTransport>,
    ) -> bool
    where
        N: Node<Transport = CanTransport>,
    {
        self.timeout = std::time::Instant::now() + self.timeout_duration;
        match transfer.header.service {
            list_1_0::SERVICE => {
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
                                            name: Name {
                                                name: list_response.name.name.clone(),
                                            },
                                            value: Value::Empty(Empty {}),
                                        },
                                        self.target_node_id.clone(),
                                    )
                                    .unwrap();
                                node.flush().unwrap();
                                thread::sleep(self.delay_time);

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
                                node.flush().unwrap();
                                thread::sleep(self.delay_time);

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
            access_1_0::SERVICE => {
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
    /// Waiting for a response with the register value
    ///
    /// The response will match the enclosed transfer ID
    Waiting(CanTransferId),
    /// The register value has been received
    Done(Value),
}

struct DebugValue<'v>(&'v Value);

impl std::fmt::Debug for DebugValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Value::Empty(_) => f.debug_struct("Empty").finish(),
            Value::String(bytes) => {
                let string = String::from_utf8_lossy(&bytes.value);
                f.debug_tuple("String").field(&string).finish()
            }
            Value::Unstructured(bytes) => f
                .debug_tuple("Unstructured")
                .field(&DebugHexBytes(&bytes.value))
                .finish(),
            Value::Bit(bits) => f.debug_tuple("Bit").field(&bits.value).finish(),
            Value::Integer64(values) => f.debug_tuple("Integer64").field(&values.value).finish(),
            Value::Integer32(values) => f.debug_tuple("Integer32").field(&values.value).finish(),
            Value::Integer16(values) => f.debug_tuple("Integer16").field(&values.value).finish(),
            Value::Integer8(values) => f.debug_tuple("Integer8").field(&values.value).finish(),
            Value::Natural64(values) => f.debug_tuple("Natural64").field(&values.value).finish(),
            Value::Natural32(values) => f.debug_tuple("Natural32").field(&values.value).finish(),
            Value::Natural16(values) => f.debug_tuple("Natural16").field(&values.value).finish(),
            Value::Natural8(values) => f.debug_tuple("Natural8").field(&values.value).finish(),
            Value::Real64(value) => f.debug_tuple("Real64").field(&value.value).finish(),
            Value::Real32(value) => f.debug_tuple("Real32").field(&value.value).finish(),
            Value::Real16(value) => f.debug_tuple("Real16").field(&value.value).finish(),
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
