//!
//! A command-line tool for optimizing CAN filters
//!
//! Usage: optimize_filters max_filters message_id...
//!
//! Provide the maximum number of filters and one or more message IDs to match. Message IDs must be
//! in hexadecimal format.
//!

extern crate canadensis_filter_config;

use canadensis_filter_config::Filter;
use std::env;
use std::error::Error;
use std::process;
use std::str;

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(-1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = get_args()?;
    let mut filters: Vec<Filter> = args
        .target_ids
        .into_iter()
        .map(Filter::exact_match)
        .collect();
    let optimized_filters = canadensis_filter_config::optimize(&mut filters, args.max_filters);

    print_filters(&optimized_filters);

    Ok(())
}

fn print_filters(filters: &[Filter]) {
    println!("ID         | Mask");
    for filter in filters {
        println!("{:#010x}   {:#010x}", filter.id(), filter.mask());
    }
}

struct Args {
    max_filters: usize,
    target_ids: Vec<u32>,
}
fn get_args() -> Result<Args, Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let max_filters = args.next().ok_or(UsageError)?.parse()?;
    let target_ids = args
        .map(|arg| parse_hex_with_prefix(&arg))
        .collect::<Result<Vec<u32>, Box<dyn Error>>>()?;
    // Check that IDs are acceptable
    for id in &target_ids {
        if (*id & !0x1fff_ffff) != 0 {
            return Err(CanIdError.into());
        }
    }
    Ok(Args {
        max_filters,
        target_ids,
    })
}

#[derive(Debug)]
struct UsageError;

impl std::fmt::Display for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Usage: optimize_filters max_filters message_id...")
    }
}

impl Error for UsageError {}

#[derive(Debug)]
struct CanIdError;

impl std::fmt::Display for CanIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A CAN ID must fit into 29 bits")
    }
}

impl Error for CanIdError {}

fn parse_hex_with_prefix(input: &str) -> Result<u32, Box<dyn Error>> {
    let bytes = input.as_bytes();
    if bytes.len() < 3 {
        return Err(NumberFormatError.into());
    }
    let (prefix, digits) = bytes.split_at(2);
    if prefix != b"0x" {
        return Err(NumberFormatError.into());
    }
    let digits_str = str::from_utf8(digits)?;
    let number = u32::from_str_radix(digits_str, 16)?;
    Ok(number)
}

#[derive(Debug)]
struct NumberFormatError;

impl std::fmt::Display for NumberFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid number format, expected a hexadecimal number with the prefix 0x"
        )
    }
}

impl Error for NumberFormatError {}
