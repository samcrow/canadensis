#[allow(unused_variables, unused_braces)]
#[deny(unaligned_references)]
#[cfg(not(target_endian = "little"))]
compile_error!("Zero-copy serialization requires a little-endian target");
pub mod canadensis {
    pub mod constants_1_0 {
        /// `canadensis.Constants.1.0`
        ///
        /// Fixed size 0 bytes
        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
        #[repr(C, packed)]
        pub struct Constants {}
        impl ::canadensis_encoding::DataType for Constants {
            const EXTENT_BYTES: Option<u32> = None;
        }
        impl ::canadensis_encoding::Message for Constants {}
        impl Constants {
            pub const A: u8 = 37;
            pub const B: u16 = 18;
            pub const C: u16 = 666;
            pub const D: bool = true;
            pub const E: bool = true;
        }
        impl ::canadensis_encoding::Serialize for Constants {
            fn size_bits(&self) -> usize {
                0
            }
            fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
            }
        }
        impl ::canadensis_encoding::Deserialize for Constants {
            fn deserialize(
                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
            where
                Self: Sized,
            {
                Ok(Self::deserialize_zero_copy(cursor))
            }
        }
        #[test]
        fn test_layout() {
            assert_eq!(::core::mem::size_of::<Constants>() * 8, 0);
        }
    }
    pub mod interesting_0_1 {
        /// `canadensis.Interesting.0.1`
        ///
        /// Size ranges from 14 to 402 bytes
        pub struct Interesting {
            /// `saturated uint1`
            ///
            /// Always aligned
            /// Size 1 bits
            pub a: u8,
            /// `saturated uint32[3]`
            ///
            /// Not always aligned
            /// Size 96 bits
            pub offset_dependent: [u32; 3],
            /// `saturated uint32[<=97]`
            ///
            /// Not always aligned
            /// Size ranges from 0 to 3104 bits
            pub others: ::heapless::Vec<u32, 97>,
        }
        impl ::canadensis_encoding::DataType for Interesting {
            const EXTENT_BYTES: Option<u32> = None;
        }
        impl ::canadensis_encoding::Message for Interesting {}
        impl Interesting {}
        impl ::canadensis_encoding::Serialize for Interesting {
            fn size_bits(&self) -> usize {
                1 + (self.offset_dependent).len() * 32 + 8 + (self.others).len() * 32 + 0
            }
            fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                cursor.write_u1(self.a);
                for value in (self.offset_dependent).iter() {
                    cursor.write_u32(*value);
                }
                cursor.write_u8((self.others).len() as u8);
                for value in (self.others).iter() {
                    cursor.write_u32(*value);
                }
            }
        }
        impl ::canadensis_encoding::Deserialize for Interesting {
            fn deserialize(
                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
            where
                Self: Sized,
            {
                Ok(Interesting {
                    a: { cursor.read_u1() as _ },
                    offset_dependent: {
                        [
                            cursor.read_u32() as _,
                            cursor.read_u32() as _,
                            cursor.read_u32() as _,
                        ]
                    },
                    others: {
                        let length = cursor.read_u8() as _;
                        if length <= 97 {
                            let mut elements = ::heapless::Vec::new();
                            for _ in 0..length {
                                let _ = elements.push(cursor.read_u32() as _);
                            }
                            elements
                        } else {
                            return Err(::canadensis_encoding::DeserializeError::ArrayLength);
                        }
                    },
                })
            }
        }
    }
    pub mod test_0_3 {
        /// `canadensis.Test.0.3`
        ///
        /// Fixed size 8 bytes
        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
        #[repr(C, packed)]
        pub struct Test {
            /// `saturated uint64`
            ///
            /// Always aligned
            /// Size 64 bits
            pub d: u64,
        }
        impl ::canadensis_encoding::DataType for Test {
            const EXTENT_BYTES: Option<u32> = None;
        }
        impl ::canadensis_encoding::Message for Test {}
        impl Test {}
        impl ::canadensis_encoding::Serialize for Test {
            fn size_bits(&self) -> usize {
                64
            }
            fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
            }
        }
        impl ::canadensis_encoding::Deserialize for Test {
            fn deserialize(
                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
            where
                Self: Sized,
            {
                Ok(Self::deserialize_zero_copy(cursor))
            }
        }
        #[test]
        fn test_layout() {
            assert_eq!(::core::mem::size_of::<Test>() * 8, 64);
            assert_eq!(::memoffset::offset_of!(Test, d) * 8, 0);
        }
    }
    pub mod test_1_0 {
        /// `canadensis.Test.1.0`
        ///
        /// Fixed size 4 bytes
        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
        #[repr(C, packed)]
        pub struct TestRequest {
            /// `saturated uint32`
            ///
            /// Always aligned
            /// Size 32 bits
            pub a: u32,
        }
        impl ::canadensis_encoding::DataType for TestRequest {
            const EXTENT_BYTES: Option<u32> = None;
        }
        impl ::canadensis_encoding::Request for TestRequest {}
        impl TestRequest {
            pub const THINGY: u8 = 3;
        }
        impl ::canadensis_encoding::Serialize for TestRequest {
            fn size_bits(&self) -> usize {
                32
            }
            fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
            }
        }
        impl ::canadensis_encoding::Deserialize for TestRequest {
            fn deserialize(
                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
            where
                Self: Sized,
            {
                Ok(Self::deserialize_zero_copy(cursor))
            }
        }
        #[test]
        fn test_layout() {
            assert_eq!(::core::mem::size_of::<TestRequest>() * 8, 32);
            assert_eq!(::memoffset::offset_of!(TestRequest, a) * 8, 0);
        }

