extern crate canadensis_core;
extern crate canadensis_linux;
extern crate canadensis_udp;
extern crate simplelog;

use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger};
use std::convert::{TryFrom, TryInto};
use std::net::Ipv4Addr;
use std::thread;
use std::time::Duration;

use canadensis_core::session::SessionDynamicMap;
use canadensis_core::time::{Clock, MicrosecondDuration64, Microseconds64};
use canadensis_core::transport::Receiver;
use canadensis_linux::SystemClock;
use canadensis_udp::{Error, NodeAddress, UdpNodeId, UdpReceiver, UdpSessionData, UdpTransferId};

fn main() {
    TermLogger::init(
        LevelFilter::Warn,
        Default::default(),
        Default::default(),
        ColorChoice::Auto,
    )
    .unwrap();

    let address = NodeAddress::try_from(Ipv4Addr::new(127, 0, 0, 121)).unwrap();
    println!(
        "This node's IP address: {}",
        Ipv4Addr::from(address.clone())
    );
    let mut clock = SystemClock::new();

    // Note: This MTU includes space for the header
    const MTU: usize = 1300;
    let mut receiver = UdpReceiver::<
        Microseconds64,
        SessionDynamicMap<Microseconds64, UdpNodeId, UdpTransferId, UdpSessionData>,
        MTU,
    >::new(address);
    receiver
        .subscribe_message(
            73.try_into().unwrap(),
            4096,
            MicrosecondDuration64::new(2_000_000),
        )
        .unwrap();

    // Instead of a real asynchronous IO system, just poll periodically
    loop {
        match receiver.accept(clock.now()) {
            Ok(Some(transfer)) => {
                println!("{:?}", transfer);
            }
            Ok(None) => {
                // Try again immediately
            }
            Err(Error::Socket(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
