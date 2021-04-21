#![no_std]

//!
//! The `node-info` feature enables handling of uavcan.node.GetInfo requests. The specification
//! encourages all nodes to support this feature.
//!

extern crate alloc;
extern crate canadensis_core;
extern crate canadensis_encoding;
extern crate fallible_collections;

#[cfg(not(feature = "node-info"))]
use core::marker::PhantomData;

#[cfg(feature = "node-info")]
mod node_info;
#[cfg(feature = "node-info")]
pub use crate::node_info::{NodeInfo, NodeInfoRequest, Version, INFO_SERVICE};

use canadensis_core::{NodeId, SubjectId};
use canadensis_encoding::{DataType, Serialize, WriteCursor};

/// The subject ID for Heartbeat messages
pub const HEARTBEAT_SUBJECT: SubjectId = SubjectId::from_truncating(7509);

/// Node health values
#[derive(Debug, Copy, Clone)]
pub enum Health {
    /// The node is functioning properly (nominal)
    Nominal = 0,
    /// A critical parameter went out of range or the node encountered a minor failure that does not
    /// prevent the subsystem from performing any of its real-time functions.
    Advisory = 1,
    /// The node encountered a major failure and is performing in a degraded mode or outside of its
    /// designed limitations
    Caution = 2,
    /// The node suffered a fatal malfunction and is unable to perform its intended function
    Warning = 3,
}

/// Node operating modes
#[derive(Debug, Copy, Clone)]
pub enum Mode {
    /// Normal operating mode
    Operational = 0,
    /// Initialization is in progress; this mode is entered immediately after startup
    Initialization = 1,
    /// Calibration, self-test, etc.
    Maintenance = 2,
    /// New software/firmware is being loaded or the bootloader is running
    SoftwareUpdate = 3,
}

pub struct Node<'info> {
    /// The identifier of this node
    id: NodeId,
    // Heartbeat message fields
    /// Uptime in seconds, saturating
    uptime: u32,
    /// Health status
    health: Health,
    /// Operating mode
    mode: Mode,
    /// Vendor-specific status information
    status_code: u8,
    /// Node information
    #[cfg(feature = "node-info")]
    info: Option<NodeInfo<'info>>,
    #[cfg(not(feature = "node-info"))]
    _info_phantom: PhantomData<&'info ()>,
}

impl<'info> Node<'info> {
    /// Creates a node with the provided ID but no node info
    ///
    /// The mode is set to Mode::Initialization.
    pub fn new(id: NodeId) -> Self {
        Node {
            id,
            uptime: 0,
            health: Health::Nominal,
            mode: Mode::Initialization,
            status_code: 0,
            #[cfg(feature = "node-info")]
            info: None,
            #[cfg(not(feature = "node-info"))]
            _info_phantom: PhantomData,
        }
    }

    /// Creates a node with the provided ID and node information
    ///
    /// The mode is set to Mode::Initialization.
    #[cfg(feature = "node-info")]
    pub fn with_info(id: NodeId, info: NodeInfo<'info>) -> Self {
        Node {
            info: Some(info),
            ..Node::new(id)
        }
    }

    /// Returns the ID of this node
    #[inline]
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Increments the uptime counter by 1 second
    pub fn increment_uptime(&mut self) {
        self.uptime = self.uptime.saturating_add(1);
    }
    /// Sets the uptime in seconds
    pub fn set_uptime(&mut self, seconds: u32) {
        self.uptime = seconds
    }
    /// Sets the health status
    pub fn set_health(&mut self, health: Health) {
        self.health = health
    }
    /// Sets the operating mode
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode
    }
    /// Sets the vendor-specific status code
    pub fn set_status_code(&mut self, code: u8) {
        self.status_code = code;
    }
    /// Returns a heartbeat message
    pub fn heartbeat(&self) -> Heartbeat<'_, 'info> {
        Heartbeat(self)
    }
    /// Returns a node information value that can be used to respond to NodeInfo requests
    pub fn info(&self) -> Option<&NodeInfo<'info>> {
        self.info.as_ref()
    }
}

