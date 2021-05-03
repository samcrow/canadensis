//!
//! An anonymous node that monitors for uavcan.node.Diagnostic.1.0 messages and prints them out
//!
//! Usage: diagnostic_console CAN-interface-name
//!

extern crate canadensis;
extern crate canadensis_data_types;
extern crate socketcan;

use canadensis::time::{PrimitiveDuration, PrimitiveInstant};
use canadensis::CanId;
use canadensis::{Clock, DataType, NodeId};
use canadensis_can::Receiver;

use canadensis_data_types::uavcan::diagnostic::record::Record;
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::process;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let interface = env::args().skip(1).next().unwrap_or_else(|| {
        eprintln!("Expected a SocketCAN interface name");
        process::exit(-1);
    });
    let can = socketcan::CANSocket::open(&interface)?;

    let mut clock = SystemClock::new();
    let mut receiver = Receiver::new(NodeId::MAX);
    receiver
        .subscribe_message(
            Record::SUBJECT,
            Record::EXTENT_BYTES.unwrap() as usize,
            PrimitiveDuration::new(10_000_000),
        )
        .unwrap();

    loop {
        let frame = can.read_frame()?;
        // Convert from SocketCAN to Canadensis
        let frame = canadensis::Frame::new(
            clock.now(),
            CanId::try_from(frame.id()).unwrap(),
            frame.data(),
        );
        if let Some(transfer) = receiver.accept(frame).unwrap() {
            // TODO: Better printing
            println!("{:?}", transfer);
        }
    }
}

#[derive(Debug, Clone)]
struct SystemClock {
    start_time: Instant,
}

impl SystemClock {
    pub fn new() -> Self {
        SystemClock {
            start_time: Instant::now(),
        }
    }
}

impl Clock for SystemClock {
    type Instant = PrimitiveInstant<u64>;

    fn now(&mut self) -> Self::Instant {
        let since_start = Instant::now().duration_since(self.start_time);
        let microseconds = since_start.as_micros();
        PrimitiveInstant::new(microseconds as u64)
    }
}
