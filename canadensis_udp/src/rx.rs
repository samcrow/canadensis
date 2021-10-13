use crate::{NodeAddress, UdpTransport};
use alloc::vec::Vec;
use canadensis_core::time::{Duration, Instant};
use canadensis_core::transfer::Transfer;
use canadensis_core::transport::{Receiver, Transport};
use canadensis_core::{ServiceId, ServiceSubscribeError, SubjectId};

/// UDP transport receiver
pub struct UdpReceiver<const MTU: usize> {
    address: NodeAddress,
}

impl<I, const MTU: usize> Receiver<I> for UdpReceiver<MTU>
where
    I: Instant,
{
    type Transport = UdpTransport<I, (), MTU>;

    fn accept(
        &mut self,
        frame: <Self::Transport as Transport>::Frame,
    ) -> Result<Option<Transfer<Vec<u8>, I, Self::Transport>>, <Self::Transport as Transport>::Error>
    {
        todo!()
    }

    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
    ) -> Result<(), <Self::Transport as Transport>::Error> {
        todo!()
    }

    fn unsubscribe_message(&mut self, subject: SubjectId) {
        todo!()
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
    ) -> Result<(), ServiceSubscribeError<<Self::Transport as Transport>::Error>> {
        todo!()
    }

    fn unsubscribe_request(&mut self, service: ServiceId) {
        todo!()
    }

    fn subscribe_response(
        &mut self,
        service: ServiceId,
        payload_size_max: usize,
        timeout: <I as Instant>::Duration,
    ) -> Result<(), ServiceSubscribeError<<Self::Transport as Transport>::Error>> {
        todo!()
    }

    fn unsubscribe_response(&mut self, service: ServiceId) {
        todo!()
    }
}
