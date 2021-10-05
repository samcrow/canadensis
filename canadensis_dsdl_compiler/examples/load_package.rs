extern crate canadensis_dsdl_compiler;
use canadensis_dsdl_compiler::package::{Error, Package};
use std::env;
use std::error::Error as _;
use std::process;

fn main() {
    if let Err(e) = run() {
        print_error(e);
        process::exit(-1);
    }
}

fn print_error<E>(error: E)
where
    E: std::error::Error,
{
    eprintln!("{}", error);
    if let Some(source) = error.source() {
        print_error(source);
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
