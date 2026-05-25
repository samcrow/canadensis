//!
//! https://github.com/samcrow/canadensis/issues/62
//!

extern crate alloc;

mod serialize {
    use alloc::vec::Vec;
    use canadensis_core::{nb, OutOfMemoryError};
    use canadensis_encoding::{Serialize, WriteCursor};
    use core::iter;
    use fallible_collections::{FallibleVec, TryReserveError};

    /// Payloads above this size (in bytes) will use a dynamically allocated buffer
    const STACK_THRESHOLD: usize = 64;

    /// Serializes a payload into a buffer and passes the buffer to a closure
    pub(crate) fn do_serialize<T, F, R, E>(payload: &T, operation: F) -> nb::Result<R, E>
    where
        T: Serialize,
        F: FnOnce(&[u8]) -> nb::Result<R, E>,
        E: From<OutOfMemoryError>,
    {
        let payload_bytes = payload.size_bits().div_ceil(8);
        if payload_bytes > STACK_THRESHOLD {
            let mut bytes: Vec<u8> =
                FallibleVec::try_with_capacity(payload_bytes).map_err(|e: TryReserveError| {
                    nb::Error::Other(E::from(OutOfMemoryError::from(e)))
                })?;
            bytes.extend(iter::repeat_n(0, payload_bytes));
            payload.serialize(&mut WriteCursor::new(&mut bytes));
            operation(&bytes)
        } else {
            let mut bytes = [0u8; STACK_THRESHOLD];
            let bytes = &mut bytes[..payload_bytes];
            payload.serialize(&mut WriteCursor::new(bytes));
            operation(bytes)
        }
    }
}

canadensis_macro::types_from_dsdl! {
    type "uavcan.primitive.String.1.0" { r#"
uint8[<=256] value

@sealed
@assert _offset_ % 8 == {0}
@assert _offset_.max / 8 == 258
    "#}

    type "test.Bar.1.0" { r#"
uint16 start_index
uint8 max_count

@extent 64

---

test.Foo.1.0[<=32] endpoints
bool complete

@extent 32 * (512 + 4) * 8 + 64
    "#}

    type "test.Foo.1.0" { r#"
uavcan.primitive.String.1.0 unit
bool enabled
# void7
@extent 512 * 8
    "#}

    generate()
}

#[test]
fn test_issue_62() {
    let mut endpoints = heapless::Vec::new();
    assert!(endpoints
        .push(test::foo_1_0::Foo {
            unit: uavcan::primitive::string_1_0::String {
                value: heapless::Vec::from_slice(b"foo").unwrap(),
            },
            enabled: false,
        })
        .is_ok());

    let payload = test::bar_1_0::BarResponse {
        endpoints,
        complete: true,
    };

    let _ = serialize::do_serialize(&payload, |_payload| {
        Ok::<(), canadensis_core::nb::Error<canadensis_core::OutOfMemoryError>>(())
    });
}
