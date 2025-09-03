//!
//! Attempts to compile various invalid DSDL packages. If any of them can be compiled successfully,
//! this test fails.
//!

extern crate canadensis_dsdl_frontend;
extern crate regex;

use canadensis_dsdl_frontend::compiled::package::CompiledPackage;
use canadensis_dsdl_frontend::{Error, Package};
use regex::Regex;
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
        println!("==== Compile-fail case {:?} ====", case_name);

        match try_compile_package(&entry.path()) {
            Ok(_) => failed_tests.push(case_name),
            Err(e) => {
                if !check_error_message(&entry.path(), &e) {
                    failed_tests.push(case_name)
                }
            }
        }
    }

    if failed_tests.is_empty() {
        Ok(())
    } else {
        panic!("Failed these compile-fail cases: {:#?}", failed_tests)
    }
}

fn try_compile_package(path: &Path) -> Result<CompiledPackage, Box<Error>> {
    let mut package = Package::new();
    package.add_files(path)?;
    package.compile()
}

struct PrintCause<'a, E>(&'a E);
impl<E> std::fmt::Display for PrintCause<'_, E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)?;
        if let Some(source) = self.0.source() {
            f.write_str("\ncaused by: ")?;
            std::fmt::Display::fmt(&PrintCause(&source), f)?;
        }
        Ok(())
    }
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
            assert!(matches!(&*e, Error::TypeNameLength { .. }));
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

/// Checks that the displayed form of an error (or its causes) matches a regular expression
/// read from the file expected_error.txt in the folder that `path` represents
///
/// If the regular expression matches any part of the error, this function returns true.
///
/// If the regular expression does not match, this function prints information about the mismatch
/// and returns false.
fn check_error_message(path: &Path, error: &Error) -> bool {
    let file_path = path.join("expected_error.txt");
    match fs::read_to_string(&file_path) {
        Ok(pattern) => {
            let pattern =
                Regex::new(&pattern).expect("Invalid regular expression in expected error file");

            let error_text = format!("{}", PrintCause(error));
            if pattern.is_match(&error_text) {
                println!("Failed to compile, as expected: {}", PrintCause(error));
                true
            } else {
                println!("Incorrect error message");
                println!("Expected to match regular expression: {}", pattern);
                println!("Actual message:\n{}", error_text);
                false
            }
        }
        Err(e) => panic!(
            "Failed to read expected error message from {}: {}",
            file_path.display(),
            e
        ),
    }
}
