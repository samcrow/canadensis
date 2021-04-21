extern crate canadensis_encoding;

use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Serialize, WriteCursor,
};

struct Inner {
    a: bool,
    b: bool,
    c: bool,
    // Really 5 bits
    d: u8,
}

impl DataType for Inner {
    /// Sealed
    const EXTENT_BYTES: Option<u32> = None;
}

struct Outer {
    // Really 13 bits
    a: u16,
    /// A sealed 8-bit composite type
    ///
    /// Although the previous field is only 13 bits, this will be 8-bit aligned like all
    /// composite types.
    inner: Inner,
    // Really 41 bits
    b: u64,
}

impl DataType for Outer {
    // 12 bytes = 96 bits extent
    const EXTENT_BYTES: Option<u32> = Some(12);
}

impl Serialize for Inner {
    fn size_bits(&self) -> usize {
        8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_bool(self.a);
        cursor.write_bool(self.a);
        cursor.write_bool(self.c);
        cursor.write_u5(self.d);
    }
}

impl Deserialize for Inner {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 8
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.a = cursor.read_bool();
        self.b = cursor.read_bool();
        self.c = cursor.read_bool();
        self.d = cursor.read_u5();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = Inner {
            a: false,
            b: false,
            c: false,
            d: 0,
        };
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}

impl Serialize for Outer {
    fn size_bits(&self) -> usize {
        // This gets rounded up to a multiple of 8, because composite types always have 8-bit
        // alignment
        72
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_u13(self.a);
        cursor.align_to_8_bits();
        cursor.write_composite(&self.inner);
        cursor.align_to_8_bits();
        cursor.write_u41(self.b);
    }
}

impl Deserialize for Outer {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 72
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.a = cursor.read_u13();
        cursor.align_to_8_bits();
        self.inner = cursor.read_composite()?;
        cursor.align_to_8_bits();
        self.b = cursor.read_u41();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = Outer {
            a: 0,
            inner: Inner {
                a: false,
                b: false,
                c: false,
                d: 0,
            },
            b: 0,
        };
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}

// TODO: Write actual tests
