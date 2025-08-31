extern crate canadensis_codegen_rust;
extern crate canadensis_dsdl_frontend;

use std::path::PathBuf;

use canadensis_dsdl_frontend::compiled::package::CompiledPackage;
use canadensis_dsdl_frontend::Package;

/// Checks that this library can compile the Cyphal public regulated data types, Nunavut test
/// types, and a few additional Canadensis test types
#[test]
fn compile_pass_combined() -> Result<(), Box<dyn std::error::Error>> {
    let paths = [
        "tests/compile_pass",
        "../canadensis_dsdl_frontend/tests/nunavut_test_types/test0",
        "../canadensis_dsdl_frontend/tests/simple_dsdl",
        "../canadensis_dsdl_frontend/tests/public_regulated_data_types",
    ];
    let absolute_paths =
        paths.map(|relative| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative));

    try_compile_and_generate_code(&absolute_paths)?;
    Ok(())
}

#[test]
fn compile_namespace_from_multiple_folders() -> Result<(), Box<dyn std::error::Error>> {
    let paths = [
        "../canadensis_dsdl_frontend/tests/split_namespace/part1",
        "../canadensis_dsdl_frontend/tests/split_namespace/part2",
    ];
    let absolute_paths =
        paths.map(|relative| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative));

    try_compile_and_generate_code(&absolute_paths)?;
    Ok(())
}

fn try_compile_package(
    paths: &[PathBuf],
) -> Result<CompiledPackage, Box<canadensis_dsdl_frontend::Error>> {
    let mut package = Package::new();
    for path in paths {
        package.add_files(path)?;
    }
    package.compile()
}

fn try_compile_and_generate_code(paths: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let package = try_compile_package(paths)?;
    let generated = canadensis_codegen_rust::generate_code(&package, &Default::default())?;
    let mut sink = std::io::sink();
    write!(sink, "{}", generated)?;
    Ok(())
}
