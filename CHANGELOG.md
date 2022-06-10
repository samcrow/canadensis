# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

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