# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- `canadensis_dsdl_parser`: Breaking change: Added Config, with options for `byte`, `utf8`, and `saturated bool`
- `canadensis_dsdl_parser`: Added support for `byte` and `utf8` types (this is a breaking change for code that uses
  this library, but it does not break compatibility with any existing DSDL file)
- `canadensis_dsdl_frontend`: Breaking change: Added Config, with options for `byte`, `utf8`, and `saturated bool`
- `canadensis_macro`: Added options for `byte`, `utf8`, and `saturated bool` (not a breaking change)
- `canadensis_codegen_rust`: Added unstable optional support for `byte` and `utf8` (code generation is the same as for uint8)
- `canadensis_codegen_rust`: Added unstable option to forbid `saturated bool`
- `canadensis`: Breaking change: Made instant non-generic, now using `fugit` time types
- `canadensis_bxcan`: Breaking change: Made instant non-generic, now using `fugit` time types
- `canadensis_can`: Breaking change: Made instant non-generic, now using `fugit` time types
- `canadensis_codegen_rust`: Generated code no longer denies the `unaligned_references` lint, which is now a hard error
- `canadensis_core`: Breaking change: Made instant non-generic, now using `fugit` time types
- `canadensis_header`: Breaking change: Made instant non-generic, now using `fugit` time types
- `canadensis_udp`: Breaking change: Made instant non-generic, now using `fugit` time types
- `canadensis_udp`: Breaking change: Removed `embedded-nal` dependency, switched to using IP address types from
  `core::net` (requires rustc 1.77.0 or later)
- `canadensis_data_types`: Updated to the latest public_regulated_data types from revision 
  f9f67906cc0ca5d7c1b429924852f6b28f313cbf

## [canadensis-v0.3.3](https://github.com/samcrow/canadensis/releases/tag/canadensis-v0.3.3) - 2025-03-23

### Fixed

- `canadensis`: Fixed `half` dependency version to avoid a breaking change

## [canadensis_codegen_rust-v0.4.3](https://github.com/samcrow/canadensis/releases/tag/canadensis_codegen_rust-v0.4.3) - 2025-03-23

### Fixed

- `canadensis_codegen_rust`: Fixed `half` dependency version for generated code to avoid a breaking change

## [canadensis_data_types-v0.3.2](https://github.com/samcrow/canadensis/releases/tag/canadensis_data_types-v0.3.2) - 2025-03-23

### Fixed

- `canadensis_data_types`: Fixed `half` dependency version to avoid a breaking change

## [canadensis_dsdl_frontend-v0.4.2](https://github.com/samcrow/canadensis/releases/tag/canadensis_dsdl_frontend-v0.4.2) - 2025-03-23

### Fixed

- `canadensis_dsdl_frontend`: Fixed `half` dependency version to avoid a breaking change

## [canadensis_encoding-v0.3.2](https://github.com/samcrow/canadensis/releases/tag/canadensis_encoding-v0.3.2) - 2025-03-23

### Fixed

- `canadensis_encoding`: Fixed `half` dependency version to avoid a breaking change

## [canadensis_macro-v0.3.2](https://github.com/samcrow/canadensis/releases/tag/canadensis_macro-v0.3.2) - 2025-03-23

### Fixed

- `canadensis_macro`: Fixed `half` dependency version to avoid a breaking change

## [canadensis-v0.3.2](https://github.com/samcrow/canadensis/releases/tag/canadensis-v0.3.2) - 2024-11-07

### Changed

