//!
//! An anonymous node that monitors for uavcan.si.unit.electric_current.Scalar.1.0 messages and
//! prints them in CSV format
//!
//! Usage: diagnostic_console CAN-interface-name subject-ID
//!

extern crate canadensis;
extern crate canadensis_data_types;
extern crate socketcan;

use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::process;
use std::time::Instant;

use canadensis_can::{CanId, Frame, Mtu, Receiver};
use canadensis_core::time::{Clock, MicrosecondDuration64, Microseconds64};
use canadensis_core::SubjectId;
use canadensis_encoding::{DataType, Deserialize, DeserializeError, Message, ReadCursor};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let interface = args.next().unwrap_or_else(|| {
        eprintln!("Expected a SocketCAN interface name");
        process::exit(-1);
    });
    let subject: SubjectId = args
        .next()
        .and_then(|arg| arg.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("Expected a subject ID");
            process::exit(-1);
        });
    let can = socketcan::CANSocket::open(&interface)?;

    let mut clock = SystemClock::new();
    let mut receiver = Receiver::new_anonymous(Mtu::Can8);
    receiver
        .subscribe_message(subject, 7, MicrosecondDuration64::new(1_000_000))
        .unwrap();

    // Print headers
    println!("Microseconds,Amps");

    loop {
        let frame = can.read_frame()?;
        // Convert from SocketCAN to Canadensis
        let frame = Frame::new(
            clock.now(),
            CanId::try_from(frame.id()).unwrap(),
            frame.data(),
        );
        let frame_time = frame.timestamp();
        if let Some(transfer) = receiver.accept(frame).unwrap() {
            match F32Message::deserialize(&mut ReadCursor::new(&transfer.payload)) {
                Ok(message) => {
                    println!("{},{}", frame_time.as_microseconds(), message.0);
                }
                Err(e) => eprintln!("Couldn't deserialize: {:?}", e),
            }
        }
    }
}

#[derive(Clone)]
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
    type Instant = Microseconds64;

    fn now(&mut self) -> Self::Instant {
        let since_start = Instant::now().duration_since(self.start_time);
        let microseconds = since_start.as_micros();
        Microseconds64::new(microseconds as u64)
    }
}

/// A message containing a single 32-bit float field
///
/// This can act as a uavcan.si.unit.electric_current.Scalar.1.0 message, or various other message
/// types.
pub struct F32Message(pub f32);

impl Message for F32Message {}

impl DataType for F32Message {
    const EXTENT_BYTES: Option<u32> = None;
}

impl Deserialize for F32Message {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 32
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.0 = cursor.read_aligned_f32();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = F32Message(0.0);
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
