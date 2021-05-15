use bxcan::filter::{BankConfig, Mask32};
use bxcan::{Can, ExtendedId, FilterOwner, Instance};
use canadensis_can::{Mtu, OutOfMemoryError};
use canadensis_core::time::Clock;
use canadensis_core::NodeId;
use canadensis_pnp_client::{AllocationMessage, PnpClient};

/// A plug-and-play node ID assignment client that uses a bxCAN peripheral
pub struct BxCanPnpClient<C: Clock, M, I: Instance> {
    /// A clock used to get the current time
    clock: C,
    /// The CAN peripheral
    can: Can<I>,
    /// The node ID allocation client
    client: PnpClient<C, M>,
}

impl<C, M, I> BxCanPnpClient<C, M, I>
where
    C: Clock,
    M: AllocationMessage,
    I: Instance + FilterOwner,
{
    /// Creates a node ID allocation client
    pub fn new(clock: C, mut can: Can<I>, unique_id: [u8; 16]) -> Result<Self, OutOfMemoryError> {
        let client = PnpClient::new(Mtu::Can8, unique_id)?;
        {
            // Configure the CAN filter to accept only node ID allocation messages
            let mut filter_banks = can.modify_filters();

            let mut filters = client.frame_fiters()?;
            rtt_target::rprintln!("Raw filters: {:?}", filters);
            let optimized_filters =
                canadensis_filter_config::optimize(&mut filters, filter_banks.num_banks().into());
            filter_banks.clear();
            for (i, filter) in optimized_filters.iter().enumerate() {
                let id = ExtendedId::new(filter.id()).unwrap();
                let mask = ExtendedId::new(filter.mask()).unwrap();
                filter_banks.enable_bank(
                    i as u8,
                    BankConfig::Mask32(Mask32::frames_with_ext_id(id, mask)),
                );
            }
        }

        Ok(BxCanPnpClient { clock, can, client })
    }

    /// Publishes an ID allocation request, sending it onto the CAN bus
    pub fn publish_request(&mut self) {
        let now = self.clock.now();
        let frame = self.client.assemble_request(now);
        let bxcan_frame = crate::uavcan_frame_to_bxcan(&frame);
        // Ignore errors. If there's a bus problem, we can't really do anything.
        let _ = self.can.transmit(&bxcan_frame);
    }

    /// Handles and parses incoming CAN frames, and returns a node ID if one was received
    pub fn handle_incoming_frames(&mut self) -> Option<NodeId> {
        loop {
            // Read a frame, ignore errors
            match self.can.receive() {
                Ok(frame) => {
                    let now = self.clock.now();
                    if let Ok(canadensis_frame) = crate::bxcan_frame_to_uavcan(&frame, now) {
                        rtt_target::rprintln!("Handling frame {:?}", canadensis_frame);
                        if let Ok(Some(received_node_id)) = self.client.accept(canadensis_frame) {
                            break Some(received_node_id);
                        }
                    }
                }
                Err(nb::Error::WouldBlock) => break None,
                Err(nb::Error::Other(())) => { /* Ignore error, keep reading */ }
            }
        }
    }

    /// Breaks up this client into its clock and CAN driver
    pub fn free(self) -> (C, Can<I>) {
        (self.clock, self.can)
    }
}
