//!
//! An anonymous node that monitors for uavcan.node.Diagnostic.1.0 messages and prints them out
//!
//! Usage: diagnostic_console CAN-interface-name
//!

extern crate canadensis;
extern crate canadensis_data_types;
extern crate socketcan;

use socketcan::Socket;
use std::env;
use std::error::Error;
use std::process;

use canadensis::core::transport::Receiver;
use canadensis::encoding::{DataType, Deserialize, ReadCursor};
use canadensis_can::{CanReceiver, Mtu};
use canadensis_core::time::MicrosecondDuration32;
use canadensis_data_types::uavcan::diagnostic::record_1_1::{self, Record};
use canadensis_data_types::uavcan::diagnostic::severity_1_0::Severity;
use canadensis_linux::{LinuxCan, SystemClock};

fn main() -> Result<(), Box<dyn Error>> {
    let interface = env::args().skip(1).next().unwrap_or_else(|| {
        eprintln!("Expected a SocketCAN interface name");
        process::exit(-1);
    });
    let can = socketcan::CanSocket::open(&interface)?;
    let mut can = LinuxCan::new(can);

    let mut clock = SystemClock::new();
    let mut receiver = CanReceiver::new_anonymous(Mtu::Can8);
    receiver
        .subscribe_message(
            record_1_1::SUBJECT,
            Record::EXTENT_BYTES.unwrap() as usize,
            MicrosecondDuration32::from_ticks(1_000_000),
            &mut can,
        )
        .unwrap();

    loop {
        match receiver.receive(&mut clock, &mut can) {
            Ok(Some(transfer)) => {
                match Record::deserialize(&mut ReadCursor::new(&transfer.payload)) {
                    Ok(log_record) => {
                        let node_text = transfer
                            .header
                            .source()
                            .map(|node| node.to_string())
                            .unwrap_or_else(|| "?".to_owned());
                        let level_text = match log_record.severity.value {
                            Severity::TRACE => 'T',
                            Severity::DEBUG => 'D',
                            Severity::INFO => 'I',
                            Severity::NOTICE => 'N',
                            Severity::WARNING => 'W',
                            Severity::ERROR => 'E',
                            Severity::CRITICAL => 'C',
                            Severity::ALERT => 'A',
                            _ => '?',
                        };
                        let text = String::from_utf8_lossy(&log_record.text);

                        println!(
                            "[{node}][{level}] {text}",
                            node = node_text,
                            level = level_text,
                            text = text
                        );
                    }
                    Err(e) => eprintln!("Couldn't deserialize log record: {:?}", e),
                }
            }
            Ok(None) => {}
            Err(e) => panic!("{:?}", e),
        }
    }
}
