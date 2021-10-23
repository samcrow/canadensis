//!
//! Plug-and-play node ID allocation
//!

use crate::BxCanDriver;
use bxcan::{Can, FilterOwner, Instance};
use canadensis::core::time::Clock;
use canadensis::core::OutOfMemoryError;
use canadensis_can::queue::{SingleFrameQueue, SingleQueueDriver};
use canadensis_can::types::{CanNodeId, CanTransport};
use canadensis_can::{CanReceiver, CanTransmitter, Mtu};
use canadensis_pnp_client::{AllocationMessage, PnpClient};

/// A plug-and-play node ID assignment client that uses a bxCAN peripheral
pub struct BxCanPnpClient<C: Clock, M, I: Instance + FilterOwner> {
    /// A clock used to get the current time
    clock: C,
    /// The node ID allocation client
    pub client: PnpClient<
        C,
        M,
        CanTransmitter<C::Instant, SingleQueueDriver<SingleFrameQueue<C::Instant>, BxCanDriver<C::Instant, I>>>,
        CanReceiver<
            C::Instant,
            BxCanDriver<C::Instant, I>>,
        >,
    >,
}

impl<C, M, I> BxCanPnpClient<C, M, I>
where
    C: Clock,
    M: AllocationMessage<CanTransport<C::Instant>>,
    I: Instance + FilterOwner,
{
    /// Creates a node ID allocation client
    pub fn new(clock: C, mut can: Can<I>, unique_id: [u8; 16]) -> Result<Self, OutOfMemoryError> {
        let transmitter = CanTransmitter::new(Mtu::Can8, SingleFrameQueue::new());
        let receiver = CanReceiver::new_anonymous(Mtu::Can8, BxCanDriver::new(can));
        let client = PnpClient::new(transmitter, receiver, unique_id)?;
        Ok(BxCanPnpClient { clock, client })
    }

    /// Publishes an ID allocation request, sending it onto the CAN bus
    ///
    /// This function returns an error if no transmit mailbox was available for the frame.
    pub fn publish_request(&mut self) {
        self.client.send_request(&mut self.clock)
    }

    /// Handles and parses incoming CAN frames, and returns a node ID if one was received
    pub fn handle_incoming_frames(&mut self, now: C::Instant) -> Option<CanNodeId> {
        match self.client.receive(now) {
            Ok(Some(id)) => Some(id),
            Ok(None) | Err(_) => None,
        }
    }

    /// Breaks up this client into its clock and CAN driver
    pub fn free(self) -> (C, Can<I>) {
        (self.clock, self.can)
    }
}
