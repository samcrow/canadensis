//!
//! Reads a DSDL file from standard input, parses it, and writes the resulting abstract
//! syntax tree to standard output
//!

extern crate canadensis_dsdl_parser;

use canadensis_dsdl_parser::Config;
use std::error::Error;
use std::io;
use std::io::Read;
use std::process;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(-1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut dsdl = String::new();
    stdin.read_to_string(&mut dsdl)?;

    let config = Config::default();
    let ast = canadensis_dsdl_parser::parse(&dsdl, &config)?;

    println!("{:#?}", ast);

    Ok(())
}
