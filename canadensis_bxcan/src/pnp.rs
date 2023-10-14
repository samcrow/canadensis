//!
//! Plug-and-play node ID allocation
//!

use crate::BxCanDriver;
use bxcan::{Can, FilterOwner, Instance, OverrunError};
use canadensis::core::time::Clock;
use canadensis_can::queue::{SingleFrameQueue, SingleQueueDriver};
use canadensis_can::{CanNodeId, CanReceiver, CanTransmitter, CanTransport, Error, Mtu};
use canadensis_pnp_client::{AllocationMessage, PnpClient};

/// A plug-and-play node ID assignment client that uses a bxCAN peripheral
pub struct BxCanPnpClient<C: Clock, M, I: Instance + FilterOwner> {
    /// A clock used to get the current time
    clock: C,
    /// The node ID allocation client
    pub client: PnpClient<
        C,
        M,
        CanTransmitter<C, SingleQueueDriver<C, SingleFrameQueue, BxCanDriver<I>>>,
        CanReceiver<C, SingleQueueDriver<C, SingleFrameQueue, BxCanDriver<I>>>,
    >,
    driver: SingleQueueDriver<C, SingleFrameQueue, BxCanDriver<I>>,
}

impl<C, M, I> BxCanPnpClient<C, M, I>
where
    C: Clock,
    M: AllocationMessage<CanTransport>,
    I: Instance + FilterOwner,
{
    /// Creates a node ID allocation client
    pub fn new(clock: C, can: Can<I>, unique_id: [u8; 16]) -> Result<Self, Error<OverrunError>> {
        let driver = BxCanDriver::new(can);
        let mut driver = SingleQueueDriver::new(SingleFrameQueue::new(), driver);
        let transmitter = CanTransmitter::new(Mtu::Can8);
        let receiver = CanReceiver::new_anonymous(Mtu::Can8);
        let client = PnpClient::new(transmitter, receiver, unique_id, &mut driver)?;
        Ok(BxCanPnpClient {
            clock,
            client,
            driver,
        })
    }

    /// Publishes an ID allocation request, sending it onto the CAN bus
    ///
    /// This function returns an error if no transmit mailbox was available for the frame.
    pub fn publish_request(&mut self) {
        self.client.send_request(&mut self.clock, &mut self.driver)
    }

    /// Handles and parses incoming CAN frames, and returns a node ID if one was received
    pub fn handle_incoming_frames(&mut self, clock: &mut C) -> Option<CanNodeId> {
        match self.client.receive(clock, &mut self.driver) {
            Ok(Some(id)) => Some(id),
            Ok(None) | Err(_) => None,
        }
    }

    /// Breaks up this client into its clock and CAN driver
    pub fn free(self) -> (C, Can<I>) {
        let (_, driver) = self.driver.into_parts();
        let can = driver.into_can();
        (self.clock, can)
    }
}
