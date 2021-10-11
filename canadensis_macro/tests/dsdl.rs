extern crate canadensis_encoding;
extern crate canadensis_macro;

use canadensis_encoding::{Deserialize, Serialize};
use canadensis_macro::types_from_dsdl;

types_from_dsdl! {
    // Loads all DSDL files in a folder
    package($CARGO_MANIFEST_DIR, "/../canadensis_dsdl_frontend/tests/simple_dsdl")
    // Parses an inline DSDL file
    type "canadensis.Test.1.0" { r#"
uint32 a
uint8 THINGY = 3
@assert _offset_ == {32}
# A comment!
@sealed
---
# The response
@print "yay"
float32[<=2] values
@sealed
    "#}
    type "canadensis.Test.0.3" { r#"
uint64 d
@sealed
    "# }
    // Generates code for all loaded DSDL types
    generate_all()
    // Generates code for one or more specific DSDL types and their dependencies
    // generate_with_dependencies("canadensis.Test.1.0")
}

#[test]
fn encoding_1() {
    use canadensis::test_1_0::TestRequest;
    let request = TestRequest { a: 99 };

    let mut bytes = [0u8; 4];
    request.serialize_to_bytes(&mut bytes);
    assert_eq!(bytes, [99, 0, 0, 0]);
    let decoded = TestRequest::deserialize_from_bytes(&bytes).unwrap();
    assert_eq!(99, { decoded.a });
}
