extern crate canadensis_dsdl_frontend;

use canadensis_dsdl_frontend::{Error, Package};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Checks that Package::add_files returns an error when given a path that is not a directory
#[test]
fn package_add_not_directory() -> io::Result<()> {
    let temp_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let not_directory_path = temp_dir.join("not a directory");
    fs::write(
        &not_directory_path,
        "This file is used in the package_add_not_directory() test",
    )?;

    let mut package = Package::new();
    let status = package.add_files(&not_directory_path);

    // Clean up
    fs::remove_file(not_directory_path)?;

    match status {
        Ok(()) => panic!("No error when adding a non-directory path"),
        Err(e) if matches!(*e, Error::NotDirectory(_)) => Ok(()),
        Err(other) => panic!("Unexpected error {:?}", other),
    }
}
