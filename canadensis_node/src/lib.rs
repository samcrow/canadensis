#![no_std]

extern crate alloc;
extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_data_types;
extern crate canadensis_derive_register_block;
extern crate canadensis_encoding;
extern crate canadensis_filter_config;
extern crate fallible_collections;
extern crate half;
extern crate heapless;
extern crate log;

mod basic;
mod minimal;
pub mod register;
pub use crate::basic::BasicNode;
pub use crate::minimal::MinimalNode;
