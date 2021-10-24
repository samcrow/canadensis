//!
//! Delimited serialization example
//!
//! <https://forum.uavcan.org/t/delimited-serialization-example/975>
//!

extern crate canadensis_encoding;
extern crate heapless;

use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor,
};

#[derive(Debug, Clone, PartialEq)]
enum A {
    Sea(BSealed),
    Del(BDelimited),
}

impl DataType for A {
    const EXTENT_BYTES: Option<u32> = Some(56);
}

impl Serialize for A {
    fn size_bits(&self) -> usize {
        match self {
            A::Sea(sealed) => 8 + 32 + align_up_to_8_bits(sealed.size_bits()),
            A::Del(delimited) => 8 + 32 + align_up_to_8_bits(delimited.size_bits()),
        }
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        match self {
            A::Sea(sealed) => {
                // Tag
                cursor.write_aligned_u8(0);
                cursor.write_composite(sealed);
            }
            A::Del(delimited) => {
                // Tag
                cursor.write_aligned_u8(1);
                cursor.write_composite(delimited);
            }
        }
    }
}

impl Deserialize for A {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let tag = cursor.read_aligned_u8();
        match tag {
            0 => Ok(A::Sea(cursor.read_composite()?)),
            1 => Ok(A::Del(cursor.read_composite()?)),
            _ => Err(DeserializeError::UnionTag),
        }
    }
}

///
/// ```ignore
/// # BSealed.1.0
/// CVariable.1.0[<=2] var
/// CFixed.1.0[<=2]    fix
/// @sealed
/// ```
///
/// Maximum length 30 bytes (var length + up to 2x 8-byte CVariables + fix length + up to 2 6-byte CFixeds)
#[derive(Debug, Clone, PartialEq)]
struct BSealed {
    var: heapless::Vec<CVariable, 2>,
    fix: heapless::Vec<CFixed, 2>,
}

impl DataType for BSealed {
    const EXTENT_BYTES: Option<u32> = None;
}

impl Serialize for BSealed {
    fn size_bits(&self) -> usize {
        8 + self
            .var
            .get(0)
            .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
            .unwrap_or(0)
            + self
                .var
                .get(1)
                .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
                .unwrap_or(0)
            + 8
            + self
                .fix
                .get(0)
                .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
                .unwrap_or(0)
            + self
                .fix
                .get(1)
                .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
                .unwrap_or(0)
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u8(self.var.len() as u8);
        for value in &self.var {
            cursor.write_composite(value);
        }
        cursor.write_u8(self.fix.len() as u8);
        for value in &self.fix {
            cursor.write_composite(value);
        }
    }
}

impl Deserialize for BSealed {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let var = {
            let mut var = heapless::Vec::new();
            let length = cursor.read_aligned_u8();
            if usize::from(length) <= var.capacity() {
                for _ in 0..length {
                    var.push(cursor.read_composite()?).unwrap();
                }
                var
            } else {
                return Err(DeserializeError::ArrayLength);
            }
        };
        let fix = {
            let mut fix = heapless::Vec::new();
            let length = cursor.read_aligned_u8();
            if usize::from(length) <= fix.capacity() {
                for _ in 0..length {
                    fix.push(cursor.read_composite()?).unwrap();
                }
                fix
            } else {
                return Err(DeserializeError::ArrayLength);
            }
        };
        Ok(BSealed { var, fix })
    }
}

///
/// ```ignore
/// # BDelimited.1.0
/// CVariable.1.0[<=2] var
/// CFixed.1.0[<=2]    fix
/// @extent 40 * 8
/// ```
///
/// Maximum length 30 bytes (var length + up to 2x 8-byte CVariables + fix length + up to 2 6-byte CFixeds)
/// plus delimiter header
#[derive(Debug, Clone, PartialEq)]
struct BDelimited {
    var: heapless::Vec<CVariable, 2>,
    fix: heapless::Vec<CFixed, 2>,
}

impl DataType for BDelimited {
    const EXTENT_BYTES: Option<u32> = Some(40);
}

