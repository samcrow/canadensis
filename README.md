# Canadensis: A UAVCAN 1.0 implementation

This project implements (most of) [UAVCAN](https://uavcan.org/) v1.0. As the UAVCAN website explains, "UAVCAN is an open
lightweight protocol designed for reliable intravehicular communication in aerospace and robotic applications over CAN
bus, Ethernet, and other robust transports. It is created to address the challenge of deterministic on-board data
exchange between systems and components of next-generation intelligent vehicles: manned and unmanned aircraft,
spacecraft, robots, and cars."

This is currently an independent project, not affiliated with the UAVCAN Consortium.

## Published crates

Crate | Description
------|------------
[`canadensis`](https://crates.io/crates/canadensis) ([documentation](https://docs.rs/canadensis)) | The main library with all core transport-independent functionality
[`canadensis_data_types`](https://crates.io/crates/canadensis_data_types) ([documentation](https://docs.rs/canadensis_data_types)) | Rust types corresponding to the [UAVCAN public regulated data types](https://github.com/UAVCAN/public_regulated_data_types)
[`canadensis_can`](https://crates.io/crates/canadensis_can) ([documentation](https://docs.rs/canadensis_bxcan)) | UAVCAN/CAN transport
[`canadensis_bxcan`](https://crates.io/crates/canadensis_bxcan) ([documentation](https://docs.rs/canadensis_bxcan)) | Compatibility for bxCAN embedded CAN controllers
[`canadensis_linux`](https://crates.io/crates/canadensis_linux) ([documentation](https://docs.rs/canadensis_linux)) | Compatibility for Linux SocketCAN interfaces
[`canadensis_serial`](https://crates.io/crates/canadensis_serial) ([documentation](https://docs.rs/canadensis_serial)) | Experimental UAVCAN/Serial transport
[`canadensis_udp`](https://crates.io/crates/canadensis_udp) ([documentation](https://docs.rs/canadensis_udp)) | Experimental UAVCAN/UDP transport
[`canadensis_pnp_client`](https://crates.io/crates/canadensis_pnp_client) ([documentation](https://docs.rs/canadensis_pnp_client)) | A client library for plug-and-play node ID allocation
[`canadensis_crc`](https://crates.io/crates/canadensis_crc) ([documentation](https://docs.rs/canadensis_crc)) | Access to the software image CRC
[`canadensis_write_crc`](https://crates.io/crates/canadensis_write_crc) ([documentation](https://docs.rs/canadensis_write_crc)) | A tool to calculate and write the CRC of a software image for use with `canadensis_crc`
[`canadensis_codegen_rust`](https://crates.io/crates/canadensis_codegen_rust) ([documentation](https://docs.rs/canadensis_codegen_rust)) | A DSDL processor that generates Rust data types and serialization code
[`canadensis_macro`](https://crates.io/crates/canadensis_macro) ([documentation](https://docs.rs/canadensis_macro)) | A procedural macro that generates Rust data types and serialization code from inline and/or external DSDL files


Other crates (`canadensis_bit_length_set`, `canadensis_core`, `canadensis_derive_register_block`,
`canadensis_dsdl_frontend`, `canadensis_dsdl_parser`, `canadensis_encoding`, and
`canadensis_filter_config`) are re-exported in various places, so you normally will not need to depend on them directly.

## Status

This code is intended to conform to version 1.0-beta of the UAVCAN specification.

Most of the functionality works. Some parts are incomplete:

* There are some tests, but there are probably several bugs in areas that have not been tested.
* The amount of dynamic memory allocation can be reduced, or at least documented better.
* It needs better documentation

## Principles

* Runs on embedded devices
* Uses dynamic memory allocation, but only when necessary
* Supports UAVCAN/CAN (classic CAN and CAN FD)
* Supports UAVCAN/Serial and UAVCAN/UDP (these transports are not fully specified yet, but the canadensis
  implementations were compatible with `pyuavcan` when they were released)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
