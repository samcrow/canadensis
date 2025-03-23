//! Utilities for reading configuration files for specific test cases

use canadensis_dsdl_frontend::Config;
use serde::Deserialize;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

#[derive(Deserialize)]
struct CaseConfig {
    #[serde(default = "default_allow_utf8_and_byte")]
    pub allow_utf8_and_byte: bool,
    #[serde(default = "default_allow_saturated_bool")]
    pub allow_saturated_bool: bool,
}

impl Default for CaseConfig {
    fn default() -> Self {
        CaseConfig {
            allow_utf8_and_byte: default_allow_utf8_and_byte(),
            allow_saturated_bool: default_allow_saturated_bool(),
        }
    }
}

fn default_allow_utf8_and_byte() -> bool {
    false
}
fn default_allow_saturated_bool() -> bool {
    true
}

impl From<CaseConfig> for Config {
    fn from(value: CaseConfig) -> Self {
        Config {
            allow_utf8_and_byte: value.allow_utf8_and_byte,
            allow_saturated_bool: value.allow_saturated_bool,
        }
    }
}

/// Attempts to read a test case configuration JSON file from the test case directory
/// that the provided path represents
///
/// If the file does not exist, this function returns a default configuration.
pub(crate) fn read_config<P: AsRef<Path>>(
    case_path: P,
) -> Result<Config, Box<dyn std::error::Error>> {
    read_config_inner(case_path.as_ref()).map(Config::from)
}

fn read_config_inner(case_path: &Path) -> Result<CaseConfig, Box<dyn std::error::Error>> {
    let json_path = case_path.join("test_case_config.json");
    match File::open(json_path) {
        Ok(json_file) => Ok(serde_json::from_reader(json_file)?),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(CaseConfig::default()),
        Err(e) => Err(e.into()),
    }
}
