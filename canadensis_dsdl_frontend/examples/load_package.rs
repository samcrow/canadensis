extern crate canadensis_dsdl_frontend;
use canadensis_dsdl_frontend::compiled::DsdlKind;
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

    for (key, compiled_dsdl) in compiled {
        println!("{}:", key);
        println!("{:#?}", compiled_dsdl);
        match compiled_dsdl.kind {
            DsdlKind::Message(message) => {
                println!("Message bit length {:?}", message.bit_length().expand())
            }
            DsdlKind::Service { request, response } => {
                println!("Request bit length {:?}", request.bit_length().expand());
                println!("Response bit length {:?}", response.bit_length().expand());
            }
        }
    }

    Ok(())
}
