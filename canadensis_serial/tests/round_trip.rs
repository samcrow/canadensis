extern crate canadensis_core;
extern crate canadensis_serial;

mod utils;

use self::utils::{MockDriver, ZeroClock};
use canadensis_core::subscription::DynamicSubscriptionManager;
use canadensis_core::time::{MicrosecondDuration32, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::{Receiver, Transmitter};
use canadensis_core::{Priority, SubjectId};
use canadensis_serial::{
    SerialNodeId, SerialReceiver, SerialTransmitter, SerialTransport, Subscription,
};
use std::convert::{TryFrom, TryInto};

#[test]
fn round_trip_no_payload() {
    let mut driver = MockDriver::default();
    let subject = SubjectId::try_from(9u16).unwrap();
    let mut tx = SerialTransmitter::<_, 39>::new();
    let transfer: Transfer<Vec<u8>, SerialTransport> = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: Microseconds32::from_ticks(0),
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
    rx.subscribe_message(
        subject,
        0,
        MicrosecondDuration32::from_ticks(0),
        &mut driver,
    )
    .unwrap();

    // Only need to call receive once. It will read all the available frames.
    let received = rx
        .receive(&mut ZeroClock, &mut driver)
        .unwrap()
        .expect("No transfer");

    assert_eq!(transfer, received);
}
