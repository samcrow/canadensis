//! Checks that this library can compile the Cyphal public regulated data types, Nunavut test
//! types, and a few additional Canadensis test types

extern crate canadensis_dsdl_frontend;

use canadensis_dsdl_frontend::compiled::package::CompiledPackage;
use canadensis_dsdl_frontend::compiled::DsdlKind;
use canadensis_dsdl_frontend::{Config, Error, Package, TypeKey};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io};

#[test]
fn compile_simple_types_only() -> Result<(), Box<Error>> {
    let config = Config {
        allow_utf8_and_byte: true,
        allow_saturated_bool: false,
    };
    test_compile_subdirs(&["tests/simple_dsdl"], &config)?;
    Ok(())
}
#[test]
fn compile_simple_types_only_no_byte_utf8() -> Result<(), Box<Error>> {
    let config = Config {
        allow_utf8_and_byte: false,
        allow_saturated_bool: true,
    };
    test_compile_subdirs(&["tests/simple_dsdl_no_byte_utf8"], &config)?;
    Ok(())
}
#[test]
fn compile_regulated_types_only() -> Result<(), Box<Error>> {
    check_public_regulated_data_types_submodule().map_err(Error::Io)?;
    test_compile_subdirs(&["tests/public_regulated_data_types"], &Config::default())?;
    Ok(())
}
#[test]
fn compile_all() -> Result<(), Box<Error>> {
    check_public_regulated_data_types_submodule().map_err(Error::Io)?;
    let config = Config {
        allow_utf8_and_byte: true,
        allow_saturated_bool: false,
    };
    test_compile_subdirs(
        &[
            "tests/public_regulated_data_types",
            "tests/nunavut_test_types/test0",
            "tests/simple_dsdl",
        ],
        &config,
    )?;
    Ok(())
}

#[test]
fn compile_split_namespace() -> Result<(), Box<Error>> {
    test_compile_subdirs(
        &["tests/split_namespace/part1", "tests/split_namespace/part2"],
        &Config::default(),
    )?;
    Ok(())
}

/// Checks that the library returns a reasonable error and does not loop forever
/// when DSDL types have cyclic dependencies.
#[test]
fn test_cycle() {
    let status = test_compile_subdirs(&["tests/cycle"], &Config::default());
    match status {
        Ok(_) => panic!("Compilation succeeded with cyclic dependency"),
        Err(e) => {
            // Expect this error:
            // Failed to compile one of the two files
            // |- Failed to compile the other one of the two files
            //    |- Couldn't find the type in the outermost file
            match &*e {
                Error::CompileFile { inner, .. } => match inner.deref() {
                    Error::CompileFile { inner, .. } => match inner.deref() {
                        Error::UnknownType(_) => { /* OK */ }
                        _ => panic!("Unexpected error {:#?}", e),
                    },
                    _ => panic!("Unexpected error {:#?}", e),
                },
                _ => panic!("Unexpected error {:#?}", e),
            }
        }
    }
}

/// Checks that when compiling a deprecated service type, both the request and response are marked
/// as deprecated
#[test]
fn compile_service_response_deprecated() -> Result<(), Box<Error>> {
    check_public_regulated_data_types_submodule().map_err(Error::Io)?;
    let package = test_compile_subdirs(
        &[
            "tests/public_regulated_data_types",
            "tests/nunavut_test_types/test0",
        ],
        &Config::default(),
    )?;

    let dsdl = package
        .get_by_key(&TypeKey::from_str("regulated.basics.DeprecatedService.0.1").unwrap())
        .expect("Type not found");

    match &dsdl.kind {
        DsdlKind::Message(_) => panic!("Not a service"),
        DsdlKind::Service { request, response } => {
            assert!(request.deprecated());
            assert!(response.deprecated());
        }
    }

    Ok(())
}

fn test_compile_subdirs(subdirs: &[&str], config: &Config) -> Result<CompiledPackage, Box<Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest_subdirectories = subdirs.iter().map(|subdir| manifest_dir.join(subdir));
    test_compile_directories(manifest_subdirectories, config)
}

/// Checks that the provided set of directories gets compiled successfully with no warnings
fn test_compile_directories<I, P>(
    directories: I,
    config: &Config,
) -> Result<CompiledPackage, Box<Error>>
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
    match package.compile(config) {
        Ok(compiled) => {
            let warnings = compiled.warnings();
            if warnings.is_empty() {
                Ok(compiled)
            } else {
                panic!("Unexpected warning(s) {:#?}", warnings);
            }
        }
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

    test_compile_directories([test_dir], &Config::default())?;
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

/// Checks that the public regulated data types submodule has been correctly checked out
///
/// This function prints an error message and panics if that is not the case.
fn check_public_regulated_data_types_submodule() -> io::Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let prdt_dir = manifest_dir.join("tests/public_regulated_data_types");
    if fs::read_dir(&prdt_dir)?.next().is_none() {
        // No entries in directory
        eprintln!(
            "The canadensis_dsdl_frontend/tests/public_regulated_data_types submodule has not \
        been checked out. To fix this, use `git clone --recursive` when cloning, or \
        `git submodule init` and `git submodule update` after cloning."
        );
        panic!("No public_regulated_data_types");
    }
    // At least one entry in the directory, assume submodule is correct
    Ok(())
}
