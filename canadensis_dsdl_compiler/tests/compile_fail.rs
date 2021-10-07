//!
//! Attempts to compile various invalid DSDL packages. If any of them can be compiled successfully,
//! this test fails.
//!

extern crate canadensis_dsdl_compiler;

use canadensis_dsdl_compiler::{Error, Package};
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

        match try_compile_package(&entry.path()) {
            Ok(()) => failed_tests.push(case_name),
            Err(_) => {}
        }
    }

    if failed_tests.is_empty() {
        Ok(())
    } else {
        panic!("Failed these compile-fail cases: {:#?}", failed_tests)
    }
}

fn try_compile_package(path: &Path) -> Result<(), Error> {
    let mut package = Package::new();
    package.add_files(path)?;
    package.compile()?;
    Ok(())
}
