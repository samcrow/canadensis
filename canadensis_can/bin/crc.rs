//!
//! A command-line tool to calculate the CRC of one or more bytes
//!
//! Usage: canadensis_crc byte...
//!
//! Each byte must be in hexadecimal, but without the 0x prefix.
//!

extern crate canadensis_can;
use canadensis_can::TransferCrc;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut crc = TransferCrc::new();
    for byte in env::args().skip(1).map(|arg| u8::from_str_radix(&arg, 16)) {
        let byte = byte?;
        crc.add(byte);
    }
    println!("{:#06x}", crc.get());

    Ok(())
}
