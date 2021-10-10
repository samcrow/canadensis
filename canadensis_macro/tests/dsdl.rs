extern crate canadensis_macro;

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
