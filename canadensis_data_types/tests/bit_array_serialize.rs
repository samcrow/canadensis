//! Checks serialization and deserialization of `uavcan.primitive.array.Bit.1.0`
//!
//! The serialized form should have 16 bits of length (because the capacity is 2048 bits)
//! followed by zero or more bits

extern crate canadensis_data_types;

use canadensis_data_types::uavcan::primitive::array::bit_1_0::Bit;
use canadensis_encoding::bits::BitArray;
use canadensis_encoding::{Deserialize, Serialize};

#[test]
fn bit_array_empty_serialize() {
    let empty_array = Bit {
        value: BitArray::new(0),
    };

    // Empty array gets encoded as just a length
    check_serialize(&empty_array, &[0x0, 0x0]);
}
#[test]
fn bit_array_1_serialize() {
    let array_0 = Bit {
        value: {
            let mut bits = BitArray::new(1);
            bits.set(0, false);
            bits
        },
    };

    check_serialize(&array_0, &[0x1, 0x0, 0x0]);

    let array_1 = Bit {
        value: {
            let mut bits = BitArray::new(1);
            bits.set(0, true);
            bits
        },
    };

    check_serialize(&array_1, &[0x1, 0x0, 0x1]);
}
#[test]
fn bit_array_2_serialize() {
    let array_0 = Bit {
        value: {
            let mut bits = BitArray::new(2);
            bits.set(0, false);
            bits.set(1, true);
            bits
        },
    };

    check_serialize(&array_0, &[0x2, 0x0, 0x2]);

    let array_1 = Bit {
        value: {
            let mut bits = BitArray::new(2);
            bits.set(0, true);
            bits.set(1, true);
            bits
        },
    };

    check_serialize(&array_1, &[0x2, 0x0, 0x3]);
}

#[test]
fn bit_array_9_serialize() {
    let array_0 = Bit {
        value: {
            let mut bits = BitArray::new(9);
            bits.set(0, false);
            bits.set(1, true);
            bits.set(7, true);
            bits.set(8, true);
            bits
        },
    };

    check_serialize(&array_0, &[0x9, 0x0, 0x82, 0x1]);
}

fn check_serialize(value: &Bit, expected: &[u8]) {
    // Maximum encoded size is 258 bytes
    let mut bytes = [0u8; 258];
    value.serialize_to_bytes(&mut bytes);
    // Empty array gets encoded as just a length
    assert_eq!(&bytes[..expected.len()], expected);

    // Try deserializing
    let deserialized = Bit::deserialize_from_bytes(&bytes).expect("Deserialize failed");
    assert_eq!(deserialized.value, value.value);
}
