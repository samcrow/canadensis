//!
//! This library contains data types and serialization code generated automatically using
//! `canadensis_codegen_rust` from the UAVCAN public regulated data types.

#![no_std]

extern crate canadensis_core;
extern crate canadensis_encoding;
extern crate half;
extern crate heapless;
extern crate zerocopy;

include!("generated.rs");
