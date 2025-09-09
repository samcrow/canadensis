extern crate canadensis_core;
extern crate canadensis_udp;

mod utils;

use crate::utils::init_test_logging;
use canadensis_core::session::SessionDynamicMap;
use canadensis_core::time::{milliseconds, Clock, Microseconds32};
use canadensis_core::transfer::{Header, MessageHeader, Transfer};
use canadensis_core::transport::Receiver;
use canadensis_core::{Priority, SubjectId};
use canadensis_udp::driver::{StdUdpSocket, UdpSocket};
use canadensis_udp::{UdpNodeId, UdpReceiver, UdpSessionData, UdpTransferId};
use std::convert::TryFrom;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};

const VALID_THREE_FRAME_TRANSFER: [&[u8]; 3] = [
    &[
        0x01, // Version
        0x06, // Priority
        0xe3, 0xf0, // Source node ID
        0xff, 0xff, // Destination node ID
        0x44, 0x04, // Subject ID
        0xfb, 0xe7, 0x48, 0x32, 0xdf, 0xa7, 0xa8, 0xfa, // Transfer ID
        0x00, 0x00, 0x00, 0x00, // Frame index and end of transfer
        0x00, 0x00, // User data
        0xc9, 0x72, // Header CRC
        0xe3, 0x39, 0x5a, 0xbe, 0x93, 0xa0, 0x00, 0x92, // Data
    ],
    &[
        0x01, // Version
        0x06, // Priority
        0xe3, 0xf0, // Source node ID
        0xff, 0xff, // Destination node ID
        0x44, 0x04, // Subject ID
        0xfb, 0xe7, 0x48, 0x32, 0xdf, 0xa7, 0xa8, 0xfa, // Transfer ID
        0x01, 0x00, 0x00, 0x00, // Frame index and end of transfer
        0x00, 0x00, // User data
        0x8c, 0xd2, // Header CRC
        0x92, 0xff, 0x00, 0x00, 0x01, // More data
    ],
    &[
        0x01, // Version
        0x06, // Priority
        0xe3, 0xf0, // Source node ID
        0xff, 0xff, // Destination node ID
        0x44, 0x04, // Subject ID
        0xfb, 0xe7, 0x48, 0x32, 0xdf, 0xa7, 0xa8, 0xfa, // Transfer ID
        0x02, 0x00, 0x00, 0x80, // Frame index and end of transfer
        0x00, 0x00, // User data
        0x79, 0x68, // Header CRC
        0x6f, 0x77, 0x6f, // More data
        0xad, 0xb3, 0xf1, 0xbf, // Transfer CRC
    ],
];
const VALID_THREE_FRAME_TRANSFER_PAYLOAD: [u8; 16] = [
    0xe3, 0x39, 0x5a, 0xbe, 0x93, 0xa0, 0x00, 0x92, 0x92, 0xff, 0x00, 0x00, 0x01, 0x6f, 0x77, 0x6f,
];

#[test]
fn receive_payload_too_large_single_frame() -> Result<(), Box<dyn Error>> {
    init_test_logging();
    let mut rx: UdpReceiver<
        StubClock,
        SessionDynamicMap<UdpNodeId, UdpTransferId, UdpSessionData>,
        StdUdpSocket,
        1472,
    > = UdpReceiver::new(Some(UdpNodeId::try_from(39).unwrap()), Ipv4Addr::LOCALHOST);

    // Loopback using two UDP sockets
    // Use OS-assigned ephemeral ports.
    let mut transmit_socket = StdUdpSocket::bind(Ipv4Addr::LOCALHOST, 0).unwrap();
    let mut receive_socket = StdUdpSocket::bind(Ipv4Addr::UNSPECIFIED, 0).unwrap();
    let receive_port = receive_socket.local_addr()?.port();
    let loopback_destination = SocketAddrV4::new(Ipv4Addr::LOCALHOST, receive_port);

    let subject = SubjectId::try_from(1092).unwrap();
    rx.subscribe_message(subject, 5, milliseconds(1000), &mut receive_socket)
        .unwrap();

    let frame = [
        0x01, // Version
        0x06, // Priority
        0xe3, 0xf0, // Source node ID
        0xff, 0xff, // Destination node ID
        0x44, 0x04, // Subject ID
        0xfb, 0xe7, 0x48, 0x32, 0xdf, 0xa7, 0xa8, 0xfa, // Transfer ID
        0x00, 0x00, 0x00, 0x80, // Frame index and end of transfer
        0x00, 0x00, // User data
        0xf2, 0x28, // Header CRC
        0xe3, 0x39, 0x5a, 0xbe, 0x93, 0xa0, 0x00, 0x92, // Data
        0x88, 0xec, 0xff, 0xe9, // Transfer CRC
    ];
    transmit_socket.send_to(&frame, loopback_destination)?;

    let mut clock = StubClock::default();
    clock.set_ticks(109932);
    let transfer = rx.receive(&mut clock, &mut receive_socket).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: Microseconds32::from_ticks(109932),
                transfer_id: UdpTransferId::try_from(0xfaa8a7df3248e7fb).unwrap(),
                priority: Priority::Slow,
                subject,
                source: Some(UdpNodeId::try_from(0xf0e3).unwrap())
            }),
            loopback: false,
            payload: vec![0xe3, 0x39, 0x5a, 0xbe, 0x93],
        })
    );
    Ok(())
}

