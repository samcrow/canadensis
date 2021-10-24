# canadensis_data_types

This library provides Rust data types corresponding to [the UAVCAN public regulated data types](https://github.com/UAVCAN/public_regulated_data_types).

# Procedure to generate/update code

```shell
git clone https://github.com/UAVCAN/public_regulated_data_types.git
canadensis_codegen_rust compile -o src/generated.rs public_regulated_data_types
rustfmt src/generated.rs
rm -rf public_regulated_data_types
```
