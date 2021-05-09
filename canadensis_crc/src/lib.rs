//!
//! Applications can use this library to access CRCs written by the `canadensis_write_crc` tool.
//!
//! # Steps (for ARM Cortex-M targets)
//!
//! * In the application, call the `get_crc` function
//! * Compile the application, generating a .elf file
//! * Run `canadensis_write_crc` on the .elf file to calculate and fill in the CRC
//!     * Caution: Don't use `cargo build` or `cargo run` at this stage. It will overwrite the
//!       binary and clear the CRC.
//! * Load the .elf file onto the target microcontroller and run it as usual
//!

#![no_std]

use core::ptr;

/// The CRC of this compiled image
///
/// This will be filled in by `canadensis_write_crc`
#[no_mangle]
static CANADENSIS_CRC: u64 = 0;
/// 1 if `CANADENSIS_CRC` is valid
///
/// This will be filled in by `canadensis_write_crc`
#[no_mangle]
static CANADENSIS_CRC_VALID: u8 = 0;

pub fn get_crc() -> Option<u64> {
    // If these are not volatile reads, the compiler will optimize them away and the symbols
    // will be missing from the binary.
    let valid = unsafe { ptr::read_volatile(&CANADENSIS_CRC_VALID) };
    if valid == 1 {
        let crc = unsafe { ptr::read_volatile(&CANADENSIS_CRC) };
        Some(crc)
    } else {
        None
    }
}
