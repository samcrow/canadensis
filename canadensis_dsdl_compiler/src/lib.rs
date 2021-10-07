extern crate canadensis_bit_length_set;
extern crate canadensis_dsdl_parser;
extern crate half;
extern crate itertools;
extern crate num_rational;
extern crate num_traits;
extern crate once_cell;
extern crate regex;
extern crate thiserror;
extern crate unicode_normalization;
extern crate walkdir;

use crate::type_key::TypeKey;

/// Creates an error associated with a span in the input
///
/// The first argument is a span expression. The others are forwarded to `::std::format!`.
macro_rules! span_error {
    ($span:expr, $($arg:tt)*) => {
        $crate::error::Error::Compile(::canadensis_dsdl_parser::make_error(::std::format!($($arg)*), $span))
    };
}

pub(crate) mod compile;
mod compiled;
pub mod error;
pub(crate) mod operators;
pub mod package;
mod type_key;
mod types;
