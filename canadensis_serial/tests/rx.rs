extern crate canadensis_core;
extern crate canadensis_serial;

mod utils;

use self::utils::{MockDriver, ZeroClock};
use canadensis_core::subscription::DynamicSubscriptionManager;
use canadensis_core::time::{milliseconds, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::Receiver;
use canadensis_core::{Priority, SubjectId};
use canadensis_serial::driver::TransmitDriver;
use canadensis_serial::{SerialNodeId, SerialReceiver, SerialTransferId, Subscription};
use std::convert::TryFrom;

#[test]
fn test_receive_payload_too_large() {
    let mut rx: SerialReceiver<ZeroClock, MockDriver, DynamicSubscriptionManager<Subscription>> =
        SerialReceiver::new(SerialNodeId::try_from(32).unwrap());
    let mut driver = MockDriver::default();

    let subject = SubjectId::try_from(993).unwrap();
    rx.subscribe_message(subject, 16, milliseconds(1000), &mut driver)
        .unwrap();

    let wire_bytes = [
        0x00, 0x00, 0x00, 0x00, // Frame delimiters
        // -- Begin header --
        0xb, // COBS group header
        0x1, // Version
        0x2, // Priority
        0xfe, 0x03, // Source node
        0xff, 0xff, // Destination node
        0xe1, 0x03, // Subject ID
        0x09, 0x30, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, // COBS altered transfer ID
        0x01, 0x01, 0x02, 0x80, // COBS altered frame index and end of transmission
        0x01, 0x1b, // COBS altered user data
        0x0d, 0xba, // Header CRC
        // -- End header --
        0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xa3, 0x90, 0x81, 0x28, 0xff, 0x01, 0x1f, 0xcc, 0xbe,
        0x99, 0x83, 0xf0, 0x73, 0x65, // Data (21 bytes, but will only collect 16)
        0xa3, 0x54, 0x55, 0x73, // Transfer CRC, little-endian
        0x00, 0x00, 0x00, // Frame delimiters
    ];
    IntoIterator::into_iter(wire_bytes).for_each(|byte: u8| driver.send_byte(byte).unwrap());
    assert_eq!(
        rx.receive(&mut ZeroClock, &mut driver).unwrap(),
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: Microseconds32::from_ticks(0),
                transfer_id: SerialTransferId::try_from(0x3009).unwrap(),
                priority: Priority::Fast,
                subject,
                source: Some(SerialNodeId::try_from(0x3fe).unwrap())
            }),
            loopback: false,
            payload: vec![
                0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xa3, 0x90, 0x81, 0x28, 0xff, 0x01, 0x1f, 0xcc,
                0xbe, 0x99
            ],
        })
    )
}
