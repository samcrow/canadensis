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
pub const HEARTBEAT_SUBJECT: SubjectId = SubjectId::from_truncating(32085);

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
    /// The node is no longer available
    Offline = 7,
}

/// 19-bit mask for status codes
const STATUS_CODE_MASK: u32 = 0x0007_ffff;

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
    /// Vendor-specific status information (this always fits within 19 bits)
    status_code: u32,
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
    ///
    /// # Panics
    /// This function panics if the status code does not fit within 19 bits
    pub fn set_status_code(&mut self, code: u32) {
        assert_eq!(
            code & !STATUS_CODE_MASK,
            0,
            "Status code does not fit within 19 bits"
        );
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

/// A Node wrapper that can be serialized into a Hearbeat message
pub struct Heartbeat<'node, 'info>(&'node Node<'info>);

impl DataType for Heartbeat<'_, '_> {}

impl Serialize for Heartbeat<'_, '_> {
    fn size_bits(&self) -> usize {
        56
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u32(self.0.uptime);
        cursor.write_u2(self.0.health as u8);
        cursor.write_u3(self.0.mode as u8);
        cursor.write_u19(self.0.status_code);
    }
}
