# canadensis_codegen_rust: A Rust code generator for Cyphal data types

This application reads Cyphal data structure description language (DSDL) files.
It generates Rust code to represent the Cyphal data types, serialize them,
and deserialize them.

## Usage

### Compiling a DSDL package 

`canadensis_codegen_rust compile -o output-file input-directory..`

Specify an output file where the code will be written, and one or more input
directories that contain DSDL files. The compiler will read all DSDL files
in the input directories and put code for all the data types in the output
file.

#### Example

Clone the [Cyphal public regulated data types repository](https://github.com/OpenCyphal/public_regulated_data_types)
and run `canadensis_codegen_rust compile -o lib.rs public_regulated_data_types`.

For easier viewing, you may want to use `rustfmt` to reformat the generated code.

### Using the generated code

The compiler produces only one `.rs` file. To compile it, you will need to put it in
some Cargo package. The file can be in a submodule along with other code, or in the root
of a library.

The generated code is compatible with `no_std`, so if necessary you can include `#![no_std]`
in the `lib.rs` file.

The generated code depends on a few external libraries for data types and serialization.
Run `canadensis_codegen_rust print-dependencies` to show the dependency specifications.
You should include the output in the package's `Cargo.toml` file.

#### Formatting

By default, the generated code does not have consistent formatting. To format it, add the `--rustfmt` option when running
`canadensis_codegen_rust`. This option requires a preinstalled `rustfmt` binary in the default path.

### External modules

For motivation, suppose you have this file `depends_on_prdt/canadensis/test/ContainsHealth.1.0.uavcan`:
```
uavcan.node.Health.1.0 health0
uavcan.node.Health.1.0 health1

@sealed
```

This data type depends on the `Health` type from the Cyphal public regulated
data types. Normally, you would need to compile both packages together:
`canadensis_codegen_rust compile -o lib.rs public_regulated_data_types depends_on_prdt`.
That would generate a file with all the public regulated data types and the
`ContainsHealth` type. 

However, if your code already depends on `canadensis_data_types`, you would
need to compile all the public regulated data types twice and the generated
`ContainsHealth` code would not be compatible with `canadensis_data_types`.

Instead, you can mark the `uavcan` and `reg` packages as external, and refer to
the pre-generated code in `canadensis_data_types` with this command:
`canadensis_codegen_rust compile public_regulated_data_types depends_on_prdt --external-package uavcan,canadensis_data_types::uavcan --external-package reg,canadensis_data_types::reg -o lib.rs`

That command will produce this code:

```rust
#[cfg(not(target_endian = "little"))]
compile_error!("Zero-copy serialization requires a little-endian target");
#[allow(unused_variables, unused_braces, unused_parens)]
#[deny(unaligned_references)]
pub mod canadensis {
    pub mod test {
        pub mod contains_health_0_1 {
            /// `canadensis.test.ContainsHealth.0.1`
            ///
            /// Fixed size 2 bytes
            pub struct ContainsHealth {
                /// `uavcan.node.Health.1.0`
                ///
                /// Always aligned
                /// Size 8 bits
                pub health0: ::canadensis_data_types::uavcan::node::health_1_0::Health,
                /// `uavcan.node.Health.1.0`
                ///
                /// Always aligned
                /// Size 8 bits
                pub health1: ::canadensis_data_types::uavcan::node::health_1_0::Health,
            }
            impl ::canadensis_encoding::DataType for ContainsHealth {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for ContainsHealth {}
            impl ContainsHealth {}
            impl ::canadensis_encoding::Serialize for ContainsHealth {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.health0);
                    cursor.write_composite(&self.health1);
                }
            }
            impl ::canadensis_encoding::Deserialize for ContainsHealth {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ContainsHealth {
                        health0: { cursor.read_composite()? },
                        health1: { cursor.read_composite()? },
                    })
                }
            }
        }
    }
}
```

Note that this file contains only the custom `ContainsHealth` type, and refers to
the existing `Health` type `canadensis_data_types::uavcan::node::health_1_0::Health`.

## Limitations

* Types that support zero-copy serialization/deserialization are always labeled
  `#[repr(C, packed)`, but sometimes they don't need to be packed and `#[repr(C)]`
  would be sufficient. Packed structs are not fun to work with because
  references to their fields are not allowed and derives can't be used on them.
* Some generated serialization/deserialization code does not take full
  advantage of fields that are always aligned
