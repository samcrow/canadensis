//!
//! https://github.com/samcrow/canadensis/issues/66
//!
//! Deserializing a variable-length boolean array with a length greater than the maximum length
//! from the data type definition should not panic.
//!

extern crate canadensis_core;
extern crate canadensis_encoding;
extern crate canadensis_macro;

use canadensis_encoding::{Deserialize, DeserializeError};
use canadensis_macro::types_from_dsdl;

types_from_dsdl! {
    package($CARGO_MANIFEST_DIR, "/../canadensis_dsdl_frontend/tests/public_regulated_data_types")
    generate()
}

const REGISTER_WRITE_BIT_ARRAY_TOO_LONG: &[u8] = &[
    4, // Name length
    b't', b'e', b's', b't', // Name
    3,    // Value tag 3 for bit array
    0x01,
    0x08, // Array length 2049, larger than the maximum length 2048 of uavcan.primitive.array.Bit.1.0
    0x55, // Only 8 bits (due to zero-padding, this part is not an error)
];

#[test]
fn variable_length_bit_array_length_error() {
    use uavcan::register::access_1_0::AccessRequest;
    let status = AccessRequest::deserialize_from_bytes(REGISTER_WRITE_BIT_ARRAY_TOO_LONG);
    assert!(matches!(status, Err(DeserializeError::ArrayLength)));
}
