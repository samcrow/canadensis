//!
//! Attempts to compile various invalid DSDL packages. If any of them can be compiled successfully,
//! this test fails.
//!

extern crate canadensis_dsdl_frontend;

use canadensis_dsdl_frontend::compiled::package::CompiledPackage;
use canadensis_dsdl_frontend::{Error, Package};
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
            Ok(_) => failed_tests.push(case_name),
            Err(_) => {}
        }
    }

    if failed_tests.is_empty() {
        Ok(())
    } else {
        panic!("Failed these compile-fail cases: {:#?}", failed_tests)
    }
}

fn try_compile_package(path: &Path) -> Result<CompiledPackage, Error> {
    let mut package = Package::new();
    package.add_files(path)?;
    package.compile()
}

/// Tests a file with a combined path and name that are 256 characters long, which is longer
/// than the maximum allowed
///
/// This is not just a file (like all the other compile-fail cases) because some Windows software
/// can't handle long paths.
#[test]
fn compile_fail_long_name() -> io::Result<()> {
    let target_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let test_dir = target_dir.join("compile_fail_long_name");
    // The full name of the DSDL type is canadensis.long_long.TwoHundred...
    let long_name_subdir = test_dir.join("canadensis/long_long");
    fs::create_dir_all(&long_name_subdir)?;
    let long_name_file_path = long_name_subdir.join("TwoHundredAndFiftySixCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong.1.0.uavcan");
    write_long_name_file(&long_name_file_path)?;

    match try_compile_package(&test_dir) {
        Ok(_) => {
            panic!("Failed long name compile-fail case");
        }
        Err(e) => {
            assert!(matches!(e, Error::TypeNameLength { .. }));
            Ok(())
        }
    }
}

fn write_long_name_file(path: &Path) -> io::Result<()> {
    fs::write(
        path,
        "# The path and name of this type are 256 characters long, which is too long.
@sealed
",
    )
}
