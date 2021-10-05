extern crate canadensis_encoding_derive;

use canadensis_encoding_derive::{DataType, Serialize};

#[derive(Serialize, DataType)]
#[canadensis(sealed)]
struct EmptyBracketed {}

#[derive(Serialize, DataType)]
#[canadensis(extent = 8 * 8)]
struct Empty;

#[derive(Serialize, DataType)]
#[canadensis(sealed)]
struct OneInteger {
    pub a: u8,
}

#[derive(Serialize, DataType)]
#[canadensis(sealed)]
struct SomeIntegers {
    pub a: u8,
    pub b: u64,
    pub c: u16,
    pub d: u32,
}

#[derive(Serialize, DataType)]
#[canadensis(sealed)]
struct OneIntegerArray {
    pub a: [u8; 32],
}

#[derive(Serialize, DataType)]
#[canadensis(sealed)]
struct NestedArray {
    pub a: [[[u8; 9]; 3]; 32],
}
