extern crate canadensis_codegen_rust;
extern crate canadensis_dsdl_frontend;

use canadensis_dsdl_frontend::Package;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut package = Package::new();
    for path in env::args_os().skip(1) {
        package.add_files(path)?;
    }
    let package = package.compile()?;

    canadensis_codegen_rust::generate_code(&package);

    Ok(())
}