        /// `canadensis.Test.1.0`
        ///
        /// Size ranges from 1 to 9 bytes
        pub struct TestResponse {
            /// `saturated float32[<=2]`
            ///
            /// Always aligned
            /// Size ranges from 0 to 64 bits
            pub values: ::heapless::Vec<f32, 2>,
        }
        impl ::canadensis_encoding::DataType for TestResponse {
            const EXTENT_BYTES: Option<u32> = None;
        }
        impl ::canadensis_encoding::Response for TestResponse {}
        impl TestResponse {}
        impl ::canadensis_encoding::Serialize for TestResponse {
            fn size_bits(&self) -> usize {
                8 + (self.values).len() * 32 + 0
            }
            fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                cursor.write_aligned_u8((self.values).len() as u8);
                for value in (self.values).iter() {
                    cursor.write_f32(*value);
                }
            }
        }
        impl ::canadensis_encoding::Deserialize for TestResponse {
            fn deserialize(
                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
            where
                Self: Sized,
            {
                Ok(TestResponse {
                    values: {
                        let length = cursor.read_u8() as _;
                        if length <= 2 {
                            let mut elements = ::heapless::Vec::new();
                            for _ in 0..length {
                                let _ = elements.push(cursor.read_f32());
                            }
                            elements
                        } else {
                            return Err(::canadensis_encoding::DeserializeError::ArrayLength);
                        }
                    },
                })
            }
        }
    }
    pub mod two_hundred_and_fifty_five_characters_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_1_0 {
        /// `canadensis.TwoHundredAndFiftyFiveCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong.1.0`
        ///
        /// Fixed size 0 bytes
        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
        #[repr(C, packed)]
        pub struct TwoHundredAndFiftyFiveCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong
        {}
        impl ::canadensis_encoding::DataType for TwoHundredAndFiftyFiveCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong {
const EXTENT_BYTES: Option<u32> = None;
}
        impl ::canadensis_encoding::Message for TwoHundredAndFiftyFiveCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong {}
        impl TwoHundredAndFiftyFiveCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong {
}
        impl ::canadensis_encoding::Serialize for TwoHundredAndFiftyFiveCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong {
fn size_bits(&self) -> usize { 0 }
fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
}
}
        impl ::canadensis_encoding::Deserialize for TwoHundredAndFiftyFiveCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong {
fn deserialize(cursor: &mut ::canadensis_encoding::ReadCursor<'_>) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError> where Self: Sized {
Ok(Self::deserialize_zero_copy(cursor))
}
}
        #[test]
        fn test_layout() {
            assert_eq!(::core::mem::size_of::<TwoHundredAndFiftyFiveCharactersLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLongLong>() * 8, 0);
        }
    }
    pub mod union_offset_around_sealed_1_0 {
        /// `canadensis.UnionOffsetAroundSealed.1.0`
        ///
        /// Size ranges from 2 to 3 bytes
        pub enum UnionOffsetAroundSealed {
            // saturated uint8
            A(u8),
            // saturated uint16
            B(u16),
        }
        impl ::canadensis_encoding::DataType for UnionOffsetAroundSealed {
            const EXTENT_BYTES: Option<u32> = None;
        }
        impl ::canadensis_encoding::Message for UnionOffsetAroundSealed {}
        impl UnionOffsetAroundSealed {}
        impl ::canadensis_encoding::Serialize for UnionOffsetAroundSealed {
            fn size_bits(&self) -> usize {
                8 + match self {
                    UnionOffsetAroundSealed::A(inner) => 8,
                    UnionOffsetAroundSealed::B(inner) => 16,
                }
            }
            fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                match self {
                    UnionOffsetAroundSealed::A(inner) => {
                        cursor.write_aligned_u8(0);
                        cursor.write_aligned_u8(inner);
                    }
                    UnionOffsetAroundSealed::B(inner) => {
                        cursor.write_aligned_u8(1);
                        cursor.write_aligned_u16(inner);
                    }
                }
            }
        }
        impl ::canadensis_encoding::Deserialize for UnionOffsetAroundSealed {
            fn deserialize(
                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
            where
                Self: Sized,
            {
                match cursor.read_aligned_u8() as _ {
                    0 => Ok(UnionOffsetAroundSealed::A({ cursor.read_u8() as _ })),
                    1 => Ok(UnionOffsetAroundSealed::B({ cursor.read_u16() as _ })),
                    _ => Err(::canadensis_encoding::DeserializeError::UnionTag),
                }
            }
        }
    }
    pub mod uses_constants_1_0 {
        /// `canadensis.UsesConstants.1.0`
        ///
        /// Fixed size 84 bytes
        pub struct UsesConstants {
            /// `saturated bool[666]`
            ///
            /// Always aligned
            /// Size 666 bits
            pub things: ::canadensis_encoding::bits::BitArray<84>,
        }
        impl ::canadensis_encoding::DataType for UsesConstants {
            const EXTENT_BYTES: Option<u32> = None;
        }
        impl ::canadensis_encoding::Message for UsesConstants {}
        impl UsesConstants {
            pub const DEPENDENT: u16 = 666;
            pub const INDEPENDENT: u16 = 192;
        }
        impl ::canadensis_encoding::Serialize for UsesConstants {
            fn size_bits(&self) -> usize {
                672
            }
            fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                (self.things).serialize(cursor);
            }
        }
        impl ::canadensis_encoding::Deserialize for UsesConstants {
            fn deserialize(
                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
            where
                Self: Sized,
            {
                Ok(UsesConstants {
                    things: {
                        ::canadensis_encoding::bits::BitArray::deserialize(666_usize, cursor)
                    },
                })
            }
        }
    }
}