#[test]
fn receive_payload_too_large_three_frames() -> Result<(), Box<dyn Error>> {
    init_test_logging();
    let mut rx: UdpReceiver<
        StubClock,
        SessionDynamicMap<UdpNodeId, UdpTransferId, UdpSessionData>,
        StdUdpSocket,
        1472,
    > = UdpReceiver::new(Some(UdpNodeId::try_from(39).unwrap()), Ipv4Addr::LOCALHOST);

    // Loopback using two UDP sockets
    // Use OS-assigned ephemeral ports.
    let mut transmit_socket = StdUdpSocket::bind(Ipv4Addr::LOCALHOST, 0).unwrap();
    let mut receive_socket = StdUdpSocket::bind(Ipv4Addr::UNSPECIFIED, 0).unwrap();
    let receive_port = receive_socket.local_addr()?.port();
    let loopback_destination = SocketAddrV4::new(Ipv4Addr::LOCALHOST, receive_port);

    let subject = SubjectId::try_from(1092).unwrap();
    rx.subscribe_message(subject, 10, milliseconds(1000), &mut receive_socket)
        .unwrap();

    for frame in VALID_THREE_FRAME_TRANSFER {
        transmit_socket.send_to(frame, loopback_destination)?;
    }

    let mut clock = StubClock::default();
    clock.set_ticks(109932);
    let transfer = rx.receive(&mut clock, &mut receive_socket).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: Microseconds32::from_ticks(109932),
                transfer_id: UdpTransferId::try_from(0xfaa8a7df3248e7fb).unwrap(),
                priority: Priority::Slow,
                subject,
                source: Some(UdpNodeId::try_from(0xf0e3).unwrap())
            }),
            loopback: false,
            payload: VALID_THREE_FRAME_TRANSFER_PAYLOAD[..10].to_vec(),
        })
    );
    Ok(())
}

#[test]
fn multi_frame_no_timeout() -> Result<(), Box<dyn Error>> {
    init_test_logging();
    let mut rx: UdpReceiver<
        StubClock,
        SessionDynamicMap<UdpNodeId, UdpTransferId, UdpSessionData>,
        StdUdpSocket,
        1472,
    > = UdpReceiver::new(Some(UdpNodeId::try_from(39).unwrap()), Ipv4Addr::LOCALHOST);

    // Loopback using two UDP sockets
    // Use OS-assigned ephemeral ports.
    let mut transmit_socket = StdUdpSocket::bind(Ipv4Addr::LOCALHOST, 0).unwrap();
    let mut receive_socket = StdUdpSocket::bind(Ipv4Addr::UNSPECIFIED, 0).unwrap();
    let receive_port = receive_socket.local_addr()?.port();
    let loopback_destination = SocketAddrV4::new(Ipv4Addr::LOCALHOST, receive_port);

    let subject = SubjectId::try_from(1092).unwrap();
    rx.subscribe_message(subject, 64, milliseconds(1000), &mut receive_socket)
        .unwrap();

    // Time between frames 0 and 2 is greater than the transfer ID timeout. Even with a long delay,
    // we should still get the transfer.
    let mut clock = StubClock::default();
    clock.set_ticks(100 * 1000);
    transmit_socket.send_to(VALID_THREE_FRAME_TRANSFER[0], loopback_destination)?;
    assert_eq!(None, rx.receive(&mut clock, &mut receive_socket).unwrap());
    clock.set_ticks(600 * 1000);
    transmit_socket.send_to(VALID_THREE_FRAME_TRANSFER[1], loopback_destination)?;
    assert_eq!(None, rx.receive(&mut clock, &mut receive_socket).unwrap());
    clock.set_ticks(39101 * 1000);
    transmit_socket.send_to(VALID_THREE_FRAME_TRANSFER[2], loopback_destination)?;
    let transfer = rx.receive(&mut clock, &mut receive_socket).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                timestamp: Microseconds32::from_ticks(100 * 1000),
                transfer_id: UdpTransferId::try_from(0xfaa8a7df3248e7fb).unwrap(),
                priority: Priority::Slow,
                subject,
                source: Some(UdpNodeId::try_from(0xf0e3).unwrap())
            }),
            loopback: false,
            payload: VALID_THREE_FRAME_TRANSFER_PAYLOAD.to_vec(),
        })
    );
    Ok(())
}

