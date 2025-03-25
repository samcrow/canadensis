extern crate canadensis_core;
extern crate canadensis_linux;
extern crate canadensis_udp;
extern crate simplelog;

use std::convert::{TryFrom, TryInto};
use std::io::Write;
use std::thread;
use std::time::Duration;

use core::net::Ipv4Addr;
use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger};
use zerocopy::AsBytes;

use canadensis_core::session::SessionDynamicMap;
use canadensis_core::time::MicrosecondDuration32;
use canadensis_core::transport::Receiver;
use canadensis_linux::SystemClock;
use canadensis_udp::driver::StdUdpSocket;
use canadensis_udp::{UdpNodeId, UdpReceiver, UdpSessionData, UdpTransferId, DEFAULT_PORT};

fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        Default::default(),
        Default::default(),
        ColorChoice::Auto,
    )
    .unwrap();

    let local_node_id = UdpNodeId::try_from(121).unwrap();
    let mut clock = SystemClock::new();
    // For loopback multicast to work, the receive socket needs to bind to the unspecified address
    let mut socket = StdUdpSocket::bind(Ipv4Addr::UNSPECIFIED, DEFAULT_PORT).unwrap();

    // Note: This MTU includes space for the header
    const MTU: usize = 1472;
    let mut receiver = UdpReceiver::<
        SystemClock,
        SessionDynamicMap<UdpNodeId, UdpTransferId, UdpSessionData>,
        StdUdpSocket,
        MTU,
    >::new(Some(local_node_id), Ipv4Addr::LOCALHOST);
    receiver
        .subscribe_message(
            73.try_into().unwrap(),
            4096,
            MicrosecondDuration32::from_ticks(2_000_000),
            &mut socket,
        )
        .unwrap();

    // Instead of a real asynchronous IO system, just poll periodically
    loop {
        match receiver.receive(&mut clock, &mut socket) {
            Ok(Some(transfer)) => {
                println!("{:#?}", transfer.header);
                for byte in transfer.payload.as_bytes() {
                    if byte.is_ascii() {
                        std::io::stdout().write(std::slice::from_ref(byte)).unwrap();
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            Ok(None) => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