/// A Node wrapper that can be serialized into a Heartbeat message
pub struct Heartbeat<'node, 'info>(&'node Node<'info>);

impl DataType for Heartbeat<'_, '_> {
    const EXTENT_BYTES: Option<u32> = Some(12);
}

impl Serialize for Heartbeat<'_, '_> {
    fn size_bits(&self) -> usize {
        56
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u32(self.0.uptime);
        cursor.write_u2(self.0.health as u8);
        cursor.align_to_8_bits();
        cursor.write_u3(self.0.mode as u8);
        cursor.align_to_8_bits();
        cursor.write_aligned_u8(self.0.status_code);
    }
}

#[cfg(test)]
mod test {
    use super::{Health, Heartbeat, Mode, Node};
    use canadensis_core::NodeId;
    use canadensis_encoding::{Serialize, WriteCursor};
    use core::convert::TryFrom;

    #[test]
    fn heartbeat_basic() {
        let mut node = Node::new(NodeId::try_from(1).unwrap());
        check_serialize(
            node.heartbeat(),
            &[
                0x0, 0x0, 0x0, 0x0, // Uptime 0
                0x0, // Health nominal
                0x1, // Mode initialization
                0x0, // Vendor-specific status 0
            ],
        );
        node.increment_uptime();
        check_serialize(
            node.heartbeat(),
            &[
                0x1, 0x0, 0x0, 0x0, // Uptime 1
                0x0, // Health nominal
                0x1, // Mode initialization
                0x0, // Vendor-specific status 0
            ],
        );
    }

    #[test]
    fn heartbeat_complex() {
        let mut node = Node::new(NodeId::try_from(1).unwrap());
        check_serialize(
            node.heartbeat(),
            &[
                0x0, 0x0, 0x0, 0x0, // Uptime 0
                0x0, // Health nominal
                0x1, // Mode initialization
                0x0, // Vendor-specific status 0
            ],
        );
        node.set_health(Health::Advisory);
        check_serialize(
            node.heartbeat(),
            &[
                0x0, 0x0, 0x0, 0x0, // Uptime 0
                0x1, // Health advisory
                0x1, // Mode initialization
                0x0, // Vendor-specific status 0
            ],
        );
        node.set_health(Health::Caution);
        check_serialize(
            node.heartbeat(),
            &[
                0x0, 0x0, 0x0, 0x0, // Uptime 0
                0x2, // Health caution
                0x1, // Mode initialization
                0x0, // Vendor-specific status 0
            ],
        );
        node.set_health(Health::Warning);
        check_serialize(
            node.heartbeat(),
            &[
                0x0, 0x0, 0x0, 0x0, // Uptime 0
                0x3, // Health warning
                0x1, // Mode initialization
                0x0, // Vendor-specific status 0
            ],
        );
        node.set_mode(Mode::SoftwareUpdate);
        check_serialize(
            node.heartbeat(),
            &[
                0x0, 0x0, 0x0, 0x0, // Uptime 0
                0x3, // Health warning
                0x3, // Mode software update
                0x0, // Vendor-specific status 0
            ],
        );
        node.set_status_code(0x5a);
        check_serialize(
            node.heartbeat(),
            &[
                0x0, 0x0, 0x0, 0x0,  // Uptime 0
                0x3,  // Health warning
                0x3,  // Mode software update
                0x5a, // Vendor-specific status
            ],
        );
    }

    fn check_serialize(heartbeat: Heartbeat<'_, '_>, expected: &[u8]) {
        // All heartbeats fit into 7 bytes
        let mut buffer = [0u8; 7];
        let mut cursor = WriteCursor::new(&mut buffer);
        heartbeat.serialize(&mut cursor);
        assert_eq!(expected, &buffer[..]);
    }
}