- Updated `fallible_collections` to v0.5.1 [#32]

### Fixed

- Avoid allocated space for more than one subscription [#32]

## [canadensis_bxcan-v0.3.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_bxcan-v0.3.1) - 2024-11-07

### Changed

- Updated `fallible_collections` to v0.5.1 [#32]

## [canadensis_can-v0.3.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_can-v0.3.1) - 2024-11-07

### Changed

- Updated `fallible_collections` to v0.5.1 [#32]

## [canadensis_core-v0.3.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_core-v0.3.1) - 2024-11-07

### Changed

- Updated `fallible_collections` to v0.5.1 [#32]

## [canadensis_serial-v0.3.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_serial-v0.3.1) - 2024-11-07

### Changed

- Updated `fallible_collections` to v0.5.1 [#32]

## [canadensis_udp-v0.4.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_udp-v0.4.1) - 2024-11-07

### Changed

- Updated `fallible_collections` to v0.5.1 [#32]

[#32]: https://github.com/samcrow/canadensis/pull/32

## [canadensis-v0.3.1](https://github.com/samcrow/canadensis/releases/tag/canadensis-v0.3.1) - 2023-10-05

### Fixed

- `canadensis`: Added default-features = false to half dependency to fix no_std compatibility

## [canadensis_macro-v0.3.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_macro-v0.3.1) - 2023-10-05

### Fixed

- `canadensis_macro`: Added default-features = false to half dependency to fix no_std compatibility

## [canadensis_data_types-v0.3.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_data_types-v0.3.1) - 2023-09-05

### Fixed

- `canadensis_data_types`: Added default-features = false to half dependency to fix no_std compatibility

## [canadensis_codegen_rust-v0.4.2](https://github.com/samcrow/canadensis/releases/tag/canadensis_codegen_rust-v0.4.2) - 2023-09-05

### Fixed

- `canadensis_codegen_rust`: Added default-features = false to half dependency in generated code to fix no_std
  compatibility

## [canadensis_encoding-v0.3.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_encoding-v0.3.1) - 2023-09-05

### Fixed

- `canadensis_encoding`: Added default-features = false to half dependency to fix no_std compatibility

## [canadensis_dsdl_frontend-v0.4.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_dsdl_frontend-v0.4.1) - 2023-09-05

### Fixed

- `canadensis_dsdl_frontend`: Fixed duplicate error text

## [canadensis_bxcan-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_bxcan-v0.3.0) - 2023-03-30

### Added

- `canadensis_bxcan`: Added support for loopback frames, with timestamps based on the transmit time calculated by
  software

### Changed

- `canadensis_bxcan`: Clippy fixes
- `canadensis_bxcan`: Breaking change: Removed deprecated functions `bxcan_frame_to_uavcan` and `uavcan_frame_to_bxcan`,
  made the replacement functions private
- `canadensis_bxcan`: Breaking change: Updated `bxcan` dependency to 0.7
- `canadensis_bxcan`: Breaking change: Moved clock down into drivers to make frame timestamps more accurate

## [canadensis_linux-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_linux-v0.3.0) - 2023-03-30
- `canadensis_linux`: Breaking change: Moved clock down into drivers to make frame timestamps more accurate

## [canadensis_pnp_client-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_pnp_client-v0.3.0) - 2023-03-30

### Changed
- `canadensis_pnp_client`: Updated `crc-any` dependency to 2.4
- `canadensis_pnp_client`: Breaking change: Moved clock down into drivers to make frame timestamps more accurate

## [canadensis-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis-v0.3.0) - 2023-03-30

### Added
- `canadensis`: Added loopback publishing/requesting functions and loopback handler function
### Changed
- `canadensis`: Breaking change: Replaced `uavcan.node.port.List.0.1` and associated data types with version 1.0.
  The DSDL and the structures of the types have not changed, only the version number.
- `canadensis`: Clippy fixes
- `canadensis`: Breaking change: Updated `half` dependency to 2.2
- `canadensis`: Updated private dependencies
- `canadensis`: Breaking change: Moved clock down into drivers to make frame timestamps more accurate

### Changed

## [canadensis_derive_register_block-v0.2.2](https://github.com/samcrow/canadensis/releases/tag/canadensis_derive_register_block-v0.2.2) - 2023-03-30

### Changed
- `canadensis_derive_register_block`: Updated `syn` dependency to 2.0

## [canadensis_data_types-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_data_types-v0.3.0) - 2023-03-30

### Added
- `canadensis_data_types`: Added documentation from DSDL comments

### Changed
- `canadensis_data_types`: Changed the order of constants to match the order in the DSDL files
- `canadensis_data_types`: Regenerated code to match revision 935973babe11755d8070e67452b3508b4b6833e2
  of <https://github.com/OpenCyphal/public_regulated_data_types/>
- `canadensis_data_types`: Breaking change: Updated `half` dependency to 2.2, `memoffset` to 0.8

## [canadensis_serial-v0.4.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_serial-v0.4.0) - 2023-03-30

### Changed
- `canadensis_serial`: Breaking change: Changed header format
- `canadensis_serial`: Updated `simplelog` dependency to 0.12
- `canadensis_serial`: Breaking change: Moved clock down into drivers to make frame timestamps more accurate

## [canadensis_udp-v0.4.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_udp-v0.4.0) - 2023-03-30

### Changed
- `canadensis_udp`: Major rework for [new version of Cyphal/UDP](https://forum.opencyphal.org/t/cyphal-udp-architectural-issues-caused-by-the-dependency-between-the-nodes-ip-address-and-its-identity/1765/60)
- `canadensis_udp`: Breaking change: Changed header format
- `canadensis_udp`: Clippy fixes
- `canadensis_udp`: Updated `simplelog` dependency to 0.12
- `canadensis_udp`: Breaking change: Moved clock down into drivers to make frame timestamps more accurate

## [canadensis_header-v0.1.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_header-v0.1.0) - 2023-03-30

### Added
- `canadensis_header`: This is a new crate that specifies [the frame header format](https://forum.opencyphal.org/t/cyphal-udp-architectural-issues-caused-by-the-dependency-between-the-nodes-ip-address-and-its-identity/1765/60)
  that Cyphal/UDP and Cyphal/Serial use

## [canadensis_can-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_can-v0.3.0) - 2023-03-30

### Added
- `canadensis_can`: Breaking change: Added support for loopback frames and transfers
### Changed
- `canadensis_can`: Breaking change: Moved clock down into drivers to make frame timestamps more accurate

## [canadensis_macro-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_macro-v0.3.0) - 2023-03-30

### Changed
- `canadensis_macro`: Code generation can fail with an error
- `canadensis_macro`: Updated `syn` dependency to 2.0
- `canadensis_macro`: Breaking change: Updated dependencies of generated code: `half` to 2.2, `memoffset` to 0.8

## [canadensis_codegen_rust-v0.4.1](https://github.com/samcrow/canadensis/releases/tag/canadensis_codegen_rust-v0.4.1) - 2023-03-30

### Fixed
- Fixed incorrect versions of dependencies printed by `canadensis_codegen_rust print-dependencies`

## [canadensis_codegen_rust-v0.4.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_codegen_rust-v0.4.0) - 2023-03-30

### Added
- `canadensis_codegen_rust`: Added the ability to produce documentation comments in generated code based on DSDL
  comments
- `canadensis_codegen_rust`: Added the ability to generate enums from DSDL types marked with `#[canadensis(enum)]`
### Changed
- `canadensis_codegen_rust`: Clippy fixes
- `canadensis_codegen_rust`: Updated `clap` dependency to 4.1
- `canadensis_codegen_rust`: Breaking change: Updated dependencies of generated code: `half` to 2.2, `memoffset` to 0.8
- `canadensis_codegen_rust`: Breaking change: Code generation can fail with an error

## [canadensis_dsdl_frontend-v0.4.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_dsdl_frontend-v0.4.0) - 2023-03-30

### Changed
- `canadensis_dsdl_frontend`: Breaking change: Added comment handling, made some struct fields private
- `canadensis_dsdl_frontend`: Constants are now stored in the order they are declared in the DSDL file, instead of
  alphabetical order by name
- `canadensis_dsdl_frontend`: Clippy fixes
- `canadensis_dsdl_frontend`: Breaking change: Updated `half` dependency to 2.2

## [canadensis_dsdl_parser-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_dsdl_parser-v0.3.0) - 2023-03-30

### Added
- `canadensis_dsdl_parser`: Breaking change: Added comment parsing
### Changed
- `canadensis_dsdl_parser`: Breaking change: Made `Error` an opaque type instead of an alias to the Pest error type

## [canadensis_encoding-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_encoding-v0.3.0) - 2023-03-30

### Changed
- `canadensis_encoding`: Clippy fixes
- `canadensis_encoding`: Breaking change: Updated `half` dependency to 2.2

## [canadensis_core-v0.3.0](https://github.com/samcrow/canadensis/releases/tag/canadensis_core-v0.3.0) - 2023-03-29

### Added
- `canadensis_core`: Added `remove` function to `SessionTracker` (Breaking change)
- `canadensis_core`: Added `loopback` field to transfer types (Breaking change)
### Changed
- `canadensis_core`: Breaking change: Moved clock down into drivers to make frame timestamps more accurate

## [canadensis_write_crc-0.1.4](https://github.com/samcrow/canadensis/releases/tag/canadensis_write_crc-v0.1.4) - 2023-03-29

### Changed
- `canadensis_write_crc`: Updated `object` dependency to 0.30

## [canadensis_core-v0.2.3](https://github.com/samcrow/canadensis/tree/canadensis_core-v0.2.3) - 2022-10-18

### Changed
- `canadensis_core`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_encoding-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_encoding-v0.2.2) - 2022-10-18

### Changed
- `canadensis_encoding`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_data_types-v0.2.3](https://github.com/samcrow/canadensis/tree/canadensis_data_types-v0.2.3) - 2022-10-18

### Changed
- Updated instructions in readme to use the `--rustfmt` option instead of a separate formatting step

## [canadensis-v0.2.4](https://github.com/samcrow/canadensis/tree/canadensis-v0.2.4) - 2022-10-18
### Fixed
- `canadensis`: `register_client` example now compiles correctly after an update to `heapless` added
  a new restriction on the size of some containers
### Changed
- `canadensis`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_filter_config-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_filter_config-v0.2.2) - 2022-10-18

### Changed
- `canadensis_filter_config`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_can-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_can-v0.2.2) - 2022-10-18

### Changed
- `canadensis_can`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_linux-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_linux-v0.2.2) - 2022-10-18

### Changed
- `canadensis_linux`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_serial-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_serial-v0.2.2) - 2022-10-18

### Changed
- `canadensis_serial`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_udp-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_udp-v0.2.2) - 2022-10-18

### Changed
- `canadensis_udp`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_bit_length_set-v0.3.1](https://github.com/samcrow/canadensis/tree/canadensis_bit_length_set-v0.3.1) - 2022-10-18

### Changed
- `canadensis_bit_length_set`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_pnp_client-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_pnp_client-v0.2.2) - 2022-10-18

### Changed
- `canadensis_pnp_client`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_bxcan-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_bxcan-v0.2.2) - 2022-10-18

### Changed
- `canadensis_bxcan`: Renamed more occurrences of UAVCAN to Cyphal
- Renamed frame conversion functions
  (the original names are still available as deprecated aliases)

## [canadensis_dsdl_parser-v0.2.3](https://github.com/samcrow/canadensis/tree/canadensis_dsdl_parser-v0.2.3) - 2022-10-18

### Changed
- `canadensis_dsdl_parser`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)
- `canadensis_dsdl_parser`: Removed copy of the public regulated data types. The test code uses the submodule under
  `canadensis_dsdl_frontend` instead.

## [canadensis_dsdl_frontend-v0.3.3](https://github.com/samcrow/canadensis/tree/canadensis_dsdl_frontend-v0.3.3) - 2022-10-18

### Added
- `canadensis_dsdl_frontend`: Added tests for reasonable handling of cyclic dependencies between DSDL types
### Changed
- `canadensis_dsdl_frontend`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)
### Fixed
- `canadensis_dsdl_frontend`: `Package::add_files` now returns an error if passed a path that is not a directory
  (this is consistent with the existing documentation)

