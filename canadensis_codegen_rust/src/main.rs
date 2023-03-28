extern crate canadensis_codegen_rust;
extern crate canadensis_dsdl_frontend;
extern crate clap;

use canadensis_dsdl_frontend::Package;
use clap::{value_parser, Arg, Command};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
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
            output_file: output_path,
            external_packages,
            rustfmt,
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
            let generated = canadensis_codegen_rust::generate_code(&package, &external_packages)?;

            let mut output_file = BufWriter::new(File::create(&output_path)?);
            writeln!(output_file, "{}", generated)?;
            drop(output_file);
            // Format
            if rustfmt {
                run_rustfmt(&output_path)?;
            }
        }
        Args::PrintDependencies => {
            print!("{}", canadensis_codegen_rust::generated_code_dependencies());
        }
    }
    Ok(())
}

/// Finds rustfmt in the default path and runs it to format the code at the provided path
fn run_rustfmt(output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let start_status = std::process::Command::new("rustfmt")
        .arg(output_path)
        .status();
    match start_status {
        Ok(exit_status) => {
            if exit_status.success() {
                Ok(())
            } else {
                let message = match exit_status.code() {
                    Some(code) => format!("rustfmt exited with status code {}", code),
                    None => String::from("rustfmt exited with unknown status"),
                };
                Err(Box::new(StringError(message)))
            }
        }
        Err(start_error) => Err(Box::new(ErrorContext::new(
            "Failed to run rustfmt".to_owned(),
            Box::new(start_error),
        ))),
    }
}

enum Args {
    Compile {
        /// Input folder paths with DSDL files to read
        input_folders: Vec<PathBuf>,
        /// Output file path
        output_file: PathBuf,
        /// DSDL packages that should not be generated, but instead refer to some other Rust module
        ///
        /// Each key is a list of Cyphal package name segments (like ["uavcan", "node"]).
        /// Each value is the path to a Rust module
        external_packages: BTreeMap<Vec<String>, Vec<String>>,
        /// Run rustfmt on the generated code
        rustfmt: bool,
    },
    PrintDependencies,
}

fn get_args() -> Args {
    let app = clap::Command::new("canadensis_generate_code")
        .version(clap::crate_version!())
        .about("Generates Rust data types and serialization code from Cyphal DSDL files")
        .subcommand_required(true)
        .subcommand(Command::new("compile").about("Parses DSDL files and generates Rust code")
        .arg(
            Arg::new("input")
                .index(1)
                .required(true)
                .num_args(1..)
                .value_parser(value_parser!(PathBuf))
                .help("One or more paths to directories with DSDL files"),
        )
        .arg(
            Arg::new("output_file")
                .short('o')
                .long("output-file")
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .default_value("lib.rs")
                .help("The file to write the generated code to"),
        )
        .arg(
            Arg::new("external_package")
                .long("external-package")
                .num_args(0..)
                .value_parser(validate_external_package)
                .value_name("cyphal-package,rust-module-path")
                .help("A DSDL package name and corresponding Rust module path that will not be generated"),
        )
            .arg(Arg::new("rustfmt")
                .long("rustfmt")
                .num_args(0)
                .help("Run rustfmt to format the generated code")
        ))
        .subcommand(Command::new("print-dependencies")
            .about("Prints the packages that the generated code depends on (for use in Cargo.toml)"));
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("compile", matches)) => Args::Compile {
            input_folders: matches
                .get_many::<PathBuf>("input")
                .unwrap()
                .cloned()
                .collect(),
            output_file: matches.get_one::<PathBuf>("output_file").unwrap().clone(),
            external_packages: matches
                .get_many::<String>("external_package")
                .map(|values| {
                    values
                        .map(|s| ExternalPackage::parse(s).expect("Invalid external package"))
                        .map(|ext| (ext.package, ext.rust_module))
                        .collect()
                })
                .unwrap_or_else(BTreeMap::new),
            rustfmt: matches.contains_id("rustfmt"),
        },
        Some(("print-dependencies", _)) => Args::PrintDependencies,
        _ => panic!("Unrecognized Subcommand"),
    }
}

/// Validates an external package name pair
fn validate_external_package(package: &str) -> Result<ExternalPackage, String> {
    ExternalPackage::parse(&package).ok_or_else(|| {
        "Invalid external package, expected [cyphal-package],[rust-module-path]".into()
    })
}

#[derive(Debug, Clone)]
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

/// Wraps an error with a message
#[derive(Debug)]
struct ErrorContext {
    context: String,
    inner: Box<dyn std::error::Error>,
}

impl ErrorContext {
    pub fn new(context: String, inner: Box<dyn std::error::Error>) -> Self {
        ErrorContext { context, inner }
    }
}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.context)
    }
}

impl std::error::Error for ErrorContext {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.inner)
    }
}

/// An error that contains only a message
#[derive(Debug)]
struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for StringError {}
