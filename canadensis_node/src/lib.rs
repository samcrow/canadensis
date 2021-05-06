#![no_std]

extern crate alloc;
extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_data_types;
extern crate canadensis_encoding;
extern crate fallible_collections;
extern crate heapless;

mod basic;
mod minimal;
pub use crate::minimal::MinimalNode;
