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
                    &UnionOffsetAroundSealed::A(inner) => {
                        cursor.write_aligned_u8(0);
                        cursor.write_aligned_u8(inner);
                    }
                    &UnionOffsetAroundSealed::B(inner) => {
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
pub mod reg {
    pub mod drone {
        pub mod physics {
            pub mod acoustics {
                pub mod note_0_1 {
                    /// `reg.drone.physics.acoustics.Note.0.1`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Note {
                        /// `uavcan.si.unit.frequency.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub frequency: crate::uavcan::si::unit::frequency::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.duration.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub duration: crate::uavcan::si::unit::duration::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.power.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub acoustic_power: crate::uavcan::si::unit::power::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Note {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Note {}
                    impl Note {}
                    impl ::canadensis_encoding::Serialize for Note {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Note {
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
                        assert_eq!(::core::mem::size_of::<Note>() * 8, 96);
                        assert_eq!(::memoffset::offset_of!(Note, frequency) * 8, 0);
                        assert_eq!(::memoffset::offset_of!(Note, duration) * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Note, acoustic_power) * 8, 64);
                    }
                }
            }
            pub mod dynamics {
                pub mod rotation {
                    pub mod planar_0_1 {
                        /// `reg.drone.physics.dynamics.rotation.Planar.0.1`
                        ///
                        /// Fixed size 16 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Planar {
/// `reg.drone.physics.kinematics.rotation.Planar.0.1`
///
/// Always aligned
/// Size 96 bits
pub kinematics: crate::reg::drone::physics::kinematics::rotation::planar_0_1::Planar,
/// `uavcan.si.unit.torque.Scalar.1.0`
///
/// Always aligned
/// Size 32 bits
pub torque: crate::uavcan::si::unit::torque::scalar_1_0::Scalar,
}
                        impl ::canadensis_encoding::DataType for Planar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Planar {}
                        impl Planar {}
                        impl ::canadensis_encoding::Serialize for Planar {
                            fn size_bits(&self) -> usize {
                                128
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Planar {
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
                            assert_eq!(::core::mem::size_of::<Planar>() * 8, 128);
                            assert_eq!(::memoffset::offset_of!(Planar, kinematics) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(Planar, torque) * 8, 96);
                        }
                    }
                    pub mod planar_ts_0_1 {
                        /// `reg.drone.physics.dynamics.rotation.PlanarTs.0.1`
                        ///
                        /// Fixed size 23 bytes
                        pub struct PlanarTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.dynamics.rotation.Planar.0.1`
///
/// Always aligned
/// Size 128 bits
pub value: crate::reg::drone::physics::dynamics::rotation::planar_0_1::Planar,
}
                        impl ::canadensis_encoding::DataType for PlanarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PlanarTs {}
                        impl PlanarTs {}
                        impl ::canadensis_encoding::Serialize for PlanarTs {
                            fn size_bits(&self) -> usize {
                                184
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PlanarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(PlanarTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                }
                pub mod translation {
                    pub mod linear_0_1 {
                        /// `reg.drone.physics.dynamics.translation.Linear.0.1`
                        ///
                        /// Fixed size 16 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Linear {
/// `reg.drone.physics.kinematics.translation.Linear.0.1`
///
/// Always aligned
/// Size 96 bits
pub kinematics: crate::reg::drone::physics::kinematics::translation::linear_0_1::Linear,
/// `uavcan.si.unit.force.Scalar.1.0`
///
/// Always aligned
/// Size 32 bits
pub force: crate::uavcan::si::unit::force::scalar_1_0::Scalar,
}
                        impl ::canadensis_encoding::DataType for Linear {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Linear {}
                        impl Linear {}
                        impl ::canadensis_encoding::Serialize for Linear {
                            fn size_bits(&self) -> usize {
                                128
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Linear {
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
                            assert_eq!(::core::mem::size_of::<Linear>() * 8, 128);
                            assert_eq!(::memoffset::offset_of!(Linear, kinematics) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(Linear, force) * 8, 96);
                        }
                    }
                    pub mod linear_ts_0_1 {
                        /// `reg.drone.physics.dynamics.translation.LinearTs.0.1`
                        ///
                        /// Fixed size 23 bytes
                        pub struct LinearTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.dynamics.translation.Linear.0.1`
///
/// Always aligned
/// Size 128 bits
pub value: crate::reg::drone::physics::dynamics::translation::linear_0_1::Linear,
}
                        impl ::canadensis_encoding::DataType for LinearTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for LinearTs {}
                        impl LinearTs {}
                        impl ::canadensis_encoding::Serialize for LinearTs {
                            fn size_bits(&self) -> usize {
                                184
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for LinearTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(LinearTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                }
            }
            pub mod electricity {
                pub mod power_0_1 {
                    /// `reg.drone.physics.electricity.Power.0.1`
                    ///
                    /// Fixed size 8 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Power {
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub current: crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.voltage.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Power {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Power {}
                    impl Power {}
                    impl ::canadensis_encoding::Serialize for Power {
                        fn size_bits(&self) -> usize {
                            64
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Power {
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
                        assert_eq!(::core::mem::size_of::<Power>() * 8, 64);
                        assert_eq!(::memoffset::offset_of!(Power, current) * 8, 0);
                        assert_eq!(::memoffset::offset_of!(Power, voltage) * 8, 32);
                    }
                }
                pub mod power_ts_0_1 {
                    /// `reg.drone.physics.electricity.PowerTs.0.1`
                    ///
                    /// Fixed size 15 bytes
                    pub struct PowerTs {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `reg.drone.physics.electricity.Power.0.1`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub value: crate::reg::drone::physics::electricity::power_0_1::Power,
                    }
                    impl ::canadensis_encoding::DataType for PowerTs {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for PowerTs {}
                    impl PowerTs {}
                    impl ::canadensis_encoding::Serialize for PowerTs {
                        fn size_bits(&self) -> usize {
                            120
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_composite(&self.value);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for PowerTs {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(PowerTs {
                                timestamp: { cursor.read_composite()? },
                                value: { cursor.read_composite()? },
                            })
                        }
                    }
                }
                pub mod source_0_1 {
                    /// `reg.drone.physics.electricity.Source.0.1`
                    ///
                    /// Fixed size 16 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Source {
                        /// `reg.drone.physics.electricity.Power.0.1`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub power: crate::reg::drone::physics::electricity::power_0_1::Power,
                        /// `uavcan.si.unit.energy.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub energy: crate::uavcan::si::unit::energy::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.energy.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub full_energy: crate::uavcan::si::unit::energy::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Source {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Source {}
                    impl Source {}
                    impl ::canadensis_encoding::Serialize for Source {
                        fn size_bits(&self) -> usize {
                            128
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Source {
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
                        assert_eq!(::core::mem::size_of::<Source>() * 8, 128);
                        assert_eq!(::memoffset::offset_of!(Source, power) * 8, 0);
                        assert_eq!(::memoffset::offset_of!(Source, energy) * 8, 64);
                        assert_eq!(::memoffset::offset_of!(Source, full_energy) * 8, 96);
                    }
                }
                pub mod source_ts_0_1 {
                    /// `reg.drone.physics.electricity.SourceTs.0.1`
                    ///
                    /// Fixed size 23 bytes
                    pub struct SourceTs {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `reg.drone.physics.electricity.Source.0.1`
                        ///
                        /// Always aligned
                        /// Size 128 bits
                        pub value: crate::reg::drone::physics::electricity::source_0_1::Source,
                    }
                    impl ::canadensis_encoding::DataType for SourceTs {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for SourceTs {}
                    impl SourceTs {}
                    impl ::canadensis_encoding::Serialize for SourceTs {
                        fn size_bits(&self) -> usize {
                            184
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_composite(&self.value);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for SourceTs {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(SourceTs {
                                timestamp: { cursor.read_composite()? },
                                value: { cursor.read_composite()? },
                            })
                        }
                    }
                }
            }
            pub mod kinematics {
                pub mod cartesian {
                    pub mod point_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.Point.0.1`
                        ///
                        /// Fixed size 24 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Point {
                            /// `uavcan.si.unit.length.WideVector3.1.0`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub value:
                                crate::uavcan::si::unit::length::wide_vector3_1_0::WideVector3,
                        }
                        impl ::canadensis_encoding::DataType for Point {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Point {}
                        impl Point {}
                        impl ::canadensis_encoding::Serialize for Point {
                            fn size_bits(&self) -> usize {
                                192
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Point {
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
                            assert_eq!(::core::mem::size_of::<Point>() * 8, 192);
                            assert_eq!(::memoffset::offset_of!(Point, value) * 8, 0);
                        }
                    }
                    pub mod point_state_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointState.0.1`
                        ///
                        /// Fixed size 36 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointState {
                            /// `reg.drone.physics.kinematics.cartesian.Point.0.1`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub position:
                                crate::reg::drone::physics::kinematics::cartesian::point_0_1::Point,
                            /// `uavcan.si.unit.velocity.Vector3.1.0`
                            ///
                            /// Always aligned
                            /// Size 96 bits
                            pub velocity: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                        }
                        impl ::canadensis_encoding::DataType for PointState {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointState {}
                        impl PointState {}
                        impl ::canadensis_encoding::Serialize for PointState {
                            fn size_bits(&self) -> usize {
                                288
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PointState {
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
                            assert_eq!(::core::mem::size_of::<PointState>() * 8, 288);
                            assert_eq!(::memoffset::offset_of!(PointState, position) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(PointState, velocity) * 8, 192);
                        }
                    }
                    pub mod point_state_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointStateVar.0.1`
                        ///
                        /// Fixed size 60 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointStateVar {
/// `reg.drone.physics.kinematics.cartesian.PointVar.0.1`
///
/// Always aligned
/// Size 288 bits
pub position: crate::reg::drone::physics::kinematics::cartesian::point_var_0_1::PointVar,
/// `reg.drone.physics.kinematics.translation.Velocity3Var.0.2`
///
/// Always aligned
/// Size 192 bits
pub velocity: crate::reg::drone::physics::kinematics::translation::velocity3_var_0_2::Velocity3Var,
}
                        impl ::canadensis_encoding::DataType for PointStateVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointStateVar {}
                        impl PointStateVar {}
                        impl ::canadensis_encoding::Serialize for PointStateVar {
                            fn size_bits(&self) -> usize {
                                480
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PointStateVar {
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
                            assert_eq!(::core::mem::size_of::<PointStateVar>() * 8, 480);
                            assert_eq!(::memoffset::offset_of!(PointStateVar, position) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(PointStateVar, velocity) * 8, 288);
                        }
                    }
                    pub mod point_state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointStateVarTs.0.1`
                        ///
                        /// Fixed size 67 bytes
                        pub struct PointStateVarTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.kinematics.cartesian.PointStateVar.0.1`
///
/// Always aligned
/// Size 480 bits
pub value: crate::reg::drone::physics::kinematics::cartesian::point_state_var_0_1::PointStateVar,
}
                        impl ::canadensis_encoding::DataType for PointStateVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointStateVarTs {}
                        impl PointStateVarTs {}
                        impl ::canadensis_encoding::Serialize for PointStateVarTs {
                            fn size_bits(&self) -> usize {
                                536
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PointStateVarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(PointStateVarTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                    pub mod point_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointVar.0.1`
                        ///
                        /// Fixed size 36 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointVar {
                            /// `reg.drone.physics.kinematics.cartesian.Point.0.1`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub value:
                                crate::reg::drone::physics::kinematics::cartesian::point_0_1::Point,
                            /// `saturated float16[6]`
                            ///
                            /// Always aligned
                            /// Size 96 bits
                            pub covariance_urt:
                                [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 6],
                        }
                        impl ::canadensis_encoding::DataType for PointVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointVar {}
                        impl PointVar {}
                        impl ::canadensis_encoding::Serialize for PointVar {
                            fn size_bits(&self) -> usize {
                                288
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PointVar {
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
                            assert_eq!(::core::mem::size_of::<PointVar>() * 8, 288);
                            assert_eq!(::memoffset::offset_of!(PointVar, value) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(PointVar, covariance_urt) * 8, 192);
                        }
                    }
                    pub mod pose_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.Pose.0.1`
                        ///
                        /// Fixed size 40 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Pose {
                            /// `reg.drone.physics.kinematics.cartesian.Point.0.1`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub position:
                                crate::reg::drone::physics::kinematics::cartesian::point_0_1::Point,
                            /// `uavcan.si.unit.angle.Quaternion.1.0`
                            ///
                            /// Always aligned
                            /// Size 128 bits
                            pub orientation:
                                crate::uavcan::si::unit::angle::quaternion_1_0::Quaternion,
                        }
                        impl ::canadensis_encoding::DataType for Pose {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Pose {}
                        impl Pose {}
                        impl ::canadensis_encoding::Serialize for Pose {
                            fn size_bits(&self) -> usize {
                                320
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Pose {
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
                            assert_eq!(::core::mem::size_of::<Pose>() * 8, 320);
                            assert_eq!(::memoffset::offset_of!(Pose, position) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(Pose, orientation) * 8, 192);
                        }
                    }
                    pub mod pose_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PoseVar.0.1`
                        ///
                        /// Fixed size 82 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PoseVar {
                            /// `reg.drone.physics.kinematics.cartesian.Pose.0.1`
                            ///
                            /// Always aligned
                            /// Size 320 bits
                            pub value:
                                crate::reg::drone::physics::kinematics::cartesian::pose_0_1::Pose,
                            /// `saturated float16[21]`
                            ///
                            /// Always aligned
                            /// Size 336 bits
                            pub covariance_urt:
                                [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 21],
                        }
                        impl ::canadensis_encoding::DataType for PoseVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PoseVar {}
                        impl PoseVar {}
                        impl ::canadensis_encoding::Serialize for PoseVar {
                            fn size_bits(&self) -> usize {
                                656
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PoseVar {
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
                            assert_eq!(::core::mem::size_of::<PoseVar>() * 8, 656);
                            assert_eq!(::memoffset::offset_of!(PoseVar, value) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(PoseVar, covariance_urt) * 8, 320);
                        }
                    }
                    pub mod pose_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PoseVarTs.0.1`
                        ///
                        /// Fixed size 89 bytes
                        pub struct PoseVarTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.kinematics.cartesian.PoseVar.0.1`
///
/// Always aligned
/// Size 656 bits
pub value: crate::reg::drone::physics::kinematics::cartesian::pose_var_0_1::PoseVar,
}
                        impl ::canadensis_encoding::DataType for PoseVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PoseVarTs {}
                        impl PoseVarTs {}
                        impl ::canadensis_encoding::Serialize for PoseVarTs {
                            fn size_bits(&self) -> usize {
                                712
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PoseVarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(PoseVarTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                    pub mod state_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.State.0.1`
                        ///
                        /// Fixed size 64 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct State {
                            /// `reg.drone.physics.kinematics.cartesian.Pose.0.1`
                            ///
                            /// Always aligned
                            /// Size 320 bits
                            pub pose:
                                crate::reg::drone::physics::kinematics::cartesian::pose_0_1::Pose,
                            /// `reg.drone.physics.kinematics.cartesian.Twist.0.1`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub twist:
                                crate::reg::drone::physics::kinematics::cartesian::twist_0_1::Twist,
                        }
                        impl ::canadensis_encoding::DataType for State {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for State {}
                        impl State {}
                        impl ::canadensis_encoding::Serialize for State {
                            fn size_bits(&self) -> usize {
                                512
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for State {
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
                            assert_eq!(::core::mem::size_of::<State>() * 8, 512);
                            assert_eq!(::memoffset::offset_of!(State, pose) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(State, twist) * 8, 320);
                        }
                    }
                    pub mod state_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.StateVar.0.1`
                        ///
                        /// Fixed size 148 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct StateVar {
/// `reg.drone.physics.kinematics.cartesian.PoseVar.0.1`
///
/// Always aligned
/// Size 656 bits
pub pose: crate::reg::drone::physics::kinematics::cartesian::pose_var_0_1::PoseVar,
/// `reg.drone.physics.kinematics.cartesian.TwistVar.0.1`
///
/// Always aligned
/// Size 528 bits
pub twist: crate::reg::drone::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                        impl ::canadensis_encoding::DataType for StateVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for StateVar {}
                        impl StateVar {}
                        impl ::canadensis_encoding::Serialize for StateVar {
                            fn size_bits(&self) -> usize {
                                1184
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for StateVar {
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
                            assert_eq!(::core::mem::size_of::<StateVar>() * 8, 1184);
                            assert_eq!(::memoffset::offset_of!(StateVar, pose) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(StateVar, twist) * 8, 656);
                        }
                    }
                    pub mod state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.StateVarTs.0.1`
                        ///
                        /// Fixed size 155 bytes
                        pub struct StateVarTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.kinematics.cartesian.StateVar.0.1`
///
/// Always aligned
/// Size 1184 bits
pub value: crate::reg::drone::physics::kinematics::cartesian::state_var_0_1::StateVar,
}
                        impl ::canadensis_encoding::DataType for StateVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for StateVarTs {}
                        impl StateVarTs {}
                        impl ::canadensis_encoding::Serialize for StateVarTs {
                            fn size_bits(&self) -> usize {
                                1240
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for StateVarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(StateVarTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                    pub mod twist_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.Twist.0.1`
                        ///
                        /// Fixed size 24 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Twist {
                            /// `uavcan.si.unit.velocity.Vector3.1.0`
                            ///
                            /// Always aligned
                            /// Size 96 bits
                            pub linear: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                            /// `uavcan.si.unit.angular_velocity.Vector3.1.0`
                            ///
                            /// Always aligned
                            /// Size 96 bits
                            pub angular:
                                crate::uavcan::si::unit::angular_velocity::vector3_1_0::Vector3,
                        }
                        impl ::canadensis_encoding::DataType for Twist {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Twist {}
                        impl Twist {}
                        impl ::canadensis_encoding::Serialize for Twist {
                            fn size_bits(&self) -> usize {
                                192
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Twist {
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
                            assert_eq!(::core::mem::size_of::<Twist>() * 8, 192);
                            assert_eq!(::memoffset::offset_of!(Twist, linear) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(Twist, angular) * 8, 96);
                        }
                    }
                    pub mod twist_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.TwistVar.0.1`
                        ///
                        /// Fixed size 66 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct TwistVar {
                            /// `reg.drone.physics.kinematics.cartesian.Twist.0.1`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub value:
                                crate::reg::drone::physics::kinematics::cartesian::twist_0_1::Twist,
                            /// `saturated float16[21]`
                            ///
                            /// Always aligned
                            /// Size 336 bits
                            pub covariance_urt:
                                [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 21],
                        }
                        impl ::canadensis_encoding::DataType for TwistVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for TwistVar {}
                        impl TwistVar {}
                        impl ::canadensis_encoding::Serialize for TwistVar {
                            fn size_bits(&self) -> usize {
                                528
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for TwistVar {
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
                            assert_eq!(::core::mem::size_of::<TwistVar>() * 8, 528);
                            assert_eq!(::memoffset::offset_of!(TwistVar, value) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(TwistVar, covariance_urt) * 8, 192);
                        }
                    }
                    pub mod twist_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.TwistVarTs.0.1`
                        ///
                        /// Fixed size 73 bytes
                        pub struct TwistVarTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.kinematics.cartesian.TwistVar.0.1`
///
/// Always aligned
/// Size 528 bits
pub value: crate::reg::drone::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                        impl ::canadensis_encoding::DataType for TwistVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for TwistVarTs {}
                        impl TwistVarTs {}
                        impl ::canadensis_encoding::Serialize for TwistVarTs {
                            fn size_bits(&self) -> usize {
                                584
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for TwistVarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(TwistVarTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                }
                pub mod geodetic {
                    pub mod point_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.Point.0.1`
                        ///
                        /// Fixed size 24 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Point {
                            /// `saturated float64`
                            ///
                            /// Always aligned
                            /// Size 64 bits
                            pub latitude: f64,
                            /// `saturated float64`
                            ///
                            /// Always aligned
                            /// Size 64 bits
                            pub longitude: f64,
                            /// `uavcan.si.unit.length.WideScalar.1.0`
                            ///
                            /// Always aligned
                            /// Size 64 bits
                            pub altitude:
                                crate::uavcan::si::unit::length::wide_scalar_1_0::WideScalar,
                        }
                        impl ::canadensis_encoding::DataType for Point {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Point {}
                        impl Point {}
                        impl ::canadensis_encoding::Serialize for Point {
                            fn size_bits(&self) -> usize {
                                192
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Point {
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
                            assert_eq!(::core::mem::size_of::<Point>() * 8, 192);
                            assert_eq!(::memoffset::offset_of!(Point, latitude) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(Point, longitude) * 8, 64);
                            assert_eq!(::memoffset::offset_of!(Point, altitude) * 8, 128);
                        }
                    }
                    pub mod point_state_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointState.0.1`
                        ///
                        /// Fixed size 36 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointState {
                            /// `reg.drone.physics.kinematics.geodetic.Point.0.1`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub position:
                                crate::reg::drone::physics::kinematics::geodetic::point_0_1::Point,
                            /// `uavcan.si.unit.velocity.Vector3.1.0`
                            ///
                            /// Always aligned
                            /// Size 96 bits
                            pub velocity: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                        }
                        impl ::canadensis_encoding::DataType for PointState {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointState {}
                        impl PointState {}
                        impl ::canadensis_encoding::Serialize for PointState {
                            fn size_bits(&self) -> usize {
                                288
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PointState {
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
                            assert_eq!(::core::mem::size_of::<PointState>() * 8, 288);
                            assert_eq!(::memoffset::offset_of!(PointState, position) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(PointState, velocity) * 8, 192);
                        }
                    }
                    pub mod point_state_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointStateVar.0.1`
                        ///
                        /// Fixed size 60 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointStateVar {
/// `reg.drone.physics.kinematics.geodetic.PointVar.0.1`
///
/// Always aligned
/// Size 288 bits
pub position: crate::reg::drone::physics::kinematics::geodetic::point_var_0_1::PointVar,
/// `reg.drone.physics.kinematics.translation.Velocity3Var.0.2`
///
/// Always aligned
/// Size 192 bits
pub velocity: crate::reg::drone::physics::kinematics::translation::velocity3_var_0_2::Velocity3Var,
}
                        impl ::canadensis_encoding::DataType for PointStateVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointStateVar {}
                        impl PointStateVar {}
                        impl ::canadensis_encoding::Serialize for PointStateVar {
                            fn size_bits(&self) -> usize {
                                480
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PointStateVar {
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
                            assert_eq!(::core::mem::size_of::<PointStateVar>() * 8, 480);
                            assert_eq!(::memoffset::offset_of!(PointStateVar, position) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(PointStateVar, velocity) * 8, 288);
                        }
                    }
                    pub mod point_state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointStateVarTs.0.1`
                        ///
                        /// Fixed size 67 bytes
                        pub struct PointStateVarTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.kinematics.geodetic.PointStateVar.0.1`
///
/// Always aligned
/// Size 480 bits
pub value: crate::reg::drone::physics::kinematics::geodetic::point_state_var_0_1::PointStateVar,
}
                        impl ::canadensis_encoding::DataType for PointStateVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointStateVarTs {}
                        impl PointStateVarTs {}
                        impl ::canadensis_encoding::Serialize for PointStateVarTs {
                            fn size_bits(&self) -> usize {
                                536
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PointStateVarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(PointStateVarTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                    pub mod point_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointVar.0.1`
                        ///
                        /// Fixed size 36 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointVar {
                            /// `reg.drone.physics.kinematics.geodetic.Point.0.1`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub value:
                                crate::reg::drone::physics::kinematics::geodetic::point_0_1::Point,
                            /// `saturated float16[6]`
                            ///
                            /// Always aligned
                            /// Size 96 bits
                            pub covariance_urt:
                                [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 6],
                        }
                        impl ::canadensis_encoding::DataType for PointVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointVar {}
                        impl PointVar {}
                        impl ::canadensis_encoding::Serialize for PointVar {
                            fn size_bits(&self) -> usize {
                                288
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PointVar {
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
                            assert_eq!(::core::mem::size_of::<PointVar>() * 8, 288);
                            assert_eq!(::memoffset::offset_of!(PointVar, value) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(PointVar, covariance_urt) * 8, 192);
                        }
                    }
                    pub mod pose_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.Pose.0.1`
                        ///
                        /// Fixed size 40 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Pose {
                            /// `reg.drone.physics.kinematics.geodetic.Point.0.1`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub position:
                                crate::reg::drone::physics::kinematics::geodetic::point_0_1::Point,
                            /// `uavcan.si.unit.angle.Quaternion.1.0`
                            ///
                            /// Always aligned
                            /// Size 128 bits
                            pub orientation:
                                crate::uavcan::si::unit::angle::quaternion_1_0::Quaternion,
                        }
                        impl ::canadensis_encoding::DataType for Pose {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Pose {}
                        impl Pose {}
                        impl ::canadensis_encoding::Serialize for Pose {
                            fn size_bits(&self) -> usize {
                                320
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Pose {
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
                            assert_eq!(::core::mem::size_of::<Pose>() * 8, 320);
                            assert_eq!(::memoffset::offset_of!(Pose, position) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(Pose, orientation) * 8, 192);
                        }
                    }
                    pub mod pose_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PoseVar.0.1`
                        ///
                        /// Fixed size 82 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PoseVar {
                            /// `reg.drone.physics.kinematics.geodetic.Pose.0.1`
                            ///
                            /// Always aligned
                            /// Size 320 bits
                            pub value:
                                crate::reg::drone::physics::kinematics::geodetic::pose_0_1::Pose,
                            /// `saturated float16[21]`
                            ///
                            /// Always aligned
                            /// Size 336 bits
                            pub covariance_urt:
                                [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 21],
                        }
                        impl ::canadensis_encoding::DataType for PoseVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PoseVar {}
                        impl PoseVar {}
                        impl ::canadensis_encoding::Serialize for PoseVar {
                            fn size_bits(&self) -> usize {
                                656
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PoseVar {
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
                            assert_eq!(::core::mem::size_of::<PoseVar>() * 8, 656);
                            assert_eq!(::memoffset::offset_of!(PoseVar, value) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(PoseVar, covariance_urt) * 8, 320);
                        }
                    }
                    pub mod state_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.State.0.1`
                        ///
                        /// Fixed size 64 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct State {
                            /// `reg.drone.physics.kinematics.geodetic.Pose.0.1`
                            ///
                            /// Always aligned
                            /// Size 320 bits
                            pub pose:
                                crate::reg::drone::physics::kinematics::geodetic::pose_0_1::Pose,
                            /// `reg.drone.physics.kinematics.cartesian.Twist.0.1`
                            ///
                            /// Always aligned
                            /// Size 192 bits
                            pub twist:
                                crate::reg::drone::physics::kinematics::cartesian::twist_0_1::Twist,
                        }
                        impl ::canadensis_encoding::DataType for State {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for State {}
                        impl State {}
                        impl ::canadensis_encoding::Serialize for State {
                            fn size_bits(&self) -> usize {
                                512
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for State {
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
                            assert_eq!(::core::mem::size_of::<State>() * 8, 512);
                            assert_eq!(::memoffset::offset_of!(State, pose) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(State, twist) * 8, 320);
                        }
                    }
                    pub mod state_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.StateVar.0.1`
                        ///
                        /// Fixed size 148 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct StateVar {
/// `reg.drone.physics.kinematics.geodetic.PoseVar.0.1`
///
/// Always aligned
/// Size 656 bits
pub pose: crate::reg::drone::physics::kinematics::geodetic::pose_var_0_1::PoseVar,
/// `reg.drone.physics.kinematics.cartesian.TwistVar.0.1`
///
/// Always aligned
/// Size 528 bits
pub twist: crate::reg::drone::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                        impl ::canadensis_encoding::DataType for StateVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for StateVar {}
                        impl StateVar {}
                        impl ::canadensis_encoding::Serialize for StateVar {
                            fn size_bits(&self) -> usize {
                                1184
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for StateVar {
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
                            assert_eq!(::core::mem::size_of::<StateVar>() * 8, 1184);
                            assert_eq!(::memoffset::offset_of!(StateVar, pose) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(StateVar, twist) * 8, 656);
                        }
                    }
                    pub mod state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.StateVarTs.0.1`
                        ///
                        /// Fixed size 155 bytes
                        pub struct StateVarTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.kinematics.geodetic.StateVar.0.1`
///
/// Always aligned
/// Size 1184 bits
pub value: crate::reg::drone::physics::kinematics::geodetic::state_var_0_1::StateVar,
}
                        impl ::canadensis_encoding::DataType for StateVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for StateVarTs {}
                        impl StateVarTs {}
                        impl ::canadensis_encoding::Serialize for StateVarTs {
                            fn size_bits(&self) -> usize {
                                1240
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for StateVarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(StateVarTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                }
                pub mod rotation {
                    pub mod planar_0_1 {
                        /// `reg.drone.physics.kinematics.rotation.Planar.0.1`
                        ///
                        /// Fixed size 12 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Planar {
                            /// `uavcan.si.unit.angle.Scalar.1.0`
                            ///
                            /// Always aligned
                            /// Size 32 bits
                            pub angular_position:
                                crate::uavcan::si::unit::angle::scalar_1_0::Scalar,
                            /// `uavcan.si.unit.angular_velocity.Scalar.1.0`
                            ///
                            /// Always aligned
                            /// Size 32 bits
                            pub angular_velocity:
                                crate::uavcan::si::unit::angular_velocity::scalar_1_0::Scalar,
                            /// `uavcan.si.unit.angular_acceleration.Scalar.1.0`
                            ///
                            /// Always aligned
                            /// Size 32 bits
                            pub angular_acceleration:
                                crate::uavcan::si::unit::angular_acceleration::scalar_1_0::Scalar,
                        }
                        impl ::canadensis_encoding::DataType for Planar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Planar {}
                        impl Planar {}
                        impl ::canadensis_encoding::Serialize for Planar {
                            fn size_bits(&self) -> usize {
                                96
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Planar {
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
                            assert_eq!(::core::mem::size_of::<Planar>() * 8, 96);
                            assert_eq!(::memoffset::offset_of!(Planar, angular_position) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(Planar, angular_velocity) * 8, 32);
                            assert_eq!(
                                ::memoffset::offset_of!(Planar, angular_acceleration) * 8,
                                64
                            );
                        }
                    }
                    pub mod planar_ts_0_1 {
                        /// `reg.drone.physics.kinematics.rotation.PlanarTs.0.1`
                        ///
                        /// Fixed size 19 bytes
                        pub struct PlanarTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.kinematics.rotation.Planar.0.1`
///
/// Always aligned
/// Size 96 bits
pub value: crate::reg::drone::physics::kinematics::rotation::planar_0_1::Planar,
}
                        impl ::canadensis_encoding::DataType for PlanarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PlanarTs {}
                        impl PlanarTs {}
                        impl ::canadensis_encoding::Serialize for PlanarTs {
                            fn size_bits(&self) -> usize {
                                152
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for PlanarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(PlanarTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                }
                pub mod translation {
                    pub mod linear_0_1 {
                        /// `reg.drone.physics.kinematics.translation.Linear.0.1`
                        ///
                        /// Fixed size 12 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Linear {
                            /// `uavcan.si.unit.length.Scalar.1.0`
                            ///
                            /// Always aligned
                            /// Size 32 bits
                            pub position: crate::uavcan::si::unit::length::scalar_1_0::Scalar,
                            /// `uavcan.si.unit.velocity.Scalar.1.0`
                            ///
                            /// Always aligned
                            /// Size 32 bits
                            pub velocity: crate::uavcan::si::unit::velocity::scalar_1_0::Scalar,
                            /// `uavcan.si.unit.acceleration.Scalar.1.0`
                            ///
                            /// Always aligned
                            /// Size 32 bits
                            pub acceleration:
                                crate::uavcan::si::unit::acceleration::scalar_1_0::Scalar,
                        }
                        impl ::canadensis_encoding::DataType for Linear {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Linear {}
                        impl Linear {}
                        impl ::canadensis_encoding::Serialize for Linear {
                            fn size_bits(&self) -> usize {
                                96
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Linear {
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
                            assert_eq!(::core::mem::size_of::<Linear>() * 8, 96);
                            assert_eq!(::memoffset::offset_of!(Linear, position) * 8, 0);
                            assert_eq!(::memoffset::offset_of!(Linear, velocity) * 8, 32);
                            assert_eq!(::memoffset::offset_of!(Linear, acceleration) * 8, 64);
                        }
                    }
                    pub mod linear_ts_0_1 {
                        /// `reg.drone.physics.kinematics.translation.LinearTs.0.1`
                        ///
                        /// Fixed size 19 bytes
                        pub struct LinearTs {
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned
/// Size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
/// `reg.drone.physics.kinematics.translation.Linear.0.1`
///
/// Always aligned
/// Size 96 bits
pub value: crate::reg::drone::physics::kinematics::translation::linear_0_1::Linear,
}
                        impl ::canadensis_encoding::DataType for LinearTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for LinearTs {}
                        impl LinearTs {}
                        impl ::canadensis_encoding::Serialize for LinearTs {
                            fn size_bits(&self) -> usize {
                                152
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for LinearTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(LinearTs {
                                    timestamp: { cursor.read_composite()? },
                                    value: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                    pub mod linear_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.translation.LinearVarTs.0.1`
                        ///
                        /// Fixed size 25 bytes
                        pub struct LinearVarTs {
/// `reg.drone.physics.kinematics.translation.LinearTs.0.1`
///
/// Always aligned
/// Size 152 bits
pub value: crate::reg::drone::physics::kinematics::translation::linear_ts_0_1::LinearTs,
/// `saturated float16`
///
/// Always aligned
/// Size 16 bits
pub position_error_variance: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
/// `saturated float16`
///
/// Always aligned
/// Size 16 bits
pub velocity_error_variance: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
/// `saturated float16`
///
/// Always aligned
/// Size 16 bits
pub acceleration_error_variance: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
}
                        impl ::canadensis_encoding::DataType for LinearVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for LinearVarTs {}
                        impl LinearVarTs {}
                        impl ::canadensis_encoding::Serialize for LinearVarTs {
                            fn size_bits(&self) -> usize {
                                200
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.value);
                                cursor.write_f16((self.position_error_variance).into());
                                cursor.write_f16((self.velocity_error_variance).into());
                                cursor.write_f16((self.acceleration_error_variance).into());
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for LinearVarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(LinearVarTs {
                                    value: { cursor.read_composite()? },
                                    position_error_variance: { cursor.read_f16().into() },
                                    velocity_error_variance: { cursor.read_f16().into() },
                                    acceleration_error_variance: { cursor.read_f16().into() },
                                })
                            }
                        }
                    }
                    pub mod velocity1_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.translation.Velocity1VarTs.0.1`
                        ///
                        /// Fixed size 13 bytes
                        pub struct Velocity1VarTs {
                            /// `uavcan.si.sample.velocity.Scalar.1.0`
                            ///
                            /// Always aligned
                            /// Size 88 bits
                            pub value: crate::uavcan::si::sample::velocity::scalar_1_0::Scalar,
                            /// `saturated float16`
                            ///
                            /// Always aligned
                            /// Size 16 bits
                            pub error_variance: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                        }
                        impl ::canadensis_encoding::DataType for Velocity1VarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Velocity1VarTs {}
                        impl Velocity1VarTs {}
                        impl ::canadensis_encoding::Serialize for Velocity1VarTs {
                            fn size_bits(&self) -> usize {
                                104
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.value);
                                cursor.write_f16((self.error_variance).into());
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Velocity1VarTs {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(Velocity1VarTs {
                                    value: { cursor.read_composite()? },
                                    error_variance: { cursor.read_f16().into() },
                                })
                            }
                        }
                    }
                    pub mod velocity3_var_0_1 {
                        /// `reg.drone.physics.kinematics.translation.Velocity3Var.0.1`
                        ///
                        /// Fixed size 31 bytes
                        pub struct Velocity3Var {
                            /// `uavcan.si.sample.velocity.Vector3.1.0`
                            ///
                            /// Always aligned
                            /// Size 152 bits
                            pub value: crate::uavcan::si::sample::velocity::vector3_1_0::Vector3,
                            /// `saturated float16[6]`
                            ///
                            /// Always aligned
                            /// Size 96 bits
                            pub covariance_urt:
                                [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 6],
                        }
                        impl ::canadensis_encoding::DataType for Velocity3Var {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Velocity3Var {}
                        impl Velocity3Var {}
                        impl ::canadensis_encoding::Serialize for Velocity3Var {
                            fn size_bits(&self) -> usize {
                                248
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.value);
                                for value in (self.covariance_urt).iter() {
                                    cursor.write_f16((*value).into());
                                }
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Velocity3Var {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(Velocity3Var {
                                    value: { cursor.read_composite()? },
                                    covariance_urt: {
                                        [
                                            cursor.read_f16().into(),
                                            cursor.read_f16().into(),
                                            cursor.read_f16().into(),
                                            cursor.read_f16().into(),
                                            cursor.read_f16().into(),
                                            cursor.read_f16().into(),
                                        ]
                                    },
                                })
                            }
                        }
                    }
                    pub mod velocity3_var_0_2 {
                        /// `reg.drone.physics.kinematics.translation.Velocity3Var.0.2`
                        ///
                        /// Fixed size 24 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Velocity3Var {
                            /// `uavcan.si.unit.velocity.Vector3.1.0`
                            ///
                            /// Always aligned
                            /// Size 96 bits
                            pub value: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                            /// `saturated float16[6]`
                            ///
                            /// Always aligned
                            /// Size 96 bits
                            pub covariance_urt:
                                [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 6],
                        }
                        impl ::canadensis_encoding::DataType for Velocity3Var {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Velocity3Var {}
                        impl Velocity3Var {}
                        impl ::canadensis_encoding::Serialize for Velocity3Var {
                            fn size_bits(&self) -> usize {
                                192
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Velocity3Var {
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
                            assert_eq!(::core::mem::size_of::<Velocity3Var>() * 8, 192);
                            assert_eq!(::memoffset::offset_of!(Velocity3Var, value) * 8, 0);
                            assert_eq!(
                                ::memoffset::offset_of!(Velocity3Var, covariance_urt) * 8,
                                96
                            );
                        }
                    }
                }
            }
            pub mod optics {
                pub mod high_color_0_1 {
                    /// `reg.drone.physics.optics.HighColor.0.1`
                    ///
                    /// Fixed size 2 bytes
                    pub struct HighColor {
                        /// `saturated uint5`
                        ///
                        /// Always aligned
                        /// Size 5 bits
                        pub red: u8,
                        /// `saturated uint6`
                        ///
                        /// Not always aligned
                        /// Size 6 bits
                        pub green: u8,
                        /// `saturated uint5`
                        ///
                        /// Not always aligned
                        /// Size 5 bits
                        pub blue: u8,
                    }
                    impl ::canadensis_encoding::DataType for HighColor {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for HighColor {}
                    impl HighColor {
                        pub const MAX_BLUE: u8 = 31;
                        pub const MAX_GREEN: u8 = 63;
                        pub const MAX_RED: u8 = 31;
                    }
                    impl ::canadensis_encoding::Serialize for HighColor {
                        fn size_bits(&self) -> usize {
                            16
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_u5(self.red);
                            cursor.write_u6(self.green);
                            cursor.write_u5(self.blue);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for HighColor {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(HighColor {
                                red: { cursor.read_u5() as _ },
                                green: { cursor.read_u6() as _ },
                                blue: { cursor.read_u5() as _ },
                            })
                        }
                    }
                }
            }
            pub mod thermodynamics {
                pub mod pressure_temp_var_ts_0_1 {
                    /// `reg.drone.physics.thermodynamics.PressureTempVarTs.0.1`
                    ///
                    /// Fixed size 21 bytes
                    pub struct PressureTempVarTs {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `uavcan.si.unit.pressure.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub pressure: crate::uavcan::si::unit::pressure::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.temperature.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub temperature: crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
                        /// `saturated float16[3]`
                        ///
                        /// Always aligned
                        /// Size 48 bits
                        pub covariance_urt: [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 3],
                    }
                    impl ::canadensis_encoding::DataType for PressureTempVarTs {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for PressureTempVarTs {}
                    impl PressureTempVarTs {}
                    impl ::canadensis_encoding::Serialize for PressureTempVarTs {
                        fn size_bits(&self) -> usize {
                            168
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_composite(&self.pressure);
                            cursor.write_composite(&self.temperature);
                            for value in (self.covariance_urt).iter() {
                                cursor.write_f16((*value).into());
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for PressureTempVarTs {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(PressureTempVarTs {
                                timestamp: { cursor.read_composite()? },
                                pressure: { cursor.read_composite()? },
                                temperature: { cursor.read_composite()? },
                                covariance_urt: {
                                    [
                                        cursor.read_f16().into(),
                                        cursor.read_f16().into(),
                                        cursor.read_f16().into(),
                                    ]
                                },
                            })
                        }
                    }
                }
            }
            pub mod time {
                pub mod tai64_0_1 {
                    /// `reg.drone.physics.time.TAI64.0.1`
                    ///
                    /// Fixed size 8 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct TAI64 {
                        /// `saturated int64`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub tai64n: i64,
                    }
                    impl ::canadensis_encoding::DataType for TAI64 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for TAI64 {}
                    impl TAI64 {}
                    impl ::canadensis_encoding::Serialize for TAI64 {
                        fn size_bits(&self) -> usize {
                            64
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for TAI64 {
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
                        assert_eq!(::core::mem::size_of::<TAI64>() * 8, 64);
                        assert_eq!(::memoffset::offset_of!(TAI64, tai64n) * 8, 0);
                    }
                }
                pub mod tai64_var_0_1 {
                    /// `reg.drone.physics.time.TAI64Var.0.1`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct TAI64Var {
                        /// `reg.drone.physics.time.TAI64.0.1`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub value: crate::reg::drone::physics::time::tai64_0_1::TAI64,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub error_variance: f32,
                    }
                    impl ::canadensis_encoding::DataType for TAI64Var {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for TAI64Var {}
                    impl TAI64Var {}
                    impl ::canadensis_encoding::Serialize for TAI64Var {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for TAI64Var {
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
                        assert_eq!(::core::mem::size_of::<TAI64Var>() * 8, 96);
                        assert_eq!(::memoffset::offset_of!(TAI64Var, value) * 8, 0);
                        assert_eq!(::memoffset::offset_of!(TAI64Var, error_variance) * 8, 64);
                    }
                }
                pub mod tai64_var_ts_0_1 {
                    /// `reg.drone.physics.time.TAI64VarTs.0.1`
                    ///
                    /// Fixed size 19 bytes
                    pub struct TAI64VarTs {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `reg.drone.physics.time.TAI64Var.0.1`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub value: crate::reg::drone::physics::time::tai64_var_0_1::TAI64Var,
                    }
                    impl ::canadensis_encoding::DataType for TAI64VarTs {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for TAI64VarTs {}
                    impl TAI64VarTs {}
                    impl ::canadensis_encoding::Serialize for TAI64VarTs {
                        fn size_bits(&self) -> usize {
                            152
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_composite(&self.value);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for TAI64VarTs {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(TAI64VarTs {
                                timestamp: { cursor.read_composite()? },
                                value: { cursor.read_composite()? },
                            })
                        }
                    }
                }
            }
        }
        pub mod service {
            pub mod actuator {
                pub mod common {
                    pub mod _0_1 {
                        /// `reg.drone.service.actuator.common._.0.1`
                        ///
                        /// Fixed size 0 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct _0 {}
                        impl ::canadensis_encoding::DataType for _0 {
                            const EXTENT_BYTES: Option<u32> = Some(0);
                        }
                        impl ::canadensis_encoding::Message for _0 {}
                        impl _0 {
                            pub const CONTROL_TIMEOUT: f32 = 1_f32;
                            pub const MAX_PUBLICATION_PERIOD: u8 = 1;
                        }
                        impl ::canadensis_encoding::Serialize for _0 {
                            fn size_bits(&self) -> usize {
                                0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for _0 {
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
                            assert_eq!(::core::mem::size_of::<_0>() * 8, 0);
                        }
                    }
                    pub mod fault_flags_0_1 {
                        /// `reg.drone.service.actuator.common.FaultFlags.0.1`
                        ///
                        /// Fixed size 2 bytes
                        pub struct FaultFlags {
                            /// `saturated bool`
                            ///
                            /// Always aligned
                            /// Size 1 bits
                            pub overload: bool,
                            /// `saturated bool`
                            ///
                            /// Not always aligned
                            /// Size 1 bits
                            pub voltage: bool,
                            /// `saturated bool`
                            ///
                            /// Not always aligned
                            /// Size 1 bits
                            pub motor_temperature: bool,
                            /// `saturated bool`
                            ///
                            /// Not always aligned
                            /// Size 1 bits
                            pub controller_temperature: bool,
                            /// `saturated bool`
                            ///
                            /// Not always aligned
                            /// Size 1 bits
                            pub velocity: bool,
                            /// `saturated bool`
                            ///
                            /// Not always aligned
                            /// Size 1 bits
                            pub mechanical: bool,
                            /// `saturated bool`
                            ///
                            /// Not always aligned
                            /// Size 1 bits
                            pub vibration: bool,
                            /// `saturated bool`
                            ///
                            /// Not always aligned
                            /// Size 1 bits
                            pub configuration: bool,
                            /// `saturated bool`
                            ///
                            /// Always aligned
                            /// Size 1 bits
                            pub control_mode: bool,
                            // 6 bits of padding
                            /// `saturated bool`
                            ///
                            /// Not always aligned
                            /// Size 1 bits
                            pub other: bool,
                        }
                        impl ::canadensis_encoding::DataType for FaultFlags {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for FaultFlags {}
                        impl FaultFlags {}
                        impl ::canadensis_encoding::Serialize for FaultFlags {
                            fn size_bits(&self) -> usize {
                                16
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_bool(self.overload);
                                cursor.write_bool(self.voltage);
                                cursor.write_bool(self.motor_temperature);
                                cursor.write_bool(self.controller_temperature);
                                cursor.write_bool(self.velocity);
                                cursor.write_bool(self.mechanical);
                                cursor.write_bool(self.vibration);
                                cursor.write_bool(self.configuration);
                                cursor.write_bool(self.control_mode);
                                cursor.skip_6();
                                cursor.write_bool(self.other);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for FaultFlags {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(FaultFlags {
                                    overload: { cursor.read_bool() },
                                    voltage: { cursor.read_bool() },
                                    motor_temperature: { cursor.read_bool() },
                                    controller_temperature: { cursor.read_bool() },
                                    velocity: { cursor.read_bool() },
                                    mechanical: { cursor.read_bool() },
                                    vibration: { cursor.read_bool() },
                                    configuration: { cursor.read_bool() },
                                    control_mode: { cursor.read_bool() },
                                    other: {
                                        cursor.skip_6();
                                        cursor.read_bool()
                                    },
                                })
                            }
                        }
                    }
                    pub mod feedback_0_1 {
                        /// `reg.drone.service.actuator.common.Feedback.0.1`
                        ///
                        /// Fixed size 3 bytes
                        pub struct Feedback {
                            /// `reg.drone.service.common.Heartbeat.0.1`
                            ///
                            /// Always aligned
                            /// Size 16 bits
                            pub heartbeat:
                                crate::reg::drone::service::common::heartbeat_0_1::Heartbeat,
                            /// `saturated int8`
                            ///
                            /// Always aligned
                            /// Size 8 bits
                            pub demand_factor_pct: i8,
                        }
                        impl ::canadensis_encoding::DataType for Feedback {
                            const EXTENT_BYTES: Option<u32> = Some(63);
                        }
                        impl ::canadensis_encoding::Message for Feedback {}
                        impl Feedback {}
                        impl ::canadensis_encoding::Serialize for Feedback {
                            fn size_bits(&self) -> usize {
                                24
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.heartbeat);
                                cursor.write_aligned_u8(self.demand_factor_pct as u8);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Feedback {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(Feedback {
                                    heartbeat: { cursor.read_composite()? },
                                    demand_factor_pct: { cursor.read_u8() as _ },
                                })
                            }
                        }
                    }
                    pub mod sp {
                        pub mod _0_1 {
                            /// `reg.drone.service.actuator.common.sp._.0.1`
                            ///
                            /// Fixed size 0 bytes
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct _0 {}
                            impl ::canadensis_encoding::DataType for _0 {
                                const EXTENT_BYTES: Option<u32> = Some(0);
                            }
                            impl ::canadensis_encoding::Message for _0 {}
                            impl _0 {
                                pub const EPSILON: ::half::f16 = ::half::f16::from_bits(4096);
                            }
                            impl ::canadensis_encoding::Serialize for _0 {
                                fn size_bits(&self) -> usize {
                                    0
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                                }
                            }
                            impl ::canadensis_encoding::Deserialize for _0 {
                                fn deserialize(
                                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                                ) -> ::core::result::Result<
                                    Self,
                                    ::canadensis_encoding::DeserializeError,
                                >
                                where
                                    Self: Sized,
                                {
                                    Ok(Self::deserialize_zero_copy(cursor))
                                }
                            }
                            #[test]
                            fn test_layout() {
                                assert_eq!(::core::mem::size_of::<_0>() * 8, 0);
                            }
                        }
                        pub mod scalar_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Scalar.0.1`
                            ///
                            /// Fixed size 2 bytes
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Scalar {
                                /// `saturated float16`
                                ///
                                /// Always aligned
                                /// Size 16 bits
                                pub value: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                            }
                            impl ::canadensis_encoding::DataType for Scalar {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Scalar {}
                            impl Scalar {}
                            impl ::canadensis_encoding::Serialize for Scalar {
                                fn size_bits(&self) -> usize {
                                    16
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                                }
                            }
                            impl ::canadensis_encoding::Deserialize for Scalar {
                                fn deserialize(
                                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                                ) -> ::core::result::Result<
                                    Self,
                                    ::canadensis_encoding::DeserializeError,
                                >
                                where
                                    Self: Sized,
                                {
                                    Ok(Self::deserialize_zero_copy(cursor))
                                }
                            }
                            #[test]
                            fn test_layout() {
                                assert_eq!(::core::mem::size_of::<Scalar>() * 8, 16);
                                assert_eq!(::memoffset::offset_of!(Scalar, value) * 8, 0);
                            }
                        }
                        pub mod vector2_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector2.0.1`
                            ///
                            /// Fixed size 4 bytes
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector2 {
                                /// `saturated float16[2]`
                                ///
                                /// Always aligned
                                /// Size 32 bits
                                pub value: [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 2],
                            }
                            impl ::canadensis_encoding::DataType for Vector2 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector2 {}
                            impl Vector2 {}
                            impl ::canadensis_encoding::Serialize for Vector2 {
                                fn size_bits(&self) -> usize {
                                    32
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                                }
                            }
                            impl ::canadensis_encoding::Deserialize for Vector2 {
                                fn deserialize(
                                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                                ) -> ::core::result::Result<
                                    Self,
                                    ::canadensis_encoding::DeserializeError,
                                >
                                where
                                    Self: Sized,
                                {
                                    Ok(Self::deserialize_zero_copy(cursor))
                                }
                            }
                            #[test]
                            fn test_layout() {
                                assert_eq!(::core::mem::size_of::<Vector2>() * 8, 32);
                                assert_eq!(::memoffset::offset_of!(Vector2, value) * 8, 0);
                            }
                        }
                        pub mod vector31_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector31.0.1`
                            ///
                            /// Fixed size 62 bytes
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector31 {
                                /// `saturated float16[31]`
                                ///
                                /// Always aligned
                                /// Size 496 bits
                                pub value: [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 31],
                            }
                            impl ::canadensis_encoding::DataType for Vector31 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector31 {}
                            impl Vector31 {}
                            impl ::canadensis_encoding::Serialize for Vector31 {
                                fn size_bits(&self) -> usize {
                                    496
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                                }
                            }
                            impl ::canadensis_encoding::Deserialize for Vector31 {
                                fn deserialize(
                                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                                ) -> ::core::result::Result<
                                    Self,
                                    ::canadensis_encoding::DeserializeError,
                                >
                                where
                                    Self: Sized,
                                {
                                    Ok(Self::deserialize_zero_copy(cursor))
                                }
                            }
                            #[test]
                            fn test_layout() {
                                assert_eq!(::core::mem::size_of::<Vector31>() * 8, 496);
                                assert_eq!(::memoffset::offset_of!(Vector31, value) * 8, 0);
                            }
                        }
                        pub mod vector3_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector3.0.1`
                            ///
                            /// Fixed size 6 bytes
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector3 {
                                /// `saturated float16[3]`
                                ///
                                /// Always aligned
                                /// Size 48 bits
                                pub value: [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 3],
                            }
                            impl ::canadensis_encoding::DataType for Vector3 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector3 {}
                            impl Vector3 {}
                            impl ::canadensis_encoding::Serialize for Vector3 {
                                fn size_bits(&self) -> usize {
                                    48
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                                }
                            }
                            impl ::canadensis_encoding::Deserialize for Vector3 {
                                fn deserialize(
                                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                                ) -> ::core::result::Result<
                                    Self,
                                    ::canadensis_encoding::DeserializeError,
                                >
                                where
                                    Self: Sized,
                                {
                                    Ok(Self::deserialize_zero_copy(cursor))
                                }
                            }
                            #[test]
                            fn test_layout() {
                                assert_eq!(::core::mem::size_of::<Vector3>() * 8, 48);
                                assert_eq!(::memoffset::offset_of!(Vector3, value) * 8, 0);
                            }
                        }
                        pub mod vector4_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector4.0.1`
                            ///
                            /// Fixed size 8 bytes
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector4 {
                                /// `saturated float16[4]`
                                ///
                                /// Always aligned
                                /// Size 64 bits
                                pub value: [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 4],
                            }
                            impl ::canadensis_encoding::DataType for Vector4 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector4 {}
                            impl Vector4 {}
                            impl ::canadensis_encoding::Serialize for Vector4 {
                                fn size_bits(&self) -> usize {
                                    64
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                                }
                            }
                            impl ::canadensis_encoding::Deserialize for Vector4 {
                                fn deserialize(
                                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                                ) -> ::core::result::Result<
                                    Self,
                                    ::canadensis_encoding::DeserializeError,
                                >
                                where
                                    Self: Sized,
                                {
                                    Ok(Self::deserialize_zero_copy(cursor))
                                }
                            }
                            #[test]
                            fn test_layout() {
                                assert_eq!(::core::mem::size_of::<Vector4>() * 8, 64);
                                assert_eq!(::memoffset::offset_of!(Vector4, value) * 8, 0);
                            }
                        }
                        pub mod vector6_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector6.0.1`
                            ///
                            /// Fixed size 12 bytes
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector6 {
                                /// `saturated float16[6]`
                                ///
                                /// Always aligned
                                /// Size 96 bits
                                pub value: [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 6],
                            }
                            impl ::canadensis_encoding::DataType for Vector6 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector6 {}
                            impl Vector6 {}
                            impl ::canadensis_encoding::Serialize for Vector6 {
                                fn size_bits(&self) -> usize {
                                    96
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                                }
                            }
                            impl ::canadensis_encoding::Deserialize for Vector6 {
                                fn deserialize(
                                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                                ) -> ::core::result::Result<
                                    Self,
                                    ::canadensis_encoding::DeserializeError,
                                >
                                where
                                    Self: Sized,
                                {
                                    Ok(Self::deserialize_zero_copy(cursor))
                                }
                            }
                            #[test]
                            fn test_layout() {
                                assert_eq!(::core::mem::size_of::<Vector6>() * 8, 96);
                                assert_eq!(::memoffset::offset_of!(Vector6, value) * 8, 0);
                            }
                        }
                        pub mod vector8_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector8.0.1`
                            ///
                            /// Fixed size 16 bytes
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector8 {
                                /// `saturated float16[8]`
                                ///
                                /// Always aligned
                                /// Size 128 bits
                                pub value: [::canadensis_encoding::f16_zerocopy::ZeroCopyF16; 8],
                            }
                            impl ::canadensis_encoding::DataType for Vector8 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector8 {}
                            impl Vector8 {}
                            impl ::canadensis_encoding::Serialize for Vector8 {
                                fn size_bits(&self) -> usize {
                                    128
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                                }
                            }
                            impl ::canadensis_encoding::Deserialize for Vector8 {
                                fn deserialize(
                                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                                ) -> ::core::result::Result<
                                    Self,
                                    ::canadensis_encoding::DeserializeError,
                                >
                                where
                                    Self: Sized,
                                {
                                    Ok(Self::deserialize_zero_copy(cursor))
                                }
                            }
                            #[test]
                            fn test_layout() {
                                assert_eq!(::core::mem::size_of::<Vector8>() * 8, 128);
                                assert_eq!(::memoffset::offset_of!(Vector8, value) * 8, 0);
                            }
                        }
                    }
                    pub mod status_0_1 {
                        /// `reg.drone.service.actuator.common.Status.0.1`
                        ///
                        /// Fixed size 14 bytes
                        pub struct Status {
/// `uavcan.si.unit.temperature.Scalar.1.0`
///
/// Always aligned
/// Size 32 bits
pub motor_temperature: crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
/// `uavcan.si.unit.temperature.Scalar.1.0`
///
/// Always aligned
/// Size 32 bits
pub controller_temperature: crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
/// `saturated uint32`
///
/// Always aligned
/// Size 32 bits
pub error_count: u32,
/// `reg.drone.service.actuator.common.FaultFlags.0.1`
///
/// Always aligned
/// Size 16 bits
pub fault_flags: crate::reg::drone::service::actuator::common::fault_flags_0_1::FaultFlags,
}
                        impl ::canadensis_encoding::DataType for Status {
                            const EXTENT_BYTES: Option<u32> = Some(63);
                        }
                        impl ::canadensis_encoding::Message for Status {}
                        impl Status {}
                        impl ::canadensis_encoding::Serialize for Status {
                            fn size_bits(&self) -> usize {
                                112
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.motor_temperature);
                                cursor.write_composite(&self.controller_temperature);
                                cursor.write_aligned_u32(self.error_count);
                                cursor.write_composite(&self.fault_flags);
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for Status {
                            fn deserialize(
                                cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                            ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                            where
                                Self: Sized,
                            {
                                Ok(Status {
                                    motor_temperature: { cursor.read_composite()? },
                                    controller_temperature: { cursor.read_composite()? },
                                    error_count: { cursor.read_u32() as _ },
                                    fault_flags: { cursor.read_composite()? },
                                })
                            }
                        }
                    }
                }
                pub mod esc {
                    pub mod _0_1 {
                        /// `reg.drone.service.actuator.esc._.0.1`
                        ///
                        /// Fixed size 0 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct _0 {}
                        impl ::canadensis_encoding::DataType for _0 {
                            const EXTENT_BYTES: Option<u32> = Some(0);
                        }
                        impl ::canadensis_encoding::Message for _0 {}
                        impl _0 {}
                        impl ::canadensis_encoding::Serialize for _0 {
                            fn size_bits(&self) -> usize {
                                0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for _0 {
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
                            assert_eq!(::core::mem::size_of::<_0>() * 8, 0);
                        }
                    }
                }
                pub mod servo {
                    pub mod _0_1 {
                        /// `reg.drone.service.actuator.servo._.0.1`
                        ///
                        /// Fixed size 0 bytes
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct _0 {}
                        impl ::canadensis_encoding::DataType for _0 {
                            const EXTENT_BYTES: Option<u32> = Some(0);
                        }
                        impl ::canadensis_encoding::Message for _0 {}
                        impl _0 {}
                        impl ::canadensis_encoding::Serialize for _0 {
                            fn size_bits(&self) -> usize {
                                0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                            }
                        }
                        impl ::canadensis_encoding::Deserialize for _0 {
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
                            assert_eq!(::core::mem::size_of::<_0>() * 8, 0);
                        }
                    }
                }
            }
            pub mod air_data_computer {
                pub mod _0_1 {
                    /// `reg.drone.service.air_data_computer._.0.1`
                    ///
                    /// Fixed size 0 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct _0 {}
                    impl ::canadensis_encoding::DataType for _0 {
                        const EXTENT_BYTES: Option<u32> = Some(0);
                    }
                    impl ::canadensis_encoding::Message for _0 {}
                    impl _0 {
                        pub const MAX_PUBLICATION_PERIOD: f32 = 0.1_f32;
                    }
                    impl ::canadensis_encoding::Serialize for _0 {
                        fn size_bits(&self) -> usize {
                            0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for _0 {
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
                        assert_eq!(::core::mem::size_of::<_0>() * 8, 0);
                    }
                }
            }
            pub mod battery {
                pub mod _0_1 {
                    /// `reg.drone.service.battery._.0.1`
                    ///
                    /// Fixed size 0 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct _0 {}
                    impl ::canadensis_encoding::DataType for _0 {
                        const EXTENT_BYTES: Option<u32> = Some(0);
                    }
                    impl ::canadensis_encoding::Message for _0 {}
                    impl _0 {}
                    impl ::canadensis_encoding::Serialize for _0 {
                        fn size_bits(&self) -> usize {
                            0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for _0 {
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
                        assert_eq!(::core::mem::size_of::<_0>() * 8, 0);
                    }
                }
                pub mod error_0_1 {
                    /// `reg.drone.service.battery.Error.0.1`
                    ///
                    /// Fixed size 1 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Error {
                        /// `saturated uint8`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub value: u8,
                    }
                    impl ::canadensis_encoding::DataType for Error {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Error {}
                    impl Error {
                        pub const BAD_BATTERY: u8 = 10;
                        pub const BMS_ERROR: u8 = 20;
                        pub const CELL_COUNT: u8 = 62;
                        pub const CELL_OVERVOLTAGE: u8 = 60;
                        pub const CELL_UNDERVOLTAGE: u8 = 61;
                        pub const CONFIGURATION: u8 = 30;
                        pub const NEEDS_SERVICE: u8 = 11;
                        pub const NONE: u8 = 0;
                        pub const OVERDISCHARGE: u8 = 50;
                        pub const OVERLOAD: u8 = 51;
                        pub const TEMPERATURE_COLD: u8 = 101;
                        pub const TEMPERATURE_HOT: u8 = 100;
                    }
                    impl ::canadensis_encoding::Serialize for Error {
                        fn size_bits(&self) -> usize {
                            8
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Error {
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
                        assert_eq!(::core::mem::size_of::<Error>() * 8, 8);
                        assert_eq!(::memoffset::offset_of!(Error, value) * 8, 0);
                    }
                }
                pub mod parameters_0_1 {
                    /// `reg.drone.service.battery.Parameters.0.1`
                    ///
                    /// Fixed size 54 bytes
                    pub struct Parameters {
                        /// `truncated uint64`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub unique_id: u64,
                        /// `uavcan.si.unit.mass.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub mass: crate::uavcan::si::unit::mass::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub design_capacity:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.voltage.Scalar.1.0[2]`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub design_cell_voltage_min_max:
                            [crate::uavcan::si::unit::voltage::scalar_1_0::Scalar; 2],
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub discharge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub discharge_current_burst:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_current_fast:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_termination_treshold:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.voltage.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                        /// `saturated uint16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub cycle_count: u16,
                        // 8 bits of padding
                        /// `saturated uint8`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub series_cell_count: u8,
                        /// `saturated uint7`
                        ///
                        /// Always aligned
                        /// Size 7 bits
                        pub state_of_health_pct: u8,
                        // 1 bits of padding
                        /// `reg.drone.service.battery.Technology.0.1`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub technology:
                            crate::reg::drone::service::battery::technology_0_1::Technology,
                    }
                    impl ::canadensis_encoding::DataType for Parameters {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Parameters {}
                    impl Parameters {}
                    impl ::canadensis_encoding::Serialize for Parameters {
                        fn size_bits(&self) -> usize {
                            432
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_u64(self.unique_id);
                            cursor.write_composite(&self.mass);
                            cursor.write_composite(&self.design_capacity);
                            for value in (self.design_cell_voltage_min_max).iter() {
                                cursor.write_composite(value);
                            }
                            cursor.write_composite(&self.discharge_current);
                            cursor.write_composite(&self.discharge_current_burst);
                            cursor.write_composite(&self.charge_current);
                            cursor.write_composite(&self.charge_current_fast);
                            cursor.write_composite(&self.charge_termination_treshold);
                            cursor.write_composite(&self.charge_voltage);
                            cursor.write_aligned_u16(self.cycle_count);
                            cursor.skip_8();
                            cursor.write_aligned_u8(self.series_cell_count);
                            cursor.write_u7(self.state_of_health_pct);
                            cursor.skip_1();
                            cursor.write_composite(&self.technology);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Parameters {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Parameters {
                                unique_id: { cursor.read_u64() as _ },
                                mass: { cursor.read_composite()? },
                                design_capacity: { cursor.read_composite()? },
                                design_cell_voltage_min_max: {
                                    [cursor.read_composite()?, cursor.read_composite()?]
                                },
                                discharge_current: { cursor.read_composite()? },
                                discharge_current_burst: { cursor.read_composite()? },
                                charge_current: { cursor.read_composite()? },
                                charge_current_fast: { cursor.read_composite()? },
                                charge_termination_treshold: { cursor.read_composite()? },
                                charge_voltage: { cursor.read_composite()? },
                                cycle_count: { cursor.read_u16() as _ },
                                series_cell_count: {
                                    cursor.skip_8();
                                    cursor.read_u8() as _
                                },
                                state_of_health_pct: { cursor.read_u7() as _ },
                                technology: {
                                    cursor.skip_1();
                                    cursor.read_composite()?
                                },
                            })
                        }
                    }
                }
                pub mod parameters_0_2 {
                    /// `reg.drone.service.battery.Parameters.0.2`
                    ///
                    /// Fixed size 54 bytes
                    pub struct Parameters {
                        /// `truncated uint64`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub unique_id: u64,
                        /// `uavcan.si.unit.mass.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub mass: crate::uavcan::si::unit::mass::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub design_capacity:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.voltage.Scalar.1.0[2]`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub design_cell_voltage_min_max:
                            [crate::uavcan::si::unit::voltage::scalar_1_0::Scalar; 2],
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub discharge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub discharge_current_burst:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_current_fast:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_termination_threshold:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.voltage.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                        /// `saturated uint16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub cycle_count: u16,
                        // 16 bits of padding
                        /// `saturated uint7`
                        ///
                        /// Always aligned
                        /// Size 7 bits
                        pub state_of_health_pct: u8,
                        // 1 bits of padding
                        /// `reg.drone.service.battery.Technology.0.1`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub technology:
                            crate::reg::drone::service::battery::technology_0_1::Technology,
                    }
                    impl ::canadensis_encoding::DataType for Parameters {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Parameters {}
                    impl Parameters {}
                    impl ::canadensis_encoding::Serialize for Parameters {
                        fn size_bits(&self) -> usize {
                            432
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_u64(self.unique_id);
                            cursor.write_composite(&self.mass);
                            cursor.write_composite(&self.design_capacity);
                            for value in (self.design_cell_voltage_min_max).iter() {
                                cursor.write_composite(value);
                            }
                            cursor.write_composite(&self.discharge_current);
                            cursor.write_composite(&self.discharge_current_burst);
                            cursor.write_composite(&self.charge_current);
                            cursor.write_composite(&self.charge_current_fast);
                            cursor.write_composite(&self.charge_termination_threshold);
                            cursor.write_composite(&self.charge_voltage);
                            cursor.write_aligned_u16(self.cycle_count);
                            cursor.skip_16();
                            cursor.write_u7(self.state_of_health_pct);
                            cursor.skip_1();
                            cursor.write_composite(&self.technology);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Parameters {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Parameters {
                                unique_id: { cursor.read_u64() as _ },
                                mass: { cursor.read_composite()? },
                                design_capacity: { cursor.read_composite()? },
                                design_cell_voltage_min_max: {
                                    [cursor.read_composite()?, cursor.read_composite()?]
                                },
                                discharge_current: { cursor.read_composite()? },
                                discharge_current_burst: { cursor.read_composite()? },
                                charge_current: { cursor.read_composite()? },
                                charge_current_fast: { cursor.read_composite()? },
                                charge_termination_threshold: { cursor.read_composite()? },
                                charge_voltage: { cursor.read_composite()? },
                                cycle_count: { cursor.read_u16() as _ },
                                state_of_health_pct: {
                                    cursor.skip_16();
                                    cursor.read_u7() as _
                                },
                                technology: {
                                    cursor.skip_1();
                                    cursor.read_composite()?
                                },
                            })
                        }
                    }
                }
                pub mod parameters_0_3 {
                    /// `reg.drone.service.battery.Parameters.0.3`
                    ///
                    /// Fixed size 58 bytes
                    pub struct Parameters {
                        /// `truncated uint64`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub unique_id: u64,
                        /// `uavcan.si.unit.mass.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub mass: crate::uavcan::si::unit::mass::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub design_capacity:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.voltage.Scalar.1.0[2]`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub design_cell_voltage_min_max:
                            [crate::uavcan::si::unit::voltage::scalar_1_0::Scalar; 2],
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub discharge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub discharge_current_burst:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_current_fast:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_termination_threshold:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        /// `uavcan.si.unit.voltage.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub charge_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                        /// `saturated uint16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub cycle_count: u16,
                        // 16 bits of padding
                        /// `saturated uint7`
                        ///
                        /// Always aligned
                        /// Size 7 bits
                        pub state_of_health_pct: u8,
                        // 1 bits of padding
                        /// `reg.drone.service.battery.Technology.0.1`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub technology:
                            crate::reg::drone::service::battery::technology_0_1::Technology,
                        /// `uavcan.si.unit.voltage.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub nominal_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Parameters {
                        const EXTENT_BYTES: Option<u32> = Some(67);
                    }
                    impl ::canadensis_encoding::Message for Parameters {}
                    impl Parameters {}
                    impl ::canadensis_encoding::Serialize for Parameters {
                        fn size_bits(&self) -> usize {
                            464
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_u64(self.unique_id);
                            cursor.write_composite(&self.mass);
                            cursor.write_composite(&self.design_capacity);
                            for value in (self.design_cell_voltage_min_max).iter() {
                                cursor.write_composite(value);
                            }
                            cursor.write_composite(&self.discharge_current);
                            cursor.write_composite(&self.discharge_current_burst);
                            cursor.write_composite(&self.charge_current);
                            cursor.write_composite(&self.charge_current_fast);
                            cursor.write_composite(&self.charge_termination_threshold);
                            cursor.write_composite(&self.charge_voltage);
                            cursor.write_aligned_u16(self.cycle_count);
                            cursor.skip_16();
                            cursor.write_u7(self.state_of_health_pct);
                            cursor.skip_1();
                            cursor.write_composite(&self.technology);
                            cursor.write_composite(&self.nominal_voltage);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Parameters {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Parameters {
                                unique_id: { cursor.read_u64() as _ },
                                mass: { cursor.read_composite()? },
                                design_capacity: { cursor.read_composite()? },
                                design_cell_voltage_min_max: {
                                    [cursor.read_composite()?, cursor.read_composite()?]
                                },
                                discharge_current: { cursor.read_composite()? },
                                discharge_current_burst: { cursor.read_composite()? },
                                charge_current: { cursor.read_composite()? },
                                charge_current_fast: { cursor.read_composite()? },
                                charge_termination_threshold: { cursor.read_composite()? },
                                charge_voltage: { cursor.read_composite()? },
                                cycle_count: { cursor.read_u16() as _ },
                                state_of_health_pct: {
                                    cursor.skip_16();
                                    cursor.read_u7() as _
                                },
                                technology: {
                                    cursor.skip_1();
                                    cursor.read_composite()?
                                },
                                nominal_voltage: { cursor.read_composite()? },
                            })
                        }
                    }
                }
                pub mod status_0_1 {
                    /// `reg.drone.service.battery.Status.0.1`
                    ///
                    /// Fixed size 23 bytes
                    pub struct Status {
                        /// `reg.drone.service.common.Heartbeat.0.1`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub heartbeat: crate::reg::drone::service::common::heartbeat_0_1::Heartbeat,
                        /// `uavcan.si.unit.temperature.Scalar.1.0[2]`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub temperature_min_max:
                            [crate::uavcan::si::unit::temperature::scalar_1_0::Scalar; 2],
                        /// `uavcan.si.unit.voltage.Scalar.1.0[2]`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub cell_voltage_min_max:
                            [crate::uavcan::si::unit::voltage::scalar_1_0::Scalar; 2],
                        /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub available_charge:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        /// `reg.drone.service.battery.Error.0.1`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub error: crate::reg::drone::service::battery::error_0_1::Error,
                    }
                    impl ::canadensis_encoding::DataType for Status {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Status {}
                    impl Status {}
                    impl ::canadensis_encoding::Serialize for Status {
                        fn size_bits(&self) -> usize {
                            184
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.heartbeat);
                            for value in (self.temperature_min_max).iter() {
                                cursor.write_composite(value);
                            }
                            for value in (self.cell_voltage_min_max).iter() {
                                cursor.write_composite(value);
                            }
                            cursor.write_composite(&self.available_charge);
                            cursor.write_composite(&self.error);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Status {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Status {
                                heartbeat: { cursor.read_composite()? },
                                temperature_min_max: {
                                    [cursor.read_composite()?, cursor.read_composite()?]
                                },
                                cell_voltage_min_max: {
                                    [cursor.read_composite()?, cursor.read_composite()?]
                                },
                                available_charge: { cursor.read_composite()? },
                                error: { cursor.read_composite()? },
                            })
                        }
                    }
                }
                pub mod status_0_2 {
                    /// `reg.drone.service.battery.Status.0.2`
                    ///
                    /// Size ranges from 24 to 534 bytes
                    pub struct Status {
                        /// `reg.drone.service.common.Heartbeat.0.1`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub heartbeat: crate::reg::drone::service::common::heartbeat_0_1::Heartbeat,
                        /// `uavcan.si.unit.temperature.Scalar.1.0[2]`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub temperature_min_max:
                            [crate::uavcan::si::unit::temperature::scalar_1_0::Scalar; 2],
                        // 64 bits of padding
                        /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub available_charge:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        /// `reg.drone.service.battery.Error.0.1`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub error: crate::reg::drone::service::battery::error_0_1::Error,
                        /// `saturated float16[<=255]`
                        ///
                        /// Always aligned
                        /// Size ranges from 0 to 4080 bits
                        pub cell_voltages:
                            ::heapless::Vec<::canadensis_encoding::f16_zerocopy::ZeroCopyF16, 255>,
                    }
                    impl ::canadensis_encoding::DataType for Status {
                        const EXTENT_BYTES: Option<u32> = Some(600);
                    }
                    impl ::canadensis_encoding::Message for Status {}
                    impl Status {
                        pub const MAX_CELLS: u8 = 255;
                    }
                    impl ::canadensis_encoding::Serialize for Status {
                        fn size_bits(&self) -> usize {
                            16 + (self.temperature_min_max).len() * 32
                                + 64
                                + 32
                                + 8
                                + 8
                                + (self.cell_voltages).len() * 16
                                + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.heartbeat);
                            for value in (self.temperature_min_max).iter() {
                                cursor.write_composite(value);
                            }
                            cursor.skip_64();
                            cursor.write_composite(&self.available_charge);
                            cursor.write_composite(&self.error);
                            cursor.write_aligned_u8((self.cell_voltages).len() as u8);
                            for value in (self.cell_voltages).iter() {
                                cursor.write_f16((*value).into());
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Status {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Status {
                                heartbeat: { cursor.read_composite()? },
                                temperature_min_max: {
                                    [cursor.read_composite()?, cursor.read_composite()?]
                                },
                                available_charge: {
                                    cursor.skip_64();
                                    cursor.read_composite()?
                                },
                                error: { cursor.read_composite()? },
                                cell_voltages: {
                                    let length = cursor.read_u8() as _;
                                    if length <= 255 {
                                        let mut elements = ::heapless::Vec::new();
                                        for _ in 0..length {
                                            let _ = elements.push(cursor.read_f16().into());
                                        }
                                        elements
                                    } else {
                                        return Err(
                                            ::canadensis_encoding::DeserializeError::ArrayLength,
                                        );
                                    }
                                },
                            })
                        }
                    }
                }
                pub mod technology_0_1 {
                    /// `reg.drone.service.battery.Technology.0.1`
                    ///
                    /// Fixed size 1 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Technology {
                        /// `saturated uint8`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub value: u8,
                    }
                    impl ::canadensis_encoding::DataType for Technology {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Technology {}
                    impl Technology {
                        pub const AL_O2: u8 = 21;
                        pub const EDLC: u8 = 200;
                        pub const LI_BCX: u8 = 11;
                        pub const LI_LCO: u8 = 100;
                        pub const LI_LCO_POUCH: u8 = 110;
                        pub const LI_LFP: u8 = 101;
                        pub const LI_LFP_POUCH: u8 = 111;
                        pub const LI_LMO: u8 = 104;
                        pub const LI_MNO2: u8 = 12;
                        pub const LI_NCA: u8 = 103;
                        pub const LI_NMC: u8 = 102;
                        pub const LI_S: u8 = 105;
                        pub const LI_SOCL2: u8 = 10;
                        pub const NI_CD: u8 = 121;
                        pub const NI_FE: u8 = 123;
                        pub const NI_MH: u8 = 120;
                        pub const NI_ZN: u8 = 122;
                        pub const OTHER: u8 = 0;
                        pub const PB_AC: u8 = 130;
                        pub const PB_AC_SEALED: u8 = 131;
                        pub const ZN_MNO2_KOH: u8 = 32;
                        pub const ZN_MNO2_NH4CL: u8 = 30;
                        pub const ZN_MNO2_ZNCL2: u8 = 31;
                        pub const ZN_O2: u8 = 20;
                    }
                    impl ::canadensis_encoding::Serialize for Technology {
                        fn size_bits(&self) -> usize {
                            8
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Technology {
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
                        assert_eq!(::core::mem::size_of::<Technology>() * 8, 8);
                        assert_eq!(::memoffset::offset_of!(Technology, value) * 8, 0);
                    }
                }
            }
            pub mod common {
                pub mod heartbeat_0_1 {
                    /// `reg.drone.service.common.Heartbeat.0.1`
                    ///
                    /// Fixed size 2 bytes
                    pub struct Heartbeat {
                        /// `reg.drone.service.common.Readiness.0.1`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub readiness: crate::reg::drone::service::common::readiness_0_1::Readiness,
                        /// `uavcan.node.Health.1.0`
                        ///
                        /// Always aligned
                        /// Size 8 bits
                        pub health: crate::uavcan::node::health_1_0::Health,
                    }
                    impl ::canadensis_encoding::DataType for Heartbeat {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Heartbeat {}
                    impl Heartbeat {
                        pub const MAX_PUBLICATION_PERIOD: u8 = 1;
                    }
                    impl ::canadensis_encoding::Serialize for Heartbeat {
                        fn size_bits(&self) -> usize {
                            16
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.readiness);
                            cursor.write_composite(&self.health);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Heartbeat {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Heartbeat {
                                readiness: { cursor.read_composite()? },
                                health: { cursor.read_composite()? },
                            })
                        }
                    }
                }
                pub mod readiness_0_1 {
                    /// `reg.drone.service.common.Readiness.0.1`
                    ///
                    /// Fixed size 1 bytes
                    pub struct Readiness {
                        /// `truncated uint2`
                        ///
                        /// Always aligned
                        /// Size 2 bits
                        pub value: u8,
                    }
                    impl ::canadensis_encoding::DataType for Readiness {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Readiness {}
                    impl Readiness {
                        pub const ENGAGED: u8 = 3;
                        pub const SLEEP: u8 = 0;
                        pub const STANDBY: u8 = 2;
                    }
                    impl ::canadensis_encoding::Serialize for Readiness {
                        fn size_bits(&self) -> usize {
                            8
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_u2(self.value);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Readiness {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Readiness {
                                value: { cursor.read_u2() as _ },
                            })
                        }
                    }
                }
            }
            pub mod gnss {
                pub mod _0_1 {
                    /// `reg.drone.service.gnss._.0.1`
                    ///
                    /// Fixed size 0 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct _0 {}
                    impl ::canadensis_encoding::DataType for _0 {
                        const EXTENT_BYTES: Option<u32> = Some(0);
                    }
                    impl ::canadensis_encoding::Message for _0 {}
                    impl _0 {}
                    impl ::canadensis_encoding::Serialize for _0 {
                        fn size_bits(&self) -> usize {
                            0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for _0 {
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
                        assert_eq!(::core::mem::size_of::<_0>() * 8, 0);
                    }
                }
                pub mod dilution_of_precision_0_1 {
                    /// `reg.drone.service.gnss.DilutionOfPrecision.0.1`
                    ///
                    /// Fixed size 14 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct DilutionOfPrecision {
                        /// `saturated float16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub geometric: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                        /// `saturated float16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub position: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                        /// `saturated float16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub horizontal: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                        /// `saturated float16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub vertical: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                        /// `saturated float16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub time: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                        /// `saturated float16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub northing: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                        /// `saturated float16`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub easting: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                    }
                    impl ::canadensis_encoding::DataType for DilutionOfPrecision {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for DilutionOfPrecision {}
                    impl DilutionOfPrecision {}
                    impl ::canadensis_encoding::Serialize for DilutionOfPrecision {
                        fn size_bits(&self) -> usize {
                            112
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for DilutionOfPrecision {
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
                        assert_eq!(::core::mem::size_of::<DilutionOfPrecision>() * 8, 112);
                        assert_eq!(
                            ::memoffset::offset_of!(DilutionOfPrecision, geometric) * 8,
                            0
                        );
                        assert_eq!(
                            ::memoffset::offset_of!(DilutionOfPrecision, position) * 8,
                            16
                        );
                        assert_eq!(
                            ::memoffset::offset_of!(DilutionOfPrecision, horizontal) * 8,
                            32
                        );
                        assert_eq!(
                            ::memoffset::offset_of!(DilutionOfPrecision, vertical) * 8,
                            48
                        );
                        assert_eq!(::memoffset::offset_of!(DilutionOfPrecision, time) * 8, 64);
                        assert_eq!(
                            ::memoffset::offset_of!(DilutionOfPrecision, northing) * 8,
                            80
                        );
                        assert_eq!(
                            ::memoffset::offset_of!(DilutionOfPrecision, easting) * 8,
                            96
                        );
                    }
                }
                pub mod heartbeat_0_1 {
                    /// `reg.drone.service.gnss.Heartbeat.0.1`
                    ///
                    /// Fixed size 25 bytes
                    pub struct Heartbeat {
/// `reg.drone.service.common.Heartbeat.0.1`
///
/// Always aligned
/// Size 16 bits
pub heartbeat: crate::reg::drone::service::common::heartbeat_0_1::Heartbeat,
/// `reg.drone.service.gnss.Sources.0.1`
///
/// Always aligned
/// Size 48 bits
pub sources: crate::reg::drone::service::gnss::sources_0_1::Sources,
/// `reg.drone.service.gnss.DilutionOfPrecision.0.1`
///
/// Always aligned
/// Size 112 bits
pub dop: crate::reg::drone::service::gnss::dilution_of_precision_0_1::DilutionOfPrecision,
/// `saturated uint8`
///
/// Always aligned
/// Size 8 bits
pub num_visible_satellites: u8,
/// `saturated uint8`
///
/// Always aligned
/// Size 8 bits
pub num_used_satellites: u8,
/// `saturated bool`
///
/// Always aligned
/// Size 1 bits
pub fix: bool,
/// `saturated bool`
///
/// Not always aligned
/// Size 1 bits
pub rtk_fix: bool,
}
                    impl ::canadensis_encoding::DataType for Heartbeat {
                        const EXTENT_BYTES: Option<u32> = Some(124);
                    }
                    impl ::canadensis_encoding::Message for Heartbeat {}
                    impl Heartbeat {}
                    impl ::canadensis_encoding::Serialize for Heartbeat {
                        fn size_bits(&self) -> usize {
                            200
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.heartbeat);
                            cursor.write_composite(&self.sources);
                            cursor.write_composite(&self.dop);
                            cursor.write_aligned_u8(self.num_visible_satellites);
                            cursor.write_aligned_u8(self.num_used_satellites);
                            cursor.write_bool(self.fix);
                            cursor.write_bool(self.rtk_fix);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Heartbeat {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Heartbeat {
                                heartbeat: { cursor.read_composite()? },
                                sources: { cursor.read_composite()? },
                                dop: { cursor.read_composite()? },
                                num_visible_satellites: { cursor.read_u8() as _ },
                                num_used_satellites: { cursor.read_u8() as _ },
                                fix: { cursor.read_bool() },
                                rtk_fix: { cursor.read_bool() },
                            })
                        }
                    }
                }
                pub mod sources_0_1 {
                    /// `reg.drone.service.gnss.Sources.0.1`
                    ///
                    /// Fixed size 6 bytes
                    pub struct Sources {
                        /// `saturated bool`
                        ///
                        /// Always aligned
                        /// Size 1 bits
                        pub gps_l1: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub gps_l2: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub gps_l5: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub glonass_l1: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub glonass_l2: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub glonass_l3: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub galileo_e1: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub galileo_e5a: bool,
                        /// `saturated bool`
                        ///
                        /// Always aligned
                        /// Size 1 bits
                        pub galileo_e5b: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub galileo_e6: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub beidou_b1: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub beidou_b2: bool,
                        // 5 bits of padding
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub sbas: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub gbas: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub rtk_base: bool,
                        // 3 bits of padding
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub imu: bool,
                        /// `saturated bool`
                        ///
                        /// Always aligned
                        /// Size 1 bits
                        pub visual_odometry: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub dead_reckoning: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub uwb: bool,
                        // 4 bits of padding
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub magnetic_compass: bool,
                        /// `saturated bool`
                        ///
                        /// Always aligned
                        /// Size 1 bits
                        pub gyro_compass: bool,
                        /// `saturated bool`
                        ///
                        /// Not always aligned
                        /// Size 1 bits
                        pub other_compass: bool,
                        // 14 bits of padding
                    }
                    impl ::canadensis_encoding::DataType for Sources {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Sources {}
                    impl Sources {}
                    impl ::canadensis_encoding::Serialize for Sources {
                        fn size_bits(&self) -> usize {
                            48
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_bool(self.gps_l1);
                            cursor.write_bool(self.gps_l2);
                            cursor.write_bool(self.gps_l5);
                            cursor.write_bool(self.glonass_l1);
                            cursor.write_bool(self.glonass_l2);
                            cursor.write_bool(self.glonass_l3);
                            cursor.write_bool(self.galileo_e1);
                            cursor.write_bool(self.galileo_e5a);
                            cursor.write_bool(self.galileo_e5b);
                            cursor.write_bool(self.galileo_e6);
                            cursor.write_bool(self.beidou_b1);
                            cursor.write_bool(self.beidou_b2);
                            cursor.skip_5();
                            cursor.write_bool(self.sbas);
                            cursor.write_bool(self.gbas);
                            cursor.write_bool(self.rtk_base);
                            cursor.skip_3();
                            cursor.write_bool(self.imu);
                            cursor.write_bool(self.visual_odometry);
                            cursor.write_bool(self.dead_reckoning);
                            cursor.write_bool(self.uwb);
                            cursor.skip_4();
                            cursor.write_bool(self.magnetic_compass);
                            cursor.write_bool(self.gyro_compass);
                            cursor.write_bool(self.other_compass);
                            cursor.skip_14();
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Sources {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Sources {
                                gps_l1: { cursor.read_bool() },
                                gps_l2: { cursor.read_bool() },
                                gps_l5: { cursor.read_bool() },
                                glonass_l1: { cursor.read_bool() },
                                glonass_l2: { cursor.read_bool() },
                                glonass_l3: { cursor.read_bool() },
                                galileo_e1: { cursor.read_bool() },
                                galileo_e5a: { cursor.read_bool() },
                                galileo_e5b: { cursor.read_bool() },
                                galileo_e6: { cursor.read_bool() },
                                beidou_b1: { cursor.read_bool() },
                                beidou_b2: { cursor.read_bool() },
                                sbas: {
                                    cursor.skip_5();
                                    cursor.read_bool()
                                },
                                gbas: { cursor.read_bool() },
                                rtk_base: { cursor.read_bool() },
                                imu: {
                                    cursor.skip_3();
                                    cursor.read_bool()
                                },
                                visual_odometry: { cursor.read_bool() },
                                dead_reckoning: { cursor.read_bool() },
                                uwb: { cursor.read_bool() },
                                magnetic_compass: {
                                    cursor.skip_4();
                                    cursor.read_bool()
                                },
                                gyro_compass: { cursor.read_bool() },
                                other_compass: { cursor.read_bool() },
                            })
                        }
                    }
                }
                pub mod time_0_1 {
                    /// `reg.drone.service.gnss.Time.0.1`
                    ///
                    /// Fixed size 21 bytes
                    pub struct Time {
                        /// `reg.drone.physics.time.TAI64VarTs.0.1`
                        ///
                        /// Always aligned
                        /// Size 152 bits
                        pub value: crate::reg::drone::physics::time::tai64_var_ts_0_1::TAI64VarTs,
                        /// `uavcan.time.TAIInfo.0.1`
                        ///
                        /// Always aligned
                        /// Size 16 bits
                        pub info: crate::uavcan::time::tai_info_0_1::TAIInfo,
                    }
                    impl ::canadensis_encoding::DataType for Time {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Time {}
                    impl Time {}
                    impl ::canadensis_encoding::Serialize for Time {
                        fn size_bits(&self) -> usize {
                            168
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.value);
                            cursor.write_composite(&self.info);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Time {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Time {
                                value: { cursor.read_composite()? },
                                info: { cursor.read_composite()? },
                            })
                        }
                    }
                }
            }
            pub mod sensor {
                pub mod status_0_1 {
                    /// `reg.drone.service.sensor.Status.0.1`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Status {
                        /// `uavcan.si.unit.duration.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub data_validity_period:
                            crate::uavcan::si::unit::duration::scalar_1_0::Scalar,
                        /// `saturated uint32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub error_count: u32,
                        /// `uavcan.si.unit.temperature.Scalar.1.0`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub sensor_temperature:
                            crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Status {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Status {}
                    impl Status {
                        pub const MAX_PUBLICATION_PERIOD: u8 = 1;
                    }
                    impl ::canadensis_encoding::Serialize for Status {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Status {
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
                        assert_eq!(::core::mem::size_of::<Status>() * 8, 96);
                        assert_eq!(::memoffset::offset_of!(Status, data_validity_period) * 8, 0);
                        assert_eq!(::memoffset::offset_of!(Status, error_count) * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Status, sensor_temperature) * 8, 64);
                    }
                }
            }
        }
    }
}
pub mod uavcan {
    pub mod diagnostic {
        pub mod record_1_0 {
            /// `uavcan.diagnostic.Record.1.0`
            ///
            /// Size ranges from 9 to 121 bytes
            pub struct Record {
                /// `uavcan.time.SynchronizedTimestamp.1.0`
                ///
                /// Always aligned
                /// Size 56 bits
                pub timestamp:
                    crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                /// `uavcan.diagnostic.Severity.1.0`
                ///
                /// Always aligned
                /// Size 8 bits
                pub severity: crate::uavcan::diagnostic::severity_1_0::Severity,
                /// `saturated uint8[<=112]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 896 bits
                pub text: ::heapless::Vec<u8, 112>,
            }
            impl ::canadensis_encoding::DataType for Record {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Message for Record {}
            impl Record {}
            impl ::canadensis_encoding::Serialize for Record {
                fn size_bits(&self) -> usize {
                    56 + 8 + 8 + (self.text).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.timestamp);
                    cursor.write_composite(&self.severity);
                    cursor.write_aligned_u8((self.text).len() as u8);
                    cursor.write_bytes(&(self.text)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for Record {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Record {
                        timestamp: { cursor.read_composite()? },
                        severity: { cursor.read_composite()? },
                        text: {
                            let length = cursor.read_u8() as _;
                            if length <= 112 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
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
        pub mod record_1_1 {
            /// `uavcan.diagnostic.Record.1.1`
            ///
            /// Size ranges from 9 to 264 bytes
            pub struct Record {
                /// `uavcan.time.SynchronizedTimestamp.1.0`
                ///
                /// Always aligned
                /// Size 56 bits
                pub timestamp:
                    crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                /// `uavcan.diagnostic.Severity.1.0`
                ///
                /// Always aligned
                /// Size 8 bits
                pub severity: crate::uavcan::diagnostic::severity_1_0::Severity,
                /// `saturated uint8[<=255]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 2040 bits
                pub text: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for Record {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Message for Record {}
            impl Record {}
            impl ::canadensis_encoding::Serialize for Record {
                fn size_bits(&self) -> usize {
                    56 + 8 + 8 + (self.text).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.timestamp);
                    cursor.write_composite(&self.severity);
                    cursor.write_aligned_u8((self.text).len() as u8);
                    cursor.write_bytes(&(self.text)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for Record {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Record {
                        timestamp: { cursor.read_composite()? },
                        severity: { cursor.read_composite()? },
                        text: {
                            let length = cursor.read_u8() as _;
                            if length <= 255 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
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
        pub mod severity_1_0 {
            /// `uavcan.diagnostic.Severity.1.0`
            ///
            /// Fixed size 1 bytes
            pub struct Severity {
                /// `saturated uint3`
                ///
                /// Always aligned
                /// Size 3 bits
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for Severity {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Severity {}
            impl Severity {
                pub const ALERT: u8 = 7;
                pub const CRITICAL: u8 = 6;
                pub const DEBUG: u8 = 1;
                pub const ERROR: u8 = 5;
                pub const INFO: u8 = 2;
                pub const NOTICE: u8 = 3;
                pub const TRACE: u8 = 0;
                pub const WARNING: u8 = 4;
            }
            impl ::canadensis_encoding::Serialize for Severity {
                fn size_bits(&self) -> usize {
                    8
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u3(self.value);
                }
            }
            impl ::canadensis_encoding::Deserialize for Severity {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Severity {
                        value: { cursor.read_u3() as _ },
                    })
                }
            }
        }
    }
    pub mod file {
        pub mod error_1_0 {
            /// `uavcan.file.Error.1.0`
            ///
            /// Fixed size 2 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct Error {
                /// `saturated uint16`
                ///
                /// Always aligned
                /// Size 16 bits
                pub value: u16,
            }
            impl ::canadensis_encoding::DataType for Error {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Error {}
            impl Error {
                pub const ACCESS_DENIED: u16 = 13;
                pub const FILE_TOO_LARGE: u16 = 27;
                pub const INVALID_VALUE: u16 = 22;
                pub const IO_ERROR: u16 = 5;
                pub const IS_DIRECTORY: u16 = 21;
                pub const NOT_FOUND: u16 = 2;
                pub const NOT_SUPPORTED: u16 = 38;
                pub const OK: u16 = 0;
                pub const OUT_OF_SPACE: u16 = 28;
                pub const UNKNOWN_ERROR: u16 = 65535;
            }
            impl ::canadensis_encoding::Serialize for Error {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for Error {
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
                assert_eq!(::core::mem::size_of::<Error>() * 8, 16);
                assert_eq!(::memoffset::offset_of!(Error, value) * 8, 0);
            }
        }
        pub mod get_info_0_1 {
            /// `uavcan.file.GetInfo.0.1`
            ///
            /// Size ranges from 1 to 113 bytes
            pub struct GetInfoRequest {
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 904 bits
                pub path: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for GetInfoRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for GetInfoRequest {}
            impl GetInfoRequest {}
            impl ::canadensis_encoding::Serialize for GetInfoRequest {
                fn size_bits(&self) -> usize {
                    (self.path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.path);
                }
            }
            impl ::canadensis_encoding::Deserialize for GetInfoRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(GetInfoRequest {
                        path: { cursor.read_composite()? },
                    })
                }
            }

            /// `uavcan.file.GetInfo.0.1`
            ///
            /// Fixed size 13 bytes
            pub struct GetInfoResponse {
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub size: u64,
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub unix_timestamp_of_last_modification: u64,
                /// `saturated bool`
                ///
                /// Always aligned
                /// Size 1 bits
                pub is_file_not_directory: bool,
                /// `saturated bool`
                ///
                /// Not always aligned
                /// Size 1 bits
                pub is_link: bool,
                /// `saturated bool`
                ///
                /// Not always aligned
                /// Size 1 bits
                pub is_readable: bool,
                /// `saturated bool`
                ///
                /// Not always aligned
                /// Size 1 bits
                pub is_writeable: bool,
                // 4 bits of padding
            }
            impl ::canadensis_encoding::DataType for GetInfoResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for GetInfoResponse {}
            impl GetInfoResponse {}
            impl ::canadensis_encoding::Serialize for GetInfoResponse {
                fn size_bits(&self) -> usize {
                    104
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                    cursor.write_u40(self.size);
                    cursor.write_u40(self.unix_timestamp_of_last_modification);
                    cursor.write_bool(self.is_file_not_directory);
                    cursor.write_bool(self.is_link);
                    cursor.write_bool(self.is_readable);
                    cursor.write_bool(self.is_writeable);
                    cursor.skip_4();
                }
            }
            impl ::canadensis_encoding::Deserialize for GetInfoResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(GetInfoResponse {
                        error: { cursor.read_composite()? },
                        size: { cursor.read_u40() as _ },
                        unix_timestamp_of_last_modification: { cursor.read_u40() as _ },
                        is_file_not_directory: { cursor.read_bool() },
                        is_link: { cursor.read_bool() },
                        is_readable: { cursor.read_bool() },
                        is_writeable: { cursor.read_bool() },
                    })
                }
            }
        }
        pub mod get_info_0_2 {
            /// `uavcan.file.GetInfo.0.2`
            ///
            /// Size ranges from 1 to 256 bytes
            pub struct GetInfoRequest {
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2048 bits
                pub path: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for GetInfoRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for GetInfoRequest {}
            impl GetInfoRequest {}
            impl ::canadensis_encoding::Serialize for GetInfoRequest {
                fn size_bits(&self) -> usize {
                    (self.path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.path);
                }
            }
            impl ::canadensis_encoding::Deserialize for GetInfoRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(GetInfoRequest {
                        path: { cursor.read_composite()? },
                    })
                }
            }

            /// `uavcan.file.GetInfo.0.2`
            ///
            /// Fixed size 13 bytes
            pub struct GetInfoResponse {
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub size: u64,
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub unix_timestamp_of_last_modification: u64,
                /// `saturated bool`
                ///
                /// Always aligned
                /// Size 1 bits
                pub is_file_not_directory: bool,
                /// `saturated bool`
                ///
                /// Not always aligned
                /// Size 1 bits
                pub is_link: bool,
                /// `saturated bool`
                ///
                /// Not always aligned
                /// Size 1 bits
                pub is_readable: bool,
                /// `saturated bool`
                ///
                /// Not always aligned
                /// Size 1 bits
                pub is_writeable: bool,
                // 4 bits of padding
            }
            impl ::canadensis_encoding::DataType for GetInfoResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for GetInfoResponse {}
            impl GetInfoResponse {}
            impl ::canadensis_encoding::Serialize for GetInfoResponse {
                fn size_bits(&self) -> usize {
                    104
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                    cursor.write_u40(self.size);
                    cursor.write_u40(self.unix_timestamp_of_last_modification);
                    cursor.write_bool(self.is_file_not_directory);
                    cursor.write_bool(self.is_link);
                    cursor.write_bool(self.is_readable);
                    cursor.write_bool(self.is_writeable);
                    cursor.skip_4();
                }
            }
            impl ::canadensis_encoding::Deserialize for GetInfoResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(GetInfoResponse {
                        error: { cursor.read_composite()? },
                        size: { cursor.read_u40() as _ },
                        unix_timestamp_of_last_modification: { cursor.read_u40() as _ },
                        is_file_not_directory: { cursor.read_bool() },
                        is_link: { cursor.read_bool() },
                        is_readable: { cursor.read_bool() },
                        is_writeable: { cursor.read_bool() },
                    })
                }
            }
        }
        pub mod list_0_1 {
            /// `uavcan.file.List.0.1`
            ///
            /// Size ranges from 9 to 121 bytes
            pub struct ListRequest {
                /// `saturated uint32`
                ///
                /// Always aligned
                /// Size 32 bits
                pub entry_index: u32,
                // 32 bits of padding
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 904 bits
                pub directory_path: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ListRequest {}
            impl ListRequest {}
            impl ::canadensis_encoding::Serialize for ListRequest {
                fn size_bits(&self) -> usize {
                    32 + 32 + (self.directory_path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u32(self.entry_index);
                    cursor.skip_32();
                    cursor.write_composite(&self.directory_path);
                }
            }
            impl ::canadensis_encoding::Deserialize for ListRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ListRequest {
                        entry_index: { cursor.read_u32() as _ },
                        directory_path: {
                            cursor.skip_32();
                            cursor.read_composite()?
                        },
                    })
                }
            }

            /// `uavcan.file.List.0.1`
            ///
            /// Size ranges from 5 to 117 bytes
            pub struct ListResponse {
                // 32 bits of padding
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 904 bits
                pub entry_base_name: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListResponse {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Response for ListResponse {}
            impl ListResponse {}
            impl ::canadensis_encoding::Serialize for ListResponse {
                fn size_bits(&self) -> usize {
                    32 + (self.entry_base_name).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.skip_32();
                    cursor.write_composite(&self.entry_base_name);
                }
            }
            impl ::canadensis_encoding::Deserialize for ListResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ListResponse {
                        entry_base_name: {
                            cursor.skip_32();
                            cursor.read_composite()?
                        },
                    })
                }
            }
        }
        pub mod list_0_2 {
            /// `uavcan.file.List.0.2`
            ///
            /// Size ranges from 9 to 264 bytes
            pub struct ListRequest {
                /// `saturated uint32`
                ///
                /// Always aligned
                /// Size 32 bits
                pub entry_index: u32,
                // 32 bits of padding
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2048 bits
                pub directory_path: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ListRequest {}
            impl ListRequest {}
            impl ::canadensis_encoding::Serialize for ListRequest {
                fn size_bits(&self) -> usize {
                    32 + 32 + (self.directory_path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u32(self.entry_index);
                    cursor.skip_32();
                    cursor.write_composite(&self.directory_path);
                }
            }
            impl ::canadensis_encoding::Deserialize for ListRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ListRequest {
                        entry_index: { cursor.read_u32() as _ },
                        directory_path: {
                            cursor.skip_32();
                            cursor.read_composite()?
                        },
                    })
                }
            }

            /// `uavcan.file.List.0.2`
            ///
            /// Size ranges from 5 to 260 bytes
            pub struct ListResponse {
                // 32 bits of padding
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2048 bits
                pub entry_base_name: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListResponse {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Response for ListResponse {}
            impl ListResponse {}
            impl ::canadensis_encoding::Serialize for ListResponse {
                fn size_bits(&self) -> usize {
                    32 + (self.entry_base_name).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.skip_32();
                    cursor.write_composite(&self.entry_base_name);
                }
            }
            impl ::canadensis_encoding::Deserialize for ListResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ListResponse {
                        entry_base_name: {
                            cursor.skip_32();
                            cursor.read_composite()?
                        },
                    })
                }
            }
        }
        pub mod modify_1_0 {
            /// `uavcan.file.Modify.1.0`
            ///
            /// Size ranges from 6 to 230 bytes
            pub struct ModifyRequest {
                /// `saturated bool`
                ///
                /// Always aligned
                /// Size 1 bits
                pub preserve_source: bool,
                /// `saturated bool`
                ///
                /// Not always aligned
                /// Size 1 bits
                pub overwrite_destination: bool,
                // 30 bits of padding
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 904 bits
                pub source: crate::uavcan::file::path_1_0::Path,
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 904 bits
                pub destination: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ModifyRequest {
                const EXTENT_BYTES: Option<u32> = Some(600);
            }
            impl ::canadensis_encoding::Request for ModifyRequest {}
            impl ModifyRequest {}
            impl ::canadensis_encoding::Serialize for ModifyRequest {
                fn size_bits(&self) -> usize {
                    1 + 1 + 30 + (self.source).size_bits() + (self.destination).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_bool(self.preserve_source);
                    cursor.write_bool(self.overwrite_destination);
                    cursor.skip_30();
                    cursor.write_composite(&self.source);
                    cursor.write_composite(&self.destination);
                }
            }
            impl ::canadensis_encoding::Deserialize for ModifyRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ModifyRequest {
                        preserve_source: { cursor.read_bool() },
                        overwrite_destination: { cursor.read_bool() },
                        source: {
                            cursor.skip_30();
                            cursor.read_composite()?
                        },
                        destination: { cursor.read_composite()? },
                    })
                }
            }

            /// `uavcan.file.Modify.1.0`
            ///
            /// Fixed size 2 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ModifyResponse {
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for ModifyResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ModifyResponse {}
            impl ModifyResponse {}
            impl ::canadensis_encoding::Serialize for ModifyResponse {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for ModifyResponse {
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
                assert_eq!(::core::mem::size_of::<ModifyResponse>() * 8, 16);
                assert_eq!(::memoffset::offset_of!(ModifyResponse, error) * 8, 0);
            }
        }
        pub mod modify_1_1 {
            /// `uavcan.file.Modify.1.1`
            ///
            /// Size ranges from 6 to 516 bytes
            pub struct ModifyRequest {
                /// `saturated bool`
                ///
                /// Always aligned
                /// Size 1 bits
                pub preserve_source: bool,
                /// `saturated bool`
                ///
                /// Not always aligned
                /// Size 1 bits
                pub overwrite_destination: bool,
                // 30 bits of padding
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2048 bits
                pub source: crate::uavcan::file::path_2_0::Path,
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2048 bits
                pub destination: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ModifyRequest {
                const EXTENT_BYTES: Option<u32> = Some(600);
            }
            impl ::canadensis_encoding::Request for ModifyRequest {}
            impl ModifyRequest {}
            impl ::canadensis_encoding::Serialize for ModifyRequest {
                fn size_bits(&self) -> usize {
                    1 + 1 + 30 + (self.source).size_bits() + (self.destination).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_bool(self.preserve_source);
                    cursor.write_bool(self.overwrite_destination);
                    cursor.skip_30();
                    cursor.write_composite(&self.source);
                    cursor.write_composite(&self.destination);
                }
            }
            impl ::canadensis_encoding::Deserialize for ModifyRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ModifyRequest {
                        preserve_source: { cursor.read_bool() },
                        overwrite_destination: { cursor.read_bool() },
                        source: {
                            cursor.skip_30();
                            cursor.read_composite()?
                        },
                        destination: { cursor.read_composite()? },
                    })
                }
            }

            /// `uavcan.file.Modify.1.1`
            ///
            /// Fixed size 2 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ModifyResponse {
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for ModifyResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ModifyResponse {}
            impl ModifyResponse {}
            impl ::canadensis_encoding::Serialize for ModifyResponse {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for ModifyResponse {
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
                assert_eq!(::core::mem::size_of::<ModifyResponse>() * 8, 16);
                assert_eq!(::memoffset::offset_of!(ModifyResponse, error) * 8, 0);
            }
        }
        pub mod path_1_0 {
            /// `uavcan.file.Path.1.0`
            ///
            /// Size ranges from 1 to 113 bytes
            pub struct Path {
                /// `saturated uint8[<=112]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 896 bits
                pub path: ::heapless::Vec<u8, 112>,
            }
            impl ::canadensis_encoding::DataType for Path {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Path {}
            impl Path {
                pub const MAX_LENGTH: u8 = 112;
                pub const SEPARATOR: u8 = 47;
            }
            impl ::canadensis_encoding::Serialize for Path {
                fn size_bits(&self) -> usize {
                    8 + (self.path).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u8((self.path).len() as u8);
                    cursor.write_bytes(&(self.path)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for Path {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Path {
                        path: {
                            let length = cursor.read_u8() as _;
                            if length <= 112 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
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
        pub mod path_2_0 {
            /// `uavcan.file.Path.2.0`
            ///
            /// Size ranges from 1 to 256 bytes
            pub struct Path {
                /// `saturated uint8[<=255]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 2040 bits
                pub path: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for Path {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Path {}
            impl Path {
                pub const MAX_LENGTH: u8 = 255;
                pub const SEPARATOR: u8 = 47;
            }
            impl ::canadensis_encoding::Serialize for Path {
                fn size_bits(&self) -> usize {
                    8 + (self.path).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u8((self.path).len() as u8);
                    cursor.write_bytes(&(self.path)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for Path {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Path {
                        path: {
                            let length = cursor.read_u8() as _;
                            if length <= 255 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
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
        pub mod read_1_0 {
            /// `uavcan.file.Read.1.0`
            ///
            /// Size ranges from 6 to 118 bytes
            pub struct ReadRequest {
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub offset: u64,
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 904 bits
                pub path: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ReadRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ReadRequest {}
            impl ReadRequest {}
            impl ::canadensis_encoding::Serialize for ReadRequest {
                fn size_bits(&self) -> usize {
                    40 + (self.path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u40(self.offset);
                    cursor.write_composite(&self.path);
                }
            }
            impl ::canadensis_encoding::Deserialize for ReadRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ReadRequest {
                        offset: { cursor.read_u40() as _ },
                        path: { cursor.read_composite()? },
                    })
                }
            }

            /// `uavcan.file.Read.1.0`
            ///
            /// Size ranges from 4 to 260 bytes
            pub struct ReadResponse {
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
                /// `saturated uint8[<=256]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 2048 bits
                pub data: ::heapless::Vec<u8, 256>,
            }
            impl ::canadensis_encoding::DataType for ReadResponse {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Response for ReadResponse {}
            impl ReadResponse {}
            impl ::canadensis_encoding::Serialize for ReadResponse {
                fn size_bits(&self) -> usize {
                    16 + 16 + (self.data).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                    cursor.write_aligned_u16((self.data).len() as u16);
                    cursor.write_bytes(&(self.data)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for ReadResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ReadResponse {
                        error: { cursor.read_composite()? },
                        data: {
                            let length = cursor.read_u16() as _;
                            if length <= 256 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
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
        pub mod read_1_1 {
            /// `uavcan.file.Read.1.1`
            ///
            /// Size ranges from 6 to 261 bytes
            pub struct ReadRequest {
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub offset: u64,
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2048 bits
                pub path: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ReadRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ReadRequest {}
            impl ReadRequest {}
            impl ::canadensis_encoding::Serialize for ReadRequest {
                fn size_bits(&self) -> usize {
                    40 + (self.path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u40(self.offset);
                    cursor.write_composite(&self.path);
                }
            }
            impl ::canadensis_encoding::Deserialize for ReadRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ReadRequest {
                        offset: { cursor.read_u40() as _ },
                        path: { cursor.read_composite()? },
                    })
                }
            }

            /// `uavcan.file.Read.1.1`
            ///
            /// Size ranges from 4 to 260 bytes
            pub struct ReadResponse {
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
                /// `uavcan.primitive.Unstructured.1.0`
                ///
                /// Always aligned
                /// Size ranges from 16 to 2064 bits
                pub data: crate::uavcan::primitive::unstructured_1_0::Unstructured,
            }
            impl ::canadensis_encoding::DataType for ReadResponse {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Response for ReadResponse {}
            impl ReadResponse {}
            impl ::canadensis_encoding::Serialize for ReadResponse {
                fn size_bits(&self) -> usize {
                    16 + (self.data).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                    cursor.write_composite(&self.data);
                }
            }
            impl ::canadensis_encoding::Deserialize for ReadResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ReadResponse {
                        error: { cursor.read_composite()? },
                        data: { cursor.read_composite()? },
                    })
                }
            }
        }
        pub mod write_1_0 {
            /// `uavcan.file.Write.1.0`
            ///
            /// Size ranges from 7 to 311 bytes
            pub struct WriteRequest {
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub offset: u64,
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 904 bits
                pub path: crate::uavcan::file::path_1_0::Path,
                /// `saturated uint8[<=192]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 1536 bits
                pub data: ::heapless::Vec<u8, 192>,
            }
            impl ::canadensis_encoding::DataType for WriteRequest {
                const EXTENT_BYTES: Option<u32> = Some(600);
            }
            impl ::canadensis_encoding::Request for WriteRequest {}
            impl WriteRequest {}
            impl ::canadensis_encoding::Serialize for WriteRequest {
                fn size_bits(&self) -> usize {
                    40 + (self.path).size_bits() + 8 + (self.data).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u40(self.offset);
                    cursor.write_composite(&self.path);
                    cursor.write_aligned_u8((self.data).len() as u8);
                    cursor.write_bytes(&(self.data)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for WriteRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(WriteRequest {
                        offset: { cursor.read_u40() as _ },
                        path: { cursor.read_composite()? },
                        data: {
                            let length = cursor.read_u8() as _;
                            if length <= 192 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
                                }
                                elements
                            } else {
                                return Err(::canadensis_encoding::DeserializeError::ArrayLength);
                            }
                        },
                    })
                }
            }

            /// `uavcan.file.Write.1.0`
            ///
            /// Fixed size 2 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct WriteResponse {
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for WriteResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for WriteResponse {}
            impl WriteResponse {}
            impl ::canadensis_encoding::Serialize for WriteResponse {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for WriteResponse {
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
                assert_eq!(::core::mem::size_of::<WriteResponse>() * 8, 16);
                assert_eq!(::memoffset::offset_of!(WriteResponse, error) * 8, 0);
            }
        }
        pub mod write_1_1 {
            /// `uavcan.file.Write.1.1`
            ///
            /// Size ranges from 8 to 519 bytes
            pub struct WriteRequest {
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub offset: u64,
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2048 bits
                pub path: crate::uavcan::file::path_2_0::Path,
                /// `uavcan.primitive.Unstructured.1.0`
                ///
                /// Always aligned
                /// Size ranges from 16 to 2064 bits
                pub data: crate::uavcan::primitive::unstructured_1_0::Unstructured,
            }
            impl ::canadensis_encoding::DataType for WriteRequest {
                const EXTENT_BYTES: Option<u32> = Some(600);
            }
            impl ::canadensis_encoding::Request for WriteRequest {}
            impl WriteRequest {}
            impl ::canadensis_encoding::Serialize for WriteRequest {
                fn size_bits(&self) -> usize {
                    40 + (self.path).size_bits() + (self.data).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u40(self.offset);
                    cursor.write_composite(&self.path);
                    cursor.write_composite(&self.data);
                }
            }
            impl ::canadensis_encoding::Deserialize for WriteRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(WriteRequest {
                        offset: { cursor.read_u40() as _ },
                        path: { cursor.read_composite()? },
                        data: { cursor.read_composite()? },
                    })
                }
            }

            /// `uavcan.file.Write.1.1`
            ///
            /// Fixed size 2 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct WriteResponse {
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for WriteResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for WriteResponse {}
            impl WriteResponse {}
            impl ::canadensis_encoding::Serialize for WriteResponse {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for WriteResponse {
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
                assert_eq!(::core::mem::size_of::<WriteResponse>() * 8, 16);
                assert_eq!(::memoffset::offset_of!(WriteResponse, error) * 8, 0);
            }
        }
    }
    pub mod internet {
        pub mod udp {
            pub mod handle_incoming_packet_0_1 {
                /// `uavcan.internet.udp.HandleIncomingPacket.0.1`
                ///
                /// Size ranges from 4 to 313 bytes
                pub struct HandleIncomingPacketRequest {
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub session_id: u16,
                    /// `saturated uint8[<=309]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2472 bits
                    pub payload: ::heapless::Vec<u8, 309>,
                }
                impl ::canadensis_encoding::DataType for HandleIncomingPacketRequest {
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Request for HandleIncomingPacketRequest {}
                impl HandleIncomingPacketRequest {}
                impl ::canadensis_encoding::Serialize for HandleIncomingPacketRequest {
                    fn size_bits(&self) -> usize {
                        16 + 16 + (self.payload).len() * 8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.session_id);
                        cursor.write_aligned_u16((self.payload).len() as u16);
                        cursor.write_bytes(&(self.payload)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for HandleIncomingPacketRequest {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(HandleIncomingPacketRequest {
                            session_id: { cursor.read_u16() as _ },
                            payload: {
                                let length = cursor.read_u16() as _;
                                if length <= 309 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }

                /// `uavcan.internet.udp.HandleIncomingPacket.0.1`
                ///
                /// Fixed size 0 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::DataType for HandleIncomingPacketResponse {
                    const EXTENT_BYTES: Option<u32> = Some(63);
                }
                impl ::canadensis_encoding::Response for HandleIncomingPacketResponse {}
                impl HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::Serialize for HandleIncomingPacketResponse {
                    fn size_bits(&self) -> usize {
                        0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for HandleIncomingPacketResponse {
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
                    assert_eq!(
                        ::core::mem::size_of::<HandleIncomingPacketResponse>() * 8,
                        0
                    );
                }
            }
            pub mod handle_incoming_packet_0_2 {
                /// `uavcan.internet.udp.HandleIncomingPacket.0.2`
                ///
                /// Size ranges from 4 to 512 bytes
                pub struct HandleIncomingPacketRequest {
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub session_id: u16,
                    /// `saturated uint8[<=508]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 4064 bits
                    pub payload: ::heapless::Vec<u8, 508>,
                }
                impl ::canadensis_encoding::DataType for HandleIncomingPacketRequest {
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Request for HandleIncomingPacketRequest {}
                impl HandleIncomingPacketRequest {}
                impl ::canadensis_encoding::Serialize for HandleIncomingPacketRequest {
                    fn size_bits(&self) -> usize {
                        16 + 16 + (self.payload).len() * 8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.session_id);
                        cursor.write_aligned_u16((self.payload).len() as u16);
                        cursor.write_bytes(&(self.payload)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for HandleIncomingPacketRequest {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(HandleIncomingPacketRequest {
                            session_id: { cursor.read_u16() as _ },
                            payload: {
                                let length = cursor.read_u16() as _;
                                if length <= 508 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }

                /// `uavcan.internet.udp.HandleIncomingPacket.0.2`
                ///
                /// Fixed size 0 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::DataType for HandleIncomingPacketResponse {
                    const EXTENT_BYTES: Option<u32> = Some(63);
                }
                impl ::canadensis_encoding::Response for HandleIncomingPacketResponse {}
                impl HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::Serialize for HandleIncomingPacketResponse {
                    fn size_bits(&self) -> usize {
                        0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for HandleIncomingPacketResponse {
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
                    assert_eq!(
                        ::core::mem::size_of::<HandleIncomingPacketResponse>() * 8,
                        0
                    );
                }
            }
            pub mod outgoing_packet_0_1 {
                /// `uavcan.internet.udp.OutgoingPacket.0.1`
                ///
                /// Size ranges from 8 to 313 bytes
                pub struct OutgoingPacket {
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub session_id: u16,
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub destination_port: u16,
                    /// `saturated uint8[<=45]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 360 bits
                    pub destination_address: ::heapless::Vec<u8, 45>,
                    /// `saturated bool`
                    ///
                    /// Always aligned
                    /// Size 1 bits
                    pub use_masquerading: bool,
                    /// `saturated bool`
                    ///
                    /// Not always aligned
                    /// Size 1 bits
                    pub use_dtls: bool,
                    // 6 bits of padding
                    /// `saturated uint8[<=260]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2080 bits
                    pub payload: ::heapless::Vec<u8, 260>,
                }
                impl ::canadensis_encoding::DataType for OutgoingPacket {
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Message for OutgoingPacket {}
                impl OutgoingPacket {
                    pub const NAT_ENTRY_MIN_TTL: u32 = 86400;
                }
                impl ::canadensis_encoding::Serialize for OutgoingPacket {
                    fn size_bits(&self) -> usize {
                        16 + 16
                            + 8
                            + (self.destination_address).len() * 8
                            + 1
                            + 1
                            + 6
                            + 16
                            + (self.payload).len() * 8
                            + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.session_id);
                        cursor.write_aligned_u16(self.destination_port);
                        cursor.write_aligned_u8((self.destination_address).len() as u8);
                        cursor.write_bytes(&(self.destination_address)[..]);
                        cursor.write_bool(self.use_masquerading);
                        cursor.write_bool(self.use_dtls);
                        cursor.skip_6();
                        cursor.write_aligned_u16((self.payload).len() as u16);
                        cursor.write_bytes(&(self.payload)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for OutgoingPacket {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(OutgoingPacket {
                            session_id: { cursor.read_u16() as _ },
                            destination_port: { cursor.read_u16() as _ },
                            destination_address: {
                                let length = cursor.read_u8() as _;
                                if length <= 45 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                            use_masquerading: { cursor.read_bool() },
                            use_dtls: { cursor.read_bool() },
                            payload: {
                                cursor.skip_6();
                                let length = cursor.read_u16() as _;
                                if length <= 260 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod outgoing_packet_0_2 {
                /// `uavcan.internet.udp.OutgoingPacket.0.2`
                ///
                /// Size ranges from 8 to 561 bytes
                pub struct OutgoingPacket {
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub session_id: u16,
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub destination_port: u16,
                    /// `saturated uint8[<=45]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 360 bits
                    pub destination_address: ::heapless::Vec<u8, 45>,
                    /// `saturated bool`
                    ///
                    /// Always aligned
                    /// Size 1 bits
                    pub use_masquerading: bool,
                    /// `saturated bool`
                    ///
                    /// Not always aligned
                    /// Size 1 bits
                    pub use_dtls: bool,
                    // 6 bits of padding
                    /// `saturated uint8[<=508]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 4064 bits
                    pub payload: ::heapless::Vec<u8, 508>,
                }
                impl ::canadensis_encoding::DataType for OutgoingPacket {
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Message for OutgoingPacket {}
                impl OutgoingPacket {
                    pub const NAT_ENTRY_MIN_TTL: u32 = 86400;
                }
                impl ::canadensis_encoding::Serialize for OutgoingPacket {
                    fn size_bits(&self) -> usize {
                        16 + 16
                            + 8
                            + (self.destination_address).len() * 8
                            + 1
                            + 1
                            + 6
                            + 16
                            + (self.payload).len() * 8
                            + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.session_id);
                        cursor.write_aligned_u16(self.destination_port);
                        cursor.write_aligned_u8((self.destination_address).len() as u8);
                        cursor.write_bytes(&(self.destination_address)[..]);
                        cursor.write_bool(self.use_masquerading);
                        cursor.write_bool(self.use_dtls);
                        cursor.skip_6();
                        cursor.write_aligned_u16((self.payload).len() as u16);
                        cursor.write_bytes(&(self.payload)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for OutgoingPacket {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(OutgoingPacket {
                            session_id: { cursor.read_u16() as _ },
                            destination_port: { cursor.read_u16() as _ },
                            destination_address: {
                                let length = cursor.read_u8() as _;
                                if length <= 45 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                            use_masquerading: { cursor.read_bool() },
                            use_dtls: { cursor.read_bool() },
                            payload: {
                                cursor.skip_6();
                                let length = cursor.read_u16() as _;
                                if length <= 508 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
        }
    }
    pub mod metatransport {
        pub mod can {
            pub mod arbitration_id_0_1 {
                /// `uavcan.metatransport.can.ArbitrationID.0.1`
                ///
                /// Fixed size 5 bytes
                pub enum ArbitrationID {
                    // uavcan.metatransport.can.BaseArbitrationID.0.1
Base(crate::uavcan::metatransport::can::base_arbitration_id_0_1::BaseArbitrationID),
// uavcan.metatransport.can.ExtendedArbitrationID.0.1
Extended(crate::uavcan::metatransport::can::extended_arbitration_id_0_1::ExtendedArbitrationID),
}
                impl ::canadensis_encoding::DataType for ArbitrationID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for ArbitrationID {}
                impl ArbitrationID {}
                impl ::canadensis_encoding::Serialize for ArbitrationID {
                    fn size_bits(&self) -> usize {
                        40
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        match self {
                            ArbitrationID::Base(inner) => {
                                cursor.write_aligned_u8(0);
                                cursor.write_composite(inner);
                            }
                            ArbitrationID::Extended(inner) => {
                                cursor.write_aligned_u8(1);
                                cursor.write_composite(inner);
                            }
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for ArbitrationID {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        match cursor.read_aligned_u8() as _ {
                            0 => Ok(ArbitrationID::Base({ cursor.read_composite()? })),
                            1 => Ok(ArbitrationID::Extended({ cursor.read_composite()? })),
                            _ => Err(::canadensis_encoding::DeserializeError::UnionTag),
                        }
                    }
                }
            }
            pub mod base_arbitration_id_0_1 {
                /// `uavcan.metatransport.can.BaseArbitrationID.0.1`
                ///
                /// Fixed size 4 bytes
                pub struct BaseArbitrationID {
                    /// `truncated uint11`
                    ///
                    /// Always aligned
                    /// Size 11 bits
                    pub value: u16,
                    // 21 bits of padding
                }
                impl ::canadensis_encoding::DataType for BaseArbitrationID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for BaseArbitrationID {}
                impl BaseArbitrationID {}
                impl ::canadensis_encoding::Serialize for BaseArbitrationID {
                    fn size_bits(&self) -> usize {
                        32
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u11(self.value);
                        cursor.skip_21();
                    }
                }
                impl ::canadensis_encoding::Deserialize for BaseArbitrationID {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(BaseArbitrationID {
                            value: { cursor.read_u11() as _ },
                        })
                    }
                }
            }
            pub mod data_classic_0_1 {
                /// `uavcan.metatransport.can.DataClassic.0.1`
                ///
                /// Size ranges from 6 to 14 bytes
                pub struct DataClassic {
                    /// `uavcan.metatransport.can.ArbitrationID.0.1`
                    ///
                    /// Always aligned
                    /// Size 40 bits
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                    /// `saturated uint8[<=8]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 64 bits
                    pub data: ::heapless::Vec<u8, 8>,
                }
                impl ::canadensis_encoding::DataType for DataClassic {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for DataClassic {}
                impl DataClassic {}
                impl ::canadensis_encoding::Serialize for DataClassic {
                    fn size_bits(&self) -> usize {
                        40 + 8 + (self.data).len() * 8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.arbitration_id);
                        cursor.write_aligned_u8((self.data).len() as u8);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for DataClassic {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(DataClassic {
                            arbitration_id: { cursor.read_composite()? },
                            data: {
                                let length = cursor.read_u8() as _;
                                if length <= 8 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod data_fd_0_1 {
                /// `uavcan.metatransport.can.DataFD.0.1`
                ///
                /// Size ranges from 6 to 70 bytes
                pub struct DataFD {
                    /// `uavcan.metatransport.can.ArbitrationID.0.1`
                    ///
                    /// Always aligned
                    /// Size 40 bits
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                    /// `saturated uint8[<=64]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 512 bits
                    pub data: ::heapless::Vec<u8, 64>,
                }
                impl ::canadensis_encoding::DataType for DataFD {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for DataFD {}
                impl DataFD {}
                impl ::canadensis_encoding::Serialize for DataFD {
                    fn size_bits(&self) -> usize {
                        40 + 8 + (self.data).len() * 8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.arbitration_id);
                        cursor.write_aligned_u8((self.data).len() as u8);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for DataFD {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(DataFD {
                            arbitration_id: { cursor.read_composite()? },
                            data: {
                                let length = cursor.read_u8() as _;
                                if length <= 64 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod error_0_1 {
                /// `uavcan.metatransport.can.Error.0.1`
                ///
                /// Fixed size 4 bytes
                pub struct Error {
                    // 32 bits of padding
                }
                impl ::canadensis_encoding::DataType for Error {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Error {}
                impl Error {}
                impl ::canadensis_encoding::Serialize for Error {
                    fn size_bits(&self) -> usize {
                        32
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.skip_32();
                    }
                }
                impl ::canadensis_encoding::Deserialize for Error {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Error {})
                    }
                }
            }
            pub mod extended_arbitration_id_0_1 {
                /// `uavcan.metatransport.can.ExtendedArbitrationID.0.1`
                ///
                /// Fixed size 4 bytes
                pub struct ExtendedArbitrationID {
                    /// `truncated uint29`
                    ///
                    /// Always aligned
                    /// Size 29 bits
                    pub value: u32,
                    // 3 bits of padding
                }
                impl ::canadensis_encoding::DataType for ExtendedArbitrationID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for ExtendedArbitrationID {}
                impl ExtendedArbitrationID {}
                impl ::canadensis_encoding::Serialize for ExtendedArbitrationID {
                    fn size_bits(&self) -> usize {
                        32
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u29(self.value);
                        cursor.skip_3();
                    }
                }
                impl ::canadensis_encoding::Deserialize for ExtendedArbitrationID {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(ExtendedArbitrationID {
                            value: { cursor.read_u29() as _ },
                        })
                    }
                }
            }
            pub mod frame_0_1 {
                /// `uavcan.metatransport.can.Frame.0.1`
                ///
                /// Size ranges from 12 to 78 bytes
                pub struct Frame {
                    /// `uavcan.time.SynchronizedTimestamp.1.0`
                    ///
                    /// Always aligned
                    /// Size 56 bits
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    /// `uavcan.metatransport.can.Manifestation.0.1`
                    ///
                    /// Always aligned
                    /// Size ranges from 40 to 568 bits
                    pub manifestation:
                        crate::uavcan::metatransport::can::manifestation_0_1::Manifestation,
                }
                impl ::canadensis_encoding::DataType for Frame {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Frame {}
                impl Frame {}
                impl ::canadensis_encoding::Serialize for Frame {
                    fn size_bits(&self) -> usize {
                        56 + (self.manifestation).size_bits() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.timestamp);
                        cursor.write_composite(&self.manifestation);
                    }
                }
                impl ::canadensis_encoding::Deserialize for Frame {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Frame {
                            timestamp: { cursor.read_composite()? },
                            manifestation: { cursor.read_composite()? },
                        })
                    }
                }
            }
            pub mod frame_0_2 {
                /// `uavcan.metatransport.can.Frame.0.2`
                ///
                /// Size ranges from 5 to 71 bytes
                pub enum Frame {
                    // uavcan.metatransport.can.Error.0.1
                    Error(crate::uavcan::metatransport::can::error_0_1::Error),
                    // uavcan.metatransport.can.DataFD.0.1
                    DataFd(crate::uavcan::metatransport::can::data_fd_0_1::DataFD),
                    // uavcan.metatransport.can.DataClassic.0.1
                    DataClassic(crate::uavcan::metatransport::can::data_classic_0_1::DataClassic),
                    // uavcan.metatransport.can.RTR.0.1
                    RemoteTransmissionRequest(crate::uavcan::metatransport::can::rtr_0_1::RTR),
                }
                impl ::canadensis_encoding::DataType for Frame {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Frame {}
                impl Frame {}
                impl ::canadensis_encoding::Serialize for Frame {
                    fn size_bits(&self) -> usize {
                        8 + match self {
                            Frame::Error(inner) => 32,
                            Frame::DataFd(inner) => (inner).size_bits(),
                            Frame::DataClassic(inner) => (inner).size_bits(),
                            Frame::RemoteTransmissionRequest(inner) => 40,
                        }
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        match self {
                            Frame::Error(inner) => {
                                cursor.write_aligned_u8(0);
                                cursor.write_composite(inner);
                            }
                            Frame::DataFd(inner) => {
                                cursor.write_aligned_u8(1);
                                cursor.write_composite(inner);
                            }
                            Frame::DataClassic(inner) => {
                                cursor.write_aligned_u8(2);
                                cursor.write_composite(inner);
                            }
                            Frame::RemoteTransmissionRequest(inner) => {
                                cursor.write_aligned_u8(3);
                                cursor.write_composite(inner);
                            }
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Frame {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        match cursor.read_aligned_u8() as _ {
                            0 => Ok(Frame::Error({ cursor.read_composite()? })),
                            1 => Ok(Frame::DataFd({ cursor.read_composite()? })),
                            2 => Ok(Frame::DataClassic({ cursor.read_composite()? })),
                            3 => Ok(Frame::RemoteTransmissionRequest({
                                cursor.read_composite()?
                            })),
                            _ => Err(::canadensis_encoding::DeserializeError::UnionTag),
                        }
                    }
                }
            }
            pub mod manifestation_0_1 {
                /// `uavcan.metatransport.can.Manifestation.0.1`
                ///
                /// Size ranges from 5 to 71 bytes
                pub enum Manifestation {
                    // uavcan.metatransport.can.Error.0.1
                    Error(crate::uavcan::metatransport::can::error_0_1::Error),
                    // uavcan.metatransport.can.DataFD.0.1
                    DataFd(crate::uavcan::metatransport::can::data_fd_0_1::DataFD),
                    // uavcan.metatransport.can.DataClassic.0.1
                    DataClassic(crate::uavcan::metatransport::can::data_classic_0_1::DataClassic),
                    // uavcan.metatransport.can.RTR.0.1
                    RemoteTransmissionRequest(crate::uavcan::metatransport::can::rtr_0_1::RTR),
                }
                impl ::canadensis_encoding::DataType for Manifestation {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Manifestation {}
                impl Manifestation {}
                impl ::canadensis_encoding::Serialize for Manifestation {
                    fn size_bits(&self) -> usize {
                        8 + match self {
                            Manifestation::Error(inner) => 32,
                            Manifestation::DataFd(inner) => (inner).size_bits(),
                            Manifestation::DataClassic(inner) => (inner).size_bits(),
                            Manifestation::RemoteTransmissionRequest(inner) => 40,
                        }
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        match self {
                            Manifestation::Error(inner) => {
                                cursor.write_aligned_u8(0);
                                cursor.write_composite(inner);
                            }
                            Manifestation::DataFd(inner) => {
                                cursor.write_aligned_u8(1);
                                cursor.write_composite(inner);
                            }
                            Manifestation::DataClassic(inner) => {
                                cursor.write_aligned_u8(2);
                                cursor.write_composite(inner);
                            }
                            Manifestation::RemoteTransmissionRequest(inner) => {
                                cursor.write_aligned_u8(3);
                                cursor.write_composite(inner);
                            }
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Manifestation {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        match cursor.read_aligned_u8() as _ {
                            0 => Ok(Manifestation::Error({ cursor.read_composite()? })),
                            1 => Ok(Manifestation::DataFd({ cursor.read_composite()? })),
                            2 => Ok(Manifestation::DataClassic({ cursor.read_composite()? })),
                            3 => Ok(Manifestation::RemoteTransmissionRequest({
                                cursor.read_composite()?
                            })),
                            _ => Err(::canadensis_encoding::DeserializeError::UnionTag),
                        }
                    }
                }
            }
            pub mod rtr_0_1 {
                /// `uavcan.metatransport.can.RTR.0.1`
                ///
                /// Fixed size 5 bytes
                pub struct RTR {
                    /// `uavcan.metatransport.can.ArbitrationID.0.1`
                    ///
                    /// Always aligned
                    /// Size 40 bits
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                }
                impl ::canadensis_encoding::DataType for RTR {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for RTR {}
                impl RTR {}
                impl ::canadensis_encoding::Serialize for RTR {
                    fn size_bits(&self) -> usize {
                        40
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.arbitration_id);
                    }
                }
                impl ::canadensis_encoding::Deserialize for RTR {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(RTR {
                            arbitration_id: { cursor.read_composite()? },
                        })
                    }
                }
            }
        }
        pub mod ethernet {
            pub mod ether_type_0_1 {
                /// `uavcan.metatransport.ethernet.EtherType.0.1`
                ///
                /// Fixed size 2 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct EtherType {
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for EtherType {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for EtherType {}
                impl EtherType {
                    pub const ARP: u16 = 2054;
                    pub const IP_V4: u16 = 2048;
                    pub const IP_V6: u16 = 34525;
                }
                impl ::canadensis_encoding::Serialize for EtherType {
                    fn size_bits(&self) -> usize {
                        16
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for EtherType {
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
                    assert_eq!(::core::mem::size_of::<EtherType>() * 8, 16);
                    assert_eq!(::memoffset::offset_of!(EtherType, value) * 8, 0);
                }
            }
            pub mod frame_0_1 {
                /// `uavcan.metatransport.ethernet.Frame.0.1`
                ///
                /// Size ranges from 16 to 9232 bytes
                pub struct Frame {
                    /// `saturated uint8[6]`
                    ///
                    /// Always aligned
                    /// Size 48 bits
                    pub destination: [u8; 6],
                    /// `saturated uint8[6]`
                    ///
                    /// Always aligned
                    /// Size 48 bits
                    pub source: [u8; 6],
                    /// `uavcan.metatransport.ethernet.EtherType.0.1`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub ethertype:
                        crate::uavcan::metatransport::ethernet::ether_type_0_1::EtherType,
                    /// `saturated uint8[<=9216]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 73728 bits
                    pub payload: ::heapless::Vec<u8, 9216>,
                }
                impl ::canadensis_encoding::DataType for Frame {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Frame {}
                impl Frame {}
                impl ::canadensis_encoding::Serialize for Frame {
                    fn size_bits(&self) -> usize {
                        (self.destination).len() * 8
                            + (self.source).len() * 8
                            + 16
                            + 16
                            + (self.payload).len() * 8
                            + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_bytes(&(self.destination)[..]);
                        cursor.write_bytes(&(self.source)[..]);
                        cursor.write_composite(&self.ethertype);
                        cursor.write_aligned_u16((self.payload).len() as u16);
                        cursor.write_bytes(&(self.payload)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for Frame {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Frame {
                            destination: {
                                [
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                ]
                            },
                            source: {
                                [
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                ]
                            },
                            ethertype: { cursor.read_composite()? },
                            payload: {
                                let length = cursor.read_u16() as _;
                                if length <= 9216 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
        }
        pub mod serial {
            pub mod fragment_0_1 {
                /// `uavcan.metatransport.serial.Fragment.0.1`
                ///
                /// Size ranges from 9 to 265 bytes
                pub struct Fragment {
                    /// `uavcan.time.SynchronizedTimestamp.1.0`
                    ///
                    /// Always aligned
                    /// Size 56 bits
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    /// `saturated uint8[<=256]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub data: ::heapless::Vec<u8, 256>,
                }
                impl ::canadensis_encoding::DataType for Fragment {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Fragment {}
                impl Fragment {
                    pub const CAPACITY_BYTES: u16 = 256;
                }
                impl ::canadensis_encoding::Serialize for Fragment {
                    fn size_bits(&self) -> usize {
                        56 + 16 + (self.data).len() * 8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.timestamp);
                        cursor.write_aligned_u16((self.data).len() as u16);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for Fragment {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Fragment {
                            timestamp: { cursor.read_composite()? },
                            data: {
                                let length = cursor.read_u16() as _;
                                if length <= 256 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod fragment_0_2 {
                /// `uavcan.metatransport.serial.Fragment.0.2`
                ///
                /// Size ranges from 2 to 2050 bytes
                pub struct Fragment {
                    /// `saturated uint8[<=2048]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 16384 bits
                    pub data: ::heapless::Vec<u8, 2048>,
                }
                impl ::canadensis_encoding::DataType for Fragment {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Fragment {}
                impl Fragment {
                    pub const CAPACITY_BYTES: u16 = 2048;
                }
                impl ::canadensis_encoding::Serialize for Fragment {
                    fn size_bits(&self) -> usize {
                        16 + (self.data).len() * 8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16((self.data).len() as u16);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for Fragment {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Fragment {
                            data: {
                                let length = cursor.read_u16() as _;
                                if length <= 2048 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
        }
        pub mod udp {
            pub mod endpoint_0_1 {
                /// `uavcan.metatransport.udp.Endpoint.0.1`
                ///
                /// Fixed size 32 bytes
                pub struct Endpoint {
                    /// `saturated uint8[16]`
                    ///
                    /// Always aligned
                    /// Size 128 bits
                    pub ip_address: [u8; 16],
                    /// `saturated uint8[6]`
                    ///
                    /// Always aligned
                    /// Size 48 bits
                    pub mac_address: [u8; 6],
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub port: u16,
                    // 64 bits of padding
                }
                impl ::canadensis_encoding::DataType for Endpoint {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Endpoint {}
                impl Endpoint {}
                impl ::canadensis_encoding::Serialize for Endpoint {
                    fn size_bits(&self) -> usize {
                        256
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_bytes(&(self.ip_address)[..]);
                        cursor.write_bytes(&(self.mac_address)[..]);
                        cursor.write_aligned_u16(self.port);
                        cursor.skip_64();
                    }
                }
                impl ::canadensis_encoding::Deserialize for Endpoint {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Endpoint {
                            ip_address: {
                                [
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                ]
                            },
                            mac_address: {
                                [
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                    cursor.read_u8() as _,
                                ]
                            },
                            port: { cursor.read_u16() as _ },
                        })
                    }
                }
            }
            pub mod frame_0_1 {
                /// `uavcan.metatransport.udp.Frame.0.1`
                ///
                /// Size ranges from 74 to 9262 bytes
                pub struct Frame {
                    /// `uavcan.time.SynchronizedTimestamp.1.0`
                    ///
                    /// Always aligned
                    /// Size 56 bits
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    // 8 bits of padding
                    /// `uavcan.metatransport.udp.Endpoint.0.1`
                    ///
                    /// Always aligned
                    /// Size 256 bits
                    pub source: crate::uavcan::metatransport::udp::endpoint_0_1::Endpoint,
                    /// `uavcan.metatransport.udp.Endpoint.0.1`
                    ///
                    /// Always aligned
                    /// Size 256 bits
                    pub destination: crate::uavcan::metatransport::udp::endpoint_0_1::Endpoint,
                    /// `saturated uint8[<=9188]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 73504 bits
                    pub data: ::heapless::Vec<u8, 9188>,
                }
                impl ::canadensis_encoding::DataType for Frame {
                    const EXTENT_BYTES: Option<u32> = Some(10240);
                }
                impl ::canadensis_encoding::Message for Frame {}
                impl Frame {
                    pub const MTU: u16 = 9188;
                }
                impl ::canadensis_encoding::Serialize for Frame {
                    fn size_bits(&self) -> usize {
                        56 + 8 + 256 + 256 + 16 + (self.data).len() * 8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.timestamp);
                        cursor.skip_8();
                        cursor.write_composite(&self.source);
                        cursor.write_composite(&self.destination);
                        cursor.write_aligned_u16((self.data).len() as u16);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for Frame {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Frame {
                            timestamp: { cursor.read_composite()? },
                            source: {
                                cursor.skip_8();
                                cursor.read_composite()?
                            },
                            destination: { cursor.read_composite()? },
                            data: {
                                let length = cursor.read_u16() as _;
                                if length <= 9188 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
        }
    }
    pub mod node {
        pub mod execute_command_1_0 {
            /// `uavcan.node.ExecuteCommand.1.0`
            ///
            /// Size ranges from 3 to 115 bytes
            pub struct ExecuteCommandRequest {
                /// `saturated uint16`
                ///
                /// Always aligned
                /// Size 16 bits
                pub command: u16,
                /// `saturated uint8[<=112]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 896 bits
                pub parameter: ::heapless::Vec<u8, 112>,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ExecuteCommandRequest {}
            impl ExecuteCommandRequest {
                pub const COMMAND_BEGIN_SOFTWARE_UPDATE: u16 = 65533;
                pub const COMMAND_EMERGENCY_STOP: u16 = 65531;
                pub const COMMAND_FACTORY_RESET: u16 = 65532;
                pub const COMMAND_POWER_OFF: u16 = 65534;
                pub const COMMAND_RESTART: u16 = 65535;
                pub const COMMAND_STORE_PERSISTENT_STATES: u16 = 65530;
            }
            impl ::canadensis_encoding::Serialize for ExecuteCommandRequest {
                fn size_bits(&self) -> usize {
                    16 + 8 + (self.parameter).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16(self.command);
                    cursor.write_aligned_u8((self.parameter).len() as u8);
                    cursor.write_bytes(&(self.parameter)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for ExecuteCommandRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ExecuteCommandRequest {
                        command: { cursor.read_u16() as _ },
                        parameter: {
                            let length = cursor.read_u8() as _;
                            if length <= 112 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
                                }
                                elements
                            } else {
                                return Err(::canadensis_encoding::DeserializeError::ArrayLength);
                            }
                        },
                    })
                }
            }

            /// `uavcan.node.ExecuteCommand.1.0`
            ///
            /// Fixed size 1 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ExecuteCommandResponse {
                /// `saturated uint8`
                ///
                /// Always aligned
                /// Size 8 bits
                pub status: u8,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ExecuteCommandResponse {}
            impl ExecuteCommandResponse {
                pub const STATUS_BAD_COMMAND: u8 = 3;
                pub const STATUS_BAD_PARAMETER: u8 = 4;
                pub const STATUS_BAD_STATE: u8 = 5;
                pub const STATUS_FAILURE: u8 = 1;
                pub const STATUS_INTERNAL_ERROR: u8 = 6;
                pub const STATUS_NOT_AUTHORIZED: u8 = 2;
                pub const STATUS_SUCCESS: u8 = 0;
            }
            impl ::canadensis_encoding::Serialize for ExecuteCommandResponse {
                fn size_bits(&self) -> usize {
                    8
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for ExecuteCommandResponse {
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
                assert_eq!(::core::mem::size_of::<ExecuteCommandResponse>() * 8, 8);
                assert_eq!(
                    ::memoffset::offset_of!(ExecuteCommandResponse, status) * 8,
                    0
                );
            }
        }
        pub mod execute_command_1_1 {
            /// `uavcan.node.ExecuteCommand.1.1`
            ///
            /// Size ranges from 3 to 258 bytes
            pub struct ExecuteCommandRequest {
                /// `saturated uint16`
                ///
                /// Always aligned
                /// Size 16 bits
                pub command: u16,
                /// `saturated uint8[<=255]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 2040 bits
                pub parameter: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ExecuteCommandRequest {}
            impl ExecuteCommandRequest {
                pub const COMMAND_BEGIN_SOFTWARE_UPDATE: u16 = 65533;
                pub const COMMAND_EMERGENCY_STOP: u16 = 65531;
                pub const COMMAND_FACTORY_RESET: u16 = 65532;
                pub const COMMAND_POWER_OFF: u16 = 65534;
                pub const COMMAND_RESTART: u16 = 65535;
                pub const COMMAND_STORE_PERSISTENT_STATES: u16 = 65530;
            }
            impl ::canadensis_encoding::Serialize for ExecuteCommandRequest {
                fn size_bits(&self) -> usize {
                    16 + 8 + (self.parameter).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16(self.command);
                    cursor.write_aligned_u8((self.parameter).len() as u8);
                    cursor.write_bytes(&(self.parameter)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for ExecuteCommandRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ExecuteCommandRequest {
                        command: { cursor.read_u16() as _ },
                        parameter: {
                            let length = cursor.read_u8() as _;
                            if length <= 255 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
                                }
                                elements
                            } else {
                                return Err(::canadensis_encoding::DeserializeError::ArrayLength);
                            }
                        },
                    })
                }
            }

            /// `uavcan.node.ExecuteCommand.1.1`
            ///
            /// Fixed size 1 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ExecuteCommandResponse {
                /// `saturated uint8`
                ///
                /// Always aligned
                /// Size 8 bits
                pub status: u8,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ExecuteCommandResponse {}
            impl ExecuteCommandResponse {
                pub const STATUS_BAD_COMMAND: u8 = 3;
                pub const STATUS_BAD_PARAMETER: u8 = 4;
                pub const STATUS_BAD_STATE: u8 = 5;
                pub const STATUS_FAILURE: u8 = 1;
                pub const STATUS_INTERNAL_ERROR: u8 = 6;
                pub const STATUS_NOT_AUTHORIZED: u8 = 2;
                pub const STATUS_SUCCESS: u8 = 0;
            }
            impl ::canadensis_encoding::Serialize for ExecuteCommandResponse {
                fn size_bits(&self) -> usize {
                    8
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for ExecuteCommandResponse {
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
                assert_eq!(::core::mem::size_of::<ExecuteCommandResponse>() * 8, 8);
                assert_eq!(
                    ::memoffset::offset_of!(ExecuteCommandResponse, status) * 8,
                    0
                );
            }
        }
        pub mod get_info_1_0 {
            /// `uavcan.node.GetInfo.1.0`
            ///
            /// Fixed size 0 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct GetInfoRequest {}
            impl ::canadensis_encoding::DataType for GetInfoRequest {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Request for GetInfoRequest {}
            impl GetInfoRequest {}
            impl ::canadensis_encoding::Serialize for GetInfoRequest {
                fn size_bits(&self) -> usize {
                    0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for GetInfoRequest {
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
                assert_eq!(::core::mem::size_of::<GetInfoRequest>() * 8, 0);
            }

            /// `uavcan.node.GetInfo.1.0`
            ///
            /// Size ranges from 33 to 313 bytes
            pub struct GetInfoResponse {
                /// `uavcan.node.Version.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub protocol_version: crate::uavcan::node::version_1_0::Version,
                /// `uavcan.node.Version.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub hardware_version: crate::uavcan::node::version_1_0::Version,
                /// `uavcan.node.Version.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub software_version: crate::uavcan::node::version_1_0::Version,
                /// `saturated uint64`
                ///
                /// Always aligned
                /// Size 64 bits
                pub software_vcs_revision_id: u64,
                /// `saturated uint8[16]`
                ///
                /// Always aligned
                /// Size 128 bits
                pub unique_id: [u8; 16],
                /// `saturated uint8[<=50]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 400 bits
                pub name: ::heapless::Vec<u8, 50>,
                /// `saturated uint64[<=1]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 64 bits
                pub software_image_crc: ::heapless::Vec<u64, 1>,
                /// `saturated uint8[<=222]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 1776 bits
                pub certificate_of_authenticity: ::heapless::Vec<u8, 222>,
            }
            impl ::canadensis_encoding::DataType for GetInfoResponse {
                const EXTENT_BYTES: Option<u32> = Some(448);
            }
            impl ::canadensis_encoding::Response for GetInfoResponse {}
            impl GetInfoResponse {}
            impl ::canadensis_encoding::Serialize for GetInfoResponse {
                fn size_bits(&self) -> usize {
                    16 + 16
                        + 16
                        + 64
                        + (self.unique_id).len() * 8
                        + 8
                        + (self.name).len() * 8
                        + 8
                        + (self.software_image_crc).len() * 64
                        + 8
                        + (self.certificate_of_authenticity).len() * 8
                        + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.protocol_version);
                    cursor.write_composite(&self.hardware_version);
                    cursor.write_composite(&self.software_version);
                    cursor.write_aligned_u64(self.software_vcs_revision_id);
                    cursor.write_bytes(&(self.unique_id)[..]);
                    cursor.write_aligned_u8((self.name).len() as u8);
                    cursor.write_bytes(&(self.name)[..]);
                    cursor.write_aligned_u8((self.software_image_crc).len() as u8);
                    for value in (self.software_image_crc).iter() {
                        cursor.write_u64(*value);
                    }
                    cursor.write_aligned_u8((self.certificate_of_authenticity).len() as u8);
                    cursor.write_bytes(&(self.certificate_of_authenticity)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for GetInfoResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(GetInfoResponse {
                        protocol_version: { cursor.read_composite()? },
                        hardware_version: { cursor.read_composite()? },
                        software_version: { cursor.read_composite()? },
                        software_vcs_revision_id: { cursor.read_u64() as _ },
                        unique_id: {
                            [
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                                cursor.read_u8() as _,
                            ]
                        },
                        name: {
                            let length = cursor.read_u8() as _;
                            if length <= 50 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
                                }
                                elements
                            } else {
                                return Err(::canadensis_encoding::DeserializeError::ArrayLength);
                            }
                        },
                        software_image_crc: {
                            let length = cursor.read_u8() as _;
                            if length <= 1 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u64() as _);
                                }
                                elements
                            } else {
                                return Err(::canadensis_encoding::DeserializeError::ArrayLength);
                            }
                        },
                        certificate_of_authenticity: {
                            let length = cursor.read_u8() as _;
                            if length <= 222 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
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
        pub mod get_transport_statistics_0_1 {
            /// `uavcan.node.GetTransportStatistics.0.1`
            ///
            /// Fixed size 0 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct GetTransportStatisticsRequest {}
            impl ::canadensis_encoding::DataType for GetTransportStatisticsRequest {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Request for GetTransportStatisticsRequest {}
            impl GetTransportStatisticsRequest {}
            impl ::canadensis_encoding::Serialize for GetTransportStatisticsRequest {
                fn size_bits(&self) -> usize {
                    0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for GetTransportStatisticsRequest {
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
                assert_eq!(
                    ::core::mem::size_of::<GetTransportStatisticsRequest>() * 8,
                    0
                );
            }

            /// `uavcan.node.GetTransportStatistics.0.1`
            ///
            /// Size ranges from 16 to 61 bytes
            pub struct GetTransportStatisticsResponse {
                /// `uavcan.node.IOStatistics.0.1`
                ///
                /// Always aligned
                /// Size 120 bits
                pub transfer_statistics: crate::uavcan::node::io_statistics_0_1::IOStatistics,
                /// `uavcan.node.IOStatistics.0.1[<=3]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 360 bits
                pub network_interface_statistics:
                    ::heapless::Vec<crate::uavcan::node::io_statistics_0_1::IOStatistics, 3>,
            }
            impl ::canadensis_encoding::DataType for GetTransportStatisticsResponse {
                const EXTENT_BYTES: Option<u32> = Some(192);
            }
            impl ::canadensis_encoding::Response for GetTransportStatisticsResponse {}
            impl GetTransportStatisticsResponse {
                pub const MAX_NETWORK_INTERFACES: u8 = 3;
            }
            impl ::canadensis_encoding::Serialize for GetTransportStatisticsResponse {
                fn size_bits(&self) -> usize {
                    120 + 8 + (self.network_interface_statistics).len() * 120 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.transfer_statistics);
                    cursor.write_aligned_u8((self.network_interface_statistics).len() as u8);
                    for value in (self.network_interface_statistics).iter() {
                        cursor.write_composite(value);
                    }
                }
            }
            impl ::canadensis_encoding::Deserialize for GetTransportStatisticsResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(GetTransportStatisticsResponse {
                        transfer_statistics: { cursor.read_composite()? },
                        network_interface_statistics: {
                            let length = cursor.read_u8() as _;
                            if length <= 3 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_composite()?);
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
        pub mod health_1_0 {
            /// `uavcan.node.Health.1.0`
            ///
            /// Fixed size 1 bytes
            pub struct Health {
                /// `saturated uint2`
                ///
                /// Always aligned
                /// Size 2 bits
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for Health {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Health {}
            impl Health {
                pub const ADVISORY: u8 = 1;
                pub const CAUTION: u8 = 2;
                pub const NOMINAL: u8 = 0;
                pub const WARNING: u8 = 3;
            }
            impl ::canadensis_encoding::Serialize for Health {
                fn size_bits(&self) -> usize {
                    8
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u2(self.value);
                }
            }
            impl ::canadensis_encoding::Deserialize for Health {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Health {
                        value: { cursor.read_u2() as _ },
                    })
                }
            }
        }
        pub mod heartbeat_1_0 {
            /// `uavcan.node.Heartbeat.1.0`
            ///
            /// Fixed size 7 bytes
            pub struct Heartbeat {
                /// `saturated uint32`
                ///
                /// Always aligned
                /// Size 32 bits
                pub uptime: u32,
                /// `uavcan.node.Health.1.0`
                ///
                /// Always aligned
                /// Size 8 bits
                pub health: crate::uavcan::node::health_1_0::Health,
                /// `uavcan.node.Mode.1.0`
                ///
                /// Always aligned
                /// Size 8 bits
                pub mode: crate::uavcan::node::mode_1_0::Mode,
                /// `saturated uint8`
                ///
                /// Always aligned
                /// Size 8 bits
                pub vendor_specific_status_code: u8,
            }
            impl ::canadensis_encoding::DataType for Heartbeat {
                const EXTENT_BYTES: Option<u32> = Some(12);
            }
            impl ::canadensis_encoding::Message for Heartbeat {}
            impl Heartbeat {
                pub const MAX_PUBLICATION_PERIOD: u16 = 1;
                pub const OFFLINE_TIMEOUT: u16 = 3;
            }
            impl ::canadensis_encoding::Serialize for Heartbeat {
                fn size_bits(&self) -> usize {
                    56
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u32(self.uptime);
                    cursor.write_composite(&self.health);
                    cursor.write_composite(&self.mode);
                    cursor.write_aligned_u8(self.vendor_specific_status_code);
                }
            }
            impl ::canadensis_encoding::Deserialize for Heartbeat {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Heartbeat {
                        uptime: { cursor.read_u32() as _ },
                        health: { cursor.read_composite()? },
                        mode: { cursor.read_composite()? },
                        vendor_specific_status_code: { cursor.read_u8() as _ },
                    })
                }
            }
        }
        pub mod id_1_0 {
            /// `uavcan.node.ID.1.0`
            ///
            /// Fixed size 2 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ID {
                /// `saturated uint16`
                ///
                /// Always aligned
                /// Size 16 bits
                pub value: u16,
            }
            impl ::canadensis_encoding::DataType for ID {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for ID {}
            impl ID {}
            impl ::canadensis_encoding::Serialize for ID {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for ID {
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
                assert_eq!(::core::mem::size_of::<ID>() * 8, 16);
                assert_eq!(::memoffset::offset_of!(ID, value) * 8, 0);
            }
        }
        pub mod io_statistics_0_1 {
            /// `uavcan.node.IOStatistics.0.1`
            ///
            /// Fixed size 15 bytes
            pub struct IOStatistics {
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub num_emitted: u64,
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub num_received: u64,
                /// `truncated uint40`
                ///
                /// Always aligned
                /// Size 40 bits
                pub num_errored: u64,
            }
            impl ::canadensis_encoding::DataType for IOStatistics {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for IOStatistics {}
            impl IOStatistics {}
            impl ::canadensis_encoding::Serialize for IOStatistics {
                fn size_bits(&self) -> usize {
                    120
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u40(self.num_emitted);
                    cursor.write_u40(self.num_received);
                    cursor.write_u40(self.num_errored);
                }
            }
            impl ::canadensis_encoding::Deserialize for IOStatistics {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(IOStatistics {
                        num_emitted: { cursor.read_u40() as _ },
                        num_received: { cursor.read_u40() as _ },
                        num_errored: { cursor.read_u40() as _ },
                    })
                }
            }
        }
        pub mod mode_1_0 {
            /// `uavcan.node.Mode.1.0`
            ///
            /// Fixed size 1 bytes
            pub struct Mode {
                /// `saturated uint3`
                ///
                /// Always aligned
                /// Size 3 bits
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for Mode {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Mode {}
            impl Mode {
                pub const INITIALIZATION: u8 = 1;
                pub const MAINTENANCE: u8 = 2;
                pub const OPERATIONAL: u8 = 0;
                pub const SOFTWARE_UPDATE: u8 = 3;
            }
            impl ::canadensis_encoding::Serialize for Mode {
                fn size_bits(&self) -> usize {
                    8
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u3(self.value);
                }
            }
            impl ::canadensis_encoding::Deserialize for Mode {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Mode {
                        value: { cursor.read_u3() as _ },
                    })
                }
            }
        }
        pub mod port {
            pub mod id_1_0 {
                /// `uavcan.node.port.ID.1.0`
                ///
                /// Fixed size 3 bytes
                pub enum ID {
                    // uavcan.node.port.SubjectID.1.0
                    SubjectId(crate::uavcan::node::port::subject_id_1_0::SubjectID),
                    // uavcan.node.port.ServiceID.1.0
                    ServiceId(crate::uavcan::node::port::service_id_1_0::ServiceID),
                }
                impl ::canadensis_encoding::DataType for ID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for ID {}
                impl ID {}
                impl ::canadensis_encoding::Serialize for ID {
                    fn size_bits(&self) -> usize {
                        24
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        match self {
                            ID::SubjectId(inner) => {
                                cursor.write_aligned_u8(0);
                                cursor.write_composite(inner);
                            }
                            ID::ServiceId(inner) => {
                                cursor.write_aligned_u8(1);
                                cursor.write_composite(inner);
                            }
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for ID {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        match cursor.read_aligned_u8() as _ {
                            0 => Ok(ID::SubjectId({ cursor.read_composite()? })),
                            1 => Ok(ID::ServiceId({ cursor.read_composite()? })),
                            _ => Err(::canadensis_encoding::DeserializeError::UnionTag),
                        }
                    }
                }
            }
            pub mod list_0_1 {
                /// `uavcan.node.port.List.0.1`
                ///
                /// Size ranges from 146 to 2194 bytes
                pub struct List {
                    /// `uavcan.node.port.SubjectIDList.0.1`
                    ///
                    /// Always aligned
                    /// Size ranges from 8 to 8200 bits
                    pub publishers: crate::uavcan::node::port::subject_id_list_0_1::SubjectIDList,
                    /// `uavcan.node.port.SubjectIDList.0.1`
                    ///
                    /// Always aligned
                    /// Size ranges from 8 to 8200 bits
                    pub subscribers: crate::uavcan::node::port::subject_id_list_0_1::SubjectIDList,
                    /// `uavcan.node.port.ServiceIDList.0.1`
                    ///
                    /// Always aligned
                    /// Size 512 bits
                    pub clients: crate::uavcan::node::port::service_id_list_0_1::ServiceIDList,
                    /// `uavcan.node.port.ServiceIDList.0.1`
                    ///
                    /// Always aligned
                    /// Size 512 bits
                    pub servers: crate::uavcan::node::port::service_id_list_0_1::ServiceIDList,
                }
                impl ::canadensis_encoding::DataType for List {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for List {}
                impl List {
                    pub const MAX_PUBLICATION_PERIOD: u8 = 10;
                }
                impl ::canadensis_encoding::Serialize for List {
                    fn size_bits(&self) -> usize {
                        32 + (self.publishers).size_bits()
                            + 32
                            + (self.subscribers).size_bits()
                            + 32
                            + 512
                            + 32
                            + 512
                            + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.publishers);
                        cursor.write_composite(&self.subscribers);
                        cursor.write_composite(&self.clients);
                        cursor.write_composite(&self.servers);
                    }
                }
                impl ::canadensis_encoding::Deserialize for List {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(List {
                            publishers: { cursor.read_composite()? },
                            subscribers: { cursor.read_composite()? },
                            clients: { cursor.read_composite()? },
                            servers: { cursor.read_composite()? },
                        })
                    }
                }
            }
            pub mod service_id_1_0 {
                /// `uavcan.node.port.ServiceID.1.0`
                ///
                /// Fixed size 2 bytes
                pub struct ServiceID {
                    /// `saturated uint9`
                    ///
                    /// Always aligned
                    /// Size 9 bits
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for ServiceID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for ServiceID {}
                impl ServiceID {
                    pub const MAX: u16 = 511;
                }
                impl ::canadensis_encoding::Serialize for ServiceID {
                    fn size_bits(&self) -> usize {
                        16
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u9(self.value);
                    }
                }
                impl ::canadensis_encoding::Deserialize for ServiceID {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(ServiceID {
                            value: { cursor.read_u9() as _ },
                        })
                    }
                }
            }
            pub mod service_id_list_0_1 {
                /// `uavcan.node.port.ServiceIDList.0.1`
                ///
                /// Fixed size 64 bytes
                pub struct ServiceIDList {
                    /// `saturated bool[512]`
                    ///
                    /// Always aligned
                    /// Size 512 bits
                    pub mask: ::canadensis_encoding::bits::BitArray<64>,
                }
                impl ::canadensis_encoding::DataType for ServiceIDList {
                    const EXTENT_BYTES: Option<u32> = Some(128);
                }
                impl ::canadensis_encoding::Message for ServiceIDList {}
                impl ServiceIDList {
                    pub const CAPACITY: u16 = 512;
                }
                impl ::canadensis_encoding::Serialize for ServiceIDList {
                    fn size_bits(&self) -> usize {
                        512
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        (self.mask).serialize(cursor);
                    }
                }
                impl ::canadensis_encoding::Deserialize for ServiceIDList {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(ServiceIDList {
                            mask: {
                                ::canadensis_encoding::bits::BitArray::deserialize(
                                    512_usize, cursor,
                                )
                            },
                        })
                    }
                }
            }
            pub mod subject_id_1_0 {
                /// `uavcan.node.port.SubjectID.1.0`
                ///
                /// Fixed size 2 bytes
                pub struct SubjectID {
                    /// `saturated uint13`
                    ///
                    /// Always aligned
                    /// Size 13 bits
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for SubjectID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for SubjectID {}
                impl SubjectID {
                    pub const MAX: u16 = 8191;
                }
                impl ::canadensis_encoding::Serialize for SubjectID {
                    fn size_bits(&self) -> usize {
                        16
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u13(self.value);
                    }
                }
                impl ::canadensis_encoding::Deserialize for SubjectID {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(SubjectID {
                            value: { cursor.read_u13() as _ },
                        })
                    }
                }
            }
            pub mod subject_id_list_0_1 {
                /// `uavcan.node.port.SubjectIDList.0.1`
                ///
                /// Size ranges from 1 to 1025 bytes
                pub enum SubjectIDList {
                    // saturated bool[8192]
                    Mask(::canadensis_encoding::bits::BitArray<1024>),
                    // uavcan.node.port.SubjectID.1.0[<=255]
                    SparseList(
                        ::heapless::Vec<crate::uavcan::node::port::subject_id_1_0::SubjectID, 255>,
                    ),
                    // uavcan.primitive.Empty.1.0
                    Total(crate::uavcan::primitive::empty_1_0::Empty),
                }
                impl ::canadensis_encoding::DataType for SubjectIDList {
                    const EXTENT_BYTES: Option<u32> = Some(4097);
                }
                impl ::canadensis_encoding::Message for SubjectIDList {}
                impl SubjectIDList {
                    pub const CAPACITY: u16 = 8192;
                }
                impl ::canadensis_encoding::Serialize for SubjectIDList {
                    fn size_bits(&self) -> usize {
                        8 + match self {
                            SubjectIDList::Mask(inner) => (inner).len() * 1,
                            SubjectIDList::SparseList(inner) => 8 + (inner).len() * 16,
                            SubjectIDList::Total(inner) => 0,
                        }
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        match self {
                            SubjectIDList::Mask(inner) => {
                                cursor.write_aligned_u8(0);
                                (inner).serialize(cursor);
                            }
                            SubjectIDList::SparseList(inner) => {
                                cursor.write_aligned_u8(1);
                                cursor.write_aligned_u8((inner).len() as u8);
                                for value in (inner).iter() {
                                    cursor.write_composite(value);
                                }
                            }
                            SubjectIDList::Total(inner) => {
                                cursor.write_aligned_u8(2);
                                cursor.write_composite(inner);
                            }
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for SubjectIDList {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        match cursor.read_aligned_u8() as _ {
                            0 => Ok(SubjectIDList::Mask({
                                ::canadensis_encoding::bits::BitArray::deserialize(
                                    8192_usize, cursor,
                                )
                            })),
                            1 => Ok(SubjectIDList::SparseList({
                                let length = cursor.read_u8() as _;
                                if length <= 255 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_composite()?);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            })),
                            2 => Ok(SubjectIDList::Total({ cursor.read_composite()? })),
                            _ => Err(::canadensis_encoding::DeserializeError::UnionTag),
                        }
                    }
                }
            }
        }
        pub mod version_1_0 {
            /// `uavcan.node.Version.1.0`
            ///
            /// Fixed size 2 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct Version {
                /// `saturated uint8`
                ///
                /// Always aligned
                /// Size 8 bits
                pub major: u8,
                /// `saturated uint8`
                ///
                /// Always aligned
                /// Size 8 bits
                pub minor: u8,
            }
            impl ::canadensis_encoding::DataType for Version {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Version {}
            impl Version {}
            impl ::canadensis_encoding::Serialize for Version {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for Version {
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
                assert_eq!(::core::mem::size_of::<Version>() * 8, 16);
                assert_eq!(::memoffset::offset_of!(Version, major) * 8, 0);
                assert_eq!(::memoffset::offset_of!(Version, minor) * 8, 8);
            }
        }
    }
    pub mod pnp {
        pub mod cluster {
            pub mod append_entries_1_0 {
                /// `uavcan.pnp.cluster.AppendEntries.1.0`
                ///
                /// Size ranges from 13 to 35 bytes
                pub struct AppendEntriesRequest {
                    /// `saturated uint32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub term: u32,
                    /// `saturated uint32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub prev_log_term: u32,
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub prev_log_index: u16,
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub leader_commit: u16,
                    /// `uavcan.pnp.cluster.Entry.1.0[<=1]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 176 bits
                    pub entries: ::heapless::Vec<crate::uavcan::pnp::cluster::entry_1_0::Entry, 1>,
                }
                impl ::canadensis_encoding::DataType for AppendEntriesRequest {
                    const EXTENT_BYTES: Option<u32> = Some(96);
                }
                impl ::canadensis_encoding::Request for AppendEntriesRequest {}
                impl AppendEntriesRequest {
                    pub const DEFAULT_MAX_ELECTION_TIMEOUT: u8 = 4;
                    pub const DEFAULT_MIN_ELECTION_TIMEOUT: u8 = 2;
                }
                impl ::canadensis_encoding::Serialize for AppendEntriesRequest {
                    fn size_bits(&self) -> usize {
                        32 + 32 + 16 + 16 + 8 + (self.entries).len() * 176 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u32(self.term);
                        cursor.write_aligned_u32(self.prev_log_term);
                        cursor.write_aligned_u16(self.prev_log_index);
                        cursor.write_aligned_u16(self.leader_commit);
                        cursor.write_aligned_u8((self.entries).len() as u8);
                        for value in (self.entries).iter() {
                            cursor.write_composite(value);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for AppendEntriesRequest {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(AppendEntriesRequest {
                            term: { cursor.read_u32() as _ },
                            prev_log_term: { cursor.read_u32() as _ },
                            prev_log_index: { cursor.read_u16() as _ },
                            leader_commit: { cursor.read_u16() as _ },
                            entries: {
                                let length = cursor.read_u8() as _;
                                if length <= 1 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_composite()?);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }

                /// `uavcan.pnp.cluster.AppendEntries.1.0`
                ///
                /// Fixed size 5 bytes
                pub struct AppendEntriesResponse {
                    /// `saturated uint32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub term: u32,
                    /// `saturated bool`
                    ///
                    /// Always aligned
                    /// Size 1 bits
                    pub success: bool,
                }
                impl ::canadensis_encoding::DataType for AppendEntriesResponse {
                    const EXTENT_BYTES: Option<u32> = Some(48);
                }
                impl ::canadensis_encoding::Response for AppendEntriesResponse {}
                impl AppendEntriesResponse {}
                impl ::canadensis_encoding::Serialize for AppendEntriesResponse {
                    fn size_bits(&self) -> usize {
                        40
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u32(self.term);
                        cursor.write_bool(self.success);
                    }
                }
                impl ::canadensis_encoding::Deserialize for AppendEntriesResponse {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(AppendEntriesResponse {
                            term: { cursor.read_u32() as _ },
                            success: { cursor.read_bool() },
                        })
                    }
                }
            }
            pub mod discovery_1_0 {
                /// `uavcan.pnp.cluster.Discovery.1.0`
                ///
                /// Size ranges from 2 to 12 bytes
                pub struct Discovery {
                    /// `saturated uint3`
                    ///
                    /// Always aligned
                    /// Size 3 bits
                    pub configured_cluster_size: u8,
                    // 5 bits of padding
                    /// `uavcan.node.ID.1.0[<=5]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 80 bits
                    pub known_nodes: ::heapless::Vec<crate::uavcan::node::id_1_0::ID, 5>,
                }
                impl ::canadensis_encoding::DataType for Discovery {
                    const EXTENT_BYTES: Option<u32> = Some(96);
                }
                impl ::canadensis_encoding::Message for Discovery {}
                impl Discovery {
                    pub const BROADCASTING_PERIOD: u8 = 1;
                    pub const MAX_CLUSTER_SIZE: u8 = 5;
                }
                impl ::canadensis_encoding::Serialize for Discovery {
                    fn size_bits(&self) -> usize {
                        3 + 5 + 8 + (self.known_nodes).len() * 16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u3(self.configured_cluster_size);
                        cursor.skip_5();
                        cursor.write_aligned_u8((self.known_nodes).len() as u8);
                        for value in (self.known_nodes).iter() {
                            cursor.write_composite(value);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Discovery {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Discovery {
                            configured_cluster_size: { cursor.read_u3() as _ },
                            known_nodes: {
                                cursor.skip_5();
                                let length = cursor.read_u8() as _;
                                if length <= 5 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_composite()?);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod entry_1_0 {
                /// `uavcan.pnp.cluster.Entry.1.0`
                ///
                /// Fixed size 22 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Entry {
                    /// `saturated uint32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub term: u32,
                    /// `saturated uint8[16]`
                    ///
                    /// Always aligned
                    /// Size 128 bits
                    pub unique_id: [u8; 16],
                    /// `uavcan.node.ID.1.0`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub node_id: crate::uavcan::node::id_1_0::ID,
                }
                impl ::canadensis_encoding::DataType for Entry {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Entry {}
                impl Entry {}
                impl ::canadensis_encoding::Serialize for Entry {
                    fn size_bits(&self) -> usize {
                        176
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Entry {
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
                    assert_eq!(::core::mem::size_of::<Entry>() * 8, 176);
                    assert_eq!(::memoffset::offset_of!(Entry, term) * 8, 0);
                    assert_eq!(::memoffset::offset_of!(Entry, unique_id) * 8, 32);
                    assert_eq!(::memoffset::offset_of!(Entry, node_id) * 8, 160);
                }
            }
            pub mod request_vote_1_0 {
                /// `uavcan.pnp.cluster.RequestVote.1.0`
                ///
                /// Fixed size 10 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct RequestVoteRequest {
                    /// `saturated uint32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub term: u32,
                    /// `saturated uint32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub last_log_term: u32,
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub last_log_index: u16,
                }
                impl ::canadensis_encoding::DataType for RequestVoteRequest {
                    const EXTENT_BYTES: Option<u32> = Some(48);
                }
                impl ::canadensis_encoding::Request for RequestVoteRequest {}
                impl RequestVoteRequest {}
                impl ::canadensis_encoding::Serialize for RequestVoteRequest {
                    fn size_bits(&self) -> usize {
                        80
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for RequestVoteRequest {
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
                    assert_eq!(::core::mem::size_of::<RequestVoteRequest>() * 8, 80);
                    assert_eq!(::memoffset::offset_of!(RequestVoteRequest, term) * 8, 0);
                    assert_eq!(
                        ::memoffset::offset_of!(RequestVoteRequest, last_log_term) * 8,
                        32
                    );
                    assert_eq!(
                        ::memoffset::offset_of!(RequestVoteRequest, last_log_index) * 8,
                        64
                    );
                }

                /// `uavcan.pnp.cluster.RequestVote.1.0`
                ///
                /// Fixed size 5 bytes
                pub struct RequestVoteResponse {
                    /// `saturated uint32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub term: u32,
                    /// `saturated bool`
                    ///
                    /// Always aligned
                    /// Size 1 bits
                    pub vote_granted: bool,
                }
                impl ::canadensis_encoding::DataType for RequestVoteResponse {
                    const EXTENT_BYTES: Option<u32> = Some(48);
                }
                impl ::canadensis_encoding::Response for RequestVoteResponse {}
                impl RequestVoteResponse {}
                impl ::canadensis_encoding::Serialize for RequestVoteResponse {
                    fn size_bits(&self) -> usize {
                        40
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u32(self.term);
                        cursor.write_bool(self.vote_granted);
                    }
                }
                impl ::canadensis_encoding::Deserialize for RequestVoteResponse {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(RequestVoteResponse {
                            term: { cursor.read_u32() as _ },
                            vote_granted: { cursor.read_bool() },
                        })
                    }
                }
            }
        }
        pub mod node_id_allocation_data_1_0 {
            /// `uavcan.pnp.NodeIDAllocationData.1.0`
            ///
            /// Size ranges from 7 to 9 bytes
            pub struct NodeIDAllocationData {
                /// `truncated uint48`
                ///
                /// Always aligned
                /// Size 48 bits
                pub unique_id_hash: u64,
                /// `uavcan.node.ID.1.0[<=1]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 16 bits
                pub allocated_node_id: ::heapless::Vec<crate::uavcan::node::id_1_0::ID, 1>,
            }
            impl ::canadensis_encoding::DataType for NodeIDAllocationData {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for NodeIDAllocationData {}
            impl NodeIDAllocationData {}
            impl ::canadensis_encoding::Serialize for NodeIDAllocationData {
                fn size_bits(&self) -> usize {
                    48 + 8 + (self.allocated_node_id).len() * 16 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u48(self.unique_id_hash);
                    cursor.write_aligned_u8((self.allocated_node_id).len() as u8);
                    for value in (self.allocated_node_id).iter() {
                        cursor.write_composite(value);
                    }
                }
            }
            impl ::canadensis_encoding::Deserialize for NodeIDAllocationData {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(NodeIDAllocationData {
                        unique_id_hash: { cursor.read_u48() as _ },
                        allocated_node_id: {
                            let length = cursor.read_u8() as _;
                            if length <= 1 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_composite()?);
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
        pub mod node_id_allocation_data_2_0 {
            /// `uavcan.pnp.NodeIDAllocationData.2.0`
            ///
            /// Fixed size 18 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct NodeIDAllocationData {
                /// `uavcan.node.ID.1.0`
                ///
                /// Always aligned
                /// Size 16 bits
                pub node_id: crate::uavcan::node::id_1_0::ID,
                /// `saturated uint8[16]`
                ///
                /// Always aligned
                /// Size 128 bits
                pub unique_id: [u8; 16],
            }
            impl ::canadensis_encoding::DataType for NodeIDAllocationData {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Message for NodeIDAllocationData {}
            impl NodeIDAllocationData {}
            impl ::canadensis_encoding::Serialize for NodeIDAllocationData {
                fn size_bits(&self) -> usize {
                    144
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for NodeIDAllocationData {
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
                assert_eq!(::core::mem::size_of::<NodeIDAllocationData>() * 8, 144);
                assert_eq!(
                    ::memoffset::offset_of!(NodeIDAllocationData, node_id) * 8,
                    0
                );
                assert_eq!(
                    ::memoffset::offset_of!(NodeIDAllocationData, unique_id) * 8,
                    16
                );
            }
        }
    }
    pub mod primitive {
        pub mod array {
            pub mod bit_1_0 {
                /// `uavcan.primitive.array.Bit.1.0`
                ///
                /// Size ranges from 2 to 258 bytes
                pub struct Bit {
                    /// `saturated bool[<=2048]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::canadensis_encoding::bits::BitArray<256>,
                }
                impl ::canadensis_encoding::DataType for Bit {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Bit {}
                impl Bit {}
                impl ::canadensis_encoding::Serialize for Bit {
                    fn size_bits(&self) -> usize {
                        16 + (self.value).len() * 1 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        (self.value).serialize(cursor);
                    }
                }
                impl ::canadensis_encoding::Deserialize for Bit {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Bit {
                            value: {
                                {
                                    let length = cursor.read_u16() as _;
                                    ::canadensis_encoding::bits::BitArray::deserialize(
                                        length, cursor,
                                    )
                                }
                            },
                        })
                    }
                }
            }
            pub mod integer16_1_0 {
                /// `uavcan.primitive.array.Integer16.1.0`
                ///
                /// Size ranges from 1 to 257 bytes
                pub struct Integer16 {
                    /// `saturated int16[<=128]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<i16, 128>,
                }
                impl ::canadensis_encoding::DataType for Integer16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer16 {}
                impl Integer16 {}
                impl ::canadensis_encoding::Serialize for Integer16 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).len() * 16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u16(*value as u16);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Integer16 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Integer16 {
                            value: {
                                let length = cursor.read_u8() as _;
                                if length <= 128 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u16() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod integer32_1_0 {
                /// `uavcan.primitive.array.Integer32.1.0`
                ///
                /// Size ranges from 1 to 257 bytes
                pub struct Integer32 {
                    /// `saturated int32[<=64]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<i32, 64>,
                }
                impl ::canadensis_encoding::DataType for Integer32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer32 {}
                impl Integer32 {}
                impl ::canadensis_encoding::Serialize for Integer32 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).len() * 32 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u32(*value as u32);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Integer32 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Integer32 {
                            value: {
                                let length = cursor.read_u8() as _;
                                if length <= 64 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u32() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod integer64_1_0 {
                /// `uavcan.primitive.array.Integer64.1.0`
                ///
                /// Size ranges from 1 to 257 bytes
                pub struct Integer64 {
                    /// `saturated int64[<=32]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<i64, 32>,
                }
                impl ::canadensis_encoding::DataType for Integer64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer64 {}
                impl Integer64 {}
                impl ::canadensis_encoding::Serialize for Integer64 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).len() * 64 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u64(*value as u64);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Integer64 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Integer64 {
                            value: {
                                let length = cursor.read_u8() as _;
                                if length <= 32 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u64() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod integer8_1_0 {
                /// `uavcan.primitive.array.Integer8.1.0`
                ///
                /// Size ranges from 2 to 258 bytes
                pub struct Integer8 {
                    /// `saturated int8[<=256]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<i8, 256>,
                }
                impl ::canadensis_encoding::DataType for Integer8 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer8 {}
                impl Integer8 {}
                impl ::canadensis_encoding::Serialize for Integer8 {
                    fn size_bits(&self) -> usize {
                        16 + (self.value).len() * 8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16((self.value).len() as u16);
                        for value in (self.value).iter() {
                            cursor.write_u8(*value as u8);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Integer8 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Integer8 {
                            value: {
                                let length = cursor.read_u16() as _;
                                if length <= 256 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod natural16_1_0 {
                /// `uavcan.primitive.array.Natural16.1.0`
                ///
                /// Size ranges from 1 to 257 bytes
                pub struct Natural16 {
                    /// `saturated uint16[<=128]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<u16, 128>,
                }
                impl ::canadensis_encoding::DataType for Natural16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural16 {}
                impl Natural16 {}
                impl ::canadensis_encoding::Serialize for Natural16 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).len() * 16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u16(*value);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Natural16 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Natural16 {
                            value: {
                                let length = cursor.read_u8() as _;
                                if length <= 128 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u16() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod natural32_1_0 {
                /// `uavcan.primitive.array.Natural32.1.0`
                ///
                /// Size ranges from 1 to 257 bytes
                pub struct Natural32 {
                    /// `saturated uint32[<=64]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<u32, 64>,
                }
                impl ::canadensis_encoding::DataType for Natural32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural32 {}
                impl Natural32 {}
                impl ::canadensis_encoding::Serialize for Natural32 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).len() * 32 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u32(*value);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Natural32 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Natural32 {
                            value: {
                                let length = cursor.read_u8() as _;
                                if length <= 64 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u32() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod natural64_1_0 {
                /// `uavcan.primitive.array.Natural64.1.0`
                ///
                /// Size ranges from 1 to 257 bytes
                pub struct Natural64 {
                    /// `saturated uint64[<=32]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<u64, 32>,
                }
                impl ::canadensis_encoding::DataType for Natural64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural64 {}
                impl Natural64 {}
                impl ::canadensis_encoding::Serialize for Natural64 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).len() * 64 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u64(*value);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Natural64 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Natural64 {
                            value: {
                                let length = cursor.read_u8() as _;
                                if length <= 32 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u64() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod natural8_1_0 {
                /// `uavcan.primitive.array.Natural8.1.0`
                ///
                /// Size ranges from 2 to 258 bytes
                pub struct Natural8 {
                    /// `saturated uint8[<=256]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<u8, 256>,
                }
                impl ::canadensis_encoding::DataType for Natural8 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural8 {}
                impl Natural8 {}
                impl ::canadensis_encoding::Serialize for Natural8 {
                    fn size_bits(&self) -> usize {
                        16 + (self.value).len() * 8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16((self.value).len() as u16);
                        cursor.write_bytes(&(self.value)[..]);
                    }
                }
                impl ::canadensis_encoding::Deserialize for Natural8 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Natural8 {
                            value: {
                                let length = cursor.read_u16() as _;
                                if length <= 256 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_u8() as _);
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod real16_1_0 {
                /// `uavcan.primitive.array.Real16.1.0`
                ///
                /// Size ranges from 1 to 257 bytes
                pub struct Real16 {
                    /// `saturated float16[<=128]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value:
                        ::heapless::Vec<::canadensis_encoding::f16_zerocopy::ZeroCopyF16, 128>,
                }
                impl ::canadensis_encoding::DataType for Real16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real16 {}
                impl Real16 {}
                impl ::canadensis_encoding::Serialize for Real16 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).len() * 16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_f16((*value).into());
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Real16 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Real16 {
                            value: {
                                let length = cursor.read_u8() as _;
                                if length <= 128 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_f16().into());
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod real32_1_0 {
                /// `uavcan.primitive.array.Real32.1.0`
                ///
                /// Size ranges from 1 to 257 bytes
                pub struct Real32 {
                    /// `saturated float32[<=64]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<f32, 64>,
                }
                impl ::canadensis_encoding::DataType for Real32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real32 {}
                impl Real32 {}
                impl ::canadensis_encoding::Serialize for Real32 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).len() * 32 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_f32(*value);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Real32 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Real32 {
                            value: {
                                let length = cursor.read_u8() as _;
                                if length <= 64 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_f32());
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
            pub mod real64_1_0 {
                /// `uavcan.primitive.array.Real64.1.0`
                ///
                /// Size ranges from 1 to 257 bytes
                pub struct Real64 {
                    /// `saturated float64[<=32]`
                    ///
                    /// Always aligned
                    /// Size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<f64, 32>,
                }
                impl ::canadensis_encoding::DataType for Real64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real64 {}
                impl Real64 {}
                impl ::canadensis_encoding::Serialize for Real64 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).len() * 64 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_f64(*value);
                        }
                    }
                }
                impl ::canadensis_encoding::Deserialize for Real64 {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Real64 {
                            value: {
                                let length = cursor.read_u8() as _;
                                if length <= 32 {
                                    let mut elements = ::heapless::Vec::new();
                                    for _ in 0..length {
                                        let _ = elements.push(cursor.read_f64());
                                    }
                                    elements
                                } else {
                                    return Err(
                                        ::canadensis_encoding::DeserializeError::ArrayLength,
                                    );
                                }
                            },
                        })
                    }
                }
            }
        }
        pub mod empty_1_0 {
            /// `uavcan.primitive.Empty.1.0`
            ///
            /// Fixed size 0 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct Empty {}
            impl ::canadensis_encoding::DataType for Empty {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Empty {}
            impl Empty {}
            impl ::canadensis_encoding::Serialize for Empty {
                fn size_bits(&self) -> usize {
                    0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for Empty {
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
                assert_eq!(::core::mem::size_of::<Empty>() * 8, 0);
            }
        }
        pub mod scalar {
            pub mod bit_1_0 {
                /// `uavcan.primitive.scalar.Bit.1.0`
                ///
                /// Fixed size 1 bytes
                pub struct Bit {
                    /// `saturated bool`
                    ///
                    /// Always aligned
                    /// Size 1 bits
                    pub value: bool,
                }
                impl ::canadensis_encoding::DataType for Bit {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Bit {}
                impl Bit {}
                impl ::canadensis_encoding::Serialize for Bit {
                    fn size_bits(&self) -> usize {
                        8
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_bool(self.value);
                    }
                }
                impl ::canadensis_encoding::Deserialize for Bit {
                    fn deserialize(
                        cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                    ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                    where
                        Self: Sized,
                    {
                        Ok(Bit {
                            value: { cursor.read_bool() },
                        })
                    }
                }
            }
            pub mod integer16_1_0 {
                /// `uavcan.primitive.scalar.Integer16.1.0`
                ///
                /// Fixed size 2 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Integer16 {
                    /// `saturated int16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub value: i16,
                }
                impl ::canadensis_encoding::DataType for Integer16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer16 {}
                impl Integer16 {}
                impl ::canadensis_encoding::Serialize for Integer16 {
                    fn size_bits(&self) -> usize {
                        16
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Integer16 {
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
                    assert_eq!(::core::mem::size_of::<Integer16>() * 8, 16);
                    assert_eq!(::memoffset::offset_of!(Integer16, value) * 8, 0);
                }
            }
            pub mod integer32_1_0 {
                /// `uavcan.primitive.scalar.Integer32.1.0`
                ///
                /// Fixed size 4 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Integer32 {
                    /// `saturated int32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub value: i32,
                }
                impl ::canadensis_encoding::DataType for Integer32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer32 {}
                impl Integer32 {}
                impl ::canadensis_encoding::Serialize for Integer32 {
                    fn size_bits(&self) -> usize {
                        32
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Integer32 {
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
                    assert_eq!(::core::mem::size_of::<Integer32>() * 8, 32);
                    assert_eq!(::memoffset::offset_of!(Integer32, value) * 8, 0);
                }
            }
            pub mod integer64_1_0 {
                /// `uavcan.primitive.scalar.Integer64.1.0`
                ///
                /// Fixed size 8 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Integer64 {
                    /// `saturated int64`
                    ///
                    /// Always aligned
                    /// Size 64 bits
                    pub value: i64,
                }
                impl ::canadensis_encoding::DataType for Integer64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer64 {}
                impl Integer64 {}
                impl ::canadensis_encoding::Serialize for Integer64 {
                    fn size_bits(&self) -> usize {
                        64
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Integer64 {
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
                    assert_eq!(::core::mem::size_of::<Integer64>() * 8, 64);
                    assert_eq!(::memoffset::offset_of!(Integer64, value) * 8, 0);
                }
            }
            pub mod integer8_1_0 {
                /// `uavcan.primitive.scalar.Integer8.1.0`
                ///
                /// Fixed size 1 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Integer8 {
                    /// `saturated int8`
                    ///
                    /// Always aligned
                    /// Size 8 bits
                    pub value: i8,
                }
                impl ::canadensis_encoding::DataType for Integer8 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer8 {}
                impl Integer8 {}
                impl ::canadensis_encoding::Serialize for Integer8 {
                    fn size_bits(&self) -> usize {
                        8
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Integer8 {
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
                    assert_eq!(::core::mem::size_of::<Integer8>() * 8, 8);
                    assert_eq!(::memoffset::offset_of!(Integer8, value) * 8, 0);
                }
            }
            pub mod natural16_1_0 {
                /// `uavcan.primitive.scalar.Natural16.1.0`
                ///
                /// Fixed size 2 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Natural16 {
                    /// `saturated uint16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for Natural16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural16 {}
                impl Natural16 {}
                impl ::canadensis_encoding::Serialize for Natural16 {
                    fn size_bits(&self) -> usize {
                        16
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Natural16 {
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
                    assert_eq!(::core::mem::size_of::<Natural16>() * 8, 16);
                    assert_eq!(::memoffset::offset_of!(Natural16, value) * 8, 0);
                }
            }
            pub mod natural32_1_0 {
                /// `uavcan.primitive.scalar.Natural32.1.0`
                ///
                /// Fixed size 4 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Natural32 {
                    /// `saturated uint32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub value: u32,
                }
                impl ::canadensis_encoding::DataType for Natural32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural32 {}
                impl Natural32 {}
                impl ::canadensis_encoding::Serialize for Natural32 {
                    fn size_bits(&self) -> usize {
                        32
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Natural32 {
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
                    assert_eq!(::core::mem::size_of::<Natural32>() * 8, 32);
                    assert_eq!(::memoffset::offset_of!(Natural32, value) * 8, 0);
                }
            }
            pub mod natural64_1_0 {
                /// `uavcan.primitive.scalar.Natural64.1.0`
                ///
                /// Fixed size 8 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Natural64 {
                    /// `saturated uint64`
                    ///
                    /// Always aligned
                    /// Size 64 bits
                    pub value: u64,
                }
                impl ::canadensis_encoding::DataType for Natural64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural64 {}
                impl Natural64 {}
                impl ::canadensis_encoding::Serialize for Natural64 {
                    fn size_bits(&self) -> usize {
                        64
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Natural64 {
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
                    assert_eq!(::core::mem::size_of::<Natural64>() * 8, 64);
                    assert_eq!(::memoffset::offset_of!(Natural64, value) * 8, 0);
                }
            }
            pub mod natural8_1_0 {
                /// `uavcan.primitive.scalar.Natural8.1.0`
                ///
                /// Fixed size 1 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Natural8 {
                    /// `saturated uint8`
                    ///
                    /// Always aligned
                    /// Size 8 bits
                    pub value: u8,
                }
                impl ::canadensis_encoding::DataType for Natural8 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural8 {}
                impl Natural8 {}
                impl ::canadensis_encoding::Serialize for Natural8 {
                    fn size_bits(&self) -> usize {
                        8
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Natural8 {
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
                    assert_eq!(::core::mem::size_of::<Natural8>() * 8, 8);
                    assert_eq!(::memoffset::offset_of!(Natural8, value) * 8, 0);
                }
            }
            pub mod real16_1_0 {
                /// `uavcan.primitive.scalar.Real16.1.0`
                ///
                /// Fixed size 2 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Real16 {
                    /// `saturated float16`
                    ///
                    /// Always aligned
                    /// Size 16 bits
                    pub value: ::canadensis_encoding::f16_zerocopy::ZeroCopyF16,
                }
                impl ::canadensis_encoding::DataType for Real16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real16 {}
                impl Real16 {}
                impl ::canadensis_encoding::Serialize for Real16 {
                    fn size_bits(&self) -> usize {
                        16
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Real16 {
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
                    assert_eq!(::core::mem::size_of::<Real16>() * 8, 16);
                    assert_eq!(::memoffset::offset_of!(Real16, value) * 8, 0);
                }
            }
            pub mod real32_1_0 {
                /// `uavcan.primitive.scalar.Real32.1.0`
                ///
                /// Fixed size 4 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Real32 {
                    /// `saturated float32`
                    ///
                    /// Always aligned
                    /// Size 32 bits
                    pub value: f32,
                }
                impl ::canadensis_encoding::DataType for Real32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real32 {}
                impl Real32 {}
                impl ::canadensis_encoding::Serialize for Real32 {
                    fn size_bits(&self) -> usize {
                        32
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Real32 {
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
                    assert_eq!(::core::mem::size_of::<Real32>() * 8, 32);
                    assert_eq!(::memoffset::offset_of!(Real32, value) * 8, 0);
                }
            }
            pub mod real64_1_0 {
                /// `uavcan.primitive.scalar.Real64.1.0`
                ///
                /// Fixed size 8 bytes
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Real64 {
                    /// `saturated float64`
                    ///
                    /// Always aligned
                    /// Size 64 bits
                    pub value: f64,
                }
                impl ::canadensis_encoding::DataType for Real64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real64 {}
                impl Real64 {}
                impl ::canadensis_encoding::Serialize for Real64 {
                    fn size_bits(&self) -> usize {
                        64
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                    }
                }
                impl ::canadensis_encoding::Deserialize for Real64 {
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
                    assert_eq!(::core::mem::size_of::<Real64>() * 8, 64);
                    assert_eq!(::memoffset::offset_of!(Real64, value) * 8, 0);
                }
            }
        }
        pub mod string_1_0 {
            /// `uavcan.primitive.String.1.0`
            ///
            /// Size ranges from 2 to 258 bytes
            pub struct String {
                /// `saturated uint8[<=256]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 2048 bits
                pub value: ::heapless::Vec<u8, 256>,
            }
            impl ::canadensis_encoding::DataType for String {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for String {}
            impl String {}
            impl ::canadensis_encoding::Serialize for String {
                fn size_bits(&self) -> usize {
                    16 + (self.value).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16((self.value).len() as u16);
                    cursor.write_bytes(&(self.value)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for String {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(String {
                        value: {
                            let length = cursor.read_u16() as _;
                            if length <= 256 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
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
        pub mod unstructured_1_0 {
            /// `uavcan.primitive.Unstructured.1.0`
            ///
            /// Size ranges from 2 to 258 bytes
            pub struct Unstructured {
                /// `saturated uint8[<=256]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 2048 bits
                pub value: ::heapless::Vec<u8, 256>,
            }
            impl ::canadensis_encoding::DataType for Unstructured {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Unstructured {}
            impl Unstructured {}
            impl ::canadensis_encoding::Serialize for Unstructured {
                fn size_bits(&self) -> usize {
                    16 + (self.value).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16((self.value).len() as u16);
                    cursor.write_bytes(&(self.value)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for Unstructured {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Unstructured {
                        value: {
                            let length = cursor.read_u16() as _;
                            if length <= 256 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
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
    }
    pub mod register {
        pub mod access_1_0 {
            /// `uavcan.register.Access.1.0`
            ///
            /// Size ranges from 2 to 515 bytes
            pub struct AccessRequest {
                /// `uavcan.register.Name.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2048 bits
                pub name: crate::uavcan::register::name_1_0::Name,
                /// `uavcan.register.Value.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2072 bits
                pub value: crate::uavcan::register::value_1_0::Value,
            }
            impl ::canadensis_encoding::DataType for AccessRequest {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Request for AccessRequest {}
            impl AccessRequest {}
            impl ::canadensis_encoding::Serialize for AccessRequest {
                fn size_bits(&self) -> usize {
                    (self.name).size_bits() + (self.value).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.name);
                    cursor.write_composite(&self.value);
                }
            }
            impl ::canadensis_encoding::Deserialize for AccessRequest {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(AccessRequest {
                        name: { cursor.read_composite()? },
                        value: { cursor.read_composite()? },
                    })
                }
            }

            /// `uavcan.register.Access.1.0`
            ///
            /// Size ranges from 9 to 267 bytes
            pub struct AccessResponse {
                /// `uavcan.time.SynchronizedTimestamp.1.0`
                ///
                /// Always aligned
                /// Size 56 bits
                pub timestamp:
                    crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                /// `saturated bool`
                ///
                /// Always aligned
                /// Size 1 bits
                pub mutable: bool,
                /// `saturated bool`
                ///
                /// Not always aligned
                /// Size 1 bits
                pub persistent: bool,
                // 6 bits of padding
                /// `uavcan.register.Value.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2072 bits
                pub value: crate::uavcan::register::value_1_0::Value,
            }
            impl ::canadensis_encoding::DataType for AccessResponse {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Response for AccessResponse {}
            impl AccessResponse {}
            impl ::canadensis_encoding::Serialize for AccessResponse {
                fn size_bits(&self) -> usize {
                    56 + 1 + 1 + 6 + (self.value).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.timestamp);
                    cursor.write_bool(self.mutable);
                    cursor.write_bool(self.persistent);
                    cursor.skip_6();
                    cursor.write_composite(&self.value);
                }
            }
            impl ::canadensis_encoding::Deserialize for AccessResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(AccessResponse {
                        timestamp: { cursor.read_composite()? },
                        mutable: { cursor.read_bool() },
                        persistent: { cursor.read_bool() },
                        value: {
                            cursor.skip_6();
                            cursor.read_composite()?
                        },
                    })
                }
            }
        }
        pub mod list_1_0 {
            /// `uavcan.register.List.1.0`
            ///
            /// Fixed size 2 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ListRequest {
                /// `saturated uint16`
                ///
                /// Always aligned
                /// Size 16 bits
                pub index: u16,
            }
            impl ::canadensis_encoding::DataType for ListRequest {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Request for ListRequest {}
            impl ListRequest {}
            impl ::canadensis_encoding::Serialize for ListRequest {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for ListRequest {
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
                assert_eq!(::core::mem::size_of::<ListRequest>() * 8, 16);
                assert_eq!(::memoffset::offset_of!(ListRequest, index) * 8, 0);
            }

            /// `uavcan.register.List.1.0`
            ///
            /// Size ranges from 1 to 256 bytes
            pub struct ListResponse {
                /// `uavcan.register.Name.1.0`
                ///
                /// Always aligned
                /// Size ranges from 8 to 2048 bits
                pub name: crate::uavcan::register::name_1_0::Name,
            }
            impl ::canadensis_encoding::DataType for ListResponse {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Response for ListResponse {}
            impl ListResponse {}
            impl ::canadensis_encoding::Serialize for ListResponse {
                fn size_bits(&self) -> usize {
                    (self.name).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.name);
                }
            }
            impl ::canadensis_encoding::Deserialize for ListResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ListResponse {
                        name: { cursor.read_composite()? },
                    })
                }
            }
        }
        pub mod name_1_0 {
            /// `uavcan.register.Name.1.0`
            ///
            /// Size ranges from 1 to 256 bytes
            pub struct Name {
                /// `saturated uint8[<=255]`
                ///
                /// Always aligned
                /// Size ranges from 0 to 2040 bits
                pub name: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for Name {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Name {}
            impl Name {}
            impl ::canadensis_encoding::Serialize for Name {
                fn size_bits(&self) -> usize {
                    8 + (self.name).len() * 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u8((self.name).len() as u8);
                    cursor.write_bytes(&(self.name)[..]);
                }
            }
            impl ::canadensis_encoding::Deserialize for Name {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Name {
                        name: {
                            let length = cursor.read_u8() as _;
                            if length <= 255 {
                                let mut elements = ::heapless::Vec::new();
                                for _ in 0..length {
                                    let _ = elements.push(cursor.read_u8() as _);
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
        pub mod value_1_0 {
            /// `uavcan.register.Value.1.0`
            ///
            /// Size ranges from 1 to 259 bytes
            pub enum Value {
                // uavcan.primitive.Empty.1.0
                Empty(crate::uavcan::primitive::empty_1_0::Empty),
                // uavcan.primitive.String.1.0
                String(crate::uavcan::primitive::string_1_0::String),
                // uavcan.primitive.Unstructured.1.0
                Unstructured(crate::uavcan::primitive::unstructured_1_0::Unstructured),
                // uavcan.primitive.array.Bit.1.0
                Bit(crate::uavcan::primitive::array::bit_1_0::Bit),
                // uavcan.primitive.array.Integer64.1.0
                Integer64(crate::uavcan::primitive::array::integer64_1_0::Integer64),
                // uavcan.primitive.array.Integer32.1.0
                Integer32(crate::uavcan::primitive::array::integer32_1_0::Integer32),
                // uavcan.primitive.array.Integer16.1.0
                Integer16(crate::uavcan::primitive::array::integer16_1_0::Integer16),
                // uavcan.primitive.array.Integer8.1.0
                Integer8(crate::uavcan::primitive::array::integer8_1_0::Integer8),
                // uavcan.primitive.array.Natural64.1.0
                Natural64(crate::uavcan::primitive::array::natural64_1_0::Natural64),
                // uavcan.primitive.array.Natural32.1.0
                Natural32(crate::uavcan::primitive::array::natural32_1_0::Natural32),
                // uavcan.primitive.array.Natural16.1.0
                Natural16(crate::uavcan::primitive::array::natural16_1_0::Natural16),
                // uavcan.primitive.array.Natural8.1.0
                Natural8(crate::uavcan::primitive::array::natural8_1_0::Natural8),
                // uavcan.primitive.array.Real64.1.0
                Real64(crate::uavcan::primitive::array::real64_1_0::Real64),
                // uavcan.primitive.array.Real32.1.0
                Real32(crate::uavcan::primitive::array::real32_1_0::Real32),
                // uavcan.primitive.array.Real16.1.0
                Real16(crate::uavcan::primitive::array::real16_1_0::Real16),
            }
            impl ::canadensis_encoding::DataType for Value {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Value {}
            impl Value {}
            impl ::canadensis_encoding::Serialize for Value {
                fn size_bits(&self) -> usize {
                    8 + match self {
                        Value::Empty(inner) => 0,
                        Value::String(inner) => (inner).size_bits(),
                        Value::Unstructured(inner) => (inner).size_bits(),
                        Value::Bit(inner) => (inner).size_bits(),
                        Value::Integer64(inner) => (inner).size_bits(),
                        Value::Integer32(inner) => (inner).size_bits(),
                        Value::Integer16(inner) => (inner).size_bits(),
                        Value::Integer8(inner) => (inner).size_bits(),
                        Value::Natural64(inner) => (inner).size_bits(),
                        Value::Natural32(inner) => (inner).size_bits(),
                        Value::Natural16(inner) => (inner).size_bits(),
                        Value::Natural8(inner) => (inner).size_bits(),
                        Value::Real64(inner) => (inner).size_bits(),
                        Value::Real32(inner) => (inner).size_bits(),
                        Value::Real16(inner) => (inner).size_bits(),
                    }
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    match self {
                        Value::Empty(inner) => {
                            cursor.write_aligned_u8(0);
                            cursor.write_composite(inner);
                        }
                        Value::String(inner) => {
                            cursor.write_aligned_u8(1);
                            cursor.write_composite(inner);
                        }
                        Value::Unstructured(inner) => {
                            cursor.write_aligned_u8(2);
                            cursor.write_composite(inner);
                        }
                        Value::Bit(inner) => {
                            cursor.write_aligned_u8(3);
                            cursor.write_composite(inner);
                        }
                        Value::Integer64(inner) => {
                            cursor.write_aligned_u8(4);
                            cursor.write_composite(inner);
                        }
                        Value::Integer32(inner) => {
                            cursor.write_aligned_u8(5);
                            cursor.write_composite(inner);
                        }
                        Value::Integer16(inner) => {
                            cursor.write_aligned_u8(6);
                            cursor.write_composite(inner);
                        }
                        Value::Integer8(inner) => {
                            cursor.write_aligned_u8(7);
                            cursor.write_composite(inner);
                        }
                        Value::Natural64(inner) => {
                            cursor.write_aligned_u8(8);
                            cursor.write_composite(inner);
                        }
                        Value::Natural32(inner) => {
                            cursor.write_aligned_u8(9);
                            cursor.write_composite(inner);
                        }
                        Value::Natural16(inner) => {
                            cursor.write_aligned_u8(10);
                            cursor.write_composite(inner);
                        }
                        Value::Natural8(inner) => {
                            cursor.write_aligned_u8(11);
                            cursor.write_composite(inner);
                        }
                        Value::Real64(inner) => {
                            cursor.write_aligned_u8(12);
                            cursor.write_composite(inner);
                        }
                        Value::Real32(inner) => {
                            cursor.write_aligned_u8(13);
                            cursor.write_composite(inner);
                        }
                        Value::Real16(inner) => {
                            cursor.write_aligned_u8(14);
                            cursor.write_composite(inner);
                        }
                    }
                }
            }
            impl ::canadensis_encoding::Deserialize for Value {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    match cursor.read_aligned_u8() as _ {
                        0 => Ok(Value::Empty({ cursor.read_composite()? })),
                        1 => Ok(Value::String({ cursor.read_composite()? })),
                        2 => Ok(Value::Unstructured({ cursor.read_composite()? })),
                        3 => Ok(Value::Bit({ cursor.read_composite()? })),
                        4 => Ok(Value::Integer64({ cursor.read_composite()? })),
                        5 => Ok(Value::Integer32({ cursor.read_composite()? })),
                        6 => Ok(Value::Integer16({ cursor.read_composite()? })),
                        7 => Ok(Value::Integer8({ cursor.read_composite()? })),
                        8 => Ok(Value::Natural64({ cursor.read_composite()? })),
                        9 => Ok(Value::Natural32({ cursor.read_composite()? })),
                        10 => Ok(Value::Natural16({ cursor.read_composite()? })),
                        11 => Ok(Value::Natural8({ cursor.read_composite()? })),
                        12 => Ok(Value::Real64({ cursor.read_composite()? })),
                        13 => Ok(Value::Real32({ cursor.read_composite()? })),
                        14 => Ok(Value::Real16({ cursor.read_composite()? })),
                        _ => Err(::canadensis_encoding::DeserializeError::UnionTag),
                    }
                }
            }
        }
    }
    pub mod si {
        pub mod sample {
            pub mod acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.acceleration.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub meter_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.meter_per_second_per_second);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                meter_per_second_per_second: { cursor.read_f32() },
                            })
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.acceleration.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub meter_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            152
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.meter_per_second_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Vector3 {
                                timestamp: { cursor.read_composite()? },
                                meter_per_second_per_second: {
                                    [cursor.read_f32(), cursor.read_f32(), cursor.read_f32()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod angle {
                pub mod quaternion_1_0 {
                    /// `uavcan.si.sample.angle.Quaternion.1.0`
                    ///
                    /// Fixed size 23 bytes
                    pub struct Quaternion {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32[4]`
                        ///
                        /// Always aligned
                        /// Size 128 bits
                        pub wxyz: [f32; 4],
                    }
                    impl ::canadensis_encoding::DataType for Quaternion {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Quaternion {}
                    impl Quaternion {}
                    impl ::canadensis_encoding::Serialize for Quaternion {
                        fn size_bits(&self) -> usize {
                            184
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.wxyz).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Quaternion {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Quaternion {
                                timestamp: { cursor.read_composite()? },
                                wxyz: {
                                    [
                                        cursor.read_f32(),
                                        cursor.read_f32(),
                                        cursor.read_f32(),
                                        cursor.read_f32(),
                                    ]
                                },
                            })
                        }
                    }
                }
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.angle.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub radian: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.radian);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                radian: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod angular_acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.angular_acceleration.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub radian_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.radian_per_second_per_second);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                radian_per_second_per_second: { cursor.read_f32() },
                            })
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.angular_acceleration.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub radian_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            152
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.radian_per_second_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Vector3 {
                                timestamp: { cursor.read_composite()? },
                                radian_per_second_per_second: {
                                    [cursor.read_f32(), cursor.read_f32(), cursor.read_f32()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod angular_velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.angular_velocity.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub radian_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.radian_per_second);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                radian_per_second: { cursor.read_f32() },
                            })
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.angular_velocity.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub radian_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            152
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.radian_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Vector3 {
                                timestamp: { cursor.read_composite()? },
                                radian_per_second: {
                                    [cursor.read_f32(), cursor.read_f32(), cursor.read_f32()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod duration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.duration.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.second);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                second: { cursor.read_f32() },
                            })
                        }
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.sample.duration.WideScalar.1.0`
                    ///
                    /// Fixed size 15 bytes
                    pub struct WideScalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float64`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub second: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideScalar {}
                    impl WideScalar {}
                    impl ::canadensis_encoding::Serialize for WideScalar {
                        fn size_bits(&self) -> usize {
                            120
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f64(self.second);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for WideScalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(WideScalar {
                                timestamp: { cursor.read_composite()? },
                                second: { cursor.read_f64() },
                            })
                        }
                    }
                }
            }
            pub mod electric_charge {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.electric_charge.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub coulomb: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.coulomb);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                coulomb: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod electric_current {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.electric_current.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub ampere: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.ampere);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                ampere: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod energy {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.energy.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub joule: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.joule);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                joule: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod force {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.force.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub newton: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.newton);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                newton: { cursor.read_f32() },
                            })
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.force.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub newton: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            152
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.newton).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Vector3 {
                                timestamp: { cursor.read_composite()? },
                                newton: {
                                    [cursor.read_f32(), cursor.read_f32(), cursor.read_f32()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod frequency {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.frequency.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub hertz: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.hertz);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                hertz: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod length {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.length.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.meter);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                meter: { cursor.read_f32() },
                            })
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.length.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            152
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.meter).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Vector3 {
                                timestamp: { cursor.read_composite()? },
                                meter: {
                                    [cursor.read_f32(), cursor.read_f32(), cursor.read_f32()]
                                },
                            })
                        }
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.sample.length.WideScalar.1.0`
                    ///
                    /// Fixed size 15 bytes
                    pub struct WideScalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float64`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub meter: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideScalar {}
                    impl WideScalar {}
                    impl ::canadensis_encoding::Serialize for WideScalar {
                        fn size_bits(&self) -> usize {
                            120
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f64(self.meter);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for WideScalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(WideScalar {
                                timestamp: { cursor.read_composite()? },
                                meter: { cursor.read_f64() },
                            })
                        }
                    }
                }
                pub mod wide_vector3_1_0 {
                    /// `uavcan.si.sample.length.WideVector3.1.0`
                    ///
                    /// Fixed size 31 bytes
                    pub struct WideVector3 {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float64[3]`
                        ///
                        /// Always aligned
                        /// Size 192 bits
                        pub meter: [f64; 3],
                    }
                    impl ::canadensis_encoding::DataType for WideVector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideVector3 {}
                    impl WideVector3 {}
                    impl ::canadensis_encoding::Serialize for WideVector3 {
                        fn size_bits(&self) -> usize {
                            248
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.meter).iter() {
                                cursor.write_f64(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for WideVector3 {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(WideVector3 {
                                timestamp: { cursor.read_composite()? },
                                meter: {
                                    [cursor.read_f64(), cursor.read_f64(), cursor.read_f64()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod magnetic_field_strength {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.magnetic_field_strength.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub tesla: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.tesla);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                tesla: { cursor.read_f32() },
                            })
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.magnetic_field_strength.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub tesla: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            152
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.tesla).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Vector3 {
                                timestamp: { cursor.read_composite()? },
                                tesla: {
                                    [cursor.read_f32(), cursor.read_f32(), cursor.read_f32()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod mass {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.mass.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub kilogram: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.kilogram);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                kilogram: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod power {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.power.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub watt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.watt);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                watt: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod pressure {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.pressure.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub pascal: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.pascal);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                pascal: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod temperature {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.temperature.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub kelvin: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.kelvin);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                kelvin: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod torque {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.torque.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub newton_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.newton_meter);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                newton_meter: { cursor.read_f32() },
                            })
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.torque.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub newton_meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            152
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.newton_meter).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Vector3 {
                                timestamp: { cursor.read_composite()? },
                                newton_meter: {
                                    [cursor.read_f32(), cursor.read_f32(), cursor.read_f32()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.velocity.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.meter_per_second);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                meter_per_second: { cursor.read_f32() },
                            })
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.velocity.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub meter_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            152
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.meter_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Vector3 {
                                timestamp: { cursor.read_composite()? },
                                meter_per_second: {
                                    [cursor.read_f32(), cursor.read_f32(), cursor.read_f32()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod voltage {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.voltage.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub volt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.volt);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                volt: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod volume {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.volume.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub cubic_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.cubic_meter);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                cubic_meter: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod volumetric_flow_rate {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.volumetric_flow_rate.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned
                        /// Size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub cubic_meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            88
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.cubic_meter_per_second);
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
                        fn deserialize(
                            cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                        ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                        where
                            Self: Sized,
                        {
                            Ok(Scalar {
                                timestamp: { cursor.read_composite()? },
                                cubic_meter_per_second: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
        }
        pub mod unit {
            pub mod acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.acceleration.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub meter_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(
                            ::memoffset::offset_of!(Scalar, meter_per_second_per_second) * 8,
                            0
                        );
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.acceleration.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub meter_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
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
                        assert_eq!(::core::mem::size_of::<Vector3>() * 8, 96);
                        assert_eq!(
                            ::memoffset::offset_of!(Vector3, meter_per_second_per_second) * 8,
                            0
                        );
                    }
                }
            }
            pub mod angle {
                pub mod quaternion_1_0 {
                    /// `uavcan.si.unit.angle.Quaternion.1.0`
                    ///
                    /// Fixed size 16 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Quaternion {
                        /// `saturated float32[4]`
                        ///
                        /// Always aligned
                        /// Size 128 bits
                        pub wxyz: [f32; 4],
                    }
                    impl ::canadensis_encoding::DataType for Quaternion {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Quaternion {}
                    impl Quaternion {}
                    impl ::canadensis_encoding::Serialize for Quaternion {
                        fn size_bits(&self) -> usize {
                            128
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Quaternion {
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
                        assert_eq!(::core::mem::size_of::<Quaternion>() * 8, 128);
                        assert_eq!(::memoffset::offset_of!(Quaternion, wxyz) * 8, 0);
                    }
                }
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.angle.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub radian: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, radian) * 8, 0);
                    }
                }
            }
            pub mod angular_acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.angular_acceleration.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub radian_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(
                            ::memoffset::offset_of!(Scalar, radian_per_second_per_second) * 8,
                            0
                        );
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.angular_acceleration.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub radian_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
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
                        assert_eq!(::core::mem::size_of::<Vector3>() * 8, 96);
                        assert_eq!(
                            ::memoffset::offset_of!(Vector3, radian_per_second_per_second) * 8,
                            0
                        );
                    }
                }
            }
            pub mod angular_velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.angular_velocity.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub radian_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, radian_per_second) * 8, 0);
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.angular_velocity.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub radian_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
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
                        assert_eq!(::core::mem::size_of::<Vector3>() * 8, 96);
                        assert_eq!(::memoffset::offset_of!(Vector3, radian_per_second) * 8, 0);
                    }
                }
            }
            pub mod duration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.duration.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, second) * 8, 0);
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.unit.duration.WideScalar.1.0`
                    ///
                    /// Fixed size 8 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct WideScalar {
                        /// `saturated float64`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub second: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideScalar {}
                    impl WideScalar {}
                    impl ::canadensis_encoding::Serialize for WideScalar {
                        fn size_bits(&self) -> usize {
                            64
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for WideScalar {
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
                        assert_eq!(::core::mem::size_of::<WideScalar>() * 8, 64);
                        assert_eq!(::memoffset::offset_of!(WideScalar, second) * 8, 0);
                    }
                }
            }
            pub mod electric_charge {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub coulomb: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, coulomb) * 8, 0);
                    }
                }
            }
            pub mod electric_current {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.electric_current.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub ampere: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, ampere) * 8, 0);
                    }
                }
            }
            pub mod energy {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.energy.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub joule: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, joule) * 8, 0);
                    }
                }
            }
            pub mod force {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.force.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub newton: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, newton) * 8, 0);
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.force.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub newton: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
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
                        assert_eq!(::core::mem::size_of::<Vector3>() * 8, 96);
                        assert_eq!(::memoffset::offset_of!(Vector3, newton) * 8, 0);
                    }
                }
            }
            pub mod frequency {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.frequency.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub hertz: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, hertz) * 8, 0);
                    }
                }
            }
            pub mod length {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.length.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, meter) * 8, 0);
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.length.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
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
                        assert_eq!(::core::mem::size_of::<Vector3>() * 8, 96);
                        assert_eq!(::memoffset::offset_of!(Vector3, meter) * 8, 0);
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.unit.length.WideScalar.1.0`
                    ///
                    /// Fixed size 8 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct WideScalar {
                        /// `saturated float64`
                        ///
                        /// Always aligned
                        /// Size 64 bits
                        pub meter: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideScalar {}
                    impl WideScalar {}
                    impl ::canadensis_encoding::Serialize for WideScalar {
                        fn size_bits(&self) -> usize {
                            64
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for WideScalar {
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
                        assert_eq!(::core::mem::size_of::<WideScalar>() * 8, 64);
                        assert_eq!(::memoffset::offset_of!(WideScalar, meter) * 8, 0);
                    }
                }
                pub mod wide_vector3_1_0 {
                    /// `uavcan.si.unit.length.WideVector3.1.0`
                    ///
                    /// Fixed size 24 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct WideVector3 {
                        /// `saturated float64[3]`
                        ///
                        /// Always aligned
                        /// Size 192 bits
                        pub meter: [f64; 3],
                    }
                    impl ::canadensis_encoding::DataType for WideVector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideVector3 {}
                    impl WideVector3 {}
                    impl ::canadensis_encoding::Serialize for WideVector3 {
                        fn size_bits(&self) -> usize {
                            192
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for WideVector3 {
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
                        assert_eq!(::core::mem::size_of::<WideVector3>() * 8, 192);
                        assert_eq!(::memoffset::offset_of!(WideVector3, meter) * 8, 0);
                    }
                }
            }
            pub mod magnetic_field_strength {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.magnetic_field_strength.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub tesla: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, tesla) * 8, 0);
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.magnetic_field_strength.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub tesla: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
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
                        assert_eq!(::core::mem::size_of::<Vector3>() * 8, 96);
                        assert_eq!(::memoffset::offset_of!(Vector3, tesla) * 8, 0);
                    }
                }
            }
            pub mod mass {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.mass.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub kilogram: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, kilogram) * 8, 0);
                    }
                }
            }
            pub mod power {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.power.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub watt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, watt) * 8, 0);
                    }
                }
            }
            pub mod pressure {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.pressure.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub pascal: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, pascal) * 8, 0);
                    }
                }
            }
            pub mod temperature {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.temperature.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub kelvin: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, kelvin) * 8, 0);
                    }
                }
            }
            pub mod torque {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.torque.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub newton_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, newton_meter) * 8, 0);
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.torque.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub newton_meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
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
                        assert_eq!(::core::mem::size_of::<Vector3>() * 8, 96);
                        assert_eq!(::memoffset::offset_of!(Vector3, newton_meter) * 8, 0);
                    }
                }
            }
            pub mod velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.velocity.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, meter_per_second) * 8, 0);
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.velocity.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned
                        /// Size 96 bits
                        pub meter_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            96
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Vector3 {
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
                        assert_eq!(::core::mem::size_of::<Vector3>() * 8, 96);
                        assert_eq!(::memoffset::offset_of!(Vector3, meter_per_second) * 8, 0);
                    }
                }
            }
            pub mod voltage {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.voltage.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub volt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, volt) * 8, 0);
                    }
                }
            }
            pub mod volume {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.volume.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub cubic_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(::memoffset::offset_of!(Scalar, cubic_meter) * 8, 0);
                    }
                }
            }
            pub mod volumetric_flow_rate {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.volumetric_flow_rate.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        /// `saturated float32`
                        ///
                        /// Always aligned
                        /// Size 32 bits
                        pub cubic_meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                        }
                    }
                    impl ::canadensis_encoding::Deserialize for Scalar {
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
                        assert_eq!(::core::mem::size_of::<Scalar>() * 8, 32);
                        assert_eq!(
                            ::memoffset::offset_of!(Scalar, cubic_meter_per_second) * 8,
                            0
                        );
                    }
                }
            }
        }
    }
    pub mod time {
        pub mod get_synchronization_master_info_0_1 {
            /// `uavcan.time.GetSynchronizationMasterInfo.0.1`
            ///
            /// Fixed size 0 bytes
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct GetSynchronizationMasterInfoRequest {}
            impl ::canadensis_encoding::DataType for GetSynchronizationMasterInfoRequest {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Request for GetSynchronizationMasterInfoRequest {}
            impl GetSynchronizationMasterInfoRequest {}
            impl ::canadensis_encoding::Serialize for GetSynchronizationMasterInfoRequest {
                fn size_bits(&self) -> usize {
                    0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));
                }
            }
            impl ::canadensis_encoding::Deserialize for GetSynchronizationMasterInfoRequest {
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
                assert_eq!(
                    ::core::mem::size_of::<GetSynchronizationMasterInfoRequest>() * 8,
                    0
                );
            }

            /// `uavcan.time.GetSynchronizationMasterInfo.0.1`
            ///
            /// Fixed size 7 bytes
            pub struct GetSynchronizationMasterInfoResponse {
                /// `saturated float32`
                ///
                /// Always aligned
                /// Size 32 bits
                pub error_variance: f32,
                /// `uavcan.time.TimeSystem.0.1`
                ///
                /// Always aligned
                /// Size 8 bits
                pub time_system: crate::uavcan::time::time_system_0_1::TimeSystem,
                /// `uavcan.time.TAIInfo.0.1`
                ///
                /// Always aligned
                /// Size 16 bits
                pub tai_info: crate::uavcan::time::tai_info_0_1::TAIInfo,
            }
            impl ::canadensis_encoding::DataType for GetSynchronizationMasterInfoResponse {
                const EXTENT_BYTES: Option<u32> = Some(192);
            }
            impl ::canadensis_encoding::Response for GetSynchronizationMasterInfoResponse {}
            impl GetSynchronizationMasterInfoResponse {}
            impl ::canadensis_encoding::Serialize for GetSynchronizationMasterInfoResponse {
                fn size_bits(&self) -> usize {
                    56
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_f32(self.error_variance);
                    cursor.write_composite(&self.time_system);
                    cursor.write_composite(&self.tai_info);
                }
            }
            impl ::canadensis_encoding::Deserialize for GetSynchronizationMasterInfoResponse {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(GetSynchronizationMasterInfoResponse {
                        error_variance: { cursor.read_f32() },
                        time_system: { cursor.read_composite()? },
                        tai_info: { cursor.read_composite()? },
                    })
                }
            }
        }
        pub mod synchronization_1_0 {
            /// `uavcan.time.Synchronization.1.0`
            ///
            /// Fixed size 7 bytes
            pub struct Synchronization {
                /// `truncated uint56`
                ///
                /// Always aligned
                /// Size 56 bits
                pub previous_transmission_timestamp_microsecond: u64,
            }
            impl ::canadensis_encoding::DataType for Synchronization {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Synchronization {}
            impl Synchronization {
                pub const MAX_PUBLICATION_PERIOD: u8 = 1;
                pub const PUBLISHER_TIMEOUT_PERIOD_MULTIPLIER: u8 = 3;
            }
            impl ::canadensis_encoding::Serialize for Synchronization {
                fn size_bits(&self) -> usize {
                    56
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u56(self.previous_transmission_timestamp_microsecond);
                }
            }
            impl ::canadensis_encoding::Deserialize for Synchronization {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(Synchronization {
                        previous_transmission_timestamp_microsecond: { cursor.read_u56() as _ },
                    })
                }
            }
        }
        pub mod synchronized_timestamp_1_0 {
            /// `uavcan.time.SynchronizedTimestamp.1.0`
            ///
            /// Fixed size 7 bytes
            pub struct SynchronizedTimestamp {
                /// `truncated uint56`
                ///
                /// Always aligned
                /// Size 56 bits
                pub microsecond: u64,
            }
            impl ::canadensis_encoding::DataType for SynchronizedTimestamp {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for SynchronizedTimestamp {}
            impl SynchronizedTimestamp {
                pub const UNKNOWN: u64 = 0;
            }
            impl ::canadensis_encoding::Serialize for SynchronizedTimestamp {
                fn size_bits(&self) -> usize {
                    56
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u56(self.microsecond);
                }
            }
            impl ::canadensis_encoding::Deserialize for SynchronizedTimestamp {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(SynchronizedTimestamp {
                        microsecond: { cursor.read_u56() as _ },
                    })
                }
            }
        }
        pub mod tai_info_0_1 {
            /// `uavcan.time.TAIInfo.0.1`
            ///
            /// Fixed size 2 bytes
            pub struct TAIInfo {
                /// `saturated uint10`
                ///
                /// Always aligned
                /// Size 10 bits
                pub difference_tai_minus_utc: u16,
            }
            impl ::canadensis_encoding::DataType for TAIInfo {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for TAIInfo {}
            impl TAIInfo {
                pub const DIFFERENCE_TAI_MINUS_GPS: u8 = 19;
                pub const DIFFERENCE_TAI_MINUS_UTC_UNKNOWN: u16 = 0;
            }
            impl ::canadensis_encoding::Serialize for TAIInfo {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u10(self.difference_tai_minus_utc);
                }
            }
            impl ::canadensis_encoding::Deserialize for TAIInfo {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(TAIInfo {
                        difference_tai_minus_utc: { cursor.read_u10() as _ },
                    })
                }
            }
        }
        pub mod time_system_0_1 {
            /// `uavcan.time.TimeSystem.0.1`
            ///
            /// Fixed size 1 bytes
            pub struct TimeSystem {
                /// `truncated uint4`
                ///
                /// Always aligned
                /// Size 4 bits
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for TimeSystem {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for TimeSystem {}
            impl TimeSystem {
                pub const APPLICATION_SPECIFIC: u8 = 15;
                pub const MONOTONIC_SINCE_BOOT: u8 = 0;
                pub const TAI: u8 = 1;
            }
            impl ::canadensis_encoding::Serialize for TimeSystem {
                fn size_bits(&self) -> usize {
                    8
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u4(self.value);
                }
            }
            impl ::canadensis_encoding::Deserialize for TimeSystem {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(TimeSystem {
                        value: { cursor.read_u4() as _ },
                    })
                }
            }
        }
    }
}
