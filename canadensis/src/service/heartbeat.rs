use crate::core::time::milliseconds;
use crate::core::Priority;
use crate::{nb, Node, PublishToken, StartSendError, Transmitter};
use canadensis_data_types::uavcan::node::health_1_0::Health;
use canadensis_data_types::uavcan::node::heartbeat_1_0::{Heartbeat, SUBJECT};
use canadensis_data_types::uavcan::node::mode_1_0::Mode;
use core::marker::PhantomData;

/// Publishes heartbeat messages
pub struct HeartbeatService<N> {
    heartbeat: Heartbeat,
    publish_token: PublishToken<Heartbeat>,
    _node: PhantomData<N>,
}

impl<N> HeartbeatService<N>
where
    N: Node,
{
    /// Creates a new HeartbeatService
    ///
    /// * `node`: The node to use for publishing
    pub fn new(
        node: &mut N,
    ) -> Result<Self, StartSendError<<N::Transmitter as Transmitter<N::Clock>>::Error>> {
        let token = node.start_publishing(SUBJECT, milliseconds(1000), Priority::Nominal.into())?;

        let heatbeat = Heartbeat {
            uptime: 0,
            health: Health {
                value: Health::NOMINAL,
            },
            mode: Mode {
                value: Mode::OPERATIONAL,
            },
            vendor_specific_status_code: 0,
        };

        Ok(Self {
            heartbeat: heatbeat,
            publish_token: token,
            _node: PhantomData,
        })
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

    /// Publishes a heartbeat message
    ///
    /// Call this once per second
    pub fn publish_heartbeat(
        &mut self,
        node: &mut N,
    ) -> nb::Result<(), <N::Transmitter as Transmitter<N::Clock>>::Error> {
        self.heartbeat.uptime = self.heartbeat.uptime.saturating_add(1);
        node.publish(&self.publish_token, &self.heartbeat)
    }
}