#[test]
fn multi_frame_duplicate() -> Result<(), Box<dyn Error>> {
    init_test_logging();
    let mut rx: UdpReceiver<
        StubClock,
        SessionDynamicMap<UdpNodeId, UdpTransferId, UdpSessionData>,
        StdUdpSocket,
        1472,
    > = UdpReceiver::new(Some(UdpNodeId::try_from(39).unwrap()), Ipv4Addr::LOCALHOST);

    // Loopback using two UDP sockets
    // Use OS-assigned ephemeral ports.
    let mut transmit_socket = StdUdpSocket::bind(Ipv4Addr::LOCALHOST, 0).unwrap();
    let mut receive_socket = StdUdpSocket::bind(Ipv4Addr::UNSPECIFIED, 0).unwrap();
    let receive_port = receive_socket.local_addr()?.port();
    let loopback_destination = SocketAddrV4::new(Ipv4Addr::LOCALHOST, receive_port);

    let subject = SubjectId::try_from(1092).unwrap();
    rx.subscribe_message(subject, 64, milliseconds(1000), &mut receive_socket)
        .unwrap();

    // Send frames at different times
    let mut clock = StubClock::default();
    clock.set_ticks(100 * 1000);
    transmit_socket.send_to(VALID_THREE_FRAME_TRANSFER[0], loopback_destination)?;
    assert_eq!(None, rx.receive(&mut clock, &mut receive_socket).unwrap());
    clock.set_ticks(200 * 1000);
    transmit_socket.send_to(VALID_THREE_FRAME_TRANSFER[1], loopback_destination)?;
    assert_eq!(None, rx.receive(&mut clock, &mut receive_socket).unwrap());
    clock.set_ticks(300 * 1000);
    transmit_socket.send_to(VALID_THREE_FRAME_TRANSFER[2], loopback_destination)?;
    let transfer = rx.receive(&mut clock, &mut receive_socket).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                // Timestamp matches the timestamp of the first frame
                timestamp: Microseconds32::from_ticks(100 * 1000),
                transfer_id: UdpTransferId::try_from(0xfaa8a7df3248e7fb).unwrap(),
                priority: Priority::Slow,
                subject,
                source: Some(UdpNodeId::try_from(0xf0e3).unwrap())
            }),
            loopback: false,
            payload: VALID_THREE_FRAME_TRANSFER_PAYLOAD.to_vec(),
        })
    );

    // Send the same frames again within the transfer ID timeout
    clock.set_ticks(1100 * 1000);
    for frame in VALID_THREE_FRAME_TRANSFER {
        transmit_socket.send_to(frame, loopback_destination)?;
    }
    assert_eq!(None, rx.receive(&mut clock, &mut receive_socket).unwrap());

    // After the timeout expires, we should get another copy of the transfer
    clock.set_ticks(1100 * 1000 + 1);
    for frame in VALID_THREE_FRAME_TRANSFER {
        transmit_socket.send_to(frame, loopback_destination)?;
    }
    let transfer = rx.receive(&mut clock, &mut receive_socket).unwrap();
    assert_eq!(
        transfer,
        Some(Transfer {
            header: Header::Message(MessageHeader {
                // Timestamp matches the timestamp of the first frame
                timestamp: Microseconds32::from_ticks(1100 * 1000 + 1),
                transfer_id: UdpTransferId::try_from(0xfaa8a7df3248e7fb).unwrap(),
                priority: Priority::Slow,
                subject,
                source: Some(UdpNodeId::try_from(0xf0e3).unwrap())
            }),
            loopback: false,
            payload: VALID_THREE_FRAME_TRANSFER_PAYLOAD.to_vec(),
        })
    );

    Ok(())
}

#[derive(Default)]
struct StubClock {
    ticks: u32,
}

impl StubClock {
    pub fn set_ticks(&mut self, ticks: u32) {
        self.ticks = ticks
    }
}

impl Clock for StubClock {
    fn now(&mut self) -> Microseconds32 {
        Microseconds32::from_ticks(self.ticks)
    }
}
