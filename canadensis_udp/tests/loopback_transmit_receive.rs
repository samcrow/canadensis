extern crate canadensis_core;
extern crate canadensis_linux;
extern crate canadensis_udp;
extern crate simplelog;

use canadensis_core::session::SessionDynamicMap;
use canadensis_core::time::{milliseconds, Clock, MicrosecondDuration64, Microseconds64};
use canadensis_core::transfer::{Header, MessageHeader, ServiceHeader, Transfer};
use canadensis_core::transport::{Receiver, TransferId, Transmitter};
use canadensis_core::{Priority, SubjectId};
use canadensis_linux::SystemClock;
use canadensis_udp::{
    UdpNodeId, UdpReceiver, UdpSessionData, UdpTransferId, UdpTransmitter, UdpTransport,
};
use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use std::convert::{TryFrom, TryInto};
use std::net::Ipv4Addr;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[test]
fn transmit_receive_message_two_frames() {
    init_test_logging();

    let transmit_node_id = UdpNodeId::try_from(120).unwrap();
    let mut clock = SystemClock::new();
    const MTU: usize = 1472;

    // Make a payload compatible with the uavcan.metatransport.ethernet.Frame.0.1 format format.
    let payload = {
        let mut payload = Vec::with_capacity(6 + 6 + 2 + 2 + MAJOR_GENERAL_SONG.len());
        // Destination
        payload.extend_from_slice(&[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6]);
        // Source
        payload.extend_from_slice(&[0x31, 0x32, 0x33, 0x34, 0x35, 0x36]);
        // Type IPv4
        payload.extend_from_slice(&[0x00, 0x08]);
        let length: u16 = MAJOR_GENERAL_SONG.len().try_into().unwrap();
        payload.extend_from_slice(&length.to_le_bytes());
        payload.extend_from_slice(MAJOR_GENERAL_SONG);
        payload
    };
    let transfer = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: milliseconds::<MicrosecondDuration64>(5000) + clock.now(),
            transfer_id: UdpTransferId::default(),
            priority: Priority::Nominal,
            subject: SubjectId::try_from(73u16).unwrap(),
            source: Some(transmit_node_id),
        }),
        payload,
    };
    check_loopback::<_, _, MTU>(
        transfer,
        &mut clock,
        |rx| {
            rx.subscribe_message(
                73.try_into().unwrap(),
                4096,
                MicrosecondDuration64::new(2_000_000),
                &mut (),
            )
            .unwrap()
        },
        |rx| rx.unsubscribe_message(73.try_into().unwrap(), &mut ()),
    );
}

#[test]
fn transmit_receive_message_one_byte_one_frame() {
    init_test_logging();
    let mut clock = SystemClock::new();
    let subject = 1030.try_into().unwrap();
    let transfer = Transfer {
        header: Header::Message(MessageHeader {
            timestamp: milliseconds::<MicrosecondDuration64>(5000) + clock.now(),
            transfer_id: 1.try_into().unwrap(),
            priority: Priority::Low,
            subject,
            source: Some(8.try_into().unwrap()),
        }),
        payload: vec![0x27],
    };
    check_loopback::<_, _, 1472>(
        transfer,
        &mut clock,
        |rx| {
            rx.subscribe_message(subject, 1, milliseconds(1000), &mut ())
                .unwrap()
        },
        |rx| rx.unsubscribe_message(subject, &mut ()),
    )
}

#[test]
fn transmit_receive_request_one_byte_one_frame() {
    init_test_logging();
    let mut clock = SystemClock::new();
    let service = 82.try_into().unwrap();
    let transfer = Transfer {
        header: Header::Request(ServiceHeader {
            timestamp: milliseconds::<MicrosecondDuration64>(5000) + clock.now(),
            transfer_id: 1.try_into().unwrap(),
            priority: Priority::Low,
            service,
            source: 8.try_into().unwrap(),
            destination: 993.try_into().unwrap(),
        }),
        payload: vec![0x27],
    };
    check_loopback::<_, _, 1472>(
        transfer,
        &mut clock,
        |rx| {
            rx.subscribe_request(service, 1, milliseconds(1000), &mut ())
                .unwrap()
        },
        |rx| rx.unsubscribe_request(service, &mut ()),
    )
}

#[test]
fn transmit_receive_response_one_byte_one_frame() {
    init_test_logging();
    let mut clock = SystemClock::new();
    let service = 82.try_into().unwrap();
    let transfer = Transfer {
        header: Header::Response(ServiceHeader {
            timestamp: milliseconds::<MicrosecondDuration64>(5000) + clock.now(),
            transfer_id: 1.try_into().unwrap(),
            priority: Priority::Low,
            service,
            source: 8.try_into().unwrap(),
            destination: 993.try_into().unwrap(),
        }),
        payload: vec![0x27],
    };
    check_loopback::<_, _, 1472>(
        transfer,
        &mut clock,
        |rx| {
            rx.subscribe_response(service, 1, milliseconds(1000), &mut ())
                .unwrap()
        },
        |rx| rx.unsubscribe_response(service, &mut ()),
    )
}

type TestUdpReceiver<const MTU: usize> = UdpReceiver<
    Microseconds64,
    SessionDynamicMap<Microseconds64, UdpNodeId, UdpTransferId, UdpSessionData>,
    MTU,
>;

