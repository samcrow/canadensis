use canadensis_core::nb;
use canadensis_core::time::{Clock, Microseconds32};
use canadensis_serial::driver::{ReceiveDriver, TransmitDriver};
use std::collections::vec_deque::VecDeque;
use std::convert::Infallible;

/// A driver that stores frames in a queue and allows frames written to be read back
#[derive(Default)]
pub struct MockDriver {
    bytes: VecDeque<u8>,
}

impl MockDriver {
    /// Returns an iterator over the bytes in the queue from front to back
    #[allow(dead_code)]
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, u8> {
        self.bytes.iter()
    }
}

impl TransmitDriver for MockDriver {
    type Error = Infallible;

    fn send_byte(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.bytes.push_back(byte);
        Ok(())
    }
}

impl ReceiveDriver for MockDriver {
    type Error = Infallible;

    fn receive_byte(&mut self) -> nb::Result<u8, Self::Error> {
        self.bytes.pop_front().ok_or(nb::Error::WouldBlock)
    }
}

/// A clock that produces a Microseconds32 value that is always zero
pub struct ZeroClock;

impl Clock for ZeroClock {
    fn now(&mut self) -> Microseconds32 {
        Microseconds32::from_ticks(0)
    }
}
