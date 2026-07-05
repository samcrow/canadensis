extern crate canadensis_core;
extern crate canadensis_encoding;
extern crate canadensis_macro;

use canadensis_encoding::{Deserialize, Serialize};
use canadensis_macro::types_from_dsdl;

types_from_dsdl! {
    type "test.Test.1.0" { r#"
int12 cell_temp
@sealed
    "#}
    generate()
}

#[test]
fn int12_round_trip() {
    let payload = test::test_1_0::Test { cell_temp: -50 };

    let mut bytes = [0u8; 2];
    payload.serialize_to_bytes(&mut bytes);
    assert_eq!(bytes, [0xce, 0x0f]);

    let deserialized =
        test::test_1_0::Test::deserialize_from_bytes(&bytes).expect("Deserialize failed");
    assert_eq!(-50, deserialized.cell_temp);
}