impl Serialize for BDelimited {
    fn size_bits(&self) -> usize {
        8 + self
            .var
            .get(0)
            .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
            .unwrap_or(0)
            + self
                .var
                .get(1)
                .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
                .unwrap_or(0)
            + 8
            + self
                .fix
                .get(0)
                .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
                .unwrap_or(0)
            + self
                .fix
                .get(1)
                .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
                .unwrap_or(0)
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u8(self.var.len() as u8);
        for value in &self.var {
            cursor.write_composite(value);
        }
        cursor.write_u8(self.fix.len() as u8);
        for value in &self.fix {
            cursor.write_composite(value);
        }
    }
}
impl Deserialize for BDelimited {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let var = {
            let mut var = heapless::Vec::new();
            let length = cursor.read_aligned_u8();
            if usize::from(length) <= var.capacity() {
                for _ in 0..length {
                    var.push(cursor.read_composite()?).unwrap();
                }
                var
            } else {
                return Err(DeserializeError::ArrayLength);
            }
        };
        let fix = {
            let mut fix = heapless::Vec::new();
            let length = cursor.read_aligned_u8();
            if usize::from(length) <= fix.capacity() {
                for _ in 0..length {
                    fix.push(cursor.read_composite()?).unwrap();
                }
                fix
            } else {
                return Err(DeserializeError::ArrayLength);
            }
        };
        Ok(BDelimited { var, fix })
    }
}

///
/// ```ignore
/// # CVariable.1.0
/// uint8[<=2] a
/// int8 b
/// @extent 4 * 8
/// ```
///
/// Maximum length 4 bytes (plus delimiter header)
#[derive(Debug, Clone, PartialEq)]
struct CVariable {
    a: heapless::Vec<u8, 2>,
    b: u8,
}

impl DataType for CVariable {
    const EXTENT_BYTES: Option<u32> = Some(4);
}

impl Serialize for CVariable {
    fn size_bits(&self) -> usize {
        8 + self.a.len() * 8 + 8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u8(self.a.len() as u8);
        cursor.write_aligned_bytes(&self.a);
        cursor.write_aligned_u8(self.b);
    }
}

impl Deserialize for CVariable {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut a = heapless::Vec::new();
        let a_length = cursor.read_aligned_u8();
        if usize::from(a_length) <= a.capacity() {
            for _ in 0..a_length {
                a.push(cursor.read_aligned_u8()).unwrap();
            }
        }
        let b = cursor.read_aligned_u8();
        Ok(CVariable { a, b })
    }
}

///
/// ```ignore
/// # CFixed.1.0
/// uint8[2] a
/// @extent 4 * 8
/// ```
///
/// Maximum length 2 bytes (plus delimiter header)
#[derive(Debug, Clone, PartialEq)]
struct CFixed {
    a: [u8; 2],
}

impl Serialize for CFixed {
    fn size_bits(&self) -> usize {
        16
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u8(self.a[0]);
        cursor.write_aligned_u8(self.a[1]);
    }
}

impl DataType for CFixed {
    const EXTENT_BYTES: Option<u32> = Some(4);
}

impl Deserialize for CFixed {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let a0 = cursor.read_aligned_u8();
        let a1 = cursor.read_aligned_u8();
        Ok(CFixed { a: [a0, a1] })
    }
}

/// Rounds a value up to a multiple of 8
fn align_up_to_8_bits(value: usize) -> usize {
    (value + 7) & !7
}

#[test]
fn serialize_1() {
    let a = A::Del(BDelimited {
        var: heapless::Vec::from_slice(&[
            CVariable {
                a: heapless::Vec::from_slice(&[1, 2]).unwrap(),
                b: 0,
            },
            CVariable {
                a: heapless::Vec::from_slice(&[3]).unwrap(),
                b: 4,
            },
        ])
        .unwrap(),
        fix: heapless::Vec::from_slice(&[CFixed { a: [5, 6] }]).unwrap(),
    });

    let expected_bytes: [u8; 28] = [
        0x01, 0x17, 0x00, 0x00, 0x00, 0x02, 0x04, 0x00, 0x00, 0x00, 0x02, 0x01, 0x02, 0x00, 0x03,
        0x00, 0x00, 0x00, 0x01, 0x03, 0x04, 0x01, 0x02, 0x00, 0x00, 0x00, 0x05, 0x06,
    ];

    let mut actual_bytes = [0u8; 28];
    a.serialize_to_bytes(&mut actual_bytes);

    assert_eq!(expected_bytes, actual_bytes);
}

