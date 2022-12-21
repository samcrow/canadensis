extern crate canadensis_encoding;
extern crate canadensis_macro;

use canadensis_encoding::{Deserialize, Serialize};
use canadensis_macro::types_from_dsdl;

types_from_dsdl! {
    type "canadensis.Sauce.1.0" { r#"
#[canadensis(enum)]
uint32 sauce
uint32 KETCHUP = 2
uint32 MUSTARD = 3
uint32 HARISSA = 4
uint32 CREME_ANGLAISE = 10
uint32 FISH_SAUCE = 0xffffffff
@assert _offset_ == {32}
# A comment!
@sealed
    "#}
    type "canadensis.GetFoodDensity.0.3" { r#"
#[canadensis(enum)]
uint3 food
uint3 FISH = 0
uint3 RICE = 1
uint3 POTATO = 7
@sealed
---
uint32 density
@sealed
    "# }
    type "canadensis.GetSauce.1.0" { r#"
uint16 food_spiciness
@sealed
---
#[canadensis(enum)]
uint4 sauce
uint4 PICO_DE_GALLO = 9
uint4 OLIVE_TAPENADE = 2
@sealed
    "#}
    // Generates code for all loaded DSDL types
    generate()
}

#[test]
fn sauce_encoding() {
    use canadensis::sauce_1_0::Sauce;
    let sauce1 = Sauce::Harissa;

    let mut bytes = [0u8; 4];
    sauce1.serialize_to_bytes(&mut bytes);
    assert_eq!(bytes, [4, 0, 0, 0]);
    let decoded = Sauce::deserialize_from_bytes(&bytes).unwrap();
    assert!(matches!(decoded, Sauce::Harissa));
}

#[test]
fn request_response_encoding() {
    use canadensis::get_food_density_0_3::GetFoodDensityRequest;
    use canadensis::get_sauce_1_0::GetSauceResponse;

    check_serialized_form(&GetFoodDensityRequest::Potato, &[7]);
    check_serialized_form(&GetSauceResponse::PicoDeGallo, &[9]);
}

fn check_serialized_form<T: Serialize + Deserialize>(value: &T, expected_bytes: &[u8]) {
    let mut buffer = vec![0u8; expected_bytes.len()];
    value.serialize_to_bytes(&mut buffer);
    assert_eq!(buffer, expected_bytes);
}
