extern crate canadensis_core;
extern crate canadensis_serial;
extern crate simplelog;

use canadensis_core::subscription::DynamicSubscriptionManager;
use canadensis_core::time::{Clock, MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::{Receiver, Transmitter};
use canadensis_core::{nb, Priority, SubjectId};
use canadensis_serial::driver::{ReceiveDriver, TransmitDriver};
use canadensis_serial::{
    SerialNodeId, SerialReceiver, SerialTransmitter, SerialTransport, Subscription,
};
use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use std::collections::VecDeque;
use std::convert::{Infallible, TryFrom, TryInto};

#[test]
fn round_trip_no_payload() {
    let _ = TermLogger::init(
        LevelFilter::Debug,
        Default::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    );

    let mut driver = MockDriver::default();
    let subject = SubjectId::try_from(9u16).unwrap();
    let mut tx = SerialTransmitter::<_, 39>::new();
    let transfer: Transfer<Vec<u8>, SerialTransport> = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: Microseconds32::new(0),
            transfer_id: 330.into(),
            priority: Priority::Low,
            subject,
            source: Some(37u16.try_into().unwrap()),
        }),
        loopback: false,
        payload: vec![],
    };
    tx.push(transfer.clone(), &mut ZeroClock, &mut driver)
        .unwrap();
    tx.flush(&mut ZeroClock, &mut driver).unwrap();
    let wire_bytes: Vec<u8> = driver.iter().copied().collect();
    println!("{:02x?}", wire_bytes);

    let mut rx: SerialReceiver<ZeroClock, MockDriver, DynamicSubscriptionManager<Subscription>> =
        SerialReceiver::new(SerialNodeId::try_from(360).unwrap());
    rx.subscribe_message(subject, 0, MicrosecondDuration32::new(0), &mut driver)
        .unwrap();

    // Only need to call receive once. It will read all the available frames.
    let received = rx
        .receive(&mut ZeroClock, &mut driver)
        .unwrap()
        .expect("No transfer");

    assert_eq!(transfer, received);
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
    fn now(&mut self) -> Microseconds32 {
        Microseconds32::new(0)
    }
}