#[test]
fn deserialize_1() {
    let a = A::Del(BDelimited {
        var: heapless::Vec::from_slice(&[
            CVariable {
                a: heapless::Vec::from_slice(&[1, 2]).unwrap(),
                b: 0,
            },
            CVariable {
                a: heapless::Vec::from_slice(&[3]).unwrap(),
                b: 4,
            },
        ])
        .unwrap(),
        fix: heapless::Vec::from_slice(&[CFixed { a: [5, 6] }]).unwrap(),
    });

    let expected_bytes: [u8; 28] = [
        0x01, 0x17, 0x00, 0x00, 0x00, 0x02, 0x04, 0x00, 0x00, 0x00, 0x02, 0x01, 0x02, 0x00, 0x03,
        0x00, 0x00, 0x00, 0x01, 0x03, 0x04, 0x01, 0x02, 0x00, 0x00, 0x00, 0x05, 0x06,
    ];
    let deserialized = A::deserialize_from_bytes(&expected_bytes).unwrap();
    assert_eq!(a, deserialized);
}

#[derive(Debug, Clone, PartialEq)]
enum A11 {
    Sea(BSealed),
    Del(BDelimited11),
}

impl DataType for A11 {
    const EXTENT_BYTES: Option<u32> = Some(56);
}

impl Serialize for A11 {
    fn size_bits(&self) -> usize {
        match self {
            A11::Sea(sealed) => 8 + 32 + align_up_to_8_bits(sealed.size_bits()),
            A11::Del(delimited) => 8 + 32 + align_up_to_8_bits(delimited.size_bits()),
        }
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        match self {
            A11::Sea(sealed) => {
                // Tag
                cursor.write_aligned_u8(0);
                cursor.write_composite(sealed);
            }
            A11::Del(delimited) => {
                // Tag
                cursor.write_aligned_u8(1);
                cursor.write_composite(delimited);
            }
        }
    }
}

impl Deserialize for A11 {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let tag = cursor.read_aligned_u8();
        match tag {
            0 => Ok(A11::Sea(cursor.read_composite()?)),
            1 => Ok(A11::Del(cursor.read_composite()?)),
            _ => Err(DeserializeError::UnionTag),
        }
    }
}

///
/// ```ignore
/// # BDelimited.1.1
/// CVariable.1.1[<=2] var
/// CFixed.1.1[<=2]    fix
/// @extent 40 * 8
/// ```
///
/// Maximum length 32 bytes (var length + up to 2x 7-byte CVariables + fix length + up to 2x 8-byte CFixeds)
/// plus delimiter header
#[derive(Debug, Clone, PartialEq)]
struct BDelimited11 {
    var: heapless::Vec<CVariable11, 2>,
    fix: heapless::Vec<CFixed11, 2>,
}

impl DataType for BDelimited11 {
    const EXTENT_BYTES: Option<u32> = Some(40);
}