## [canadensis_codegen_rust-v0.3.2](https://github.com/samcrow/canadensis/tree/canadensis_codegen_rust-v0.3.2) - 2022-10-18

### Added
- `canadensis_codegen_rust`: Added `--rustfmt` option to format the generated code

### Changed
- `canadensis_codegen_rust`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_macro-v0.2.3](https://github.com/samcrow/canadensis/tree/canadensis_macro-v0.2.3) - 2022-10-18

### Changed
- `canadensis_macro`: Renamed more occurrences of UAVCAN to Cyphal (no public API changes)

## [canadensis_write_crc-v0.1.3](https://github.com/samcrow/canadensis/tree/canadensis_write_crc-v0.1.3) - 2022-10-18

### Changed
- Minor code cleanup (this did not warrant a release)

## [canadensis_data_types-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_data_types-v0.2.2) - 2022-07-14

### Added

- Generated types are marked as deprecated if the
  corresponding DSDL types are deprecated

### Fixed

- Fixed a bug in generated code that caused variable-
  length bit arrays to be serialized incorrectly (missing the length field)

### Changed

- Improved documentation of generated fields

## [canadensis_codegen_rust-v0.3.1](https://github.com/samcrow/canadensis/tree/canadensis_codegen_rust-v0.3.1) - 2022-07-14

### Added

- The compile process now reports warnings when it detects names using
  non-standard case conventions
- Generated types are marked as deprecated if the
  corresponding DSDL types are deprecated

### Fixed

- Fixed a bug that caused generated code to not
  serialize the length field of a variable-length array of bits

### Changed

- Updated private `heck` dependency to 0.4.0
- Application now prints help text instead of
  panicking when run with no subcommand
- Improved documentation of generated fields

## [canadensis_dsdl_frontend-v0.3.2](https://github.com/samcrow/canadensis/tree/canadensis_dsdl_frontend-v0.3.2) - 2022-07-14

### Added

- Support the new `.dsdl` extension for cyphal data types [#12](https://github.com/samcrow/canadensis/pull/12)
- Report warnings for type and field names that do
  not follow the DSDL case conventions

### Fixed

- Fixed a bug that caused service responses to
  not be marked as deprecated
- Added a check that prohibits using a deprecated
  type in a non-deprecated type (the specification requires this check)

### Changed

- The public regulated data types in `canadensis_dsdl_frontend` are now a submodule. This should not
  impact any downstream code, but `git clone --recursive` is now required to make all the tests pass.
  [#13](https://github.com/samcrow/canadensis/pull/13)
- Improved performance by adding a special case for BitLengthSet % integer

## [canadensis_dsdl_frontend-v0.3.1](https://github.com/samcrow/canadensis/tree/canadensis_dsdl_frontend-v0.3.1) - 2022-06-10

- canadensis_dsdl_frontend: Removed long name DSDL tests from filesystem [#10](https://github.com/samcrow/canadensis/pull/10)

## [canadensis-v0.2.3](https://github.com/samcrow/canadensis/tree/canadensis-v0.2.3) - 2022-04-12

### Added

- Implemented `RegisterType` for bool values [#7](https://github.com/samcrow/canadensis/pull/7)

### Changed

- Renamed most instances of UAVCAN in the documentation to Cyphal, following the [renaming](https://forum.opencyphal.org/t/uavcan-v1-is-now-cyphal/1622)

## Renames - 2022-04-12

This section applies to several crate versions:
- [canadensis_core-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_core-v0.2.2)
- [canadensis_encoding-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_encoding-v0.2.1)
- [canadensis_data_types-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_data_types-v0.2.1)
- [canadensis_derive_register_block-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_derive_register_block-v0.2.1)
- [canadensis_filter_config-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_filter_config-v0.2.1)
- [canadensis_can-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_can-v0.2.1)
- [canadensis_linux-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_linux-v0.2.1)
- [canadensis_serial-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_serial-v0.2.1)
- [canadensis_udp-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_udp-v0.2.1)
- [canadensis_pnp_client-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_pnp_client-v0.2.1)
- [canadensis_bxcan-v0.2.1](https://github.com/samcrow/canadensis/tree/canadensis_bxcan-v0.2.1)
- [canadensis_dsdl_parser-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_dsdl_parser-v0.2.2)
- [canadensis_crc-v0.1.1](https://github.com/samcrow/canadensis/tree/canadensis_crc-v0.1.1)
- [canadensis_macro-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis_macro-v0.2.2)
- [canadensis_write_crc-v0.1.2](https://github.com/samcrow/canadensis/tree/canadensis_write_crc-v0.1.2)

### Changed

- Renamed most instances of UAVCAN in the documentation to Cyphal, following the [renaming](https://forum.opencyphal.org/t/uavcan-v1-is-now-cyphal/1622)

## [canadensis_codegen_rust-v0.3.0](https://github.com/samcrow/canadensis/tree/canadensis_codegen_rust-v0.3.0) - 2022-04-12

### Changed

- Updated canadensis_bit_length_set dependency to 0.3.0
  (this probably does not actually change the public interface)
- Renamed most instances of UAVCAN in the documentation to Cyphal, following the [renaming](https://forum.opencyphal.org/t/uavcan-v1-is-now-cyphal/1622)

## [canadensis_dsdl_frontend-v0.3.0](https://github.com/samcrow/canadensis/tree/canadensis_dsdl_frontend-v0.3.0) - 2022-04-12

### Changed

- Breaking change: Added BitLengthSet variant of Value, so bit length sets can be represented symbolically
  during DSDL processing (this may improve performance)
- Breaking change: Updated canadensis_bit_length_set dependency to 0.3.0
- Renamed most instances of UAVCAN in the documentation to Cyphal, following the [renaming](https://forum.opencyphal.org/t/uavcan-v1-is-now-cyphal/1622)

## [canadensis_bit_length_set-v0.3.0](https://github.com/samcrow/canadensis/tree/canadensis_bit_length_set-v0.3.0) - 2022-04-12

### Changed

- Added implementations of Ord and some other traits
- Breaking change: Renamed BitLengthSet functions `min` and `max` to `min_value` and `max_value` to avoid conflicts with
  Ord functions
- Renamed most instances of UAVCAN in the documentation to Cyphal, following the [renaming](https://forum.opencyphal.org/t/uavcan-v1-is-now-cyphal/1622)

## [canadensis-v0.2.2](https://github.com/samcrow/canadensis/tree/canadensis-v0.2.2) - 2022-01-02

### Added

- Added documentation of some values that must be powers of two

### Fixed

- Fixed examples to work with heapless 0.7.9

## [canadensis_write_crc-v0.1.1](https://github.com/samcrow/canadensis/tree/canadensis_write_crc-v0.1.1) - 2022-01-02

### Fixed

- Fixed incorrect CRC calculation in canadensis_write_crc

## [0.2.1](https://github.com/samcrow/canadensis/tree/v0.2.1) - 2021-11-06

This release applies only to the crates `canadensis_core`, `canadensis`, `canadensis_dsdl_parser`, `canadensis_dsdl_frontend`, `canadensis_codegen_rust`, `canadensis_macro`, and `canadensis_bit_length_set`

### Added

- Added missing repository to the Cargo.toml files of `canadensis_dsdl_parser`, `canadensis_dsdl_frontend`, `canadensis_codegen_rust`, `canadensis_macro`, and `canadensis_bit_length_set`
- Added `Display` implementation for `SubjectId`
- Added some `Debug` implementations

### Fixed

- Various index maps using `TrivialHasher` produced incorrect results.
  These bugs were fixed by replacing it with the default `FnvHasher`.

## [0.2.0](https://github.com/samcrow/canadensis/tree/v0.2.0) - 2021-10-31

This section is not complete because version 0.2.0 had too many changes.

### Added

- Experimental UAVCAN/UDP transport
- Experimental UAVCAN/Serial transport
- Transport abstraction (a node can now use any type of transport)
- DSDL parser, frontend, and Rust code generator
- Procedural macro that generates Rust code from inline or external DSDL

### Changed

- The `canadensis` library no longer re-exports anything from `canadensis_can`. Applications need to depend on
  additional libraries for the transports they use.
- Node types moved from `canadensis_node` to `canadensis`
- Node ID and transfer ID types removed from `canadensis_core` because they were CAN-specific (each transport now has
  its own versions of these types)
- Many other changes

## [0.1.0](https://github.com/samcrow/canadensis/tree/v0.1.0) - 2021-07-11

Initial release