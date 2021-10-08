//! Checks that this library can compile the UAVCAN public regulated data types, Nunavut test
//! types, and a few additional Canadensis test types

extern crate canadensis_dsdl_frontend;

use canadensis_dsdl_frontend::{Error, Package};
use std::path::PathBuf;

#[test]
fn compile_simple_types_only() -> Result<(), Error> {
    test_compile(&["tests/simple_dsdl"])
}
#[test]
fn compile_regulated_types_only() -> Result<(), Error> {
    test_compile(&["tests/public_regulated_data_types"])
}
#[test]
fn compile_all() -> Result<(), Error> {
    test_compile(&[
        "tests/public_regulated_data_types",
        "tests/nunavut_test_types/test0",
        "tests/simple_dsdl",
    ])
}

fn test_compile(subdirs: &[&str]) -> Result<(), Error> {
    let mut package = Package::new();
    for subdir in subdirs {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(subdir);
        match package.add_files(path) {
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
