extern crate canadensis_bit_length_set;
extern crate canadensis_dsdl_parser;
extern crate half;
extern crate num_rational;
extern crate num_traits;
extern crate once_cell;
extern crate regex;
extern crate thiserror;
extern crate unicode_normalization;
extern crate walkdir;

use crate::type_key::TypeKey;
use crate::types::Value;
use std::collections::BTreeMap;

pub(crate) mod compile;
mod compiled;
pub mod package;
mod type_key;
mod types;
