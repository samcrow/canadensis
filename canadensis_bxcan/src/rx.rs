use crate::bxcan_frame_to_uavcan;
use alloc::vec::Vec;
use bxcan::{Instance, Rx};
use canadensis::time::Instant;
use canadensis::transfer::Transfer;
use canadensis::Receiver;

pub struct ReceiveAdapter<C, I: Instant> {
    can: Rx<C>,
    uavcan_receiver: Receiver<I>,
}

impl<C, I> ReceiveAdapter<C, I>
where
    C: Instance,
    I: Instant,
{
    pub fn new(can: Rx<C>, uavcan_receiver: Receiver<I>) -> Self {
        ReceiveAdapter {
            can,
            uavcan_receiver,
        }
    }
    pub fn receive_frames<N, H>(&mut self, mut now: N, mut transfer_handler: H)
    where
        N: FnMut() -> I,
        H: FnMut(Transfer<Vec<u8>, I>),
    {
        loop {
            match self.can.receive() {
                // Need to call now() for each frame to give it an accurate timestamp.
                // When a frame completes a transfer, it may take a significant amount of time
                // to process the transfer before the next frame can be received.
                Ok(frame) => self.handle_incoming_bxcan_frame(frame, now(), &mut transfer_handler),
                Err(nb::Error::Other(())) => {
                    // The receive FIFO has overflowed and at least one frame has been lost.
                    // What can we do?
                }
                Err(nb::Error::WouldBlock) => break,
            }
        }
    }

    fn handle_incoming_bxcan_frame<H>(
        &mut self,
        frame: bxcan::Frame,
        timestamp: I,
        transfer_handler: &mut H,
    ) where
        H: FnMut(Transfer<Vec<u8>, I>),
    {
        if let Ok(uavcan_frame) = bxcan_frame_to_uavcan(&frame, timestamp) {
            self.handle_incoming_frame(uavcan_frame, transfer_handler);
        }
    }

    fn handle_incoming_frame<H>(&mut self, frame: canadensis::Frame<I>, transfer_handler: &mut H)
    where
        H: FnMut(Transfer<Vec<u8>, I>),
    {
        match self.uavcan_receiver.accept(frame) {
            Ok(Some(transfer_in)) => transfer_handler(transfer_in),
            Ok(None) => {}
            Err(_oom) => {
                // Out of memory
                // What can we do?
            }
        }
    }
}
