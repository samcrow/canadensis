extern crate canadensis_dsdl_compiler;
use canadensis_dsdl_compiler::package::{Error, Package};
use std::env;
use std::error::Error as _;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        if let Some(mut source) = e.source() {
            eprintln!("Caused by: {}", source);
            while let Some(new_source) = source.source() {
                source = new_source;
                eprintln!("Caused by: {}", source);
            }
        }
        process::exit(-1);
    }
}

fn run() -> Result<(), Error> {
    let path = env::args_os().skip(1).next().expect("No path provided");

    let mut package = Package::new();
    package.add_files(path)?;

    let output = package.compile()?;
    println!("{:#?}", output);

    Ok(())
}
