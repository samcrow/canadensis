extern crate canadensis_dsdl_frontend;
use canadensis_dsdl_frontend::{Error, Package};
use std::env;
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
    let paths = env::args_os().skip(1);

    let mut package = Package::new();
    for path in paths {
        package.add_files(path)?;
    }

    let compiled = package.compile()?;

    if let Some((key, compiled_dsdl)) = compiled.iter().next() {
        println!("{}:", key);
        println!("{:#?}", compiled_dsdl);
    }

    Ok(())
}
