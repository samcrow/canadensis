extern crate canadensis_core;
extern crate canadensis_serial;

use canadensis_core::time::Microseconds32;
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::Transmitter;
use canadensis_core::Priority;
use canadensis_serial::{SerialTransmitter, SerialTransport};
use std::convert::TryInto;

#[test]
fn transmit_capacity_1() {
    let mut tx = SerialTransmitter::<1>::new();
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
    assert!(tx.push(transfer).is_err());
}
#[test]
fn transmit_minimum_capacity() {
    // Minimum queue capacity: 1 delimiter + 32 bytes header + 0 payload + 4 CRC + 1 delimiter + 1 zero escaping = 39 bytes
    let mut tx = SerialTransmitter::<39>::new();
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
    tx.push(transfer).unwrap();
    let queue: Vec<u8> = tx.queue().iter().copied().collect();
    assert_eq!(queue.len(), 39)
}