fn check_loopback<S, U, const MTU: usize>(
    mut transfer: Transfer<Vec<u8>, Microseconds64, UdpTransport>,
    clock: &mut SystemClock,
    subscribe: S,
    unsubscribe: U,
) where
    S: FnOnce(&mut TestUdpReceiver<MTU>),
    U: FnOnce(&mut TestUdpReceiver<MTU>),
{
    // Receiver node ID must match the destination of a service transfer. For non-service transfers,
    // it can be anything.
    let receive_node_id = match &transfer.header {
        Header::Message(_) => UdpNodeId::try_from(3).unwrap(),
        Header::Request(header) | Header::Response(header) => header.destination,
    };
    // Use an OS-assigned ephemeral port
    let mut receiver =
        TestUdpReceiver::<MTU>::new(Some(receive_node_id), Ipv4Addr::LOCALHOST, 0).unwrap();
    let receiver_port = receiver.local_addr().unwrap().port();

    let mut transmitter = UdpTransmitter::<MTU>::new(Ipv4Addr::LOCALHOST, receiver_port).unwrap();

    send_and_expect_not_received(&mut transmitter, &mut receiver, clock, &mut transfer);

    subscribe(&mut receiver);

    for _ in 0..10 {
        transmitter.push(transfer.clone(), clock, &mut ()).unwrap();
        transmitter.flush(clock, &mut ()).unwrap();
        // Increment transfer ID
        match &mut transfer.header {
            Header::Message(header) => {
                header.transfer_id = header.transfer_id.increment();
            }
            Header::Request(header) | Header::Response(header) => {
                header.transfer_id = header.transfer_id.increment()
            }
        };

        let timeout = Instant::now() + Duration::from_secs(1);
        loop {
            match receiver.receive(clock.now(), &mut ()) {
                Ok(Some(received_transfer)) => {
                    assert_eq!(&received_transfer.payload, &transfer.payload);
                    break;
                }
                Ok(None) => {
                    sleep(Duration::from_millis(10));
                }
                Err(e) => panic!("Receive error {:?}", e),
            }

            if Instant::now() > timeout {
                panic!("Timed out waiting for receive");
            }
        }
    }

    unsubscribe(&mut receiver);

    // Send the transfer again. It should not be received.
}

fn send_and_expect_not_received<const MTU: usize>(
    transmitter: &mut UdpTransmitter<MTU>,
    receiver: &mut TestUdpReceiver<MTU>,
    clock: &mut SystemClock,
    transfer: &mut Transfer<Vec<u8>, Microseconds64, UdpTransport>,
) {
    for _ in 0..10 {
        transmitter.push(transfer.clone(), clock, &mut ()).unwrap();
        transmitter.flush(clock, &mut ()).unwrap();

        // Increment transfer ID
        match &mut transfer.header {
            Header::Message(header) => {
                header.transfer_id = header.transfer_id.increment();
            }
            Header::Request(header) | Header::Response(header) => {
                header.transfer_id = header.transfer_id.increment()
            }
        };

        let timeout = Instant::now() + Duration::from_millis(100);
        loop {
            match receiver.receive(clock.now(), &mut ()) {
                Ok(Some(received_transfer)) => {
                    panic!(
                        "Received transfer when not subscribed: {:#?}",
                        received_transfer
                    );
                }
                Ok(None) => {
                    sleep(Duration::from_millis(10));
                }
                Err(e) => panic!("Receive error {:?}", e),
            }

            if Instant::now() > timeout {
                break;
            }
        }
    }
}

const MAJOR_GENERAL_SONG: &[u8] = br#"I am the very model of a modern Major-Gineral,
I've information vegetable, animal, and mineral,
I know the kings of England, and I quote the fights historical
From Marathon to Waterloo, in order categorical;
I'm very well acquainted, too, with matters mathematical,
I understand equations, both the simple and quadratical,
About binomial theorem I'm teeming with a lot o' news,
With many cheerful facts about the square of the hypotenuse.
I'm very good at integral and differential calculus;
I know the scientific names of beings animalculous:
In short, in matters vegetable, animal, and mineral,
I am the very model of a modern Major-Gineral.
I know our mythic history, King Arthur's and Sir Caradoc's;
I answer hard acrostics, I've a pretty taste for paradox,
I quote in elegiacs all the crimes of Heliogabalus,
In conics I can floor peculiarities parabolous;
I can tell undoubted Raphaels from Gerard Dows and Zoffanies,
I know the croaking chorus from The Frogs of Aristophanes!
Then I can hum a fugue of which I've heard the music's din afore,
And whistle all the airs from that infernal nonsense Pinafore.
Then I can write a washing bill in Babylonic cuneiform,
And tell you ev'ry detail of Caractacus's uniform:
In short, in matters vegetable, animal, and mineral,
I am the very model of a modern Major-Gineral.
In fact, when I know what is meant by "mamelon" and "ravelin",
When I can tell at sight a Mauser rifle from a javelin,
When such affairs as sorties and surprises I'm more wary at,
And when I know precisely what is meant by "commissariat",
When I have learnt what progress has been made in modern gunnery,
When I know more of tactics than a novice in a nunnery -
In short, when I've a smattering of elemental strategy -
You'll say a better Major-General has never sat a gee.
For my military knowledge, though I'm plucky and adventury,
Has only been brought down to the beginning of the century;
But still, in matters vegetable, animal, and mineral,
I am the very model of a modern Major-Gineral."#;

fn init_test_logging() {
    let _ = TermLogger::init(
        LevelFilter::Trace,
        Default::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    );
}
