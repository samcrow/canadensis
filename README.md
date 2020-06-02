# Canadensis: A UAVCAN 1.0 implementation

This project implements (most of) [UAVCAN](https://uavcan.org/) v1.0. As the UAVCAN website explains, "UAVCAN is an open lightweight protocol designed for reliable intravehicular communication in aerospace and robotic applications over CAN bus, Ethernet, and other robust transports. It is created to address the challenge of deterministic on-board data exchange between systems and components of next-generation intelligent vehicles: manned and unmanned aircraft, spacecraft, robots, and cars."

## Principles

* Runs on embedded devices
* Uses dynamic memory allocation, but only when necessary

## Current features

* Common definitions (`canadensis_core`)
* A transport layer for CAN and CAN FD (`canadensis_can`)
* Transfer serialization and deserialization (`canadensis_encoding`)
* Basic node functions (`canadensis_node`)
* A basic presentation layer for CAN and CAN FD, reexporting other crates (`canadensis`)

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
