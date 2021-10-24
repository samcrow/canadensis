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

/// Creates an error associated with a span in the input
///
/// The first argument is a span expression. The others are forwarded to `::std::format!`.
macro_rules! span_error {
    ($span:expr, $($arg:tt)*) => {
        $crate::error::Error::Compile(::canadensis_dsdl_parser::make_error(::std::format!($($arg)*), $span))
    };
}

pub(crate) mod compile;
pub mod compiled;
pub mod constants;
pub(crate) mod error;
pub(crate) mod operators;
mod package;
mod type_key;
pub mod types;

pub use crate::error::Error;
pub use crate::package::Package;
pub use crate::type_key::{TypeFullName, TypeKey};
