//! Examples from the UAVCAN specification

extern crate canadensis_bit_length_set;

use canadensis_bit_length_set::bit_length;

// uint16[<=3] foo
// @sealed
#[test]
fn basic1() {
    let u8_length = bit_length![8];
    let u16_length = bit_length![16];
    let composite_length = u8_length
        .concatenate([u16_length.repeat_range(..=3)])
        .pad_to_alignment(8);
    assert_eq!(
        composite_length.expand(),
        bit_length![8, 24, 40, 56].expand()
    );
}

// uint16[<=3] foo
// int2 bar
// @sealed
#[test]
fn basic2() {
    let u8_length = bit_length![8];
    let u16_length = bit_length![16];
    let composite_length = u8_length
        .concatenate([u16_length.repeat_range(..=3), bit_length![2]])
        .pad_to_alignment(8);
    assert_eq!(
        composite_length.expand(),
        bit_length![16, 32, 48, 64].expand()
    );
}

// bool[<=3] foo
// @sealed
#[test]
fn basic3() {
    let u8_length = bit_length![8];
    let composite_length = u8_length
        .concatenate([bit_length![1].repeat_range(..=3)])
        .pad_to_alignment(8);
    assert_eq!(composite_length.expand(), bit_length![8, 16].expand());
}
