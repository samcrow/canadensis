//!
//! Attempts to generate code from various invalid DSDL packages. If any of them succeeds,
//! this test fails.
//!

extern crate canadensis_codegen_rust;
extern crate canadensis_dsdl_frontend;

use canadensis_dsdl_frontend::compiled::package::CompiledPackage;
use canadensis_dsdl_frontend::{Config, Package};
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[test]
fn compile_fail() -> io::Result<()> {
    let package_holder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/compile_fail");

    let mut failed_tests: Vec<OsString> = Vec::new();

    for entry in fs::read_dir(package_holder)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let case_name = entry.file_name();

        eprintln!("Compile fail: Checking folder {}", entry.path().display());
        match try_compile_and_generate_code(&entry.path()) {
            Ok(_) => failed_tests.push(case_name),
            // Error should be a codegen error, not a parsing or DSDL frontend error
            Err(e) => match e.downcast_ref::<canadensis_codegen_rust::Error>() {
                Some(_) => { /* OK */ }
                None => {
                    panic!("Unexpected error {:?}, expected a codegen error", e);
                }
            },
        }
    }

    if failed_tests.is_empty() {
        Ok(())
    } else {
        panic!("Failed these compile-fail cases: {:#?}", failed_tests)
    }
}

fn try_compile_package(path: &Path) -> Result<CompiledPackage, canadensis_dsdl_frontend::Error> {
    let mut package = Package::new();
    package.add_files(path)?;
    let config = Config::default();
    package.compile(&config)
}

fn try_compile_and_generate_code(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let package = try_compile_package(path)?;
    let generated = canadensis_codegen_rust::generate_code(&package, &Default::default())?;
    let mut sink = std::io::sink();
    write!(sink, "{}", generated)?;
    Ok(())
}
