extern crate canadensis_codegen_rust;
extern crate canadensis_dsdl_frontend;
extern crate clap;

use canadensis_dsdl_frontend::Package;
use clap::{AppSettings, Arg, SubCommand};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::{env, process};

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            print_error(&*e);
            process::exit(-1);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = get_args();
    match args {
        Args::Compile {
            input_folders,
            output_file,
            external_packages,
        } => {
            let mut package = Package::new();
            for path in input_folders {
                package.add_files(path)?;
            }
            let package = match package.compile_with_warnings() {
                Ok(package) => package,
                Err((e, warnings)) => {
                    for warning in warnings {
                        eprintln!("Warning: {}", warning);
                    }
                    return Err(e.into());
                }
            };

            // Report warnings
            for warning in package.warnings() {
                eprintln!("Warning: {}", warning);
            }

            // Generate code
            let generated = canadensis_codegen_rust::generate_code(&package, &external_packages);

            let mut output_file = BufWriter::new(File::create(output_file)?);
            writeln!(output_file, "{}", generated)?;
        }
        Args::PrintDependencies => {
            print!("{}", canadensis_codegen_rust::generated_code_dependencies());
        }
    }
    Ok(())
}

enum Args {
    Compile {
        /// Input folder paths with DSDL files to read
        input_folders: Vec<OsString>,
        /// Output file path
        output_file: OsString,
        /// DSDL packages that should not be generated, but instead refer to some other Rust module
        ///
        /// Each key is a list of UAVCAN package name segments (like ["uavcan", "node"]).
        /// Each value is the path to a Rust module
        external_packages: BTreeMap<Vec<String>, Vec<String>>,
    },
    PrintDependencies,
}

fn get_args() -> Args {
    let app = clap::App::new("canadensis_generate_code")
        .version(clap::crate_version!())
        .about("Generates Rust data types and serialization code from UAVCAN DSDL files")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("compile").about("Parses DSDL files and generates Rust code")
        .arg(
            Arg::with_name("input")
                .index(1)
                .required(true)
                .multiple(true)
                .help("One or more paths to directories with DSDL files"),
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .required(true)
                .default_value("lib.rs")
                .help("The file to write the generated code to"),
        )
        .arg(
            Arg::with_name("external_package")
                .long("external-package")
                .multiple(true)
                .validator(validate_external_package)
                .value_name("uavcan-package,rust-module-path")
                .help("A DSDL package name and corresponding Rust module path that will not be generated"),
        ))
        .subcommand(SubCommand::with_name("print-dependencies")
            .about("Prints the packages that the generated code depends on (for use in Cargo.toml)"));
    let matches = app.get_matches();

    match matches.subcommand() {
        ("compile", Some(matches)) => Args::Compile {
            input_folders: matches
                .values_of_os("input")
                .unwrap()
                .map(OsString::from)
                .collect(),
            output_file: matches.value_of_os("output_file").unwrap().into(),
            external_packages: matches
                .values_of("external_package")
                .map(|values| {
                    values
                        .map(|s| ExternalPackage::parse(s).expect("Invalid external package"))
                        .map(|ext| (ext.package, ext.rust_module))
                        .collect()
                })
                .unwrap_or_else(|| BTreeMap::new()),
        },
        ("print-dependencies", _) => Args::PrintDependencies,
        _ => panic!("Unrecognized subcommand"),
    }
}

/// Validates an external package name pair
fn validate_external_package(package: String) -> Result<(), String> {
    ExternalPackage::parse(&package)
        .ok_or_else(|| {
            "Invalid external package, expected [uavcan-package],[rust-module-path]".into()
        })
        .map(drop)
}

struct ExternalPackage {
    package: Vec<String>,
    rust_module: Vec<String>,
}

impl ExternalPackage {
    fn parse(package: &str) -> Option<Self> {
        let mut parts = package.split(',');
        let package = parts.next()?;
        let rust_module = parts.next()?;

        Some(ExternalPackage {
            package: package.split('.').map(String::from).collect(),
            rust_module: rust_module.split("::").map(String::from).collect(),
        })
    }
}

fn print_error(e: &dyn std::error::Error) {
    eprintln!("{}", e);
    if let Some(source) = e.source() {
        eprintln!("Caused by:");
        print_error(source);
    }
}
