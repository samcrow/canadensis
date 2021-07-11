//!
//! # Canadensis CRC writer
//!
//! This application modifies an ELF binary file that uses the `canadensis_crc` library. It
//! calculates the CRC of the binary and writes the CRC value so that the `canadensis_crc`
//! functions will return the correct value.
//!
//! ## Usage
//!
//! `canadensis_write_crc file-path`
//!

extern crate object;

use object::read::{File, Object};
use object::{ObjectSection, ObjectSymbol};
use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::{env, iter};

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Failed to add CRC: {}", e);
            process::exit(-1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let input_path = get_input_path();
    let mut binary: Vec<u8> = fs::read(&input_path)?;

    let offsets = find_file_offsets(&binary)?;

    // Set the CRC to 0 but CRC valid to true
    clear_crc(&mut binary, offsets.crc);
    write_crc_valid(&mut binary, offsets.crc_valid);

    // Calculate CRC of binary with CRC set to 0 but CRC valid set to true
    let crc = crc_64_we(&binary);
    println!("CRC = {:#x}", crc);

    write_crc(&mut binary, offsets.crc, crc);

    // Overwrite: Create a temporary file, then rename
    let temporary_path = input_path.with_extension("tmp");
    fs::write(&temporary_path, binary)?;
    fs::rename(temporary_path, input_path)?;

    Ok(())
}

fn clear_crc(binary: &mut [u8], offset: u64) {
    binary[offset as usize..][..8].copy_from_slice(&[0u8; 8]);
}

fn write_crc(binary: &mut [u8], offset: u64, crc: u64) {
    let crc_in_binary = &mut binary[offset as usize..][..8];
    let crc_bytes = crc.to_le_bytes();
    crc_in_binary.copy_from_slice(&crc_bytes);
}

fn write_crc_valid(binary: &mut [u8], offset: u64) {
    binary[offset as usize] = 1;
}

struct FileOffsets {
    pub crc: u64,
    pub crc_valid: u64,
}

fn find_file_offsets(binary: &[u8]) -> Result<FileOffsets, Box<dyn Error>> {
    let in_object = File::parse(binary)?;
    Ok(FileOffsets {
        crc: find_symbol_file_offset(&in_object, "CANADENSIS_CRC")?,
        crc_valid: find_symbol_file_offset(&in_object, "CANADENSIS_CRC_VALID")?,
    })
}

/// Finds a symbol with the provided name and returns the offset from the start of the binary
/// file to that symbol
fn find_symbol_file_offset(in_object: &File, name: &str) -> Result<u64, StringError> {
    for symbol in in_object.symbols() {
        if symbol.name() == Ok(name) {
            let section_index = match symbol.section_index() {
                Some(section_index) => section_index,
                None => return Err(StringError(format!("Symbol {} is not in a section", name))),
            };
            let section = match in_object.section_by_index(section_index) {
                Ok(section) => section,
                Err(e) => {
                    return Err(StringError(format!(
                        "Couldn't find section at index {} containing symbol {}: {}",
                        section_index.0, name, e
                    )));
                }
            };
            let file_range_start = match section.file_range() {
                Some((start, _end)) => start,
                None => {
                    return Err(StringError(format!(
                        "Section {} is not in the binary file",
                        section.name().unwrap_or("<unknown>")
                    )));
                }
            };
            return Ok(file_range_start + (symbol.address() - section.address()));
        }
    }
    Err(StringError(format!("Symbol {} not found", name)))
}

#[derive(Debug)]
struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for StringError {}

fn get_input_path() -> PathBuf {
    match env::args_os().skip(1).next() {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!("Usage: canadensis_write_crc binary-path");
            process::exit(-1);
        }
    }
}

/// Calculates a Wolfgang Ehrhardt 64-bit CRC on the provided data.
///
/// If the length of data is not a multiple of 8 bytes, it will be padded with zero bytes.
fn crc_64_we(data: &[u8]) -> u64 {
    let data = &*pad_to_8_bytes(data);
    let table = create_crc64_table();

    let mut crc = 0_u64;

    for byte in data {
        crc = (crc << 8) ^ table[(((crc >> 56) ^ *byte as u64) & 0x00000000000000FF_u64) as usize];
    }

    crc
}

fn pad_to_8_bytes(data: &[u8]) -> Cow<'_, [u8]> {
    let extra = data.len() % 8;
    if extra == 0 {
        Cow::Borrowed(data)
    } else {
        let to_add = 8 - extra;
        let mut data = data.to_vec();
        data.extend(iter::repeat(0u8).take(to_add));
        debug_assert!(data.len() % 8 == 0);
        Cow::Owned(data)
    }
}

const CRC_POLY_64: u64 = 0x42F0E1EBA9EA3693_u64;

fn create_crc64_table() -> [u64; 256] {
    let mut table = [0u64; 256];

    for (i, entry) in table.iter_mut().enumerate() {
        let mut crc = 0;
        let c = (i as u64) << 56;
        for _ in 0..8 {
            if ((crc ^ c) & 0x8000000000000000_u64) != 0 {
                crc = (crc << 1) ^ CRC_POLY_64;
            } else {
                crc <<= 1;
            }
        }
        *entry = crc;
    }

    table
}
