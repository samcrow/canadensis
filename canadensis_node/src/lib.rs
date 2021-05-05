#![no_std]

extern crate alloc;
extern crate canadensis;
extern crate canadensis_core;
extern crate canadensis_data_types;
extern crate canadensis_encoding;
extern crate fallible_collections;

use canadensis::{Clock, Node};
use canadensis_core::time::Instant;
use canadensis_data_types::uavcan::node::health::Health;
use canadensis_data_types::uavcan::node::heartbeat::Heartbeat;
use canadensis_data_types::uavcan::node::mode::Mode;

/// A node with the minimum required application-layer functionality
///
/// A `BasicNode` wraps a [`canadensis::Node`] and adds functionality to periodically send
/// `uavcan.node.Heartbeat.1.0` messages.
pub struct BasicNode<C, Q, const P: usize, const R: usize>
where
    C: Clock,
{
    /// The inner node
    node: Node<C, Q, P, R>,
    /// The heartbeat message that will be periodically sent
    heartbeat: Heartbeat,
    // TODO: Setup to do something every second
}

impl<C: Clock, Q, const P: usize, const R: usize> BasicNode<C, Q, P, R> {
    /// Sets the operating mode that will be reported in the heartbeat messages
    pub fn set_mode(&mut self, mode: Mode) {
        self.heartbeat.mode = mode;
    }
    /// Sets the health status that will be reported in the heartbeat messages
    pub fn set_health(&mut self, health: Health) {
        self.heartbeat.health = health;
    }
    /// Sets the vendor-specific status code that will be reported in the heartbeat messages
    pub fn set_status_code(&mut self, status: u8) {
        self.heartbeat.vendor_specific_status_code = status;
    }
}
