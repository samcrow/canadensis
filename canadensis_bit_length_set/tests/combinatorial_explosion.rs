//!
//! Calculates the bit length set of a pathological data type that has a huge bit length set
//!
//! Based on: <https://github.com/UAVCAN/nunavut/blob/main/verification/nunavut_test_types/test0/regulated/CombinatorialExplosion.0.1.uavcan>
//!
//! If this takes too long, performance has regressed and combinatorial explosion is probably
//! happening.
//!

extern crate canadensis_bit_length_set;

use canadensis_bit_length_set::bit_length;

#[test]
fn combinatorial_explosion() {
    let u8_length = bit_length![8];
    let u16_length = bit_length![16];
    let u32_length = bit_length![32];
    // Length of uavcan.primitive.String.1.0
    let string_length = u16_length
        .clone()
        .concatenate([u8_length.repeat_range(..=256)]);

    // The length needs to be 32 bits so it can hold the value 65536
    let explosion_length = u32_length.concatenate([
        string_length.clone().repeat_range(..=65536),
        string_length.repeat(65536),
    ]);

    let explosion_min_length = explosion_length.min_value();
    let explosion_max_length = explosion_length.max_value();
    let explosion_byte_aligned = explosion_length.is_byte_aligned();

    println!(
        "Explosion size ranges from {} to {} bits, byte aligned {}",
        explosion_min_length, explosion_max_length, explosion_byte_aligned
    );

    // Minimum length for each string is 16 bits (just the length)
    // Minimum explosion length is 32 bits + 65536 * 16 bits = 1048592 bits
    assert_eq!(1048608, explosion_min_length);
    // Maximum length for each string is 16 bits length + 8 * 256 bits characters = 2064 bits
    // Maximum explosion length is 32 bits + 65536 * 2064 bits + 65536 * 2064 bits = 270532640 bits
    assert_eq!(270532640, explosion_max_length);
    assert!(explosion_byte_aligned);
}
