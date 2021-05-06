use canadensis::{CapacityError, Node, PublishToken};
use canadensis_can::OutOfMemoryError;
use canadensis_core::time::{Clock, Duration, Instant};
use canadensis_core::Priority;
use canadensis_data_types::uavcan::node::health::Health;
use canadensis_data_types::uavcan::node::heartbeat::Heartbeat;
use canadensis_data_types::uavcan::node::mode::Mode;

/// A node with the minimum required application-layer functionality
///
/// A `BasicNode` wraps a [`canadensis::Node`] and adds functionality to periodically send
/// `uavcan.node.Heartbeat.1.0` messages. This is the only application-layer function that
/// is required for all nodes.
///
/// A BasicNode uses up one publisher slot in the enclosed Node.
pub struct MinimalNode<N>
where
    N: Node,
{
    /// The inner node
    node: N,
    /// The heartbeat message that will be periodically sent
    heartbeat: Heartbeat,
    /// The time this node was created
    start_time: <N::Clock as Clock>::Instant,
    /// The whole seconds of the time when the heartbeat message was last sent
    last_heartbeat_seconds: u32,
    /// The token used to publish heartbeat messages
    heartbeat_token: PublishToken<Heartbeat>,
}

impl<N> MinimalNode<N>
where
    N: Node,
{
    pub fn new(mut node: N) -> Result<Self, CapacityError> {
        // Default heartbeat settings
        let heartbeat = Heartbeat {
            uptime: 0,
            health: Health::Nominal,
            mode: Mode::Operational,
            vendor_specific_status_code: 0,
        };
        let start_time = node.clock_mut().now();
        let heartbeat_timeout =
            <<N::Clock as Clock>::Instant as Instant>::Duration::from_millis(500)
                .expect("Duration type can't represent 500 milliseconds");

        let heartbeat_token =
            node.start_publishing(Heartbeat::SUBJECT, heartbeat_timeout, Priority::Low)?;

        Ok(MinimalNode {
            node,
            heartbeat,
            last_heartbeat_seconds: 0,
            start_time,
            heartbeat_token,
        })
    }

    /// This function must be called once per second (or more frequently) to send heartbeat
    /// messages
    pub fn run_periodic_tasks(&mut self) -> Result<(), OutOfMemoryError> {
        // Determine if a new heartbeat should be sent
        let time_since_start = self.node.clock_mut().now().duration_since(&self.start_time);
        let seconds_since_start = time_since_start.as_secs() as u32;
        if seconds_since_start != self.last_heartbeat_seconds {
            self.last_heartbeat_seconds = seconds_since_start;
            self.send_heartbeat()
        } else {
            // Nothing to do right now
            Ok(())
        }
    }

    /// Publishes a heartbeat message
    fn send_heartbeat(&mut self) -> Result<(), OutOfMemoryError> {
        self.heartbeat.uptime = self.heartbeat.uptime.saturating_add(1);
        self.node.publish(&self.heartbeat_token, &self.heartbeat)
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

    /// Returns the time when this node was created
    pub(crate) fn start_time(&self) -> <N::Clock as Clock>::Instant {
        self.start_time.clone()
    }
}
