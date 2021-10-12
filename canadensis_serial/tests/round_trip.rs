extern crate canadensis_core;
extern crate canadensis_serial;
extern crate simplelog;

use canadensis_core::time::{MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::{Receiver, Transmitter};
use canadensis_core::{Priority, SubjectId};
use canadensis_serial::{SerialNodeId, SerialReceiver, SerialTransmitter, SerialTransport};
use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use std::convert::{TryFrom, TryInto};

#[test]
fn round_trip_no_payload() {
    let _ = TermLogger::init(
        LevelFilter::Debug,
        Default::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    );

    let subject = SubjectId::try_from(9u16).unwrap();
    let mut tx = SerialTransmitter::<39>::new();
    let transfer: Transfer<Vec<u8>, Microseconds32, SerialTransport> = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: Microseconds32::new(0),
            transfer_id: 330.into(),
            priority: Priority::Low,
            subject,
            source: Some(37u16.try_into().unwrap()),
        }),
        payload: vec![],
    };
    tx.push(transfer.clone()).unwrap();
    let wire_bytes: Vec<u8> = tx.queue().iter().copied().collect();
    println!("{:02x?}", wire_bytes);

    let mut rx = SerialReceiver::<Microseconds32>::new(SerialNodeId::try_from(360).unwrap());
    rx.subscribe_message(subject, 0, MicrosecondDuration32::new(0))
        .unwrap();

    let (&last_byte, others) = wire_bytes.split_last().unwrap();
    for &byte in others {
        let status = rx.accept(byte);
        assert!(status.unwrap().is_none());
    }
    let received = rx.accept(last_byte).unwrap().expect("No transfer");

    assert_eq!(transfer, received);
}
