# canadensis_data_types

This library provides Rust data types corresponding to [the Cyphal public regulated data types](https://github.com/OpenCyphal/public_regulated_data_types).

# Procedure to generate/update code

```shell
canadensis_codegen_rust compile --rustfmt -o src/generated.rs ../canadensis_dsdl_frontend/tests/public_regulated_data_types
```
