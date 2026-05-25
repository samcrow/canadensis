extern crate canadensis_core;
extern crate canadensis_encoding;
extern crate canadensis_macro;

use canadensis_encoding::{Deserialize, Serialize};
use canadensis_macro::types_from_dsdl;

types_from_dsdl! {
    // Load all the Cyphal public regulated data types
    package($CARGO_MANIFEST_DIR, "/../canadensis_dsdl_frontend/tests/public_regulated_data_types")
    type "test.Bar.1.0" { r#"
uint16 start_index
uint8 max_count

@extent 64

---
# This is an array of a delimited type with a length that is not a multiple of 8.
# In the array, we add padding to align each element to 8 bits.
test.Foo.1.0[<=32] endpoints
bool complete

@extent 32 * (512 + 4) * 8 + 64
    "#}

    type "test.Foo.1.0" { r#"
uavcan.primitive.String.1.0 unit
bool enabled
@extent 512 * 8
    "#}
    generate()
}

#[test]
fn odd_size_in_array_one_element() {
    let mut endpoints = heapless::Vec::new();
    let foo = test::foo_1_0::Foo {
        // Size: 16 bits length + 24 bits characters = 40 bits
        unit: uavcan::primitive::string_1_0::String {
            value: heapless::Vec::from_slice(b"foo").unwrap(),
        },
        // Size: 1 bit
        enabled: false,
    };
    assert_eq!(41, foo.size_bits());
    assert!(endpoints
        // Size of this array element: 32 bits delimiter header + 48 bits data (rounded up) = 80 bits
        .push(foo)
        .is_ok());

    // Size of payload: 89 bits
    let payload = test::bar_1_0::BarResponse {
        // Size of endpoints: 8 bits length + 80 bits endpoints[0]
        endpoints,
        // Size of complete: 1 bit
        complete: true,
    };

    assert_eq!(89, payload.size_bits());
    assert_eq!(12, payload.size_bits().div_ceil(8));

    let mut bytes = [0u8; 12];
    payload.serialize_to_bytes(&mut bytes);
    assert_eq!(bytes, [1, 6, 0, 0, 0, 3, 0, b'f', b'o', b'o', 0, 1]);

    let deserialized =
        test::bar_1_0::BarResponse::deserialize_from_bytes(&bytes).expect("Deserialize failed");
    assert_eq!(1, deserialized.endpoints.len());
    assert_eq!(b"foo", deserialized.endpoints[0].unit.value.as_slice());
    assert_eq!(false, deserialized.endpoints[0].enabled);
    assert_eq!(true, deserialized.complete);
}

#[test]
fn odd_size_in_array_two_elements() {
    let mut endpoints = heapless::Vec::new();
    let foo = test::foo_1_0::Foo {
        // Size: 16 bits length + 24 bits characters = 40 bits
        unit: uavcan::primitive::string_1_0::String {
            value: heapless::Vec::from_slice(b"foo").unwrap(),
        },
        // Size: 1 bit
        enabled: false,
    };
    assert_eq!(41, foo.size_bits());
    let spaghetti = test::foo_1_0::Foo {
        // Size: 16 bits length + 72 bits characters = 88 bits
        unit: uavcan::primitive::string_1_0::String {
            value: heapless::Vec::from_slice(b"Spaghetti").unwrap(),
        },
        // Size: 1 bit
        enabled: true,
    };
    assert_eq!(89, spaghetti.size_bits());
    assert!(endpoints
        // Size of this array element: 32 bits delimiter header + 48 bits data (rounded up) = 80 bits
        .push(foo)
        .is_ok());
    assert!(endpoints
        // Size of this array element: 32 bits delimiter header + 96 bits data (rounded up) = 128 bits
        .push(spaghetti)
        .is_ok());

    // Size of payload: 217 bits
    let payload = test::bar_1_0::BarResponse {
        // Size of endpoints: 8 bits length + 80 bits endpoints[0] + 128 bits endpoint[1]
        endpoints,
        // Size of complete: 1 bit
        complete: false,
    };

    assert_eq!(217, payload.size_bits());
    assert_eq!(28, payload.size_bits().div_ceil(8));

    let mut bytes = [0u8; 28];
    payload.serialize_to_bytes(&mut bytes);
    assert_eq!(
        bytes,
        [
            2, 6, 0, 0, 0, 3, 0, b'f', b'o', b'o', 0, 12, 0, 0, 0, 9, 0, b'S', b'p', b'a', b'g',
            b'h', b'e', b't', b't', b'i', 1, 0
        ]
    );

    let deserialized =
        test::bar_1_0::BarResponse::deserialize_from_bytes(&bytes).expect("Deserialize failed");
    assert_eq!(2, deserialized.endpoints.len());
    assert_eq!(b"foo", deserialized.endpoints[0].unit.value.as_slice());
    assert_eq!(false, deserialized.endpoints[0].enabled);
    assert_eq!(
        b"Spaghetti",
        deserialized.endpoints[1].unit.value.as_slice()
    );
    assert_eq!(true, deserialized.endpoints[1].enabled);
    assert_eq!(false, deserialized.complete);
}
