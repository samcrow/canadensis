//!
//! This library parses Cyphal data structure definition language (DSDL) files.
//!

pub extern crate num_bigint;
pub extern crate num_rational;
extern crate num_traits;
extern crate pest;
extern crate pest_derive;

mod ast;

use crate::parser::{DsdlParser, Rule};
use pest::Parser;

pub use crate::ast::types::*;

use pest::error::ErrorVariant;
/// A range of characters in the input text
///
pub use pest::Span;

mod parser {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "dsdl.pest"]
    pub struct DsdlParser;
}

/// A parse error
///
/// This type is returned from [`parse`] and can also be used in other libraries
/// to represent errors in other steps of the process
#[derive(Debug, Clone)]
pub struct Error(Box<pest::error::Error<Rule>>);

/// Attempts to parse the text of a DSDL file into an abstract syntax tree
///
/// # Errors
///
/// This function returns an error if the DSDL has invalid syntax.
pub fn parse<'i>(dsdl: &'i str, config: &Config) -> Result<Definition<'i>, Error> {
    let parse_tree = DsdlParser::parse(Rule::definition, dsdl).map_err(|e| Error(Box::new(e)))?;
    ast::parse_to_ast(parse_tree, config)
}

/// Convenience function to make an error value with a custom message
pub fn make_error<S>(message: S, span: Span<'_>) -> Error
where
    S: Into<String>,
{
    Error(Box::new(pest::error::Error::new_from_span(
        ErrorVariant::CustomError {
            message: message.into(),
        },
        span,
    )))
}

mod error_impl {
    use super::Error;
    use std::fmt;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.0.source()
        }
    }
}

/// Parser configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Allow `utf8` and `byte` types in DSDL
    ///
    /// Based on <https://github.com/OpenCyphal/specification/issues/51>
    ///
    /// Warning: The specification does not have details for this, so the behavior may change.
    ///
    /// Default false
    pub allow_utf8_and_byte: bool,
    /// Allow the `saturated bool` type in DSDL
    ///
    /// v1.0 of the specification allows this, but pydsdl plans to remove it:
    /// <https://github.com/OpenCyphal/pydsdl/pull/97>
    ///
    /// Default true
    pub allow_saturated_bool: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            allow_utf8_and_byte: false,
            allow_saturated_bool: true,
        }
    }
}
