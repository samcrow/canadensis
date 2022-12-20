//! Checks that this library can compile the Cyphal public regulated data types, Nunavut test
//! types, and a few additional Canadensis test types

extern crate canadensis_codegen_rust;
extern crate canadensis_dsdl_frontend;

use std::path::{Path, PathBuf};

use canadensis_dsdl_frontend::compiled::package::CompiledPackage;
use canadensis_dsdl_frontend::Package;

#[test]
fn compile_pass_combined() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/compile_pass");
    try_compile_and_generate_code(&path)?;
    Ok(())
}

fn try_compile_package(path: &Path) -> Result<CompiledPackage, canadensis_dsdl_frontend::Error> {
    let mut package = Package::new();
    package.add_files(path)?;
    package.compile()
}

fn try_compile_and_generate_code(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let package = try_compile_package(path)?;
    let generated = canadensis_codegen_rust::generate_code(&package, &Default::default())?;
    let mut sink = std::io::sink();
    write!(sink, "{}", generated)?;
    Ok(())
}
