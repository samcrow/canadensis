//! Checks that the parser can parse all of the public regulated data type DSDL files without errors

extern crate canadensis_dsdl_parser;
extern crate walkdir;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

#[test]
fn parse_public_regulated_data_types() -> io::Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let prdt_dir = manifest_dir
        .join("tests")
        .join("public_regulated_data_types");

    for entry in WalkDir::new(prdt_dir) {
        let entry = entry?;
        if is_dsdl_file(&entry) {
            try_parse_file(entry.path())?;
        }
    }
    Ok(())
}

fn is_dsdl_file(entry: &DirEntry) -> bool {
    if entry.file_type().is_file() {
        match entry.file_name().to_str() {
            Some(name) if name.ends_with(".uavcan") => true,
            Some(_) | None => false,
        }
    } else {
        false
    }
}

fn try_parse_file(path: &Path) -> io::Result<()> {
    println!("Parsing {}", path.display());
    let content = fs::read_to_string(path)?;
    match canadensis_dsdl_parser::parse(&content) {
        Ok(_ast) => {}
        Err(e) => {
            println!("Error parsing {}", path.display());
            panic!("{}", e)
        }
    }
    Ok(())
}
