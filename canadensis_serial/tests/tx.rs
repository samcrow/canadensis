extern crate canadensis_core;
extern crate canadensis_serial;

use canadensis_core::time::{Clock, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::Transmitter;
use canadensis_core::{nb, Priority};
use canadensis_serial::driver::{ReceiveDriver, TransmitDriver};
use canadensis_serial::{SerialTransmitter, SerialTransport};
use std::collections::VecDeque;
use std::convert::{Infallible, TryInto};

#[test]
fn transmit_capacity_1() {
    let mut driver = MockDriver::default();
    let mut tx = SerialTransmitter::<_, 1>::new();
    let transfer: Transfer<[u8; 0], Microseconds32, SerialTransport> = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: Microseconds32::new(0),
            transfer_id: 0.into(),
            priority: Priority::Low,
            subject: 9u16.try_into().unwrap(),
            source: Some(37u16.try_into().unwrap()),
        }),
        payload: [],
    };
    assert!(tx.push(transfer, &mut ZeroClock, &mut driver).is_err());
}
#[test]
fn transmit_minimum_capacity() {
    let mut driver = MockDriver::default();
    // Minimum queue capacity: 1 delimiter + 32 bytes header + 0 payload + 4 CRC + 1 delimiter + 1 zero escaping = 39 bytes
    let mut tx = SerialTransmitter::<_, 39>::new();
    let transfer: Transfer<[u8; 0], Microseconds32, SerialTransport> = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: Microseconds32::new(0),
            transfer_id: 330.into(),
            priority: Priority::Low,
            subject: 9u16.try_into().unwrap(),
            source: Some(37u16.try_into().unwrap()),
        }),
        payload: [],
    };
    tx.push(transfer, &mut ZeroClock, &mut driver).unwrap();
    tx.flush(&mut ZeroClock, &mut driver).unwrap();
    let queue: Vec<u8> = driver.iter().copied().collect();
    assert_eq!(queue.len(), 39)
}

/// A driver that stores frames in a queue and allows frames written to be read back
#[derive(Default)]
pub struct MockDriver {
    bytes: VecDeque<u8>,
}

impl MockDriver {
    /// Returns an iterator over the bytes in the queue from front to back
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
    type Instant = Microseconds32;

    fn now(&mut self) -> Self::Instant {
        Microseconds32::new(0)
    }
}
