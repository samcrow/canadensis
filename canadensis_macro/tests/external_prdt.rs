extern crate canadensis_macro;

canadensis_macro::types_from_dsdl! {
    // Load all the UAVCAN public regulated data types
    package($CARGO_MANIFEST_DIR, "/../canadensis_dsdl_frontend/tests/public_regulated_data_types")
    // Declare the two packages from the public regulated data types as external, coming from
    // canadensis_data_types
    make_external(uavcan, canadensis_data_types::uavcan)
    make_external(reg, canadensis_data_types::reg)
    // Add a custom type
    type "testing.custom.DependsOnHealth.1.0" { r#"
uavcan.node.Health.1.0 health
uavcan.time.SynchronizedTimestamp.1.0 time
@sealed
    "#}
    // Generate code for only the custom type
    generate()
}
