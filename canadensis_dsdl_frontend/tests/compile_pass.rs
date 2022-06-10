//! Checks that this library can compile the UAVCAN public regulated data types, Nunavut test
//! types, and a few additional Canadensis test types

extern crate canadensis_dsdl_frontend;

use canadensis_dsdl_frontend::{Error, Package};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[test]
fn compile_simple_types_only() -> Result<(), Error> {
    test_compile_subdirs(&["tests/simple_dsdl"])
}
#[test]
fn compile_regulated_types_only() -> Result<(), Error> {
    test_compile_subdirs(&["tests/public_regulated_data_types"])
}
#[test]
fn compile_all() -> Result<(), Error> {
    test_compile_subdirs(&[
        "tests/public_regulated_data_types",
        "tests/nunavut_test_types/test0",
        "tests/simple_dsdl",
    ])
}

fn test_compile_subdirs(subdirs: &[&str]) -> Result<(), Error> {
    let manifset_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest_subdirectories = subdirs.iter().map(|subdir| manifset_dir.join(subdir));
    test_compile_directories(manifest_subdirectories)
}

fn test_compile_directories<I, P>(directories: I) -> Result<(), Error>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut package = Package::new();
    for directory in directories {
        match package.add_files(directory.as_ref()) {
            Ok(()) => {}
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        }
    }
    match package.compile() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}

/// Tests a file with a combined path and name that are 255 characters long, which is equal
/// to the maximum allowed length
///
/// This is not just a file (like all the other compile cases) because some Windows software
/// can't handle long paths.
#[test]
fn compile_long_name() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let test_dir = target_dir.join("compile_long_name");
    // The full name of the DSDL type is canadensis.TwoHundred...
    let long_name_subdir = test_dir.join("canadensis");
    fs::create_dir_all(&long_name_subdir)?;
    let long_name_file_path = long_name_subdir.join("TwoHundredAndFiftyFiveCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong.1.0.uavcan");
    write_long_name_file(&long_name_file_path)?;

    test_compile_directories([test_dir])?;
    Ok(())
}

fn write_long_name_file(path: &Path) -> io::Result<()> {
    fs::write(
        path,
        "# The package and name of this type (not including the version) are 255 characters long, which is the maximum allowed.
@sealed
",
    )
}