impl Serialize for BDelimited11 {
    fn size_bits(&self) -> usize {
        8 + self
            .var
            .get(0)
            .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
            .unwrap_or(0)
            + self
                .var
                .get(1)
                .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
                .unwrap_or(0)
            + 8
            + self
                .fix
                .get(0)
                .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
                .unwrap_or(0)
            + self
                .fix
                .get(1)
                .map(|entry| 32 + align_up_to_8_bits(entry.size_bits()))
                .unwrap_or(0)
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u8(self.var.len() as u8);
        for value in &self.var {
            cursor.write_composite(value);
        }
        cursor.write_u8(self.fix.len() as u8);
        for value in &self.fix {
            cursor.write_composite(value);
        }
    }
}
impl Deserialize for BDelimited11 {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let var = {
            let mut var = heapless::Vec::new();
            let length = cursor.read_aligned_u8();
            if usize::from(length) <= var.capacity() {
                for _ in 0..length {
                    var.push(cursor.read_composite()?).unwrap();
                }
                var
            } else {
                return Err(DeserializeError::ArrayLength);
            }
        };
        let fix = {
            let mut fix = heapless::Vec::new();
            let length = cursor.read_aligned_u8();
            if usize::from(length) <= fix.capacity() {
                for _ in 0..length {
                    fix.push(cursor.read_composite()?).unwrap();
                }
                fix
            } else {
                return Err(DeserializeError::ArrayLength);
            }
        };
        Ok(BDelimited11 { var, fix })
    }
}

///
/// ```ignore
/// # CVariable.1.1
/// uint8[<=2] a
/// @extent 4 * 8
/// ```
///
/// Maximum length 3 bytes (plus delimiter header)
#[derive(Debug, Clone, PartialEq)]
struct CVariable11 {
    a: heapless::Vec<u8, 2>,
}

impl DataType for CVariable11 {
    const EXTENT_BYTES: Option<u32> = Some(4);
}

impl Serialize for CVariable11 {
    fn size_bits(&self) -> usize {
        8 + self.a.len() * 8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u8(self.a.len() as u8);
        cursor.write_aligned_bytes(&self.a);
    }
}

impl Deserialize for CVariable11 {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut a = heapless::Vec::new();
        let a_length = cursor.read_aligned_u8();
        if usize::from(a_length) <= a.capacity() {
            for _ in 0..a_length {
                a.push(cursor.read_aligned_u8()).unwrap();
            }
        }
        Ok(CVariable11 { a })
    }
}

///
/// ```ignore
/// # CFixed.1.1
/// uint8[3] a
/// int8 b
/// @extent 4 * 8
/// ```
///
/// Maximum length 4 bytes (plus delimiter header)
#[derive(Debug, Clone, PartialEq)]
struct CFixed11 {
    a: [u8; 3],
    b: u8,
}

impl Serialize for CFixed11 {
    fn size_bits(&self) -> usize {
        32
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u8(self.a[0]);
        cursor.write_aligned_u8(self.a[1]);
        cursor.write_aligned_u8(self.a[2]);
        cursor.write_aligned_u8(self.b);
    }
}

impl DataType for CFixed11 {
    const EXTENT_BYTES: Option<u32> = Some(4);
}

impl Deserialize for CFixed11 {
    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let a0 = cursor.read_aligned_u8();
        let a1 = cursor.read_aligned_u8();
        let a2 = cursor.read_aligned_u8();
        let b = cursor.read_aligned_u8();
        Ok(CFixed11 { a: [a0, a1, a2], b })
    }
}

/// Takes bytes serialized from A v1.0 and deserializes them as A v1.1
#[test]
fn deserialize_new_version() {
    let a = A11::Del(BDelimited11 {
        var: heapless::Vec::from_slice(&[
            CVariable11 {
                a: heapless::Vec::from_slice(&[1, 2]).unwrap(),
                // b implicitly truncated
            },
            CVariable11 {
                a: heapless::Vec::from_slice(&[3]).unwrap(),
                // b implicitly truncated
            },
        ])
        .unwrap(),
        // a[2] and b implicitly zero-extended
        fix: heapless::Vec::from_slice(&[CFixed11 { a: [5, 6, 0], b: 0 }]).unwrap(),
    });

    let expected_bytes: [u8; 28] = [
        0x01, 0x17, 0x00, 0x00, 0x00, 0x02, 0x04, 0x00, 0x00, 0x00, 0x02, 0x01, 0x02, 0x00, 0x03,
        0x00, 0x00, 0x00, 0x01, 0x03, 0x04, 0x01, 0x02, 0x00, 0x00, 0x00, 0x05, 0x06,
    ];
    let deserialized = A11::deserialize_from_bytes(&expected_bytes).unwrap();
    assert_eq!(a, deserialized);
}
