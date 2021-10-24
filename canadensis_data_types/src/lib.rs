//!
//! This library contains data types and serialization code generated automatically using
//! `canadensis_codegen_rust` from the UAVCAN public regulated data types.

#![no_std]

extern crate canadensis_encoding;
extern crate canadensis_macro;
extern crate half;
extern crate heapless;
extern crate zerocopy;

canadensis_macro::types_from_dsdl! {
    package($CARGO_MANIFEST_DIR, "/public_regulated_data_types")
    generate()
}
