use bxcan::{Can, FilterOwner, Instance};
use canadensis_can::{Mtu, OutOfMemoryError};
use canadensis_core::time::Clock;
use canadensis_core::NodeId;
use canadensis_pnp_client::{AllocationMessage, PnpClient};
use core::convert::Infallible;

/// A plug-and-play node ID assignment client that uses a bxCAN peripheral
pub struct BxCanPnpClient<C: Clock, M, I: Instance> {
    /// A clock used to get the current time
    clock: C,
    /// The CAN peripheral
    pub can: Can<I>,
    /// The node ID allocation client
    pub client: PnpClient<C, M>,
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
        configure_pnp_filters(&client, &mut can)?;
        Ok(BxCanPnpClient { clock, can, client })
    }

    /// Publishes an ID allocation request, sending it onto the CAN bus
    ///
    /// This function returns an error if no transmit mailbox was available for the frame.
    pub fn publish_request(&mut self) -> nb::Result<(), Infallible> {
        publish_request(&mut self.clock, &mut self.client, &mut self.can)
    }

    /// Handles and parses incoming CAN frames, and returns a node ID if one was received
    pub fn handle_incoming_frames(&mut self) -> Option<NodeId> {
        loop {
            // Read a frame, ignore errors
            match self.can.receive() {
                Ok(frame) => {
                    let now = self.clock.now();
                    if let Ok(canadensis_frame) = crate::bxcan_frame_to_uavcan(&frame, now) {
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

/// Configures a CAN interface to accept only node ID allocation messages
pub fn configure_pnp_filters<C, M, I>(
    client: &PnpClient<C, M>,
    can: &mut Can<I>,
) -> Result<(), OutOfMemoryError>
where
    C: Clock,
    M: AllocationMessage,
    I: Instance + FilterOwner,
{
    let mut filters = client.frame_fiters()?;
    crate::optimize_and_apply_filters(&mut filters, can);
    Ok(())
}

/// Creates a node ID allocation request and sends it using the provided CAN interface
pub fn publish_request<C, M, I>(
    clock: &mut C,
    client: &mut PnpClient<C, M>,
    can: &mut Can<I>,
) -> nb::Result<(), Infallible>
where
    C: Clock,
    M: AllocationMessage,
    I: Instance,
{
    let now = clock.now();
    let frame = client.assemble_request(now);
    let bxcan_frame = crate::uavcan_frame_to_bxcan(&frame);
    can.transmit(&bxcan_frame)
        .map(|displaced_frame| drop(displaced_frame))
}
