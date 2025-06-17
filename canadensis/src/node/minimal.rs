use crate::{Node, PublishError, StartSendError};
use canadensis_core::time::MicrosecondDuration32;
use canadensis_core::transport::Transmitter;
use canadensis_core::{nb, Priority};
use canadensis_data_types::uavcan::node::health_1_0::Health;
use canadensis_data_types::uavcan::node::heartbeat_1_0::{self, Heartbeat};
use canadensis_data_types::uavcan::node::mode_1_0::Mode;

/// A node with the minimum required application-layer functionality
///
/// A `BasicNode` wraps a [`Node`](crate::Node) and adds functionality to send a
/// `uavcan.node.Heartbeat.1.0` message every second. This is the only application-layer function
/// that is required for all nodes.
///
/// A BasicNode uses up one publisher slot in the underlying Node.
///
/// The underlying node type `N` is usually a [`CoreNode`](crate::node::CoreNode).
pub struct MinimalNode<N>
where
    N: Node,
{
    /// The inner node
    node: N,
    /// The heartbeat message that will be periodically sent
    heartbeat: Heartbeat,
}

impl<N> MinimalNode<N>
where
    N: Node,
{
    /// Creates a new minimal node
    ///
    /// * `node`: The underlying node (this is usually a [`CoreNode`](crate::node::CoreNode))
    pub fn new(
        mut node: N,
    ) -> Result<Self, StartSendError<<N::Transmitter as Transmitter<N::Clock>>::Error>> {
        // Default heartbeat settings
        let heartbeat = Heartbeat {
            uptime: 0,
            health: Health {
                value: Health::NOMINAL,
            },
            mode: Mode {
                value: Mode::OPERATIONAL,
            },
            vendor_specific_status_code: 0,
        };
        let heartbeat_timeout = MicrosecondDuration32::from_ticks(500_000);

        node.start_publishing(
            heartbeat_1_0::SUBJECT,
            heartbeat_timeout,
            Priority::Nominal.into(),
        )?;

        Ok(MinimalNode { node, heartbeat })
    }

    /// This function must be called once per second to send heartbeat messages
    ///
    /// Unlike [`run_periodic_tasks`](#method.run_periodic_tasks), this function does not check
    /// if one second has passed since the last time it was called.
    ///
    /// Either `run_periodic_tasks` or `run_per_second_tasks` should be called, but not both.
    pub fn run_per_second_tasks(
        &mut self,
    ) -> nb::Result<(), PublishError<<N::Transmitter as Transmitter<N::Clock>>::Error>> {
        self.send_heartbeat()
    }

    /// Publishes a heartbeat message
    fn send_heartbeat(
        &mut self,
    ) -> nb::Result<(), PublishError<<N::Transmitter as Transmitter<N::Clock>>::Error>> {
        self.heartbeat.uptime = self.heartbeat.uptime.saturating_add(1);
        self.node.publish(heartbeat_1_0::SUBJECT, &self.heartbeat)
    }

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

    /// Returns a reference to the enclosed node
    pub fn node(&self) -> &N {
        &self.node
    }

    /// Returns a mutable reference to the enclosed node
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.node
    }
}
