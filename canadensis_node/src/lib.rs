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
use crate::node_info::NodeInfo;
use canadensis_core::NodeId;

/// Node health values
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
    info: Option<&'info NodeInfo<'info>>,
    #[cfg(not(feature = "node-info"))]
    _info_phantom: PhantomData<&'info ()>,
}

impl<'info> Node<'info> {
    /// Creates a node with the provided ID
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
    pub fn with_info(id: NodeId, info: &'info NodeInfo<'info>) -> Self {
        Node {
            info: Some(info),
            ..Node::new(id)
        }
    }
}
