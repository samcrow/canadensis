#[cfg(not(target_endian = "little"))]
compile_error!("Zero-copy serialization requires a little-endian target");
#[allow(unused_variables, unused_braces, unused_parens)]
#[allow(clippy::identity_op)]
pub mod reg {
    pub mod udral {
        pub mod physics {
            pub mod acoustics {
                pub mod note_0_1 {
                    /// `reg.udral.physics.acoustics.Note.0.1`
                    ///
                    /// Fixed size 12 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Description of a generic musical note in terms of basic physical quantities.\n\n This type may be used to control sound notification emitters assuming the best effort policy:\n if the requested parameters exceed the capabilities of the emitter, the closest possible values should be assumed."
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Note {
                        ///
                        /// `uavcan.si.unit.frequency.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub frequency: crate::uavcan::si::unit::frequency::scalar_1_0::Scalar,
                        ///
                        /// `uavcan.si.unit.duration.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub duration: crate::uavcan::si::unit::duration::scalar_1_0::Scalar,
                        ///
                        /// `uavcan.si.unit.power.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub acoustic_power: crate::uavcan::si::unit::power::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Note {
                        /// This type is sealed.
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
                        /// `reg.udral.physics.dynamics.rotation.Planar.0.1`
                        ///
                        /// Fixed size 16 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Positive torque is co-directed with positive position/velocity/acceleration.\n Provided states may allow the consumer to deduce certain hidden states such as the moment of inertia."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Planar {
///
/// `reg.udral.physics.kinematics.rotation.Planar.0.1`
///
/// Always aligned,
/// size 96 bits
pub kinematics: crate::reg::udral::physics::kinematics::rotation::planar_0_1::Planar,
#[cfg_attr(not(doctest), doc = " NaN if unknown")]
///
/// `uavcan.si.unit.torque.Scalar.1.0`
///
/// Always aligned,
/// size 32 bits
pub torque: crate::uavcan::si::unit::torque::scalar_1_0::Scalar,
}
                        impl ::canadensis_encoding::DataType for Planar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.dynamics.rotation.PlanarTs.0.1`
                        ///
                        /// Fixed size 23 bytes
                        ///
                        pub struct PlanarTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.dynamics.rotation.Planar.0.1`
///
/// Always aligned,
/// size 128 bits
pub value: crate::reg::udral::physics::dynamics::rotation::planar_0_1::Planar,
}
                        impl ::canadensis_encoding::DataType for PlanarTs {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.dynamics.translation.Linear.0.1`
                        ///
                        /// Fixed size 16 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Positive force is co-directed with positive position/velocity/acceleration.\n Provided kinetic states may allow the consumer to deduce certain hidden states such as the mass of the load."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Linear {
///
/// `reg.udral.physics.kinematics.translation.Linear.0.1`
///
/// Always aligned,
/// size 96 bits
pub kinematics: crate::reg::udral::physics::kinematics::translation::linear_0_1::Linear,
#[cfg_attr(not(doctest), doc = " NaN if unknown")]
///
/// `uavcan.si.unit.force.Scalar.1.0`
///
/// Always aligned,
/// size 32 bits
pub force: crate::uavcan::si::unit::force::scalar_1_0::Scalar,
}
                        impl ::canadensis_encoding::DataType for Linear {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.dynamics.translation.LinearTs.0.1`
                        ///
                        /// Fixed size 23 bytes
                        ///
                        pub struct LinearTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.dynamics.translation.Linear.0.1`
///
/// Always aligned,
/// size 128 bits
pub value: crate::reg::udral::physics::dynamics::translation::linear_0_1::Linear,
}
                        impl ::canadensis_encoding::DataType for LinearTs {
                            /// This type is sealed.
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
                    /// `reg.udral.physics.electricity.Power.0.1`
                    ///
                    /// Fixed size 8 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " DC or AC line electric power quantities. Generally, the following current sign convention applies:\n\n   - Positive current flows from the electric power supply network to the load (e.g., an actuator).\n\n   - If the electric network is the load itself powered from a source (e.g., battery), the current is negative."
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Power {
                        ///
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub current: crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        ///
                        /// `uavcan.si.unit.voltage.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Power {
                        /// This type is sealed.
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
                    /// `reg.udral.physics.electricity.PowerTs.0.1`
                    ///
                    /// Fixed size 15 bytes
                    ///
                    pub struct PowerTs {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `reg.udral.physics.electricity.Power.0.1`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub value: crate::reg::udral::physics::electricity::power_0_1::Power,
                    }
                    impl ::canadensis_encoding::DataType for PowerTs {
                        /// This type is sealed.
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
                    /// `reg.udral.physics.electricity.Source.0.1`
                    ///
                    /// Fixed size 16 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " A generic source or sink of electric power (battery, turbogenerator, braking resistor, etc.).\n Low-pass filtering should be applied to avoid aliasing effects (as is the case everywhere else)."
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Source {
                        #[cfg_attr(
                            not(doctest),
                            doc = " Total instant load power.\n Positive current flows into the source (power sinking).\n Negative current flows from the source to the power supply network (power sourcing)."
                        )]
                        ///
                        /// `reg.udral.physics.electricity.Power.0.1`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub power: crate::reg::udral::physics::electricity::power_0_1::Power,
                        #[cfg_attr(
                            not(doctest),
                            doc = " A pessimistic estimate of the amount of energy that can be reclaimed from the source in its current state.\n This may be dependent on the state of charge/health (for batteries), temperature, load profile, humidity, etc.\n Negative values may be reported to indicate overdischarge or depletion of the reserve energy.\n\n This value approximates (full_energy + int(load_power dt)) plus the environmental influences on the source.\n\n Having the instant power, the time to depletion is estimated as (energy/-power).\n When charging (for batteries), the remaining time to full charge can be found similarly as\n ((full_energy-energy)/power).\n\n For the sake of illustration, if this type was used to represent the state of a braking resistor,\n then this value would be negative indicating the amount of dissipated energy."
                        )]
                        ///
                        /// `uavcan.si.unit.energy.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub energy: crate::uavcan::si::unit::energy::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " A pessimistic estimate of the amount of energy that can be reclaimed from a fresh source (fully fueled generator\n or a fully charged battery) under the current conditions (SoH, temperature, load profile, etc)."
                        )]
                        ///
                        /// `uavcan.si.unit.energy.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub full_energy: crate::uavcan::si::unit::energy::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Source {
                        /// This type is sealed.
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
                    /// `reg.udral.physics.electricity.SourceTs.0.1`
                    ///
                    /// Fixed size 23 bytes
                    ///
                    pub struct SourceTs {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `reg.udral.physics.electricity.Source.0.1`
                        ///
                        /// Always aligned,
                        /// size 128 bits
                        pub value: crate::reg::udral::physics::electricity::source_0_1::Source,
                    }
                    impl ::canadensis_encoding::DataType for SourceTs {
                        /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.Point.0.1`
                        ///
                        /// Fixed size 24 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Cartesian coordinates of a point in space."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Point {
                            ///
                            /// `uavcan.si.unit.length.WideVector3.1.0`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub value:
                                crate::uavcan::si::unit::length::wide_vector3_1_0::WideVector3,
                        }
                        impl ::canadensis_encoding::DataType for Point {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.PointState.0.1`
                        ///
                        /// Fixed size 36 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " The kinematic state of a point, as opposed to that of a body, is devoid of rotation information.\n Therefore, the velocity is specified in the parent coordinate frame."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointState {
                            ///
                            /// `reg.udral.physics.kinematics.cartesian.Point.0.1`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub position:
                                crate::reg::udral::physics::kinematics::cartesian::point_0_1::Point,
                            ///
                            /// `uavcan.si.unit.velocity.Vector3.1.0`
                            ///
                            /// Always aligned,
                            /// size 96 bits
                            pub velocity: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                        }
                        impl ::canadensis_encoding::DataType for PointState {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.PointStateVar.0.1`
                        ///
                        /// Fixed size 60 bytes
                        ///
                        #[cfg_attr(not(doctest), doc = " See PointState for details.")]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointStateVar {
///
/// `reg.udral.physics.kinematics.cartesian.PointVar.0.1`
///
/// Always aligned,
/// size 288 bits
pub position: crate::reg::udral::physics::kinematics::cartesian::point_var_0_1::PointVar,
///
/// `reg.udral.physics.kinematics.translation.Velocity3Var.0.2`
///
/// Always aligned,
/// size 192 bits
pub velocity: crate::reg::udral::physics::kinematics::translation::velocity3_var_0_2::Velocity3Var,
}
                        impl ::canadensis_encoding::DataType for PointStateVar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.PointStateVarTs.0.1`
                        ///
                        /// Fixed size 67 bytes
                        ///
                        pub struct PointStateVarTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.kinematics.cartesian.PointStateVar.0.1`
///
/// Always aligned,
/// size 480 bits
pub value: crate::reg::udral::physics::kinematics::cartesian::point_state_var_0_1::PointStateVar,
}
                        impl ::canadensis_encoding::DataType for PointStateVarTs {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.PointVar.0.1`
                        ///
                        /// Fixed size 36 bytes
                        ///
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointVar {
                            ///
                            /// `reg.udral.physics.kinematics.cartesian.Point.0.1`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub value:
                                crate::reg::udral::physics::kinematics::cartesian::point_0_1::Point,
                            #[cfg_attr(
                                not(doctest),
                                doc = " [meter^2]  Upper-right triangle of the covariance matrix."
                            )]
                            ///
                            /// `saturated float16[6]`
                            ///
                            /// Always aligned,
                            /// size 96 bits
                            pub covariance_urt: [::half::f16; 6],
                        }
                        impl ::canadensis_encoding::DataType for PointVar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.Pose.0.1`
                        ///
                        /// Fixed size 40 bytes
                        ///
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Pose {
                            ///
                            /// `reg.udral.physics.kinematics.cartesian.Point.0.1`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub position:
                                crate::reg::udral::physics::kinematics::cartesian::point_0_1::Point,
                            ///
                            /// `uavcan.si.unit.angle.Quaternion.1.0`
                            ///
                            /// Always aligned,
                            /// size 128 bits
                            pub orientation:
                                crate::uavcan::si::unit::angle::quaternion_1_0::Quaternion,
                        }
                        impl ::canadensis_encoding::DataType for Pose {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.PoseVar.0.1`
                        ///
                        /// Fixed size 82 bytes
                        ///
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PoseVar {
                            ///
                            /// `reg.udral.physics.kinematics.cartesian.Pose.0.1`
                            ///
                            /// Always aligned,
                            /// size 320 bits
                            pub value:
                                crate::reg::udral::physics::kinematics::cartesian::pose_0_1::Pose,
                            #[cfg_attr(
                                not(doctest),
                                doc = " Upper-right triangle of the covariance matrix:\n\n                    [parent frame]        [child (body) frame]\n                translation along axis    rotation about axis\n                   X       Y       Z       X       Y       Z\n               +-----------------------------------------------\n X position    |\n Y position    |          m^2                     m*rad\n Z position    |\n X rotation    |\n Y rotation    |                                  rad^2\n Z rotation    |"
                            )]
                            ///
                            /// `saturated float16[21]`
                            ///
                            /// Always aligned,
                            /// size 336 bits
                            pub covariance_urt: [::half::f16; 21],
                        }
                        impl ::canadensis_encoding::DataType for PoseVar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.PoseVarTs.0.1`
                        ///
                        /// Fixed size 89 bytes
                        ///
                        pub struct PoseVarTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.kinematics.cartesian.PoseVar.0.1`
///
/// Always aligned,
/// size 656 bits
pub value: crate::reg::udral::physics::kinematics::cartesian::pose_var_0_1::PoseVar,
}
                        impl ::canadensis_encoding::DataType for PoseVarTs {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.State.0.1`
                        ///
                        /// Fixed size 64 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " First-order kinematic state of a body in space: pose and twist.\n The pose defines a coordinate system transformation from the parent frame to the child frame.\n The twist is specified in the child frame (body frame)."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct State {
                            ///
                            /// `reg.udral.physics.kinematics.cartesian.Pose.0.1`
                            ///
                            /// Always aligned,
                            /// size 320 bits
                            pub pose:
                                crate::reg::udral::physics::kinematics::cartesian::pose_0_1::Pose,
                            ///
                            /// `reg.udral.physics.kinematics.cartesian.Twist.0.1`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub twist:
                                crate::reg::udral::physics::kinematics::cartesian::twist_0_1::Twist,
                        }
                        impl ::canadensis_encoding::DataType for State {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.StateVar.0.1`
                        ///
                        /// Fixed size 148 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " See State for details. This type extends it with covariance matrices."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct StateVar {
///
/// `reg.udral.physics.kinematics.cartesian.PoseVar.0.1`
///
/// Always aligned,
/// size 656 bits
pub pose: crate::reg::udral::physics::kinematics::cartesian::pose_var_0_1::PoseVar,
///
/// `reg.udral.physics.kinematics.cartesian.TwistVar.0.1`
///
/// Always aligned,
/// size 528 bits
pub twist: crate::reg::udral::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                        impl ::canadensis_encoding::DataType for StateVar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.StateVarTs.0.1`
                        ///
                        /// Fixed size 155 bytes
                        ///
                        pub struct StateVarTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.kinematics.cartesian.StateVar.0.1`
///
/// Always aligned,
/// size 1184 bits
pub value: crate::reg::udral::physics::kinematics::cartesian::state_var_0_1::StateVar,
}
                        impl ::canadensis_encoding::DataType for StateVarTs {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.Twist.0.1`
                        ///
                        /// Fixed size 24 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Motion of a rigid body in 3D space defined in the body frame."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Twist {
                            #[cfg_attr(not(doctest), doc = " Linear velocity in the body frame.")]
                            ///
                            /// `uavcan.si.unit.velocity.Vector3.1.0`
                            ///
                            /// Always aligned,
                            /// size 96 bits
                            pub linear: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                            #[cfg_attr(
                                not(doctest),
                                doc = " Angular velocity about the fixed axes of the body frame (extrinsic)."
                            )]
                            ///
                            /// `uavcan.si.unit.angular_velocity.Vector3.1.0`
                            ///
                            /// Always aligned,
                            /// size 96 bits
                            pub angular:
                                crate::uavcan::si::unit::angular_velocity::vector3_1_0::Vector3,
                        }
                        impl ::canadensis_encoding::DataType for Twist {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.TwistVar.0.1`
                        ///
                        /// Fixed size 66 bytes
                        ///
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct TwistVar {
                            ///
                            /// `reg.udral.physics.kinematics.cartesian.Twist.0.1`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub value:
                                crate::reg::udral::physics::kinematics::cartesian::twist_0_1::Twist,
                            #[cfg_attr(
                                not(doctest),
                                doc = " Upper-right triangle of the covariance matrix:\n\n                     translation along axis    rotation about axis\n                        X       Y       Z       X       Y       Z\n                     +----------------------------------------------\n X velocity          |\n Y velocity          |      (m/s)^2                (m*rad)/s^2\n Z velocity          |\n X angular velocity  |\n Y angular velocity  |                              (rad/s)^2\n Z angular velocity  |"
                            )]
                            ///
                            /// `saturated float16[21]`
                            ///
                            /// Always aligned,
                            /// size 336 bits
                            pub covariance_urt: [::half::f16; 21],
                        }
                        impl ::canadensis_encoding::DataType for TwistVar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.cartesian.TwistVarTs.0.1`
                        ///
                        /// Fixed size 73 bytes
                        ///
                        pub struct TwistVarTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.kinematics.cartesian.TwistVar.0.1`
///
/// Always aligned,
/// size 528 bits
pub value: crate::reg::udral::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                        impl ::canadensis_encoding::DataType for TwistVarTs {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.Point.0.1`
                        ///
                        /// Fixed size 24 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Geodetic position: latitude, longitude, and altitude.\n The order is chosen to match the axis ordering of the NED frame.\n The size and layout of this structure is equal to the Cartesian pose type."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Point {
                            #[cfg_attr(not(doctest), doc = " [radian]")]
                            ///
                            /// `saturated float64`
                            ///
                            /// Always aligned,
                            /// size 64 bits
                            pub latitude: f64,
                            #[cfg_attr(not(doctest), doc = " [radian]")]
                            ///
                            /// `saturated float64`
                            ///
                            /// Always aligned,
                            /// size 64 bits
                            pub longitude: f64,
                            #[cfg_attr(
                                not(doctest),
                                doc = " Distance between the local mean sea level (MSL) and the focal point of the antenna. Positive altitude above the MSL."
                            )]
                            ///
                            /// `uavcan.si.unit.length.WideScalar.1.0`
                            ///
                            /// Always aligned,
                            /// size 64 bits
                            pub altitude:
                                crate::uavcan::si::unit::length::wide_scalar_1_0::WideScalar,
                        }
                        impl ::canadensis_encoding::DataType for Point {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.PointState.0.1`
                        ///
                        /// Fixed size 36 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " The kinematic state of a point, as opposed to that of a body, is devoid of rotation information.\n Therefore, the velocity is specified in the parent coordinate frame."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointState {
                            ///
                            /// `reg.udral.physics.kinematics.geodetic.Point.0.1`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub position:
                                crate::reg::udral::physics::kinematics::geodetic::point_0_1::Point,
                            ///
                            /// `uavcan.si.unit.velocity.Vector3.1.0`
                            ///
                            /// Always aligned,
                            /// size 96 bits
                            pub velocity: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                        }
                        impl ::canadensis_encoding::DataType for PointState {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.PointStateVar.0.1`
                        ///
                        /// Fixed size 60 bytes
                        ///
                        #[cfg_attr(not(doctest), doc = " See PointState for details.")]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointStateVar {
///
/// `reg.udral.physics.kinematics.geodetic.PointVar.0.1`
///
/// Always aligned,
/// size 288 bits
pub position: crate::reg::udral::physics::kinematics::geodetic::point_var_0_1::PointVar,
///
/// `reg.udral.physics.kinematics.translation.Velocity3Var.0.2`
///
/// Always aligned,
/// size 192 bits
pub velocity: crate::reg::udral::physics::kinematics::translation::velocity3_var_0_2::Velocity3Var,
}
                        impl ::canadensis_encoding::DataType for PointStateVar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.PointStateVarTs.0.1`
                        ///
                        /// Fixed size 67 bytes
                        ///
                        pub struct PointStateVarTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.kinematics.geodetic.PointStateVar.0.1`
///
/// Always aligned,
/// size 480 bits
pub value: crate::reg::udral::physics::kinematics::geodetic::point_state_var_0_1::PointStateVar,
}
                        impl ::canadensis_encoding::DataType for PointStateVarTs {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.PointVar.0.1`
                        ///
                        /// Fixed size 36 bytes
                        ///
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PointVar {
                            ///
                            /// `reg.udral.physics.kinematics.geodetic.Point.0.1`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub value:
                                crate::reg::udral::physics::kinematics::geodetic::point_0_1::Point,
                            #[cfg_attr(
                                not(doctest),
                                doc = " [meter^2]\n Upper-right triangle of the covariance matrix.\n The position covariance is defined relative to a tangential plane through the specified latitude/longitude.\n Element ordering: latitude, longitude, altitude. It is chosen to match the axis ordering of the NED frame."
                            )]
                            ///
                            /// `saturated float16[6]`
                            ///
                            /// Always aligned,
                            /// size 96 bits
                            pub covariance_urt: [::half::f16; 6],
                        }
                        impl ::canadensis_encoding::DataType for PointVar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.Pose.0.1`
                        ///
                        /// Fixed size 40 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Zero rotation is the state where the axes of the body frame are aligned with the axes of the local NED frame:\n X points north, Y points east, Z points down."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Pose {
                            ///
                            /// `reg.udral.physics.kinematics.geodetic.Point.0.1`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub position:
                                crate::reg::udral::physics::kinematics::geodetic::point_0_1::Point,
                            ///
                            /// `uavcan.si.unit.angle.Quaternion.1.0`
                            ///
                            /// Always aligned,
                            /// size 128 bits
                            pub orientation:
                                crate::uavcan::si::unit::angle::quaternion_1_0::Quaternion,
                        }
                        impl ::canadensis_encoding::DataType for Pose {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.PoseVar.0.1`
                        ///
                        /// Fixed size 82 bytes
                        ///
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct PoseVar {
                            ///
                            /// `reg.udral.physics.kinematics.geodetic.Pose.0.1`
                            ///
                            /// Always aligned,
                            /// size 320 bits
                            pub value:
                                crate::reg::udral::physics::kinematics::geodetic::pose_0_1::Pose,
                            #[cfg_attr(
                                not(doctest),
                                doc = " Upper-right triangle of the covariance matrix:\n\n                    [parent frame]        [child (body) frame]\n               translation along axis     rotation about axis\n                   X       Y       Z       X       Y       Z\n               +-----------------------------------------------\n X position    |\n Y position    |          m^2                     m*rad\n Z position    |\n X rotation    |\n Y rotation    |                                  rad^2\n Z rotation    |"
                            )]
                            ///
                            /// `saturated float16[21]`
                            ///
                            /// Always aligned,
                            /// size 336 bits
                            pub covariance_urt: [::half::f16; 21],
                        }
                        impl ::canadensis_encoding::DataType for PoseVar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.State.0.1`
                        ///
                        /// Fixed size 64 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " First-order kinematic state of a body near the surface of a planet.\n The pose defines a coordinate system transformation from the parent frame to the child frame.\n The twist is specified in the child frame (body frame)."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct State {
                            ///
                            /// `reg.udral.physics.kinematics.geodetic.Pose.0.1`
                            ///
                            /// Always aligned,
                            /// size 320 bits
                            pub pose:
                                crate::reg::udral::physics::kinematics::geodetic::pose_0_1::Pose,
                            ///
                            /// `reg.udral.physics.kinematics.cartesian.Twist.0.1`
                            ///
                            /// Always aligned,
                            /// size 192 bits
                            pub twist:
                                crate::reg::udral::physics::kinematics::cartesian::twist_0_1::Twist,
                        }
                        impl ::canadensis_encoding::DataType for State {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.StateVar.0.1`
                        ///
                        /// Fixed size 148 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " See State for details. This type extends it with covariance matrices."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct StateVar {
///
/// `reg.udral.physics.kinematics.geodetic.PoseVar.0.1`
///
/// Always aligned,
/// size 656 bits
pub pose: crate::reg::udral::physics::kinematics::geodetic::pose_var_0_1::PoseVar,
///
/// `reg.udral.physics.kinematics.cartesian.TwistVar.0.1`
///
/// Always aligned,
/// size 528 bits
pub twist: crate::reg::udral::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                        impl ::canadensis_encoding::DataType for StateVar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.geodetic.StateVarTs.0.1`
                        ///
                        /// Fixed size 155 bytes
                        ///
                        pub struct StateVarTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.kinematics.geodetic.StateVar.0.1`
///
/// Always aligned,
/// size 1184 bits
pub value: crate::reg::udral::physics::kinematics::geodetic::state_var_0_1::StateVar,
}
                        impl ::canadensis_encoding::DataType for StateVarTs {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.rotation.Planar.0.1`
                        ///
                        /// Fixed size 12 bytes
                        ///
                        #[cfg_attr(not(doctest), doc = " Rotation about an axis.")]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Planar {
                            ///
                            /// `uavcan.si.unit.angle.Scalar.1.0`
                            ///
                            /// Always aligned,
                            /// size 32 bits
                            pub angular_position:
                                crate::uavcan::si::unit::angle::scalar_1_0::Scalar,
                            ///
                            /// `uavcan.si.unit.angular_velocity.Scalar.1.0`
                            ///
                            /// Always aligned,
                            /// size 32 bits
                            pub angular_velocity:
                                crate::uavcan::si::unit::angular_velocity::scalar_1_0::Scalar,
                            ///
                            /// `uavcan.si.unit.angular_acceleration.Scalar.1.0`
                            ///
                            /// Always aligned,
                            /// size 32 bits
                            pub angular_acceleration:
                                crate::uavcan::si::unit::angular_acceleration::scalar_1_0::Scalar,
                        }
                        impl ::canadensis_encoding::DataType for Planar {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.rotation.PlanarTs.0.1`
                        ///
                        /// Fixed size 19 bytes
                        ///
                        pub struct PlanarTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.kinematics.rotation.Planar.0.1`
///
/// Always aligned,
/// size 96 bits
pub value: crate::reg::udral::physics::kinematics::rotation::planar_0_1::Planar,
}
                        impl ::canadensis_encoding::DataType for PlanarTs {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.translation.Linear.0.1`
                        ///
                        /// Fixed size 12 bytes
                        ///
                        #[cfg_attr(not(doctest), doc = " Movement along an axis.")]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Linear {
                            ///
                            /// `uavcan.si.unit.length.Scalar.1.0`
                            ///
                            /// Always aligned,
                            /// size 32 bits
                            pub position: crate::uavcan::si::unit::length::scalar_1_0::Scalar,
                            ///
                            /// `uavcan.si.unit.velocity.Scalar.1.0`
                            ///
                            /// Always aligned,
                            /// size 32 bits
                            pub velocity: crate::uavcan::si::unit::velocity::scalar_1_0::Scalar,
                            ///
                            /// `uavcan.si.unit.acceleration.Scalar.1.0`
                            ///
                            /// Always aligned,
                            /// size 32 bits
                            pub acceleration:
                                crate::uavcan::si::unit::acceleration::scalar_1_0::Scalar,
                        }
                        impl ::canadensis_encoding::DataType for Linear {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.translation.LinearTs.0.1`
                        ///
                        /// Fixed size 19 bytes
                        ///
                        pub struct LinearTs {
///
/// `uavcan.time.SynchronizedTimestamp.1.0`
///
/// Always aligned,
/// size 56 bits
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
///
/// `reg.udral.physics.kinematics.translation.Linear.0.1`
///
/// Always aligned,
/// size 96 bits
pub value: crate::reg::udral::physics::kinematics::translation::linear_0_1::Linear,
}
                        impl ::canadensis_encoding::DataType for LinearTs {
                            /// This type is sealed.
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
                        /// `reg.udral.physics.kinematics.translation.LinearVarTs.0.1`
                        ///
                        /// Fixed size 25 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " This is a structural subtype of LinearTs.\n Use best guess if the error variance is unknown."
                        )]
                        pub struct LinearVarTs {
///
/// `reg.udral.physics.kinematics.translation.LinearTs.0.1`
///
/// Always aligned,
/// size 152 bits
pub value: crate::reg::udral::physics::kinematics::translation::linear_ts_0_1::LinearTs,
#[cfg_attr(not(doctest), doc = " [meter^2]")]
///
/// `saturated float16`
///
/// Always aligned,
/// size 16 bits
pub position_error_variance: ::half::f16,
#[cfg_attr(not(doctest), doc = " [(meter/second)^2]")]
///
/// `saturated float16`
///
/// Always aligned,
/// size 16 bits
pub velocity_error_variance: ::half::f16,
#[cfg_attr(not(doctest), doc = " [(meter/second^2)^2]")]
///
/// `saturated float16`
///
/// Always aligned,
/// size 16 bits
pub acceleration_error_variance: ::half::f16,
}
                        impl ::canadensis_encoding::DataType for LinearVarTs {
                            /// This type is sealed.
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
                                cursor.write_f16(self.position_error_variance);
                                cursor.write_f16(self.velocity_error_variance);
                                cursor.write_f16(self.acceleration_error_variance);
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
                                    position_error_variance: { cursor.read_f16() },
                                    velocity_error_variance: { cursor.read_f16() },
                                    acceleration_error_variance: { cursor.read_f16() },
                                })
                            }
                        }
                    }
                    pub mod velocity1_var_ts_0_1 {
                        /// `reg.udral.physics.kinematics.translation.Velocity1VarTs.0.1`
                        ///
                        /// Fixed size 13 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Linear velocity with timestamp and covariance.\n Observe that this is a structural subtype of uavcan.si.sample.velocity.Scalar.1.0.\n For a non-timestamped estimate without covariance use the raw SI type directly."
                        )]
                        pub struct Velocity1VarTs {
                            ///
                            /// `uavcan.si.sample.velocity.Scalar.1.0`
                            ///
                            /// Always aligned,
                            /// size 88 bits
                            pub value: crate::uavcan::si::sample::velocity::scalar_1_0::Scalar,
                            #[cfg_attr(not(doctest), doc = " [(meter/second)^2]")]
                            ///
                            /// `saturated float16`
                            ///
                            /// Always aligned,
                            /// size 16 bits
                            pub error_variance: ::half::f16,
                        }
                        impl ::canadensis_encoding::DataType for Velocity1VarTs {
                            /// This type is sealed.
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
                                cursor.write_f16(self.error_variance);
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
                                    error_variance: { cursor.read_f16() },
                                })
                            }
                        }
                    }
                    #[allow(deprecated)]
                    #[deprecated]
                    pub mod velocity3_var_0_1 {
                        /// `reg.udral.physics.kinematics.translation.Velocity3Var.0.1`
                        ///
                        /// Fixed size 31 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Linear velocity with covariance.\n Observe that this is a structural subtype of uavcan.si.unit.velocity.Scalar.1.0."
                        )]
                        #[deprecated]
                        pub struct Velocity3Var {
                            ///
                            /// `uavcan.si.sample.velocity.Vector3.1.0`
                            ///
                            /// Always aligned,
                            /// size 152 bits
                            pub value: crate::uavcan::si::sample::velocity::vector3_1_0::Vector3,
                            #[cfg_attr(
                                not(doctest),
                                doc = " [(meter/second)^2] Upper-right triangle of the covariance matrix."
                            )]
                            ///
                            /// `saturated float16[6]`
                            ///
                            /// Always aligned,
                            /// size 96 bits
                            pub covariance_urt: [::half::f16; 6],
                        }
                        impl ::canadensis_encoding::DataType for Velocity3Var {
                            /// This type is sealed.
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
                                    cursor.write_f16(*value);
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
                                            cursor.read_f16(),
                                            cursor.read_f16(),
                                            cursor.read_f16(),
                                            cursor.read_f16(),
                                            cursor.read_f16(),
                                            cursor.read_f16(),
                                        ]
                                    },
                                })
                            }
                        }
                    }
                    pub mod velocity3_var_0_2 {
                        /// `reg.udral.physics.kinematics.translation.Velocity3Var.0.2`
                        ///
                        /// Fixed size 24 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Linear velocity with covariance.\n Observe that this is a structural subtype of uavcan.si.unit.velocity.Scalar.1.0."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct Velocity3Var {
                            ///
                            /// `uavcan.si.unit.velocity.Vector3.1.0`
                            ///
                            /// Always aligned,
                            /// size 96 bits
                            pub value: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                            #[cfg_attr(
                                not(doctest),
                                doc = " [(meter/second)^2] Upper-right triangle of the covariance matrix."
                            )]
                            ///
                            /// `saturated float16[6]`
                            ///
                            /// Always aligned,
                            /// size 96 bits
                            pub covariance_urt: [::half::f16; 6],
                        }
                        impl ::canadensis_encoding::DataType for Velocity3Var {
                            /// This type is sealed.
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
                    /// `reg.udral.physics.optics.HighColor.0.1`
                    ///
                    /// Fixed size 2 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Color in the standard 16-bit 5-6-5 RGB format (green is wider due to non-uniform color sensitivity of the human eye).\n https://en.wikipedia.org/wiki/High_color\n\n For reasons of unification, a monochrome light can be modeled using the same type,\n where the brightness is defined as the mean of the color components normalized to one:\n\n   brightness = (red/MAX_RED + green/MAX_GREEN + blue/MAX_BLUE) / 3"
                    )]
                    pub struct HighColor {
                        ///
                        /// `saturated uint5`
                        ///
                        /// Always aligned,
                        /// size 5 bits
                        pub red: u8,
                        ///
                        /// `saturated uint6`
                        ///
                        /// Not always aligned,
                        /// size 6 bits
                        pub green: u8,
                        ///
                        /// `saturated uint5`
                        ///
                        /// Not always aligned,
                        /// size 5 bits
                        pub blue: u8,
                    }
                    impl ::canadensis_encoding::DataType for HighColor {
                        /// This type is sealed.
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for HighColor {}
                    impl HighColor {
                        pub const MAX_RED: u8 = 31;
                        pub const MAX_GREEN: u8 = 63;
                        pub const MAX_BLUE: u8 = 31;
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
                    /// `reg.udral.physics.thermodynamics.PressureTempVarTs.0.1`
                    ///
                    /// Fixed size 21 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Timestamped fluid pressure and temperature (sampled synchronously) with covariance.\n Observe that this is a structural subtype of uavcan.si.sample.pressure.Scalar.1.0."
                    )]
                    pub struct PressureTempVarTs {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `uavcan.si.unit.pressure.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub pressure: crate::uavcan::si::unit::pressure::scalar_1_0::Scalar,
                        ///
                        /// `uavcan.si.unit.temperature.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub temperature: crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The upper-right triangle of the covariance matrix (following the matrix packing rules defined in Specification).\n   0 -- pascal^2\n   1 -- pascal*kelvin\n   2 -- kelvin^2"
                        )]
                        ///
                        /// `saturated float16[3]`
                        ///
                        /// Always aligned,
                        /// size 48 bits
                        pub covariance_urt: [::half::f16; 3],
                    }
                    impl ::canadensis_encoding::DataType for PressureTempVarTs {
                        /// This type is sealed.
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
                                cursor.write_f16(*value);
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
                                    [cursor.read_f16(), cursor.read_f16(), cursor.read_f16()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod time {
                pub mod tai64_0_1 {
                    /// `reg.udral.physics.time.TAI64.0.1`
                    ///
                    /// Fixed size 8 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Standard TAI64N time label (https://cr.yp.to/libtai/tai64.html). Quote from the source:\n\n   TAI stands for Temps Atomique International, the current international real-time standard.\n   One TAI second is defined as the duration of 9192631770 periods of the radiation corresponding\n   to the transition between the two hyperfine levels of the ground state of the cesium atom.\n   TAI also specifies a frame of reference. Further discussion of special relativity is outside\n   the scope of this document.\n\n   A TAI64 label is an integer between 0 and 2^64 referring to a particular second of real time. Integer s refers to:\n\n       - the TAI second beginning exactly 2^62 - s seconds before the beginning of 1970 TAI,\n         if s is between 0 inclusive and 2^62 exclusive; or\n\n       - the TAI second beginning exactly s - 2^62 seconds after the beginning of 1970 TAI,\n         if s is between 2^62 inclusive and 2^63 exclusive.\n"
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct TAI64 {
                        #[cfg_attr(
                            not(doctest),
                            doc = " [nanosecond] Nanoseconds elapsed since 1970-01-01T00:00:00Z TAI."
                        )]
                        ///
                        /// `saturated int64`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub tai64n: i64,
                    }
                    impl ::canadensis_encoding::DataType for TAI64 {
                        /// This type is sealed.
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
                    /// `reg.udral.physics.time.TAI64Var.0.1`
                    ///
                    /// Fixed size 12 bytes
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct TAI64Var {
                        ///
                        /// `reg.udral.physics.time.TAI64.0.1`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub value: crate::reg::udral::physics::time::tai64_0_1::TAI64,
                        #[cfg_attr(
                            not(doctest),
                            doc = " [second^2]\n Error variance, in second squared, of the time estimate.\n Infinity indicates that the time estimates are not yet available.\n A non-positive value indicates that the error variance is unknown."
                        )]
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub error_variance: f32,
                    }
                    impl ::canadensis_encoding::DataType for TAI64Var {
                        /// This type is sealed.
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
                    /// `reg.udral.physics.time.TAI64VarTs.0.1`
                    ///
                    /// Fixed size 19 bytes
                    ///
                    pub struct TAI64VarTs {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `reg.udral.physics.time.TAI64Var.0.1`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub value: crate::reg::udral::physics::time::tai64_var_0_1::TAI64Var,
                    }
                    impl ::canadensis_encoding::DataType for TAI64VarTs {
                        /// This type is sealed.
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
                        /// `reg.udral.service.actuator.common._.0.1`
                        ///
                        /// Fixed size 0 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " An actuator is a device that actuates a mechanical load using electric energy from the high-voltage DC power bus.\n There are multiple kinds of actuators with a dedicated namespace for each; additionally, this \"common\" namespace\n hosts certain elements shared between several (or all) kinds."
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct _0 {}
                        impl ::canadensis_encoding::DataType for _0 {
                            /// This type is delimited with an extent of 0 bytes.
                            const EXTENT_BYTES: Option<u32> = Some(0);
                        }
                        impl ::canadensis_encoding::Message for _0 {}
                        impl _0 {
                            #[cfg_attr(
                                not(doctest),
                                doc = " [seconds]\n The actuator is allowed to enter a safe state (e.g., stop the controlled mechanism, disconnect itself from\n the load, etc. depending on the specifics of the application at hand) if no setpoint or readiness control messages\n have been received in this amount of time. Implementations are allowed to reduce this value, but never increase it."
                            )]
                            pub const CONTROL_TIMEOUT: f32 = 1_f32;
                            #[cfg_attr(
                                not(doctest),
                                doc = " [second]\n This is the maximum publication period (minimum frequency) for all subjects described in this service.\n Subjects that are clocked by the setpoint should continue being published at least at this rate when setpoint is not\n being updated, unless the actuator is in the SLEEP state.\n The publication periods should be consistent across the group."
                            )]
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
                        /// `reg.udral.service.actuator.common.FaultFlags.0.1`
                        ///
                        /// Fixed size 2 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " A collection of detailed fault flags indicating problems detected by the service provider.\n A fault flag is set when the corresponding parameter exceeds its safe operating area (SOA) as defined by the vendor;\n see https://en.wikipedia.org/wiki/Safe_operating_area.\n As long as at least one flag is set, the service health should not be NOMINAL."
                        )]
                        pub struct FaultFlags {
                            #[cfg_attr(
                                not(doctest),
                                doc = " The load is above SOA or regeneration below the SOA."
                            )]
                            ///
                            /// `saturated bool`
                            ///
                            /// Always aligned,
                            /// size 1 bits
                            pub overload: bool,
                            #[cfg_attr(
                                not(doctest),
                                doc = " Supply voltage is above or below the SOA."
                            )]
                            ///
                            /// `saturated bool`
                            ///
                            /// Not always aligned,
                            /// size 1 bits
                            pub voltage: bool,
                            ///
                            /// `saturated bool`
                            ///
                            /// Not always aligned,
                            /// size 1 bits
                            pub motor_temperature: bool,
                            #[cfg_attr(
                                not(doctest),
                                doc = " Temperature is above or below the SOA."
                            )]
                            ///
                            /// `saturated bool`
                            ///
                            /// Not always aligned,
                            /// size 1 bits
                            pub controller_temperature: bool,
                            #[cfg_attr(
                                not(doctest),
                                doc = " The absolute velocity of the load is above the SOA."
                            )]
                            ///
                            /// `saturated bool`
                            ///
                            /// Not always aligned,
                            /// size 1 bits
                            pub velocity: bool,
                            #[cfg_attr(
                                not(doctest),
                                doc = " The load cannot be driven due to a mechanical failure."
                            )]
                            ///
                            /// `saturated bool`
                            ///
                            /// Not always aligned,
                            /// size 1 bits
                            pub mechanical: bool,
                            #[cfg_attr(
                                not(doctest),
                                doc = " The mechanical vibration level exceeds the SOA."
                            )]
                            ///
                            /// `saturated bool`
                            ///
                            /// Not always aligned,
                            /// size 1 bits
                            pub vibration: bool,
                            #[cfg_attr(
                                not(doctest),
                                doc = " Configuration is missing or invalid."
                            )]
                            ///
                            /// `saturated bool`
                            ///
                            /// Not always aligned,
                            /// size 1 bits
                            pub configuration: bool,
                            #[cfg_attr(
                                not(doctest),
                                doc = " The requested control mode is not supported by the actuator."
                            )]
                            ///
                            /// `saturated bool`
                            ///
                            /// Always aligned,
                            /// size 1 bits
                            pub control_mode: bool,
                            // 6 bits of padding
                            #[cfg_attr(
                                not(doctest),
                                doc = " None of the above (vendor-specific)."
                            )]
                            ///
                            /// `saturated bool`
                            ///
                            /// Not always aligned,
                            /// size 1 bits
                            pub other: bool,
                        }
                        impl ::canadensis_encoding::DataType for FaultFlags {
                            /// This type is sealed.
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
                        /// `reg.udral.service.actuator.common.Feedback.0.1`
                        ///
                        /// Fixed size 3 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " This high-rate feedback should be published once immediately after a setpoint is applied.\n It follows that the publication rate of these messages equals that of the setpoint messages.\n When setpoint messages are not being emitted, the publication rate is implementation-defined, but it should not\n be lower than the defined limit.\n The priority of this message should be the same as that of the corresponding setpoint message."
                        )]
                        pub struct Feedback {
                            #[cfg_attr(
                                not(doctest),
                                doc = " If ENGAGED, the actuator provides service according to its nominal performance characteristics.\n Otherwise, no availability guarantees are provided.\n Notice that the feedback type is a structural subtype of the heartbeat type, so one can subscribe to a\n feedback subject using the heartbeat type. Similarly, the heartbeat type is a structural subtype of the\n Readiness type, meaning that one can use the Readiness type as well."
                            )]
                            ///
                            /// `reg.udral.service.common.Heartbeat.0.1`
                            ///
                            /// Always aligned,
                            /// size 16 bits
                            pub heartbeat:
                                crate::reg::udral::service::common::heartbeat_0_1::Heartbeat,
                            #[cfg_attr(
                                not(doctest),
                                doc = " [percent]\n Percentage of the maximum rated output intensity. May exceed +-100% in case of overload.\n Positive value indicates that power is applied to the load; negative indicates that power is being sunk from the\n load into the actuator power source.\n The consumer of this message may leverage this information to manage the control loop saturation."
                            )]
                            ///
                            /// `saturated int8`
                            ///
                            /// Always aligned,
                            /// size 8 bits
                            pub demand_factor_pct: i8,
                        }
                        impl ::canadensis_encoding::DataType for Feedback {
                            /// This type is delimited with an extent of 63 bytes.
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
                            /// `reg.udral.service.actuator.common.sp._.0.1`
                            ///
                            /// Fixed size 0 bytes
                            ///
                            #[cfg_attr(
                                not(doctest),
                                doc = " This is a collection of weakly-typed primitives used to control groups of actuators synchronously.\n\n Actuators are expected to subscribe using the largest array type. Publishers would choose the array type\n depending on the number of actuators in the group. The actuators would be expecting the largest array type,\n where the missing elements will be zero-filled automatically by the protocol stack thanks to the\n Implicit Zero Extension Rule (refer to the Cyphal Specification for details).\n\n The physical meaning of the values contained in the array is defined by the respective actuator service specification.\n If ratiometric control is used, then the range should be [-1, +1].\n\n It follows that a standalone actuator (that is not a member of any group) is just a special case of a group of 1,\n where the setpoint type is a single scalar.\n\n The Cyphal Specification might benefit from supporting flexible array fields to avoid having to deal with redundant\n similar types: https://en.wikipedia.org/wiki/Flexible_array_member, so that instead of having multiple types that\n differ only in size of the array fields, one could just say `float16[0] value` such that the size of zero indicates\n that the array is a flex array."
                            )]
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct _0 {}
                            impl ::canadensis_encoding::DataType for _0 {
                                /// This type is delimited with an extent of 0 bytes.
                                const EXTENT_BYTES: Option<u32> = Some(0);
                            }
                            impl ::canadensis_encoding::Message for _0 {}
                            impl _0 {
                                #[cfg_attr(
                                    not(doctest),
                                    doc = " The float epsilon defined for convenience.\n See https://en.wikipedia.org/wiki/Machine_epsilon."
                                )]
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
                            /// `reg.udral.service.actuator.common.sp.Scalar.0.1`
                            ///
                            /// Fixed size 2 bytes
                            ///
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Scalar {
                                ///
                                /// `saturated float16`
                                ///
                                /// Always aligned,
                                /// size 16 bits
                                pub value: ::half::f16,
                            }
                            impl ::canadensis_encoding::DataType for Scalar {
                                /// This type is delimited with an extent of 512 bytes.
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
                            /// `reg.udral.service.actuator.common.sp.Vector2.0.1`
                            ///
                            /// Fixed size 4 bytes
                            ///
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector2 {
                                ///
                                /// `saturated float16[2]`
                                ///
                                /// Always aligned,
                                /// size 32 bits
                                pub value: [::half::f16; 2],
                            }
                            impl ::canadensis_encoding::DataType for Vector2 {
                                /// This type is delimited with an extent of 512 bytes.
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
                            /// `reg.udral.service.actuator.common.sp.Vector31.0.1`
                            ///
                            /// Fixed size 62 bytes
                            ///
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector31 {
                                ///
                                /// `saturated float16[31]`
                                ///
                                /// Always aligned,
                                /// size 496 bits
                                pub value: [::half::f16; 31],
                            }
                            impl ::canadensis_encoding::DataType for Vector31 {
                                /// This type is delimited with an extent of 512 bytes.
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
                            /// `reg.udral.service.actuator.common.sp.Vector3.0.1`
                            ///
                            /// Fixed size 6 bytes
                            ///
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector3 {
                                ///
                                /// `saturated float16[3]`
                                ///
                                /// Always aligned,
                                /// size 48 bits
                                pub value: [::half::f16; 3],
                            }
                            impl ::canadensis_encoding::DataType for Vector3 {
                                /// This type is delimited with an extent of 512 bytes.
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
                            /// `reg.udral.service.actuator.common.sp.Vector4.0.1`
                            ///
                            /// Fixed size 8 bytes
                            ///
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector4 {
                                ///
                                /// `saturated float16[4]`
                                ///
                                /// Always aligned,
                                /// size 64 bits
                                pub value: [::half::f16; 4],
                            }
                            impl ::canadensis_encoding::DataType for Vector4 {
                                /// This type is delimited with an extent of 512 bytes.
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
                            /// `reg.udral.service.actuator.common.sp.Vector6.0.1`
                            ///
                            /// Fixed size 12 bytes
                            ///
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector6 {
                                ///
                                /// `saturated float16[6]`
                                ///
                                /// Always aligned,
                                /// size 96 bits
                                pub value: [::half::f16; 6],
                            }
                            impl ::canadensis_encoding::DataType for Vector6 {
                                /// This type is delimited with an extent of 512 bytes.
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
                            /// `reg.udral.service.actuator.common.sp.Vector8.0.1`
                            ///
                            /// Fixed size 16 bytes
                            ///
                            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                            #[repr(C, packed)]
                            pub struct Vector8 {
                                ///
                                /// `saturated float16[8]`
                                ///
                                /// Always aligned,
                                /// size 128 bits
                                pub value: [::half::f16; 8],
                            }
                            impl ::canadensis_encoding::DataType for Vector8 {
                                /// This type is delimited with an extent of 512 bytes.
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
                        /// `reg.udral.service.actuator.common.Status.0.1`
                        ///
                        /// Fixed size 14 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " Auxiliary actuator status information published at a low rate asynchronously, usually at 1 Hz.\n It is mostly intended for diagnostics and logging purposes.\n In this revision this type is common for all kinds of actuators, but in the future it may be replaced\n with per-kind specializations."
                        )]
                        pub struct Status {
///
/// `uavcan.si.unit.temperature.Scalar.1.0`
///
/// Always aligned,
/// size 32 bits
pub motor_temperature: crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
#[cfg_attr(not(doctest), doc = " Sampled temperatures. If multiple values are available, reduction is implementation-defined.")]
///
/// `uavcan.si.unit.temperature.Scalar.1.0`
///
/// Always aligned,
/// size 32 bits
pub controller_temperature: crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
#[cfg_attr(not(doctest), doc = " Incremented once per occurrence. Reset to zero when ENGAGED.\n The exact definition of what constitutes an error is implementation-dependent.")]
///
/// `saturated uint32`
///
/// Always aligned,
/// size 32 bits
pub error_count: u32,
#[cfg_attr(not(doctest), doc = " TODO: add vibration")]
///
/// `reg.udral.service.actuator.common.FaultFlags.0.1`
///
/// Always aligned,
/// size 16 bits
pub fault_flags: crate::reg::udral::service::actuator::common::fault_flags_0_1::FaultFlags,
}
                        impl ::canadensis_encoding::DataType for Status {
                            /// This type is delimited with an extent of 63 bytes.
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
                        /// `reg.udral.service.actuator.esc._.0.1`
                        ///
                        /// Fixed size 0 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " The electronic speed controller (ESC) service is designed for controlling and monitoring electric drives.\n From the standpoint of this standard, an electric drive is just a special case of a servo. For generality,\n COTS electric drives are recommended to also support the servo interface defined in the adjacent namespace.\n\n ESCs (drives) are segregated into groups. Each ESC in a group has an index that is unique within the group.\n Drives in a group are commanded synchronously by publishing a message containing an array of setpoints.\n There are several subjects defined:\n\n   - Setpoint array subject. Every participant subscribes to the same setpoint subject.\n     Every message is consumed by all participants according to their index in the group.\n     The setpoint subject defines the group. There may be an arbitrary number of such groups in the network.\n\n   - Readiness subject. Every participant subscribes to the same readiness control subject which is used to command\n     the state of the group: sleep, standby, or engaged. In many cases there will be one global subject controlling\n     the state of the entire system; in other cases there will be dedicated controls on a per-subsystem basis.\n\n   - Feedback subjects published by each ESC separately, as shown on the diagram below.\n\n                                                   SUBJECT NAME        SUBJECT TYPE\n  +----------------+\n  |   Controller   |---------+------------+----... setpoint            reg.udral.service.actuator.common.sp.*\n  |                |-------+-)----------+-)----... readiness           reg.udral.service.common.Readiness\n  +----------------+       | |          | |\n   ^ ^ ^ ^  ^ ^ ^ ^        v v          v v\n   | | | |  | | | |   +---------+  +---------+\n   | | | |  | | | |   |Drive i=0|  |Drive i=1| ...\n   | | | |  | | | |   +---------+  +---------+\n   | | | |  | | | |     | | | |      | | | |\n   | | | |  | | | +-----+ | | |      | | | |       feedback            reg.udral.service.actuator.common.Feedback\n   | | | |  | | +---------+ | |      | | | |       status              reg.udral.service.actuator.common.Status\n   | | | |  | +-------------+ |      | | | |       power               reg.udral.physics.electricity.PowerTs\n   | | | |  +-----------------+      | | | |       dynamics            reg.udral.physics.dynamics.rotation.PlanarTs\n   | | | |                           | | | |\n   | | | +---------------------------+ | | |\n   | | +-------------------------------+ | |\n   | +-----------------------------------+ |\n   +---------------------------------------+\n\n Notice that the physics subjects are timestamped.\n\n Vendor/application-specific subjects are not shown here.\n Vendors are encouraged to publish additional data (e.g., temperatures) on separate subjects.\n\n\n   SETPOINT SUBJECT\n\n The setpoint subject is ignored unless the drive is ENGAGED. As long as the drive is not ENGAGED, it shall not apply\n any power to the load excepting non-operational scenarios such as maintenance and diagnostics, which are\n outside of the scope of this service definition. More on readiness and safety in the next section.\n\n Upon reception of a setpoint message, a group participant fetches its setpoint from the array using the array\n element whose index equals the index of the group participant. By virtue of the Implicit Zero Extension Rule,\n if the message is too short, the setpoint will be interpreted as zero.\n\n If no valid setpoint was received in CONTROL_TIMEOUT or a lower implementation-specific value,\n the drive should assume a zero setpoint for safety reasons.\n The minimum setpoint publication period should be at least twice lower than its timeout.\n\n While stopped, the drive may either allow the load to freewheel or it may force it to a particular parking position,\n depending on the implementation requirements. The actual state of the load may be continuously reported using the\n dynamics subject. Notice that per the safety rule introduced earlier, the parking position may be impossile\n to enforce unless the drive is ENGAGED because it may require delivering power to the load.\n\n The setpoint message types that can be used to command a group of drives are defined in\n reg.udral.service.actuator.common.sp; please read the documentation related to that namespace for further information.\n Servo setpoint message types may also be supported on an implementation-specific basis for enhanced interoperability.\n If the group is controlled using different setpoint subjects concurrently, the behavior is implementation-defined.\n\n The following control modes are defined, none of which are mandatory to support.\n The control mode in use is to be specified using the register API.\n This service does not support switching the control mode or setting the motion profile at runtime;\n for that, please refer to the servo service.\n\n   0. Ratiometric voltage control. Each setpoint scalar is a value normalized/saturated in [-1, +1] representing\n      the Q-axis/phase/armature (depending on the type of the drive) voltage as a fraction of the maximum.\n      This control mode emulates the behavior of a typical RCPWM-controlled BLDC drive.\n\n   1. Ratiometric current/torque control. Each setpoint scalar is a value normalized/saturated in [-1, +1] representing\n      the Q-axis/phase/armature (depending on the type of the drive) current as a fraction of the maximum.\n      A negative setpoint during forward rotation (positive during reverse rotation) commands braking.\n\n   2. Speed control. Each setpoint scalar contains the target angular velocity of the load in radian/second.\n\n   -. More control modes may be added later. Which control modes are supported is implementation-defined.\n\n Considerations that apply to all control modes:\n  -  Negative setpoint values represent reversal; a positive setpoint is co-directed with positive rotation/torque.\n  -  If reverse operation is not supported, negative values should be clamped to zero.\n  -  A non-finite setpoint is to be treated as zero.\n\n\n   READINESS SUBJECT\n\n The default state is STANDBY. While in this state, the drive is not allowed to deliver power to the load,\n and the setpoint subject is ignored. The drive shall enter this state automatically if the readiness subject\n is not updated for CONTROL_TIMEOUT.\n\n While the drive is ENGAGED, the setpoint commands are processed normally as described in the adjacent section.\n If the drive does not support bidirectional operation, implementations are recommended to ensure that the load\n is driven at some minimum power level (idling) while the drive is ENGAGED regardless of the commanded setpoint,\n unless such behavior is deemed incompatible with the functional requirements of the controlled drive.\n\n If the selected readiness state is SLEEP, the behavior is implementation-defined. Implementations are recommended to\n power off the high-voltage circuitry and all non-essential components (e.g., LED indication, sensors, etc.)\n to minimize the power consumption.\n\n Implementations are recommended to announce transitions between the readiness states using audiovisual feedback.\n\n The worst-case state transition latency is not defined. The controlling element (that is, the unit that publishes\n to the setpoint and readiness subjects) is expected to monitor the actual readiness status of each component using\n the feedback subject. For example, a sensorless electric motor drive may choose to spool-up before entering the\n ENGAGED state, which would obviously take time; as soon as the spool-up is finished, the drive would switch its\n reported status from STANDBY to ENGAGED, thereby indicating that it is ready for normal operation.\n\n\n   PUBLISHED SUBJECTS\n\n The following subjects shall be published immediately after a new setpoint is applied even if the drive is STANDBY:\n\n   SUBJECT             RECOMMENDED PRIORITY\n   ---------------------------------------------\n   feedback            same as the setpoint\n   power               second to the setpoint\n   dynamics            second to the setpoint\n\n If no setpoint is being published, these subjects should continue being updated at least at 1/MAX_PUBLICATION_PERIOD.\n The publication rate requirements do not apply if the readiness state is SLEEP.\n\n If the setpoint publication rate exceeds 50 Hz, implementations are allowed (but not required) to throttle these\n subjects by dropping some of the messages such that the publication rate of each subject does not exceed 50 Hz.\n Implementations operating over Classic CAN are recommended to do this.\n\n The other subjects may be published at an implementation-defined rate and priority,\n which should be consistent across the group.\n\n Implementations are encouraged to provide additional subjects for enhanced feedback and monitoring.\n\n The measurements carried by the published messages should be low-pass filtered with an adequate cutoff frequency to\n avoid aliasing effects. Implementations should strive to sample all parameters simultaneously.\n\n If a float-typed reported quantity is unknown, the corresponding value should be NaN.\n\n\n   CONVENTIONS AND ASSUMPTIONS\n\n A drive powers a rotary mechanical load that may be connected via a gearbox. It is the responsibility of\n the drive to account for the gear ratio of the gearbox when calculating related parameters such as angular\n velocity or torque.\n\n It is assumed that there is a well-defined direction of rotation that is referred to as forward rotation.\n A positive angular velocity represents forward rotation. Likewise, forward torque is positive.\n\n It is assumed that the drive is powered from a DC electric power supply network. A positive electric current\n represents current flowing from the network into the drive, also referred to as the state of driving/motoring.\n The opposite -- braking/regeneration -- is represented by negative current.\n\n Excepting edge cases and transients, torque and current are generally of the same sign.\n The above is summarized on the following four-quadrant diagram:\n\n                                   +velocity\n                                       ^\n                               braking,| forward,\n                               negative| positive\n                               power   | power\n                            -----------+---------->  +torque/current\n                               reverse,| braking,\n                               positive| negative\n                               power   | power\n"
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct _0 {}
                        impl ::canadensis_encoding::DataType for _0 {
                            /// This type is delimited with an extent of 0 bytes.
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
                        /// `reg.udral.service.actuator.servo._.0.1`
                        ///
                        /// Fixed size 0 bytes
                        ///
                        #[cfg_attr(
                            not(doctest),
                            doc = " A servo can actuate either a translational or rotary load using electric power from the high-voltage DC bus.\n\n The type of load (translational or rotational) dictates which type is used for commanding the setpoint and reporting\n the status:\n   - reg.udral.physics.dynamics.rotation.Planar[Ts]\n   - reg.udral.physics.dynamics.translation.Linear[Ts]\n For generality, either or both of these types are referred to as \"timestamped dynamics\" or \"non-timestamped dynamics\".\n\n The default readiness state is STANDBY. While in this state, the servo is not allowed to apply force to the load,\n and the setpoint subject is ignored. The servo shall enter the STANDBY state automatically if the readiness subject\n is not updated for CONTROL_TIMEOUT.\n\n The subjects defined by this service are shown on the following canvas. Implementers are encouraged to add\n custom subjects with additional data. Notice that the physics subjects are timestamped.\n\n                  SUBJECT NAME                         SUBJECT TYPE                                    RATE\n\n   +------------+ setpoint             +------------+  (non-timestamped dynamics) (see below)          R\n   |            |--------------------->|            |\n   |            | readiness            |            |  reg.udral.service.common.Readiness              any\n   |            |--------------------->|            |\n   |            | feedback             |            |  reg.udral.service.actuator.common.Feedback      R\n   |            |<---------------------|            |\n   | Controller | status               |   Servo    |  reg.udral.service.actuator.common.Status        any\n   |            |<---------------------|            |\n   |            | power                |            |  reg.udral.physics.electricity.PowerTs           R\n   |            |<---------------------|            |\n   |            | dynamics             |            |  (timestamped dynamics)                          R\n   |            |<---------------------|            |\n   +------------+                      +------------+\n\n Should it be necessary to control a group of servos in lockstep, an arbitrary number of them may subscribe\n to the same setpoint subject (their published subjects would be different of course).\n\n If the servo is ENGAGED, setpoint messages are processed as follows: the first field of the kinematic setpoint type\n that contains a finite value is taken as the commanded setpoint. The following non-negative finite fields define\n the motion profile, where negative and non-finite values are ignored.\n\n For example, a translational dynamics message containing the following values:\n   position     = +0.35\n   velocity     = NaN\n   acceleration = NaN\n   force        = 30\n ...is interpreted as follows: position the load at 0.35 meters relative to the neutral, limit the force to 30 newton,\n do not limit the velocity and acceleration. Here is another example:\n   angular position     = NaN\n   angular velocity     = +400\n   angular acceleration = NaN\n   torque               = 50\n which is interpreted as follows: reach the angular velocity of 400 radian/second in the forward direction,\n limit the torque to 50 newton*meters, do not limit the acceleration.\n\n The motion profile parameters that are not supported are to be silently ignored by the servo. If the commanded\n parameter cannot be controlled by the servo, the setpoint is to be ignored. For example, in the second example above,\n if the servo does not support angular velocity control, the setpoint message would be discarded.\n\n The above describes the typical use case where each servo is controlled over a dedicated setpoint\n subject independently (or a group of servos are controlled in lockstep using the same setpoint subject).\n Some applications may require synchronous independent control of multiple servos in a group, similar to ESC.\n To address this, a compliant servo should support another operating mode where the controlled quantity\n (position, velocity, force, etc.) is selected statically along with the motion profile (using the register API),\n and the servo subscribes to the setpoint subject of type \"reg.udral.service.actuator.common.sp.*\".\n Having its index in the group configured statically, the servo fetches the setpoint from the appropriate\n index in the setpoint array.\n The resulting topology closely resembles that of the ESC service:\n\n                                                   SUBJECT NAME            SUBJECT TYPE\n  +----------------+\n  |   Controller   |---------+------------+----... setpoint                reg.udral.service.actuator.common.sp.*\n  |                |-------+-)----------+-)----... readiness               reg.udral.service.common.Readiness\n  +----------------+       | |          | |\n   ^ ^ ^ ^  ^ ^ ^ ^        v v          v v\n   | | | |  | | | |   +---------+  +---------+\n   | | | |  | | | |   |Servo i=0|  |Servo i=1| ...\n   | | | |  | | | |   +---------+  +---------+\n   | | | |  | | | |     | | | |      | | | |\n   | | | |  | | | +-----+ | | |      | | | |       feedback                reg.udral.service.actuator.common.Feedback\n   | | | |  | | +---------+ | |      | | | |       status                  reg.udral.service.actuator.common.Status\n   | | | |  | +-------------+ |      | | | |       power                   reg.udral.physics.electricity.PowerTs\n   | | | |  +-----------------+      | | | |       dynamics                (timestamped dynamics)\n   | | | |                           | | | |\n   | | | +---------------------------+ | | |\n   | | +-------------------------------+ | |\n   | +-----------------------------------+ |\n   +---------------------------------------+\n\n If the selected readiness state is SLEEP, the behavior is implementation-defined. Implementations are recommended to\n power off the high-voltage circuitry and all non-essential components (e.g., LED indication, sensors, etc.)\n to minimize the power consumption. The publication rate requirements do not apply if the state is SLEEP.\n\n The worst-case readiness state transition latency is not defined.\n\n The following subjects shall be published immediately after a new setpoint is applied even if the servo is STANDBY:\n\n   SUBJECT NAME        RECOMMENDED PRIORITY\n   ---------------------------------------------\n   feedback            same as the setpoint\n   power               second to the setpoint\n   dynamics            second to the setpoint\n\n If no setpoint is being published, these subjects should continue being updated at least at 1/MAX_PUBLICATION_PERIOD.\n\n If the setpoint publication rate exceeds 50 Hz, implementations are allowed (but not required) to throttle these\n subjects by dropping some of the messages such that the publication rate of each subject does not exceed 50 Hz.\n Implementations operating over Classic CAN are recommended to do this.\n\n The other subjects may be published at an implementation-defined rate and priority,\n which should be consistent across the group.\n\n The measurements carried by the published messages should be low-pass filtered with an adequate cutoff frequency to\n avoid aliasing effects. Implementations should strive to sample all parameters simultaneously.\n\n It is assumed that the servo is powered from a DC electric power supply network. A positive electric current\n represents current flowing from the DC network into the servo (negative represents regeneration).\n\n Excepting edge cases and transients, torque/force and current are generally of the same sign (barring the difference\n introduced by the power dissipated by the servo itself).\n\n                                   +velocity\n                                       ^\n                               braking,| forward,\n                               negative| positive\n                               power   | power\n                            -----------+---------->  +torque/force/current\n                               reverse,| braking,\n                               positive| negative\n                               power   | power\n\n An example implementation is available at https://github.com/OpenCyphal/demos"
                        )]
                        #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                        #[repr(C, packed)]
                        pub struct _0 {}
                        impl ::canadensis_encoding::DataType for _0 {
                            /// This type is delimited with an extent of 0 bytes.
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
            pub mod battery {
                pub mod _0_1 {
                    /// `reg.udral.service.battery._.0.1`
                    ///
                    /// Fixed size 0 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " This is the smart battery monitoring service. A smart battery is required to publish on the following subjects:\n\n   SUBJECT                         TYPE                                            TYP. RATE [Hz]\n   energy_source                   reg.udral.physics.electricity.SourceTs          1...100\n   status                          reg.udral.service.battery.Status                ~1\n   parameters                      reg.udral.service.battery.Parameters            ~0.2\n\n Observe that only the first subject can be used for estimating the endurance of the power source. The other subjects\n are designed for monitoring, diagnostics, and maintenance.\n\n Optionally, the battery service can subscribe to a readiness control subject (see reg.udral.service.common.Readiness),\n which enables the following two optional capabilities:\n\n   - SLEEP mode: when the readiness subject commands the sleep state, the battery management system may enter a\n     low power consumption state, possibly deactivating some of its capabilities.\n\n   - STANDBY mode: the battery management system may implement additional safety protections that may otherwise\n     interfere with the normal operation of the vehicle. For example, the traction battery may limit the maximum\n     load current and the depth of discharge unless the commanded state is ENGAGED. By doing so, the battery can\n     protect itself and the supplied high-voltage DC network from accidental damage while the vehicle is parked.\n     Limiting the output power or discharge of the traction battery might lead to catastrophic consequences in\n     an aerial vehicle, hence such safety checks are to be disabled once the battery is commanded into the ENGAGED\n     state.\n\n If readiness state selection is not supported, the battery may not subscribe to the readiness control subject,\n in which case it should permanently report its state as ENGAGED unless the battery is unfit for use (e.g., due\n to degradation or a failure).\n\n By convention, positive current flows from the DC network into the battery. Therefore, the current is\n negative when the battery powers the system, and positive when it is being charged.\n\n Systems that leverage multiple battery packs simultaneously should be configured to publish the status of each\n pack on a separate subject.\n\n Published quantities should be low-pass filtered to avoid aliasing effects.\n Publishers should strive to sample all parameters atomically.\n\n The reported quantities are focused on the amount of energy that can be reclaimed from the battery. In a\n simplified view, this can be seen as the amount of energy that is \"stored\" in the battery; however, this\n interpretation is not strictly correct because the amount of retrievable energy may be dependent on external\n factors such as the temperature of the battery or the load current. Energy estimation is hard and requires\n accurate modeling of the state of the battery, which may be impossible to do without precise tracking of each\n charging cycle. Despite the complications, this is considered to be a superior approach compared to the commonly\n used alternative where the state estimation is focused on the electric charge, because the latter cannot be used\n directly to predict the endurance of the system.\n\n The methods of energy estimation are implementation-defined."
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct _0 {}
                    impl ::canadensis_encoding::DataType for _0 {
                        /// This type is delimited with an extent of 0 bytes.
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
                    /// `reg.udral.service.battery.Error.0.1`
                    ///
                    /// Fixed size 1 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Generic error codes reported by the service provider.\n An error is reported when the corresponding parameter exceeds its safe operating area (SOA) as defined by the vendor;\n see https://en.wikipedia.org/wiki/Safe_operating_area.\n As long as an error condition is present, the service health should not be NOMINAL.\n\n If there are multiple error conditions present, the most severe one should be reported. The severity ordering\n is implementation-defined. Barring special requirements, it is recommended to give preference to errors whose\n code is smaller (e.g., BAD_BATTERY trumps TEMPERATURE_COLD)."
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Error {
                        ///
                        /// `saturated uint8`
                        ///
                        /// Always aligned,
                        /// size 8 bits
                        pub value: u8,
                    }
                    impl ::canadensis_encoding::DataType for Error {
                        /// This type is sealed.
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Error {}
                    impl Error {
                        #[cfg_attr(not(doctest), doc = " Normal operation.")]
                        pub const NONE: u8 = 0;
                        #[cfg_attr(
                            not(doctest),
                            doc = " The battery should not be used anymore. Detection criteria are implementation-defined."
                        )]
                        pub const BAD_BATTERY: u8 = 10;
                        #[cfg_attr(
                            not(doctest),
                            doc = " The battery requires offline maintenance."
                        )]
                        pub const NEEDS_SERVICE: u8 = 11;
                        #[cfg_attr(
                            not(doctest),
                            doc = " An internal error in the battery management system, not related to the battery itself."
                        )]
                        pub const BMS_ERROR: u8 = 20;
                        #[cfg_attr(
                            not(doctest),
                            doc = " The battery/BMS/node/service configuration is missing or invalid."
                        )]
                        pub const CONFIGURATION: u8 = 30;
                        #[cfg_attr(
                            not(doctest),
                            doc = " The battery is discharged beyond the design limits and may have incurred damage."
                        )]
                        pub const OVERDISCHARGE: u8 = 50;
                        #[cfg_attr(
                            not(doctest),
                            doc = " The charge or discharge rate exceeds the safe operating limits."
                        )]
                        pub const OVERLOAD: u8 = 51;
                        pub const CELL_OVERVOLTAGE: u8 = 60;
                        #[cfg_attr(
                            not(doctest),
                            doc = " Voltage of one of the battery cells exceeds its SOA."
                        )]
                        pub const CELL_UNDERVOLTAGE: u8 = 61;
                        #[cfg_attr(
                            not(doctest),
                            doc = " The sum of cell voltages is far from the total pack voltage.\n The threshold is implementation-defined."
                        )]
                        pub const CELL_COUNT: u8 = 62;
                        pub const TEMPERATURE_HOT: u8 = 100;
                        #[cfg_attr(
                            not(doctest),
                            doc = " At least one cell is above/below the temperature SOA."
                        )]
                        pub const TEMPERATURE_COLD: u8 = 101;
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
                pub mod parameters_0_3 {
                    /// `reg.udral.service.battery.Parameters.0.3`
                    ///
                    /// Size ranges from 64 to 128 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Smart battery parameter message. It is mostly intended for automated battery charging and maintenance systems.\n This message is modeled after the Smart Battery Data Specification (SBS) and the MAVLink battery status messages.\n\n The values carried by this message are either constant or slow-changing, so, generally, the publishing frequency\n should not be higher than 0.2 Hz, and the priority should be either OPTIONAL or SLOW.\n\n All parameters are required unless specifically stated otherwise.\n For non-rechargeable batteries all \"charge_*\" parameters should be NaN."
                    )]
                    pub struct Parameters {
                        #[cfg_attr(
                            not(doctest),
                            doc = " A statistically unique number that can be used to identify this exact battery for logging and diagnostic purposes.\n This value should be invariant to the identity of the reporting node unless it is an integral part of the battery.\n If the battery supports SBS, the recommended way to populate this field is from two CRC-32C (Castagnoli) values as:\n   - 32 most significant bits identify the vendor as:   CRC32C(ManufacturerName)\n   - 32 least significant bits identify the battery as: CRC32C(DeviceName + ManufactureDate + SerialNumber)\n If the battery does not support SBS, the vendor may choose arbitrary random numbers.\n Note that these are mere recommendations. The only hard requirement for this field is to be statistically unique."
                        )]
                        ///
                        /// `truncated uint64`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub unique_id: u64,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The total mass of the battery, including the packaging, electronics, cabling, and all auxiliary items, if any.\n May be used for predicting the kinematic parameters of the vehicle.\n NaN if unknown."
                        )]
                        ///
                        /// `uavcan.si.unit.mass.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub mass: crate::uavcan::si::unit::mass::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The maximum total charge of the pack, at 100% SoH, specified by the manufacturer."
                        )]
                        ///
                        /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub design_capacity:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The minimum (end of discharge) and the maximum (end of charge) resting cell voltage specified by the manufacturer\n at 100% SoH. Example: {2.8, 4.2} V. These voltages correspond to resting voltages; i.e., the stabilized voltages after\n the discharge/charge has been terminated. Voltage below the min may be observed during discharge due to the cell's\n internal resistance. Voltage above the max voltage may be observed during regenerative braking/charging etc due to\n the cell's internal resistance."
                        )]
                        ///
                        /// `uavcan.si.unit.voltage.Scalar.1.0[2]`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub design_cell_voltage_min_max:
                            [crate::uavcan::si::unit::voltage::scalar_1_0::Scalar; 2],
                        #[cfg_attr(
                            not(doctest),
                            doc = " Recommended continuous discharge current of the battery."
                        )]
                        ///
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub discharge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " Maximum current that may be safely discharged at least for 5 seconds."
                        )]
                        ///
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub discharge_current_burst:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " Recommended continuous charge current of the battery."
                        )]
                        ///
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub charge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " Recommended safest highest continuous charge current for the battery.\n This may cause accelerated aging of the battery."
                        )]
                        ///
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub charge_current_fast:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " End-of-charging current threshold. Charging may be terminated when the current falls below this threshold."
                        )]
                        ///
                        /// `uavcan.si.unit.electric_current.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub charge_termination_threshold:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The total voltage (not per-cell) that may be used by the charger to charge the battery pack."
                        )]
                        ///
                        /// `uavcan.si.unit.voltage.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub charge_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The number of charge-discharge cycles. Zero if the battery is new. May increase at runtime.\n What constitutes a charge-discharge cycle is implementation-defined."
                        )]
                        ///
                        /// `saturated uint16`
                        ///
                        /// Always aligned,
                        /// size 16 bits
                        pub cycle_count: u16,
                        // 8 bits of padding
                        #[cfg_attr(
                            not(doctest),
                            doc = " The number of cells connected in series. This value should match the array of cell voltages reported via Status."
                        )]
                        ///
                        /// `saturated uint8`
                        ///
                        /// Always aligned,
                        /// size 8 bits
                        pub series_cell_count: u8,
                        #[cfg_attr(
                            not(doctest),
                            doc = " [percent]\n The SoH of the battery, or best guess thereof; ranges from 0 (unusable) to 100 (new)."
                        )]
                        ///
                        /// `saturated uint7`
                        ///
                        /// Always aligned,
                        /// size 7 bits
                        pub state_of_health_pct: u8,
                        // 1 bits of padding
                        #[cfg_attr(
                            not(doctest),
                            doc = " The battery technology information may be leveraged by the charger to choose the appropriate charging strategy."
                        )]
                        ///
                        /// `reg.udral.service.battery.Technology.0.1`
                        ///
                        /// Always aligned,
                        /// size 8 bits
                        pub technology:
                            crate::reg::udral::service::battery::technology_0_1::Technology,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The nominal voltage of the battery pack (not per-cell) as defined by the vendor.\n E.g., a typical 22S LiCoO2 pack would usually report 81.4 V here."
                        )]
                        ///
                        /// `uavcan.si.unit.voltage.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub nominal_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The approximate UNIX Epoch time when the battery was manufactured, zero if unknown."
                        )]
                        ///
                        /// `truncated uint40`
                        ///
                        /// Always aligned,
                        /// size 40 bits
                        pub unix_manufacture_time: u64,
                        #[cfg_attr(
                            not(doctest),
                            doc = " An arbitrary human-readable textual description of this battery. Empty if unknown/unused.\n Batteries that support SBS are recommended to report the manufacturer name and the device name here."
                        )]
                        ///
                        /// `saturated uint8[<=64]`
                        ///
                        /// Always aligned,
                        /// size ranges from 0 to 512 bits
                        pub name: ::heapless::Vec<u8, 64>,
                    }
                    impl ::canadensis_encoding::DataType for Parameters {
                        /// This type is delimited with an extent of 300 bytes.
                        const EXTENT_BYTES: Option<u32> = Some(300);
                    }
                    impl ::canadensis_encoding::Message for Parameters {}
                    impl Parameters {}
                    impl ::canadensis_encoding::Serialize for Parameters {
                        fn size_bits(&self) -> usize {
                            64 + 32
                                + 32
                                + (self.design_cell_voltage_min_max).len() * 32
                                + 32
                                + 32
                                + 32
                                + 32
                                + 32
                                + 32
                                + 16
                                + 8
                                + 8
                                + 7
                                + 1
                                + 8
                                + 32
                                + 40
                                + 8
                                + (self.name).len() * 8
                                + 0
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
                            cursor.skip_8();
                            cursor.write_aligned_u8(self.series_cell_count);
                            cursor.write_u7(self.state_of_health_pct);
                            cursor.skip_1();
                            cursor.write_composite(&self.technology);
                            cursor.write_composite(&self.nominal_voltage);
                            cursor.write_u40(self.unix_manufacture_time);
                            cursor.write_aligned_u8((self.name).len() as u8);
                            cursor.write_bytes(&(self.name)[..]);
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
                                series_cell_count: {
                                    cursor.skip_8();
                                    cursor.read_u8() as _
                                },
                                state_of_health_pct: { cursor.read_u7() as _ },
                                technology: {
                                    cursor.skip_1();
                                    cursor.read_composite()?
                                },
                                nominal_voltage: { cursor.read_composite()? },
                                unix_manufacture_time: { cursor.read_u40() as _ },
                                name: {
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
                pub mod status_0_2 {
                    /// `reg.udral.service.battery.Status.0.2`
                    ///
                    /// Size ranges from 16 to 526 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " This low-rate battery status should be published at least once per second."
                    )]
                    pub struct Status {
                        #[cfg_attr(
                            not(doctest),
                            doc = " Note that the health code generally should not reflect the battery charge unless the service provider knows\n that the availability of energy in the battery is critical for the safe operation of the vehicle, which is usually\n not the case. For example, if the vehicle is equipped with several batteries that are discharged in series, one\n after another, the depletion of energy in the first battery is not a fault condition and it should not be reported\n as such. This follows from the good service design principles reviewed in https://opencyphal.org/guide.\n\n The readiness state depicts the ability of the battery (or its power electronics) to deliver full rated power\n and whether the overdischarge protections are active.\n When the battery is not ENGAGED, it may limit the output power below the nominal rated value and disconnect the load\n should the charge level fall below the critical level.\n When the battery is ENGAGED, it is not permitted to limit the output power or energy regardless of the risk of damage.\n If the adaptive protection is not supported, the battery should always report its status as ENGAGED."
                        )]
                        ///
                        /// `reg.udral.service.common.Heartbeat.0.1`
                        ///
                        /// Always aligned,
                        /// size 16 bits
                        pub heartbeat: crate::reg::udral::service::common::heartbeat_0_1::Heartbeat,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The minimum and maximum readings of the pack temperature sensors.\n For example, if the pack is equipped with three distributed temperature sensors that read {288, 258.15, 360.5} K,\n the reported array value would be {258.15, 360.5} K.\n If there is only one temperature sensor, both elements shall be of the same value."
                        )]
                        ///
                        /// `uavcan.si.unit.temperature.Scalar.1.0[2]`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub temperature_min_max:
                            [crate::uavcan::si::unit::temperature::scalar_1_0::Scalar; 2],
                        #[cfg_attr(
                            not(doctest),
                            doc = " The estimated electric charge currently stored in the battery. This is intended for charging and maintenance only.\n Do not use this parameter for endurance prediction! Instead, use the correct energy type from the physics namespace.\n The depth of discharge (DoD), or the state of charge (SoC), can be derived by dividing this value by the\n nominal battery capacity reported in the Parameters message."
                        )]
                        ///
                        /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub available_charge:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        ///
                        /// `reg.udral.service.battery.Error.0.1`
                        ///
                        /// Always aligned,
                        /// size 8 bits
                        pub error: crate::reg::udral::service::battery::error_0_1::Error,
                        #[cfg_attr(
                            not(doctest),
                            doc = " [volt]\n The voltages of individual cells in the battery pack."
                        )]
                        ///
                        /// `saturated float16[<=255]`
                        ///
                        /// Always aligned,
                        /// size ranges from 0 to 4080 bits
                        pub cell_voltages: ::heapless::Vec<::half::f16, 255>,
                    }
                    impl ::canadensis_encoding::DataType for Status {
                        /// This type is delimited with an extent of 600 bytes.
                        const EXTENT_BYTES: Option<u32> = Some(600);
                    }
                    impl ::canadensis_encoding::Message for Status {}
                    impl Status {
                        pub const MAX_CELLS: u8 = 255;
                    }
                    impl ::canadensis_encoding::Serialize for Status {
                        fn size_bits(&self) -> usize {
                            16 + (self.temperature_min_max).len() * 32
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
                            cursor.write_composite(&self.available_charge);
                            cursor.write_composite(&self.error);
                            cursor.write_aligned_u8((self.cell_voltages).len() as u8);
                            for value in (self.cell_voltages).iter() {
                                cursor.write_f16(*value);
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
                                available_charge: { cursor.read_composite()? },
                                error: { cursor.read_composite()? },
                                cell_voltages: {
                                    let length = cursor.read_u8() as _;
                                    if length <= 255 {
                                        let mut elements = ::heapless::Vec::new();
                                        for _ in 0..length {
                                            let _ = elements.push(cursor.read_f16());
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
                    /// `reg.udral.service.battery.Technology.0.1`
                    ///
                    /// Fixed size 1 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Battery chemistry type and its form-factor.\n Observe that there is no item to represent unknown technology because it is required to be known.\n This information may be used by charging systems to select the appropriate charging strategy.\n If the battery is of an uncommon type, it may be preferred to report the closest-matching type listed here\n instead of OTHER."
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Technology {
                        ///
                        /// `saturated uint8`
                        ///
                        /// Always aligned,
                        /// size 8 bits
                        pub value: u8,
                    }
                    impl ::canadensis_encoding::DataType for Technology {
                        /// This type is sealed.
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Technology {}
                    impl Technology {
                        #[cfg_attr(
                            not(doctest),
                            doc = " The technology is not specified in this enumeration. Please submit a pull request.\n NON-RECHARGEABLE"
                        )]
                        pub const OTHER: u8 = 0;
                        #[cfg_attr(not(doctest), doc = " Lithium-thionyl chloride (Li-SOCl2)")]
                        pub const LI_SOCL2: u8 = 10;
                        #[cfg_attr(
                            not(doctest),
                            doc = " Lithium-thionyl chloride + bromine chloride (Li-BCX)"
                        )]
                        pub const LI_BCX: u8 = 11;
                        #[cfg_attr(
                            not(doctest),
                            doc = " Lithium-manganese dioxide (Li-MnO2) (e.g., lithium coin cell, lithium 9V)"
                        )]
                        pub const LI_MNO2: u8 = 12;
                        #[cfg_attr(not(doctest), doc = " Zinc-Air")]
                        pub const ZN_O2: u8 = 20;
                        #[cfg_attr(not(doctest), doc = " Aluminum-Air")]
                        pub const AL_O2: u8 = 21;
                        #[cfg_attr(
                            not(doctest),
                            doc = " Zinc-manganese dioxide - ammonium chloride electrolyte (aka zinc-carbon)"
                        )]
                        pub const ZN_MNO2_NH4CL: u8 = 30;
                        #[cfg_attr(
                            not(doctest),
                            doc = " Zinc-manganese dioxide - zinc chloride electrolyte (aka heavy duty zinc-carbon)"
                        )]
                        pub const ZN_MNO2_ZNCL2: u8 = 31;
                        #[cfg_attr(
                            not(doctest),
                            doc = " Zinc-manganese dioxide - potassium hydroxide electrolyte (aka alkaline)\n RECHARGEABLE"
                        )]
                        pub const ZN_MNO2_KOH: u8 = 32;
                        #[cfg_attr(
                            not(doctest),
                            doc = " Lithium cobalt oxide (commonly known as just \"lithium-ion\")"
                        )]
                        pub const LI_LCO: u8 = 100;
                        #[cfg_attr(not(doctest), doc = " Lithium iron phosphate (LiFePO4)")]
                        pub const LI_LFP: u8 = 101;
                        #[cfg_attr(not(doctest), doc = " Lithium nickel manganese cobalt oxide")]
                        pub const LI_NMC: u8 = 102;
                        #[cfg_attr(not(doctest), doc = " Lithium nickel cobalt aluminium oxide")]
                        pub const LI_NCA: u8 = 103;
                        #[cfg_attr(not(doctest), doc = " Lithium manganese oxide")]
                        pub const LI_LMO: u8 = 104;
                        #[cfg_attr(not(doctest), doc = " Lithium-sulfur (LiS)")]
                        pub const LI_S: u8 = 105;
                        #[cfg_attr(
                            not(doctest),
                            doc = " LiCoO2 in pouch form factor, commonly known as \"lithium-ion polymer\" or \"LiPo\"."
                        )]
                        pub const LI_LCO_POUCH: u8 = 110;
                        #[cfg_attr(
                            not(doctest),
                            doc = " LiFePO4 in pouch form factor, commonly known as \"LiFePO4 polymer\"."
                        )]
                        pub const LI_LFP_POUCH: u8 = 111;
                        #[cfg_attr(not(doctest), doc = " Nickel-metal hydride")]
                        pub const NI_MH: u8 = 120;
                        #[cfg_attr(not(doctest), doc = " Nickel-cadmium")]
                        pub const NI_CD: u8 = 121;
                        #[cfg_attr(not(doctest), doc = " Nickel-zinc")]
                        pub const NI_ZN: u8 = 122;
                        #[cfg_attr(not(doctest), doc = " Nickel-iron")]
                        pub const NI_FE: u8 = 123;
                        #[cfg_attr(not(doctest), doc = " Lead acid")]
                        pub const PB_AC: u8 = 130;
                        #[cfg_attr(not(doctest), doc = " Also known as SLA")]
                        pub const PB_AC_SEALED: u8 = 131;
                        #[cfg_attr(not(doctest), doc = " Electrostatic double-layer capacitor")]
                        pub const EDLC: u8 = 200;
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
                    /// `reg.udral.service.common.Heartbeat.0.1`
                    ///
                    /// Fixed size 2 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " The function of the service heartbeat is similar to that of the node heartbeat defined in the standard namespace,\n except that it is used on a per-service basis, meaning that there may be more than one publisher per node.\n\n The service heartbeat should be published either on a separate subject, or as a structural supertype of a\n service-specific status subject. The publication rate is service-specific but it should not be lower than 1 Hz.\n\n This is a structural subtype of the Readiness type."
                    )]
                    pub struct Heartbeat {
                        ///
                        /// `reg.udral.service.common.Readiness.0.1`
                        ///
                        /// Always aligned,
                        /// size 8 bits
                        pub readiness: crate::reg::udral::service::common::readiness_0_1::Readiness,
                        ///
                        /// `uavcan.node.Health.1.0`
                        ///
                        /// Always aligned,
                        /// size 8 bits
                        pub health: crate::uavcan::node::health_1_0::Health,
                    }
                    impl ::canadensis_encoding::DataType for Heartbeat {
                        /// This type is sealed.
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Heartbeat {}
                    impl Heartbeat {
                        #[cfg_attr(
                            not(doctest),
                            doc = " Any service that is not in the SLEEP state should publish its heartbeat (or a derived status) at least at this rate."
                        )]
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
                    /// `reg.udral.service.common.Readiness.0.1`
                    ///
                    /// Fixed size 1 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " The readiness state is used to command or report the availability status of a networked service (subsystem).\n\n Any system shall have at least one readiness command subject that acts as a global power switch.\n Every subsystem controlled in such way would usually report its readiness status back to account for the fact that\n the transition between different readiness states may not be instantaneous.\n The readiness status reporting is done by means of the service heartbeat type that is also defined in this namespace;\n the service heartbeat type is a structural subtype of this type.\n\n   +------------+\n   | Controller |----------+----------------+----------------+---------...     readiness command subject\n   +------------+          |                |                |\n     ^   ^   ^             v                v                v\n     |   |   |        +---------+      +---------+      +---------+\n     |   |   |        | Service |      | Service |      | Service |    ...\n     |   |   |        +---------+      +---------+      +---------+\n     |   |   |             |                |                |\n     |   |   +-------------+                |                |\n     |   +----------------------------------+                |                 service heartbeat subjects\n     +-------------------------------------------------------+\n\n In a less trivial use case there may be an arbitrary number of such readiness command subjects (local power switches)\n controlling various systems within the vehicle (e.g., propulsion, perception sensors, communication, etc).\n\n The publication rate is defined on a per-service basis, but it should never be lower than 1 Hz,\n excepting services that are in the SLEEP state, in which case it is permitted to cease all network activity."
                    )]
                    pub struct Readiness {
                        ///
                        /// `truncated uint2`
                        ///
                        /// Always aligned,
                        /// size 2 bits
                        pub value: u8,
                    }
                    impl ::canadensis_encoding::DataType for Readiness {
                        /// This type is sealed.
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Readiness {}
                    impl Readiness {
                        #[cfg_attr(
                            not(doctest),
                            doc = " The long-term state of minimal power consumption.\n Typically, most subsystems are switched into the SLEEP mode when the vehicle is parked and powered off.\n Subsystems that do not support the SLEEP state should treat it as an equivalent of STANDBY.\n\n A subsystem may require a substantial amount of time to exit the sleep mode (for example, time may be needed to\n boot the operating system and run the self test procedures).\n\n While in the SLEEP mode, the subsystem is allowed to cease service provision and stop all network activity\n regardless of other requirements, except that it shall be able to reactivate itself if a Readiness message is\n received commanding any state other than SLEEP.\n Value 1 is invalid and shall never be commanded.\n Implementations receiving this value should interpret it either as SLEEP or STANDBY."
                        )]
                        pub const SLEEP: u8 = 0;
                        #[cfg_attr(
                            not(doctest),
                            doc = " The state of being ready to enter the normal operating mode in a short order.\n A subsystem that is in STANDBY state should be able to participate in the normal network activity.\n This is the default state that the subsystem should reside in after power-on until explicitly commanded otherwise."
                        )]
                        pub const STANDBY: u8 = 2;
                        #[cfg_attr(
                            not(doctest),
                            doc = " When ENGAGED, the subsystem is performing its main intended function at the nominal performance characteristics.\n A subsystem may require a short amount of time, possibly under a few seconds, to switch between the ENGAGED and\n STANDBY states (in any direction).\n Some subsystems may not differentiate between STANDBY and ENGAGED (e.g., offboard communication hardware).\n The subsystem may disengage itself autonomously in the event of a fatal malfunction, in which case\n the reported service health status should be WARNING."
                        )]
                        pub const ENGAGED: u8 = 3;
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
            pub mod sensor {
                pub mod status_0_1 {
                    /// `reg.udral.service.sensor.Status.0.1`
                    ///
                    /// Fixed size 12 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " A generic sensor status information.\n This data should be published at a low rate but not lower than the specified limit."
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Status {
                        #[cfg_attr(
                            not(doctest),
                            doc = " Data samples obtained at time Ts are valid at time Tr if: (Tr - Ts) < data_validity_period\n Expired data should be discarded."
                        )]
                        ///
                        /// `uavcan.si.unit.duration.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub data_validity_period:
                            crate::uavcan::si::unit::duration::scalar_1_0::Scalar,
                        #[cfg_attr(
                            not(doctest),
                            doc = " Incremented once per occurrence. Reset to zero when the sensor is ENGAGED.\n The exact definition of what constitutes an error is implementation-dependent."
                        )]
                        ///
                        /// `saturated uint32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub error_count: u32,
                        #[cfg_attr(
                            not(doctest),
                            doc = " The temperature of the sensing element.\n If there are multiple sensing elements or multiple temperature probes per sensor,\n the reduction is implementation-defined.\n In a later revision this field may be moved into a separate type."
                        )]
                        ///
                        /// `uavcan.si.unit.temperature.Scalar.1.0`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub sensor_temperature:
                            crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Status {
                        /// This type is delimited with an extent of 63 bytes.
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Status {}
                    impl Status {
                        #[cfg_attr(not(doctest), doc = " [second]")]
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
#[allow(unused_variables, unused_braces, unused_parens)]
#[allow(clippy::identity_op)]
pub mod uavcan {
    pub mod diagnostic {
        #[allow(deprecated)]
        #[deprecated]
        pub mod record_1_0 {
            #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
            #[deprecated]
            pub const SUBJECT: ::canadensis_core::SubjectId =
                ::canadensis_core::SubjectId::from_truncating(8184);

            /// `uavcan.diagnostic.Record.1.0`
            ///
            /// Size ranges from 9 to 121 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Generic human-readable text message for logging and displaying purposes.\n Generally, it should be published at the lowest priority level."
            )]
            #[deprecated]
            pub struct Record {
                #[cfg_attr(
                    not(doctest),
                    doc = " Optional timestamp in the network-synchronized time system; zero if undefined.\n The timestamp value conveys the exact moment when the reported event took place."
                )]
                ///
                /// `uavcan.time.SynchronizedTimestamp.1.0`
                ///
                /// Always aligned,
                /// size 56 bits
                pub timestamp:
                    crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                ///
                /// `uavcan.diagnostic.Severity.1.0`
                ///
                /// Always aligned,
                /// size 8 bits
                pub severity: crate::uavcan::diagnostic::severity_1_0::Severity,
                #[cfg_attr(
                    not(doctest),
                    doc = " Message text.\n Normally, messages should be kept as short as possible, especially those of high severity."
                )]
                ///
                /// `saturated uint8[<=112]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 896 bits
                pub text: ::heapless::Vec<u8, 112>,
            }
            impl ::canadensis_encoding::DataType for Record {
                /// This type is delimited with an extent of 300 bytes.
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
            #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
            pub const SUBJECT: ::canadensis_core::SubjectId =
                ::canadensis_core::SubjectId::from_truncating(8184);

            /// `uavcan.diagnostic.Record.1.1`
            ///
            /// Size ranges from 9 to 264 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Generic human-readable text message for logging and displaying purposes.\n Generally, it should be published at the lowest priority level."
            )]
            pub struct Record {
                #[cfg_attr(
                    not(doctest),
                    doc = " Optional timestamp in the network-synchronized time system; zero if undefined.\n The timestamp value conveys the exact moment when the reported event took place."
                )]
                ///
                /// `uavcan.time.SynchronizedTimestamp.1.0`
                ///
                /// Always aligned,
                /// size 56 bits
                pub timestamp:
                    crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                ///
                /// `uavcan.diagnostic.Severity.1.0`
                ///
                /// Always aligned,
                /// size 8 bits
                pub severity: crate::uavcan::diagnostic::severity_1_0::Severity,
                #[cfg_attr(
                    not(doctest),
                    doc = " Message text.\n Normally, messages should be kept as short as possible, especially those of high severity."
                )]
                ///
                /// `saturated uint8[<=255]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 2040 bits
                pub text: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for Record {
                /// This type is delimited with an extent of 300 bytes.
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
            ///
            #[cfg_attr(not(doctest), doc = " Generic message severity representation.")]
            pub struct Severity {
                #[cfg_attr(
                    not(doctest),
                    doc = " The severity level ranging from 0 to 7, where low values represent low-severity (unimportant) messages, and\n high values represent high-severity (important) messages. Several mnemonics for the severity levels are\n defined below. Nodes are advised to implement output filtering mechanisms, allowing users to select\n the minimal severity for emitted messages; messages of the selected and higher severity levels will\n be published, and messages of lower severity will be suppressed (discarded)."
                )]
                ///
                /// `saturated uint3`
                ///
                /// Always aligned,
                /// size 3 bits
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for Severity {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Severity {}
            impl Severity {
                #[cfg_attr(
                    not(doctest),
                    doc = " Messages of this severity can be used only during development.\n They shall not be used in a fielded operational system."
                )]
                pub const TRACE: u8 = 0;
                #[cfg_attr(
                    not(doctest),
                    doc = " Messages that can aid in troubleshooting.\n Messages of this severity and lower should be disabled by default."
                )]
                pub const DEBUG: u8 = 1;
                #[cfg_attr(
                    not(doctest),
                    doc = " General informational messages of low importance.\n Messages of this severity and lower should be disabled by default."
                )]
                pub const INFO: u8 = 2;
                #[cfg_attr(
                    not(doctest),
                    doc = " General informational messages of high importance.\n Messages of this severity and lower should be disabled by default."
                )]
                pub const NOTICE: u8 = 3;
                #[cfg_attr(
                    not(doctest),
                    doc = " Messages reporting abnormalities and warning conditions.\n Messages of this severity and higher should be enabled by default."
                )]
                pub const WARNING: u8 = 4;
                #[cfg_attr(
                    not(doctest),
                    doc = " Messages reporting problems and error conditions.\n Messages of this severity and higher should be enabled by default."
                )]
                pub const ERROR: u8 = 5;
                #[cfg_attr(
                    not(doctest),
                    doc = " Messages reporting serious problems and critical conditions.\n Messages of this severity and higher should be always enabled."
                )]
                pub const CRITICAL: u8 = 6;
                #[cfg_attr(
                    not(doctest),
                    doc = " Notifications of dangerous circumstances that demand immediate attention.\n Messages of this severity should be always enabled."
                )]
                pub const ALERT: u8 = 7;
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Nested type.\n Result of a file system operation."
            )]
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct Error {
                ///
                /// `saturated uint16`
                ///
                /// Always aligned,
                /// size 16 bits
                pub value: u16,
            }
            impl ::canadensis_encoding::DataType for Error {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Error {}
            impl Error {
                pub const OK: u16 = 0;
                pub const UNKNOWN_ERROR: u16 = 65535;
                pub const NOT_FOUND: u16 = 2;
                pub const IO_ERROR: u16 = 5;
                pub const ACCESS_DENIED: u16 = 13;
                #[cfg_attr(
                    not(doctest),
                    doc = " I.e., attempted read/write on a path that points to a directory"
                )]
                pub const IS_DIRECTORY: u16 = 21;
                #[cfg_attr(
                    not(doctest),
                    doc = " E.g., file name is not valid for the target file system"
                )]
                pub const INVALID_VALUE: u16 = 22;
                pub const FILE_TOO_LARGE: u16 = 27;
                pub const OUT_OF_SPACE: u16 = 28;
                pub const NOT_SUPPORTED: u16 = 38;
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
        #[allow(deprecated)]
        #[deprecated]
        pub mod get_info_0_1 {
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            #[deprecated]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(405);

            /// `uavcan.file.GetInfo.0.1`
            ///
            /// Size ranges from 1 to 113 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Information about a remote file system entry (file, directory, etc)."
            )]
            #[deprecated]
            pub struct GetInfoRequest {
                ///
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 904 bits
                pub path: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for GetInfoRequest {
                /// This type is delimited with an extent of 300 bytes.
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
            ///
            #[deprecated]
            pub struct GetInfoResponse {
                #[cfg_attr(not(doctest), doc = " Result of the operation.")]
                ///
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
                #[cfg_attr(
                    not(doctest),
                    doc = " File size in bytes. Should be set to zero for directories."
                )]
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub size: u64,
                #[cfg_attr(
                    not(doctest),
                    doc = " The UNIX Epoch time when the entry was last modified. Zero if unknown."
                )]
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub unix_timestamp_of_last_modification: u64,
                #[cfg_attr(not(doctest), doc = " True if file, false if directory.")]
                ///
                /// `saturated bool`
                ///
                /// Always aligned,
                /// size 1 bits
                pub is_file_not_directory: bool,
                #[cfg_attr(
                    not(doctest),
                    doc = " This is a link to another entry; the above flag indicates the type of the target."
                )]
                ///
                /// `saturated bool`
                ///
                /// Not always aligned,
                /// size 1 bits
                pub is_link: bool,
                #[cfg_attr(
                    not(doctest),
                    doc = " The item can be read by the caller (applies to files and directories)."
                )]
                ///
                /// `saturated bool`
                ///
                /// Not always aligned,
                /// size 1 bits
                pub is_readable: bool,
                #[cfg_attr(
                    not(doctest),
                    doc = " The item can be written by the caller (applies to files and directories).\n If such entry does not exist, all flags should be cleared/ignored."
                )]
                ///
                /// `saturated bool`
                ///
                /// Not always aligned,
                /// size 1 bits
                pub is_writeable: bool,
                // 4 bits of padding
            }
            impl ::canadensis_encoding::DataType for GetInfoResponse {
                /// This type is delimited with an extent of 48 bytes.
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(405);

            /// `uavcan.file.GetInfo.0.2`
            ///
            /// Size ranges from 1 to 256 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Information about a remote file system entry (file, directory, etc)."
            )]
            pub struct GetInfoRequest {
                ///
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2048 bits
                pub path: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for GetInfoRequest {
                /// This type is delimited with an extent of 300 bytes.
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
            ///
            pub struct GetInfoResponse {
                #[cfg_attr(not(doctest), doc = " Result of the operation.")]
                ///
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
                #[cfg_attr(
                    not(doctest),
                    doc = " File size in bytes. Should be set to zero for directories."
                )]
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub size: u64,
                #[cfg_attr(
                    not(doctest),
                    doc = " The UNIX Epoch time when the entry was last modified. Zero if unknown."
                )]
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub unix_timestamp_of_last_modification: u64,
                #[cfg_attr(not(doctest), doc = " True if file, false if directory.")]
                ///
                /// `saturated bool`
                ///
                /// Always aligned,
                /// size 1 bits
                pub is_file_not_directory: bool,
                #[cfg_attr(
                    not(doctest),
                    doc = " This is a link to another entry; the above flag indicates the type of the target."
                )]
                ///
                /// `saturated bool`
                ///
                /// Not always aligned,
                /// size 1 bits
                pub is_link: bool,
                #[cfg_attr(
                    not(doctest),
                    doc = " The item can be read by the caller (applies to files and directories)."
                )]
                ///
                /// `saturated bool`
                ///
                /// Not always aligned,
                /// size 1 bits
                pub is_readable: bool,
                #[cfg_attr(
                    not(doctest),
                    doc = " The item can be written by the caller (applies to files and directories).\n If such entry does not exist, all flags should be cleared/ignored."
                )]
                ///
                /// `saturated bool`
                ///
                /// Not always aligned,
                /// size 1 bits
                pub is_writeable: bool,
                // 4 bits of padding
            }
            impl ::canadensis_encoding::DataType for GetInfoResponse {
                /// This type is delimited with an extent of 48 bytes.
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
        #[allow(deprecated)]
        #[deprecated]
        pub mod list_0_1 {
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            #[deprecated]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(406);

            /// `uavcan.file.List.0.1`
            ///
            /// Size ranges from 9 to 121 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " This service can be used to list a remote directory, one entry per request.\n\n The client should query each entry independently, iterating 'entry_index' from 0 until the last entry.\n When the index reaches the number of elements in the directory, the server will report that there is\n no such entry by returning an empty name.\n\n The field entry_index shall be applied to an ordered list of directory entries (e.g. alphabetically ordered).\n The exact sorting criteria does not matter as long as it provides the same ordering for subsequent service calls.\n\n Observe that this listing operation is fundamentally non-atomic. The caller shall beware of possible race conditions\n and is responsible for handling them properly. Particularly, consider what happens if a new item is inserted into\n the directory between two subsequent calls: if the item happened to be inserted at the index that is lower than the\n index of the next request, the next returned item (or several, if more items were inserted) will repeat the ones\n that were listed earlier. The caller should handle that properly, either by ignoring the repeated items or by\n restarting the listing operation from the beginning (index 0)."
            )]
            #[deprecated]
            pub struct ListRequest {
                ///
                /// `saturated uint32`
                ///
                /// Always aligned,
                /// size 32 bits
                pub entry_index: u32,
                // 32 bits of padding
                ///
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 904 bits
                pub directory_path: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListRequest {
                /// This type is delimited with an extent of 300 bytes.
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
            ///
            #[deprecated]
            pub struct ListResponse {
                // 32 bits of padding
                #[cfg_attr(
                    not(doctest),
                    doc = " The base name of the referenced entry, i.e., relative to the outer directory.\n The outer directory path is not included to conserve bandwidth.\n Empty if such entry does not exist.\n\n For example, suppose there is a file \"/foo/bar/baz.bin\". Listing the directory with the path \"/foo/bar/\" (the slash\n at the end is optional) at the index 0 will return \"baz.bin\". Listing the same directory at the index 1 (or any\n higher) will return an empty name \"\", indicating that the caller has reached the end of the list."
                )]
                ///
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 904 bits
                pub entry_base_name: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListResponse {
                /// This type is delimited with an extent of 300 bytes.
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(406);

            /// `uavcan.file.List.0.2`
            ///
            /// Size ranges from 9 to 264 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " This service can be used to list a remote directory, one entry per request.\n\n The client should query each entry independently, iterating 'entry_index' from 0 until the last entry.\n When the index reaches the number of elements in the directory, the server will report that there is\n no such entry by returning an empty name.\n\n The field entry_index shall be applied to an ordered list of directory entries (e.g. alphabetically ordered).\n The exact sorting criteria does not matter as long as it provides the same ordering for subsequent service calls.\n\n Observe that this listing operation is fundamentally non-atomic. The caller shall beware of possible race conditions\n and is responsible for handling them properly. Particularly, consider what happens if a new item is inserted into\n the directory between two subsequent calls: if the item happened to be inserted at the index that is lower than the\n index of the next request, the next returned item (or several, if more items were inserted) will repeat the ones\n that were listed earlier. The caller should handle that properly, either by ignoring the repeated items or by\n restarting the listing operation from the beginning (index 0)."
            )]
            pub struct ListRequest {
                ///
                /// `saturated uint32`
                ///
                /// Always aligned,
                /// size 32 bits
                pub entry_index: u32,
                // 32 bits of padding
                ///
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2048 bits
                pub directory_path: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListRequest {
                /// This type is delimited with an extent of 300 bytes.
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
            ///
            pub struct ListResponse {
                // 32 bits of padding
                #[cfg_attr(
                    not(doctest),
                    doc = " The base name of the referenced entry, i.e., relative to the outer directory.\n The outer directory path is not included to conserve bandwidth.\n Empty if such entry does not exist.\n\n For example, suppose there is a file \"/foo/bar/baz.bin\". Listing the directory with the path \"/foo/bar/\" (the slash\n at the end is optional) at the index 0 will return \"baz.bin\". Listing the same directory at the index 1 (or any\n higher) will return an empty name \"\", indicating that the caller has reached the end of the list."
                )]
                ///
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2048 bits
                pub entry_base_name: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListResponse {
                /// This type is delimited with an extent of 300 bytes.
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
        #[allow(deprecated)]
        #[deprecated]
        pub mod modify_1_0 {
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            #[deprecated]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(407);

            /// `uavcan.file.Modify.1.0`
            ///
            /// Size ranges from 6 to 230 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Manipulate a remote file system entry. Applies to files, directories, and links alike.\n If the remote entry is a directory, all nested entries will be affected, too.\n\n The server should perform all operations atomically, unless atomicity is not supported by\n the underlying file system.\n\n Atomic copying can be effectively employed by remote nodes before reading or after writing\n the file to minimize the possibility of race conditions.\n For example, before reading a large file from the server, the cilent might opt to create\n a temporary copy of it first, then read the copy, and delete it upon completion. Likewise,\n a similar strategy can be employed for writing, where the file is first written at a\n temporary location, and then moved to its final destination. These approaches, however,\n may lead to creation of dangling temporary files if the client failed to dispose of them\n properly, so that risk should be taken into account.\n\n Move/Copy\n   Specify the source path and the destination path.\n   If the source does not exist, the operation will fail.\n   Set the preserve_source flag to copy rather than move.\n   If the destination exists and overwrite_destination is not set, the operation will fail.\n   If the target path includes non-existent directories, they will be created (like \"mkdir -p\").\n\n Touch\n   Specify the destination path and make the source path empty.\n   If the path exists (file/directory/link), its modification time will be updated.\n   If the path does not exist, an empty file will be created.\n   If the target path includes non-existent directories, they will be created (like \"mkdir -p\").\n   Flags are ignored.\n\n Remove\n   Specify the source path (file/directory/link) and make the destination path empty.\n   Fails if the path does not exist.\n   Flags are ignored."
            )]
            #[deprecated]
            pub struct ModifyRequest {
                #[cfg_attr(
                    not(doctest),
                    doc = " Do not remove the source. Used to copy instead of moving."
                )]
                ///
                /// `saturated bool`
                ///
                /// Always aligned,
                /// size 1 bits
                pub preserve_source: bool,
                #[cfg_attr(
                    not(doctest),
                    doc = " If the destination exists, remove it beforehand."
                )]
                ///
                /// `saturated bool`
                ///
                /// Not always aligned,
                /// size 1 bits
                pub overwrite_destination: bool,
                // 30 bits of padding
                ///
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 904 bits
                pub source: crate::uavcan::file::path_1_0::Path,
                ///
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 904 bits
                pub destination: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ModifyRequest {
                /// This type is delimited with an extent of 600 bytes.
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
            ///
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            #[deprecated]
            pub struct ModifyResponse {
                ///
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for ModifyResponse {
                /// This type is delimited with an extent of 48 bytes.
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(407);

            /// `uavcan.file.Modify.1.1`
            ///
            /// Size ranges from 6 to 516 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Manipulate a remote file system entry. Applies to files, directories, and links alike.\n If the remote entry is a directory, all nested entries will be affected, too.\n\n The server should perform all operations atomically, unless atomicity is not supported by\n the underlying file system.\n\n Atomic copying can be effectively employed by remote nodes before reading or after writing\n the file to minimize the possibility of race conditions.\n For example, before reading a large file from the server, the cilent might opt to create\n a temporary copy of it first, then read the copy, and delete it upon completion. Likewise,\n a similar strategy can be employed for writing, where the file is first written at a\n temporary location, and then moved to its final destination. These approaches, however,\n may lead to creation of dangling temporary files if the client failed to dispose of them\n properly, so that risk should be taken into account.\n\n Move/Copy\n   Specify the source path and the destination path.\n   If the source does not exist, the operation will fail.\n   Set the preserve_source flag to copy rather than move.\n   If the destination exists and overwrite_destination is not set, the operation will fail.\n   If the target path includes non-existent directories, they will be created (like \"mkdir -p\").\n\n Touch\n   Specify the destination path and make the source path empty.\n   If the path exists (file/directory/link), its modification time will be updated.\n   If the path does not exist, an empty file will be created.\n   If the target path includes non-existent directories, they will be created (like \"mkdir -p\").\n   Flags are ignored.\n\n Remove\n   Specify the source path (file/directory/link) and make the destination path empty.\n   Fails if the path does not exist.\n   Flags are ignored."
            )]
            pub struct ModifyRequest {
                #[cfg_attr(
                    not(doctest),
                    doc = " Do not remove the source. Used to copy instead of moving."
                )]
                ///
                /// `saturated bool`
                ///
                /// Always aligned,
                /// size 1 bits
                pub preserve_source: bool,
                #[cfg_attr(
                    not(doctest),
                    doc = " If the destination exists, remove it beforehand."
                )]
                ///
                /// `saturated bool`
                ///
                /// Not always aligned,
                /// size 1 bits
                pub overwrite_destination: bool,
                // 30 bits of padding
                ///
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2048 bits
                pub source: crate::uavcan::file::path_2_0::Path,
                ///
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2048 bits
                pub destination: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ModifyRequest {
                /// This type is delimited with an extent of 600 bytes.
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
            ///
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ModifyResponse {
                ///
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for ModifyResponse {
                /// This type is delimited with an extent of 48 bytes.
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
        #[allow(deprecated)]
        #[deprecated]
        pub mod path_1_0 {
            /// `uavcan.file.Path.1.0`
            ///
            /// Size ranges from 1 to 113 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Nested type.\n A file system path encoded in UTF8. The only valid separator is the forward slash \"/\".\n A single slash (\"/\") refers to the root directory (the location of which is defined by the server).\n Relative references (e.g. \"..\") are not defined and not permitted (although this may change in the future).\n Conventions (not enforced):\n   - A path pointing to a file or a link to file should not end with a separator.\n   - A path pointing to a directory or to a link to directory should end with a separator.\n\n The maximum path length limit is chosen as a trade-off between compatibility with deep directory structures and\n the worst-case transfer length. The limit is 112 bytes, which allows all transfers containing a single instance\n of path and no other large data chunks to fit into two CAN FD frames."
            )]
            #[deprecated]
            pub struct Path {
                ///
                /// `saturated uint8[<=112]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 896 bits
                pub path: ::heapless::Vec<u8, 112>,
            }
            impl ::canadensis_encoding::DataType for Path {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Path {}
            impl Path {
                pub const SEPARATOR: u8 = 47;
                pub const MAX_LENGTH: u8 = 112;
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Nested type.\n A file system path encoded in UTF8. The only valid separator is the forward slash \"/\".\n A single slash (\"/\") refers to the root directory (the location of which is defined by the server).\n Relative references (e.g. \"..\") are not defined and not permitted (although this may change in the future).\n Conventions (not enforced):\n   - A path pointing to a file or a link to file should not end with a separator.\n   - A path pointing to a directory or to a link to directory should end with a separator."
            )]
            pub struct Path {
                ///
                /// `saturated uint8[<=255]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 2040 bits
                pub path: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for Path {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Path {}
            impl Path {
                pub const SEPARATOR: u8 = 47;
                pub const MAX_LENGTH: u8 = 255;
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
        #[allow(deprecated)]
        #[deprecated]
        pub mod read_1_0 {
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            #[deprecated]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(408);

            /// `uavcan.file.Read.1.0`
            ///
            /// Size ranges from 6 to 118 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Read file from a remote node.\n\n There are two possible outcomes of a successful call:\n  1. Data array size equals its capacity. This means that the end of the file is not reached yet.\n  2. Data array size is less than its capacity, possibly zero. This means that the end of the file is reached.\n\n Thus, if the client needs to fetch the entire file, it should repeatedly call this service while increasing the\n offset, until a non-full data array is returned.\n\n If the object pointed by 'path' cannot be read (e.g. it is a directory or it does not exist), an appropriate error\n code will be returned, and the data array will be empty.\n\n It is easy to see that this protocol is prone to race conditions because the remote file can be modified\n between read operations which might result in the client obtaining a damaged file. To combat this,\n application designers are recommended to adhere to the following convention. Let every file whose integrity\n is of interest have a hash or a digital signature, which is stored in an adjacent file under the same name\n suffixed with the appropriate extension according to the type of hash or digital signature used.\n For example, let there be file \"image.bin\", integrity of which shall be ensured by the client upon downloading.\n Suppose that the file is hashed using SHA-256, so the appropriate file extension for the hash would be\n \".sha256\". Following this convention, the hash of \"image.bin\" would be stored in \"image.bin.sha256\".\n After downloading the file, the client would read the hash (being small, the hash can be read in a single\n request) and check it against a locally computed value. Some servers may opt to generate such hash files\n automatically as necessary; for example, if such file is requested but it does not exist, the server would\n compute the necessary signature or hash (the type of hash/signature can be deduced from the requested file\n extension) and return it as if the file existed. Obviously, this would be impractical for very large files;\n in that case, hash/signature should be pre-computed and stored in a real file. If this approach is followed,\n implementers are advised to use only SHA-256 for hashing, in order to reduce the number of fielded\n incompatible implementations."
            )]
            #[deprecated]
            pub struct ReadRequest {
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub offset: u64,
                ///
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 904 bits
                pub path: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ReadRequest {
                /// This type is delimited with an extent of 300 bytes.
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
            ///
            #[deprecated]
            pub struct ReadResponse {
                ///
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
                ///
                /// `saturated uint8[<=256]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 2048 bits
                pub data: ::heapless::Vec<u8, 256>,
            }
            impl ::canadensis_encoding::DataType for ReadResponse {
                /// This type is delimited with an extent of 300 bytes.
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(408);

            /// `uavcan.file.Read.1.1`
            ///
            /// Size ranges from 6 to 261 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Read file from a remote node.\n\n There are two possible outcomes of a successful call:\n  1. Data array size equals its capacity. This means that the end of the file is not reached yet.\n  2. Data array size is less than its capacity, possibly zero. This means that the end of the file is reached.\n\n Thus, if the client needs to fetch the entire file, it should repeatedly call this service while increasing the\n offset, until a non-full data array is returned.\n\n If the object pointed by 'path' cannot be read (e.g. it is a directory or it does not exist), an appropriate error\n code will be returned, and the data array will be empty.\n\n It is easy to see that this protocol is prone to race conditions because the remote file can be modified\n between read operations which might result in the client obtaining a damaged file. To combat this,\n application designers are recommended to adhere to the following convention. Let every file whose integrity\n is of interest have a hash or a digital signature, which is stored in an adjacent file under the same name\n suffixed with the appropriate extension according to the type of hash or digital signature used.\n For example, let there be file \"image.bin\", integrity of which shall be ensured by the client upon downloading.\n Suppose that the file is hashed using SHA-256, so the appropriate file extension for the hash would be\n \".sha256\". Following this convention, the hash of \"image.bin\" would be stored in \"image.bin.sha256\".\n After downloading the file, the client would read the hash (being small, the hash can be read in a single\n request) and check it against a locally computed value. Some servers may opt to generate such hash files\n automatically as necessary; for example, if such file is requested but it does not exist, the server would\n compute the necessary signature or hash (the type of hash/signature can be deduced from the requested file\n extension) and return it as if the file existed. Obviously, this would be impractical for very large files;\n in that case, hash/signature should be pre-computed and stored in a real file. If this approach is followed,\n implementers are advised to use only SHA-256 for hashing, in order to reduce the number of fielded\n incompatible implementations."
            )]
            pub struct ReadRequest {
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub offset: u64,
                ///
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2048 bits
                pub path: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ReadRequest {
                /// This type is delimited with an extent of 300 bytes.
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
            ///
            pub struct ReadResponse {
                ///
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
                ///
                /// `uavcan.primitive.Unstructured.1.0`
                ///
                /// Always aligned,
                /// size ranges from 16 to 2064 bits
                pub data: crate::uavcan::primitive::unstructured_1_0::Unstructured,
            }
            impl ::canadensis_encoding::DataType for ReadResponse {
                /// This type is delimited with an extent of 300 bytes.
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
        #[allow(deprecated)]
        #[deprecated]
        pub mod write_1_0 {
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            #[deprecated]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(409);

            /// `uavcan.file.Write.1.0`
            ///
            /// Size ranges from 7 to 311 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Write into a remote file.\n The server shall place the contents of the field 'data' into the file pointed by 'path' at the offset specified by\n the field 'offset'.\n\n When writing a file, the client should repeatedly call this service with data while advancing the offset until the\n file is written completely. When the write sequence is completed, the client shall call the service one last time,\n with the offset set to the size of the file and with the data field empty, which will signal the server that the\n transfer is finished.\n\n When the write operation is complete, the server shall truncate the resulting file past the specified offset."
            )]
            #[deprecated]
            pub struct WriteRequest {
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub offset: u64,
                ///
                /// `uavcan.file.Path.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 904 bits
                pub path: crate::uavcan::file::path_1_0::Path,
                #[cfg_attr(
                    not(doctest),
                    doc = " 192 = 128 + 64; the write protocol permits usage of smaller chunks."
                )]
                ///
                /// `saturated uint8[<=192]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 1536 bits
                pub data: ::heapless::Vec<u8, 192>,
            }
            impl ::canadensis_encoding::DataType for WriteRequest {
                /// This type is delimited with an extent of 600 bytes.
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
            ///
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            #[deprecated]
            pub struct WriteResponse {
                ///
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for WriteResponse {
                /// This type is delimited with an extent of 48 bytes.
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(409);

            /// `uavcan.file.Write.1.1`
            ///
            /// Size ranges from 8 to 519 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Write into a remote file.\n The server shall place the contents of the field 'data' into the file pointed by 'path' at the offset specified by\n the field 'offset'.\n\n When writing a file, the client should repeatedly call this service with data while advancing the offset until the\n file is written completely. When the write sequence is completed, the client shall call the service one last time,\n with the offset set to the size of the file and with the data field empty, which will signal the server that the\n transfer is finished.\n\n When the write operation is complete, the server shall truncate the resulting file past the specified offset."
            )]
            pub struct WriteRequest {
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub offset: u64,
                ///
                /// `uavcan.file.Path.2.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2048 bits
                pub path: crate::uavcan::file::path_2_0::Path,
                ///
                /// `uavcan.primitive.Unstructured.1.0`
                ///
                /// Always aligned,
                /// size ranges from 16 to 2064 bits
                pub data: crate::uavcan::primitive::unstructured_1_0::Unstructured,
            }
            impl ::canadensis_encoding::DataType for WriteRequest {
                /// This type is delimited with an extent of 600 bytes.
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
            ///
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct WriteResponse {
                ///
                /// `uavcan.file.Error.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for WriteResponse {
                /// This type is delimited with an extent of 48 bytes.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod handle_incoming_packet_0_1 {
                #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
                #[deprecated]
                pub const SERVICE: ::canadensis_core::ServiceId =
                    ::canadensis_core::ServiceId::from_truncating(500);

                /// `uavcan.internet.udp.HandleIncomingPacket.0.1`
                ///
                /// Size ranges from 4 to 313 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " This message carries UDP packets sent from a remote host on the Internet or a LAN to a node on the local Cyphal bus.\n Please refer to the definition of the message type OutgoingPacket for a general overview of the packet forwarding\n logic.\n\n This data type has been made a service type rather than a message type in order to make its transfers addressable,\n allowing nodes to employ hardware acceptance filters for filtering out forwarded datagrams that are not addressed\n to them. Additionally, requiring the destination nodes to always respond upon reception of the forwarded datagram\n opens interesting opportunities for future extensions of the forwarding protocol. If the service invocation times\n out, the modem node is permitted to remove the corresponding entry from the NAT table immediately, not waiting\n for its TTL to expire.\n\n It should be noted that this data type definition intentionally leaves out the source address. This is done in\n order to simplify the implementation, reduce the bus traffic overhead, and because the nature of the\n communication patterns proposed by this set of messages does not provide a valid way to implement server hosts\n on the local Cyphal bus. It is assumed that local nodes can be only clients, and therefore, they will be able to\n determine the address of the sender simply by mapping the field session_id to their internally maintained states.\n Furthermore, it is uncertain what is the optimal way of representing the source address for\n client nodes: it is assumed that the local nodes will mostly use DNS names rather than IP addresses, so if there\n was a source address field, modem nodes would have to perform reverse mapping from the IP address they received\n the datagram from to the corresponding DNS name that was used by the local node with the outgoing message. This\n approach creates a number of troubling corner cases and adds a fair amount of hidden complexities to the\n implementation of modem nodes.\n\n It is recommended to perform service invocations at the same transfer priority level as was used for broadcasting\n the latest matching message of type OutgoingPacket. However, meeting this recommendation would require the modem\n node to implement additional logic, which may be undesirable. Therefore, implementers are free to deviate from\n this recommendation and resort to a fixed priority level instead. In the case of a fixed priority level, it is\n advised to use the lowest transfer priority level."
                )]
                #[deprecated]
                pub struct HandleIncomingPacketRequest {
                    #[cfg_attr(
                        not(doctest),
                        doc = " This field shall contain the same value that was used by the local node when sending the corresponding outgoing\n packet using the message type OutgoingPacket. This value will be used by the local node to match the response\n with its local context."
                    )]
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub session_id: u16,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Effective payload. This data will be forwarded from the remote host verbatim.\n UDP packets that contain more than 508 bytes of payload may be dropped by some types of\n communication equipment. Refer to RFC 791 and 2460 for an in-depth review.\n Cyphal further limits the maximum packet size to reduce the memory and traffic burden on the nodes.\n Datagrams that exceed the capacity of this field should be discarded by the modem node."
                    )]
                    ///
                    /// `saturated uint8[<=309]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2472 bits
                    pub payload: ::heapless::Vec<u8, 309>,
                }
                impl ::canadensis_encoding::DataType for HandleIncomingPacketRequest {
                    /// This type is delimited with an extent of 600 bytes.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                #[deprecated]
                pub struct HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::DataType for HandleIncomingPacketResponse {
                    /// This type is delimited with an extent of 63 bytes.
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
                #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
                pub const SERVICE: ::canadensis_core::ServiceId =
                    ::canadensis_core::ServiceId::from_truncating(500);

                /// `uavcan.internet.udp.HandleIncomingPacket.0.2`
                ///
                /// Size ranges from 4 to 512 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " This message carries UDP packets sent from a remote host on the Internet or a LAN to a node on the local Cyphal bus.\n Please refer to the definition of the message type OutgoingPacket for a general overview of the packet forwarding\n logic.\n\n This data type has been made a service type rather than a message type in order to make its transfers addressable,\n allowing nodes to employ hardware acceptance filters for filtering out forwarded datagrams that are not addressed\n to them. Additionally, requiring the destination nodes to always respond upon reception of the forwarded datagram\n opens interesting opportunities for future extensions of the forwarding protocol. If the service invocation times\n out, the modem node is permitted to remove the corresponding entry from the NAT table immediately, not waiting\n for its TTL to expire.\n\n It should be noted that this data type definition intentionally leaves out the source address. This is done in\n order to simplify the implementation, reduce the bus traffic overhead, and because the nature of the\n communication patterns proposed by this set of messages does not provide a valid way to implement server hosts\n on the local Cyphal bus. It is assumed that local nodes can be only clients, and therefore, they will be able to\n determine the address of the sender simply by mapping the field session_id to their internally maintained states.\n Furthermore, it is uncertain what is the optimal way of representing the source address for\n client nodes: it is assumed that the local nodes will mostly use DNS names rather than IP addresses, so if there\n was a source address field, modem nodes would have to perform reverse mapping from the IP address they received\n the datagram from to the corresponding DNS name that was used by the local node with the outgoing message. This\n approach creates a number of troubling corner cases and adds a fair amount of hidden complexities to the\n implementation of modem nodes.\n\n It is recommended to perform service invocations at the same transfer priority level as was used for broadcasting\n the latest matching message of type OutgoingPacket. However, meeting this recommendation would require the modem\n node to implement additional logic, which may be undesirable. Therefore, implementers are free to deviate from\n this recommendation and resort to a fixed priority level instead. In the case of a fixed priority level, it is\n advised to use the lowest transfer priority level."
                )]
                pub struct HandleIncomingPacketRequest {
                    #[cfg_attr(
                        not(doctest),
                        doc = " This field shall contain the same value that was used by the local node when sending the corresponding outgoing\n packet using the message type OutgoingPacket. This value will be used by the local node to match the response\n with its local context."
                    )]
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub session_id: u16,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Effective payload. This data will be forwarded from the remote host verbatim.\n UDP packets that contain more than 508 bytes of payload may be dropped by some types of\n communication equipment. Refer to RFC 791 and 2460 for an in-depth review.\n Datagrams that exceed the capacity of this field should be discarded by the modem node."
                    )]
                    ///
                    /// `saturated uint8[<=508]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 4064 bits
                    pub payload: ::heapless::Vec<u8, 508>,
                }
                impl ::canadensis_encoding::DataType for HandleIncomingPacketRequest {
                    /// This type is delimited with an extent of 600 bytes.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::DataType for HandleIncomingPacketResponse {
                    /// This type is delimited with an extent of 63 bytes.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod outgoing_packet_0_1 {
                #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
                #[deprecated]
                pub const SUBJECT: ::canadensis_core::SubjectId =
                    ::canadensis_core::SubjectId::from_truncating(8174);

                /// `uavcan.internet.udp.OutgoingPacket.0.1`
                ///
                /// Size ranges from 8 to 313 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " This message carries UDP packets from a node on the local bus to a remote host on the Internet or a LAN.\n\n Any node can broadcast a message of this type.\n\n All nodes that are capable of communication with the Internet or a LAN should subscribe to messages\n of this type and forward the payload to the indicated host and port using exactly one UDP datagram\n per message (i.e. additional fragmentation is to be avoided). Such nodes will be referred to as\n \"modem nodes\".\n\n It is expected that some systems will have more than one modem node available.\n Each modem node is supposed to forward every message it sees, which will naturally create\n some degree of modular redundancy and fault tolerance. The remote host should therefore be able to\n properly handle possibly duplicated messages from different source addresses, in addition to\n possible duplications introduced by the UDP/IP protocol itself. There are at least two obvious\n strategies that can be employed by the remote host:\n\n   - Accept only the first message, ignore duplicates. This approach requires that the UDP stream\n     should contain some metadata necessary for the remote host to determine the source and ordering\n     of each received datum. This approach works best for periodic data, such as telemetry, where\n     the sender does not expect any responses.\n\n   - Process all messages, including duplicates. This approach assumes that the remote host acts\n     as a server, processing all received requests and providing responses to each. This arrangement\n     implies that the client may receive duplicated responses. It is therefore the client's\n     responsibility to resolve the possible ambiguity. An obvious solution is to accept the first\n     arrived response and ignore the later ones.\n\n Applications are free to choose whatever redundancy management strategy works best for them.\n\n If the source node expects that the remote host will send some data back, it shall explicitly notify\n the modem nodes about this, so that they could prepare to perform reverse forwarding when the\n expected data arrives from the remote host. The technique of reverse forwarding is known in\n networking as IP Masquerading, or (in general) Network Address Translation (NAT). The notification\n is performed by means of setting one of the corresponding flags defined below.\n\n In order to be able to match datagrams received from remote hosts and the local nodes they should\n be forwarded to, modem nodes are required to keep certain metadata about outgoing datagrams. Such\n metadata is stored in a data structure referred to as \"NAT table\", where every entry would normally\n contain at least the following fields:\n   - The local UDP port number that was used to send the outgoing datagram from.\n     Per RFC 4787, the port number is chosen by the modem node automatically.\n   - The node-ID of the local node that has sent the outgoing datagram.\n   - Value of the field session_id defined below.\n   - Possibly some other data, depending on the implementation.\n\n The modem nodes are required to keep each NAT table entry for at least NAT_ENTRY_MIN_TTL seconds\n since the last reverse forwarding action was performed. Should the memory resources of the modem node\n be exhausted, it is allowed to remove old NAT entries earlier, following the policy of least recent use.\n\n Having received a UDP packet from a remote host, the modem node would check the NAT table in order\n to determine where on the Cyphal bus the received data should be forwarded to. If the NAT table\n contains no matches, the received data should be silently dropped. If a match is found, the\n modem node will forward the data to the recipient node using the service HandleIncomingPacket.\n If the service invocation times out, the modem node is permitted to remove the corresponding entry from\n the NAT table immediately (but it is not required). This will ensure that the modem nodes will not be\n tasked with translations for client nodes that are no longer online or are unreachable.\n Additionally, client nodes will be able to hint the modem nodes to remove translation entries they no\n longer need by simply refusing to respond to the corresponding service invocation. Please refer to\n the definition of that service data type for a more in-depth review of the reverse forwarding process.\n\n Modem nodes can also perform traffic shaping, if needed, by means of delaying or dropping UDP\n datagrams that exceed the quota.\n\n To summarize, a typical data exchange occurrence should amount to the following actions:\n\n   - A local Cyphal node broadcasts a message of type OutgoingPacket with the payload it needs\n     to forward. If the node expects the remote host to send any data back, it sets the masquerading flag.\n\n   - Every modem node on the bus receives the message and performs the following actions:\n\n       - The domain name is resolved, unless the destination address provided in the message\n         is already an IP address, in which case this step should be skipped.\n\n       - The domain name to IP address mapping is added to the local DNS cache, although this\n         part is entirely implementation defined and is not required.\n\n       - The masquerading flag is checked. If it is set, a new entry is added to the NAT table.\n         If such entry already existed, its expiration timeout is reset. If no such entry existed\n         and a new one cannot be added because of memory limitations, the least recently used\n         (i.e. oldest) entry of the NAT table is replaced with the new one.\n\n       - The payload is forwarded to the determined IP address.\n\n   - At this point, direct forwarding is complete. Should any of the modem nodes receive an incoming\n     packet, they would attempt to perform a reverse forwarding according to the above provided algorithm.\n\n It is recommended to use the lowest transport priority level when broadcasting messages of this type,\n in order to avoid interference with a real-time traffic on the bus. Usage of higher priority levels is\n unlikely to be practical because the latency and throughput limitations introduced by the on-board radio\n communication equipment are likely to vastly exceed those of the local CAN bus."
                )]
                #[deprecated]
                pub struct OutgoingPacket {
                    #[cfg_attr(
                        not(doctest),
                        doc = " This field is set to an arbitrary value by the transmitting node in order to be able to match the response\n with the locally kept context. The function of this field is virtually identical to that of UDP/IP port\n numbers. This value can be set to zero safely if the sending node does not have multiple contexts to\n distinguish between."
                    )]
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub session_id: u16,
                    #[cfg_attr(not(doctest), doc = " UDP destination port number.")]
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub destination_port: u16,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Domain name or IP address where the payload should be forwarded to.\n Note that broadcast addresses are allowed here, for example, 255.255.255.255.\n Broadcasting with masquerading enabled works the same way as unicasting with masquerading enabled: the modem\n node should take care to channel all traffic arriving at the opened port from any source to the node that\n requested masquerading.\n The full domain name length may not exceed 253 octets, according to the DNS specification.\n Cyphal imposes a stricter length limit in order to reduce the memory and traffic burden on the bus: 45 characters.\n 45 characters is the amount of space that is required to represent the longest possible form of an IPv6 address\n (an IPv4-mapped IPv6 address). Examples:\n   \"forum.opencyphal.org\"                          - domain name\n   \"192.168.1.1\"                                   - IPv4 address\n   \"2001:0db8:85a3:0000:0000:8a2e:0370:7334\"       - IPv6 address, full form\n   \"2001:db8:85a3::8a2e:370:7334\"                  - IPv6 address, same as above, short form (preferred)\n   \"ABCD:ABCD:ABCD:ABCD:ABCD:ABCD:192.168.158.190\" - IPv4-mapped IPv6, full form (length limit, 45 characters)"
                    )]
                    ///
                    /// `saturated uint8[<=45]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 360 bits
                    pub destination_address: ::heapless::Vec<u8, 45>,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Expect data back (i.e., instruct the modem to use the NAT table)."
                    )]
                    ///
                    /// `saturated bool`
                    ///
                    /// Always aligned,
                    /// size 1 bits
                    pub use_masquerading: bool,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Use Datagram Transport Layer Security. Drop the packet if DTLS is not supported.\n Option flags."
                    )]
                    ///
                    /// `saturated bool`
                    ///
                    /// Not always aligned,
                    /// size 1 bits
                    pub use_dtls: bool,
                    // 6 bits of padding
                    #[cfg_attr(
                        not(doctest),
                        doc = " Effective payload. This data will be forwarded to the remote host verbatim.\n UDP packets that contain more than 508 bytes of payload may be dropped by some types of\n communication equipment. Refer to RFC 791 and 2460 for an in-depth review.\n Cyphal further limits the maximum packet size to reduce the memory and traffic burden on the nodes."
                    )]
                    ///
                    /// `saturated uint8[<=260]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2080 bits
                    pub payload: ::heapless::Vec<u8, 260>,
                }
                impl ::canadensis_encoding::DataType for OutgoingPacket {
                    /// This type is delimited with an extent of 600 bytes.
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Message for OutgoingPacket {}
                impl OutgoingPacket {
                    #[cfg_attr(
                        not(doctest),
                        doc = " [second]\n Modem nodes are required to keep the NAT table entries alive for at least this amount of time, unless the\n table is overflowed, in which case they are allowed to remove least recently used entries in favor of\n newer ones. Modem nodes are required to be able to accommodate at least 100 entries in the NAT table."
                    )]
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
                #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
                pub const SUBJECT: ::canadensis_core::SubjectId =
                    ::canadensis_core::SubjectId::from_truncating(8174);

                /// `uavcan.internet.udp.OutgoingPacket.0.2`
                ///
                /// Size ranges from 8 to 561 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " This message carries UDP packets from a node on the local bus to a remote host on the Internet or a LAN.\n\n Any node can broadcast a message of this type.\n\n All nodes that are capable of communication with the Internet or a LAN should subscribe to messages\n of this type and forward the payload to the indicated host and port using exactly one UDP datagram\n per message (i.e. additional fragmentation is to be avoided). Such nodes will be referred to as\n \"modem nodes\".\n\n It is expected that some systems will have more than one modem node available.\n Each modem node is supposed to forward every message it sees, which will naturally create\n some degree of modular redundancy and fault tolerance. The remote host should therefore be able to\n properly handle possibly duplicated messages from different source addresses, in addition to\n possible duplications introduced by the UDP/IP protocol itself. There are at least two obvious\n strategies that can be employed by the remote host:\n\n   - Accept only the first message, ignore duplicates. This approach requires that the UDP stream\n     should contain some metadata necessary for the remote host to determine the source and ordering\n     of each received datum. This approach works best for periodic data, such as telemetry, where\n     the sender does not expect any responses.\n\n   - Process all messages, including duplicates. This approach assumes that the remote host acts\n     as a server, processing all received requests and providing responses to each. This arrangement\n     implies that the client may receive duplicated responses. It is therefore the client's\n     responsibility to resolve the possible ambiguity. An obvious solution is to accept the first\n     arrived response and ignore the later ones.\n\n Applications are free to choose whatever redundancy management strategy works best for them.\n\n If the source node expects that the remote host will send some data back, it shall explicitly notify\n the modem nodes about this, so that they could prepare to perform reverse forwarding when the\n expected data arrives from the remote host. The technique of reverse forwarding is known in\n networking as IP Masquerading, or (in general) Network Address Translation (NAT). The notification\n is performed by means of setting one of the corresponding flags defined below.\n\n In order to be able to match datagrams received from remote hosts and the local nodes they should\n be forwarded to, modem nodes are required to keep certain metadata about outgoing datagrams. Such\n metadata is stored in a data structure referred to as \"NAT table\", where every entry would normally\n contain at least the following fields:\n   - The local UDP port number that was used to send the outgoing datagram from.\n     Per RFC 4787, the port number is chosen by the modem node automatically.\n   - The node-ID of the local node that has sent the outgoing datagram.\n   - Value of the field session_id defined below.\n   - Possibly some other data, depending on the implementation.\n\n The modem nodes are required to keep each NAT table entry for at least NAT_ENTRY_MIN_TTL seconds\n since the last reverse forwarding action was performed. Should the memory resources of the modem node\n be exhausted, it is allowed to remove old NAT entries earlier, following the policy of least recent use.\n\n Having received a UDP packet from a remote host, the modem node would check the NAT table in order\n to determine where on the Cyphal bus the received data should be forwarded to. If the NAT table\n contains no matches, the received data should be silently dropped. If a match is found, the\n modem node will forward the data to the recipient node using the service HandleIncomingPacket.\n If the service invocation times out, the modem node is permitted to remove the corresponding entry from\n the NAT table immediately (but it is not required). This will ensure that the modem nodes will not be\n tasked with translations for client nodes that are no longer online or are unreachable.\n Additionally, client nodes will be able to hint the modem nodes to remove translation entries they no\n longer need by simply refusing to respond to the corresponding service invocation. Please refer to\n the definition of that service data type for a more in-depth review of the reverse forwarding process.\n\n Modem nodes can also perform traffic shaping, if needed, by means of delaying or dropping UDP\n datagrams that exceed the quota.\n\n To summarize, a typical data exchange occurrence should amount to the following actions:\n\n   - A local Cyphal node broadcasts a message of type OutgoingPacket with the payload it needs\n     to forward. If the node expects the remote host to send any data back, it sets the masquerading flag.\n\n   - Every modem node on the bus receives the message and performs the following actions:\n\n       - The domain name is resolved, unless the destination address provided in the message\n         is already an IP address, in which case this step should be skipped.\n\n       - The domain name to IP address mapping is added to the local DNS cache, although this\n         part is entirely implementation defined and is not required.\n\n       - The masquerading flag is checked. If it is set, a new entry is added to the NAT table.\n         If such entry already existed, its expiration timeout is reset. If no such entry existed\n         and a new one cannot be added because of memory limitations, the least recently used\n         (i.e. oldest) entry of the NAT table is replaced with the new one.\n\n       - The payload is forwarded to the determined IP address.\n\n   - At this point, direct forwarding is complete. Should any of the modem nodes receive an incoming\n     packet, they would attempt to perform a reverse forwarding according to the above provided algorithm.\n\n It is recommended to use the lowest transport priority level when broadcasting messages of this type,\n in order to avoid interference with a real-time traffic on the bus. Usage of higher priority levels is\n unlikely to be practical because the latency and throughput limitations introduced by the on-board radio\n communication equipment are likely to vastly exceed those of the local CAN bus."
                )]
                pub struct OutgoingPacket {
                    #[cfg_attr(
                        not(doctest),
                        doc = " This field is set to an arbitrary value by the transmitting node in order to be able to match the response\n with the locally kept context. The function of this field is virtually identical to that of UDP/IP port\n numbers. This value can be set to zero safely if the sending node does not have multiple contexts to\n distinguish between."
                    )]
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub session_id: u16,
                    #[cfg_attr(not(doctest), doc = " UDP destination port number.")]
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub destination_port: u16,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Domain name or IP address where the payload should be forwarded to.\n Note that broadcast addresses are allowed here, for example, 255.255.255.255.\n Broadcasting with masquerading enabled works the same way as unicasting with masquerading enabled: the modem\n node should take care to channel all traffic arriving at the opened port from any source to the node that\n requested masquerading.\n The full domain name length may not exceed 253 octets, according to the DNS specification.\n Cyphal imposes a stricter length limit in order to reduce the memory and traffic burden on the bus: 45 characters.\n 45 characters is the amount of space that is required to represent the longest possible form of an IPv6 address\n (an IPv4-mapped IPv6 address). Examples:\n   \"forum.opencyphal.org\"                          - domain name\n   \"192.168.1.1\"                                   - IPv4 address\n   \"2001:0db8:85a3:0000:0000:8a2e:0370:7334\"       - IPv6 address, full form\n   \"2001:db8:85a3::8a2e:370:7334\"                  - IPv6 address, same as above, short form (preferred)\n   \"ABCD:ABCD:ABCD:ABCD:ABCD:ABCD:192.168.158.190\" - IPv4-mapped IPv6, full form (length limit, 45 characters)"
                    )]
                    ///
                    /// `saturated uint8[<=45]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 360 bits
                    pub destination_address: ::heapless::Vec<u8, 45>,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Expect data back (i.e., instruct the modem to use the NAT table)."
                    )]
                    ///
                    /// `saturated bool`
                    ///
                    /// Always aligned,
                    /// size 1 bits
                    pub use_masquerading: bool,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Use Datagram Transport Layer Security. Drop the packet if DTLS is not supported.\n Option flags."
                    )]
                    ///
                    /// `saturated bool`
                    ///
                    /// Not always aligned,
                    /// size 1 bits
                    pub use_dtls: bool,
                    // 6 bits of padding
                    #[cfg_attr(
                        not(doctest),
                        doc = " Effective payload. This data will be forwarded to the remote host verbatim.\n UDP packets that contain more than 508 bytes of payload may be dropped by some types of\n communication equipment. Refer to RFC 791 and 2460 for an in-depth review."
                    )]
                    ///
                    /// `saturated uint8[<=508]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 4064 bits
                    pub payload: ::heapless::Vec<u8, 508>,
                }
                impl ::canadensis_encoding::DataType for OutgoingPacket {
                    /// This type is delimited with an extent of 600 bytes.
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Message for OutgoingPacket {}
                impl OutgoingPacket {
                    #[cfg_attr(
                        not(doctest),
                        doc = " [second]\n Modem nodes are required to keep the NAT table entries alive for at least this amount of time, unless the\n table is overflowed, in which case they are allowed to remove least recently used entries in favor of\n newer ones. Modem nodes are required to be able to accommodate at least 100 entries in the NAT table."
                    )]
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
                ///
                #[cfg_attr(not(doctest), doc = " CAN frame arbitration field.")]
                pub enum ArbitrationID {
                    ///
/// uavcan.metatransport.can.BaseArbitrationID.0.1
Base
(crate::uavcan::metatransport::can::base_arbitration_id_0_1::BaseArbitrationID)
,///
/// uavcan.metatransport.can.ExtendedArbitrationID.0.1
Extended
(crate::uavcan::metatransport::can::extended_arbitration_id_0_1::ExtendedArbitrationID)
,}
                impl ::canadensis_encoding::DataType for ArbitrationID {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(not(doctest), doc = " 11-bit identifier.")]
                pub struct BaseArbitrationID {
                    ///
                    /// `truncated uint11`
                    ///
                    /// Always aligned,
                    /// size 11 bits
                    pub value: u16,
                    // 21 bits of padding
                }
                impl ::canadensis_encoding::DataType for BaseArbitrationID {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(not(doctest), doc = " Classic data frame payload.")]
                pub struct DataClassic {
                    ///
                    /// `uavcan.metatransport.can.ArbitrationID.0.1`
                    ///
                    /// Always aligned,
                    /// size 40 bits
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                    ///
                    /// `saturated uint8[<=8]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 64 bits
                    pub data: ::heapless::Vec<u8, 8>,
                }
                impl ::canadensis_encoding::DataType for DataClassic {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(not(doctest), doc = " CAN FD data frame payload.")]
                pub struct DataFD {
                    ///
                    /// `uavcan.metatransport.can.ArbitrationID.0.1`
                    ///
                    /// Always aligned,
                    /// size 40 bits
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                    ///
                    /// `saturated uint8[<=64]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 512 bits
                    pub data: ::heapless::Vec<u8, 64>,
                }
                impl ::canadensis_encoding::DataType for DataFD {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " CAN bus error report: either an intentionally generated error frame or a disturbance."
                )]
                pub struct Error {
                    // 32 bits of padding
                }
                impl ::canadensis_encoding::DataType for Error {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(not(doctest), doc = " 29-bit identifier.")]
                pub struct ExtendedArbitrationID {
                    ///
                    /// `truncated uint29`
                    ///
                    /// Always aligned,
                    /// size 29 bits
                    pub value: u32,
                    // 3 bits of padding
                }
                impl ::canadensis_encoding::DataType for ExtendedArbitrationID {
                    /// This type is sealed.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod frame_0_1 {
                /// `uavcan.metatransport.can.Frame.0.1`
                ///
                /// Size ranges from 12 to 78 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " CAN 2.0 or CAN FD frame representation. This is the top-level data type in its namespace.\n See next version."
                )]
                #[deprecated]
                pub struct Frame {
                    ///
                    /// `uavcan.time.SynchronizedTimestamp.1.0`
                    ///
                    /// Always aligned,
                    /// size 56 bits
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    ///
                    /// `uavcan.metatransport.can.Manifestation.0.1`
                    ///
                    /// Always aligned,
                    /// size ranges from 40 to 568 bits
                    pub manifestation:
                        crate::uavcan::metatransport::can::manifestation_0_1::Manifestation,
                }
                impl ::canadensis_encoding::DataType for Frame {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " Classic CAN or CAN FD frame representation. This is the top-level data type in its namespace."
                )]
                pub enum Frame {
                    #[cfg_attr(not(doctest), doc = " CAN error (intentional or disturbance)")]
                    ///
                    /// uavcan.metatransport.can.Error.0.1
                    Error(crate::uavcan::metatransport::can::error_0_1::Error),
                    #[cfg_attr(not(doctest), doc = " Bit rate switch flag active")]
                    ///
                    /// uavcan.metatransport.can.DataFD.0.1
                    DataFd(crate::uavcan::metatransport::can::data_fd_0_1::DataFD),
                    #[cfg_attr(not(doctest), doc = " Bit rate switch flag not active")]
                    ///
                    /// uavcan.metatransport.can.DataClassic.0.1
                    DataClassic(crate::uavcan::metatransport::can::data_classic_0_1::DataClassic),
                    #[cfg_attr(
                        not(doctest),
                        doc = " Bit rate switch flag not active\n Sealed because the structure is rigidly dictated by an external standard."
                    )]
                    ///
                    /// uavcan.metatransport.can.RTR.0.1
                    RemoteTransmissionRequest(crate::uavcan::metatransport::can::rtr_0_1::RTR),
                }
                impl ::canadensis_encoding::DataType for Frame {
                    /// This type is sealed.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod manifestation_0_1 {
                /// `uavcan.metatransport.can.Manifestation.0.1`
                ///
                /// Size ranges from 5 to 71 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " CAN frame properties that can be manifested on the bus.\n See Frame.0.2 as a replacement"
                )]
                #[deprecated]
                pub enum Manifestation {
                    #[cfg_attr(not(doctest), doc = " CAN error (intentional or disturbance)")]
                    ///
                    /// uavcan.metatransport.can.Error.0.1
                    Error(crate::uavcan::metatransport::can::error_0_1::Error),
                    #[cfg_attr(not(doctest), doc = " Bit rate switch flag active")]
                    ///
                    /// uavcan.metatransport.can.DataFD.0.1
                    DataFd(crate::uavcan::metatransport::can::data_fd_0_1::DataFD),
                    #[cfg_attr(not(doctest), doc = " Bit rate switch flag not active")]
                    ///
                    /// uavcan.metatransport.can.DataClassic.0.1
                    DataClassic(crate::uavcan::metatransport::can::data_classic_0_1::DataClassic),
                    #[cfg_attr(not(doctest), doc = " Bit rate switch flag not active")]
                    ///
                    /// uavcan.metatransport.can.RTR.0.1
                    RemoteTransmissionRequest(crate::uavcan::metatransport::can::rtr_0_1::RTR),
                }
                impl ::canadensis_encoding::DataType for Manifestation {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " Classic remote transmission request (not defined for CAN FD)."
                )]
                pub struct RTR {
                    ///
                    /// `uavcan.metatransport.can.ArbitrationID.0.1`
                    ///
                    /// Always aligned,
                    /// size 40 bits
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                }
                impl ::canadensis_encoding::DataType for RTR {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " Standard EtherType constants as defined by IEEE Registration Authority and IANA.\n This list is only a small subset of constants that are considered to be relevant for Cyphal."
                )]
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct EtherType {
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for EtherType {
                    /// This type is sealed.
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for EtherType {}
                impl EtherType {
                    pub const IP_V4: u16 = 2048;
                    pub const ARP: u16 = 2054;
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " IEEE 802.3 Ethernet frame encapsulation.\n In terms of libpcap/tcpdump, the corresponding link type is LINKTYPE_ETHERNET/DLT_EN10MB."
                )]
                pub struct Frame {
                    ///
                    /// `saturated uint8[6]`
                    ///
                    /// Always aligned,
                    /// size 48 bits
                    pub destination: [u8; 6],
                    ///
                    /// `saturated uint8[6]`
                    ///
                    /// Always aligned,
                    /// size 48 bits
                    pub source: [u8; 6],
                    ///
                    /// `uavcan.metatransport.ethernet.EtherType.0.1`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub ethertype:
                        crate::uavcan::metatransport::ethernet::ether_type_0_1::EtherType,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Supports conventional jumbo frames (up to 9 KiB)."
                    )]
                    ///
                    /// `saturated uint8[<=9216]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 73728 bits
                    pub payload: ::heapless::Vec<u8, 9216>,
                }
                impl ::canadensis_encoding::DataType for Frame {
                    /// This type is sealed.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod fragment_0_1 {
                /// `uavcan.metatransport.serial.Fragment.0.1`
                ///
                /// Size ranges from 9 to 265 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A chunk of raw bytes exchanged over a serial transport. Serial links do not support framing natively.\n The chunk may be of arbitrary size.\n See next version."
                )]
                #[deprecated]
                pub struct Fragment {
                    ///
                    /// `uavcan.time.SynchronizedTimestamp.1.0`
                    ///
                    /// Always aligned,
                    /// size 56 bits
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    ///
                    /// `saturated uint8[<=256]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub data: ::heapless::Vec<u8, 256>,
                }
                impl ::canadensis_encoding::DataType for Fragment {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A chunk of raw bytes exchanged over a serial transport. Serial links do not support framing natively.\n The chunk may be of arbitrary size.\n\n If this data type is used to encapsulate Cyphal/serial, then it is recommended to ensure that each message\n contains at most one Cyphal/serial transport frame (frames are separated by zero-valued delimiter bytes)."
                )]
                pub struct Fragment {
                    ///
                    /// `saturated uint8[<=2048]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 16384 bits
                    pub data: ::heapless::Vec<u8, 2048>,
                }
                impl ::canadensis_encoding::DataType for Fragment {
                    /// This type is sealed.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod endpoint_0_1 {
                /// `uavcan.metatransport.udp.Endpoint.0.1`
                ///
                /// Fixed size 32 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A UDP/IP endpoint address specification.\n Replaced by uavcan.metatransport.ethernet"
                )]
                #[deprecated]
                pub struct Endpoint {
                    #[cfg_attr(
                        not(doctest),
                        doc = " The IP address of the host in the network byte order (big endian).\n IPv6 addresses are represented as-is.\n IPv4 addresses are represented using IPv4-mapped IPv6 addresses."
                    )]
                    ///
                    /// `saturated uint8[16]`
                    ///
                    /// Always aligned,
                    /// size 128 bits
                    pub ip_address: [u8; 16],
                    #[cfg_attr(
                        not(doctest),
                        doc = " MAC address of the host in the network byte order (big endian)."
                    )]
                    ///
                    /// `saturated uint8[6]`
                    ///
                    /// Always aligned,
                    /// size 48 bits
                    pub mac_address: [u8; 6],
                    #[cfg_attr(not(doctest), doc = " The UDP port number.")]
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub port: u16,
                    // 64 bits of padding
                }
                impl ::canadensis_encoding::DataType for Endpoint {
                    /// This type is sealed.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod frame_0_1 {
                /// `uavcan.metatransport.udp.Frame.0.1`
                ///
                /// Size ranges from 74 to 9262 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A generic UDP/IP frame.\n Jumboframes are supported in the interest of greater application compatibility.\n Replaced by uavcan.metatransport.ethernet"
                )]
                #[deprecated]
                pub struct Frame {
                    ///
                    /// `uavcan.time.SynchronizedTimestamp.1.0`
                    ///
                    /// Always aligned,
                    /// size 56 bits
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    // 8 bits of padding
                    ///
                    /// `uavcan.metatransport.udp.Endpoint.0.1`
                    ///
                    /// Always aligned,
                    /// size 256 bits
                    pub source: crate::uavcan::metatransport::udp::endpoint_0_1::Endpoint,
                    ///
                    /// `uavcan.metatransport.udp.Endpoint.0.1`
                    ///
                    /// Always aligned,
                    /// size 256 bits
                    pub destination: crate::uavcan::metatransport::udp::endpoint_0_1::Endpoint,
                    ///
                    /// `saturated uint8[<=9188]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 73504 bits
                    pub data: ::heapless::Vec<u8, 9188>,
                }
                impl ::canadensis_encoding::DataType for Frame {
                    /// This type is delimited with an extent of 10240 bytes.
                    const EXTENT_BYTES: Option<u32> = Some(10240);
                }
                impl ::canadensis_encoding::Message for Frame {}
                impl Frame {
                    #[cfg_attr(
                        not(doctest),
                        doc = " Max jumbo frame 9 KiB, IP header min 20 B, UDP header 8 B."
                    )]
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
        #[allow(deprecated)]
        #[deprecated]
        pub mod execute_command_1_0 {
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            #[deprecated]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(435);

            /// `uavcan.node.ExecuteCommand.1.0`
            ///
            /// Size ranges from 3 to 115 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Instructs the server node to execute or commence execution of a simple predefined command.\n All standard commands are optional; i.e., not guaranteed to be supported by all nodes."
            )]
            #[deprecated]
            pub struct ExecuteCommandRequest {
                #[cfg_attr(
                    not(doctest),
                    doc = " Standard pre-defined commands are at the top of the range (defined below).\n Vendors can define arbitrary, vendor-specific commands in the bottom part of the range (starting from zero).\n Vendor-specific commands shall not use identifiers above 32767."
                )]
                ///
                /// `saturated uint16`
                ///
                /// Always aligned,
                /// size 16 bits
                pub command: u16,
                #[cfg_attr(
                    not(doctest),
                    doc = " A string parameter supplied to the command. The format and interpretation is command-specific.\n The standard commands do not use this field (ignore it), excepting the following:\n   - COMMAND_BEGIN_SOFTWARE_UPDATE\n Two CAN FD frames max"
                )]
                ///
                /// `saturated uint8[<=112]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 896 bits
                pub parameter: ::heapless::Vec<u8, 112>,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandRequest {
                /// This type is delimited with an extent of 300 bytes.
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ExecuteCommandRequest {}
            impl ExecuteCommandRequest {
                #[cfg_attr(
                    not(doctest),
                    doc = " Reboot the node.\n Note that some standard commands may or may not require a restart in order to take effect; e.g., factory reset."
                )]
                pub const COMMAND_RESTART: u16 = 65535;
                #[cfg_attr(
                    not(doctest),
                    doc = " Shut down the node; further access will not be possible until the power is turned back on."
                )]
                pub const COMMAND_POWER_OFF: u16 = 65534;
                #[cfg_attr(
                    not(doctest),
                    doc = " Begin the software update process using uavcan.file.Read. This command makes use of the \"parameter\" field below.\n The parameter contains the path to the new software image file to be downloaded by the server from the client\n using the standard service uavcan.file.Read. Observe that this operation swaps the roles of the client and\n the server.\n\n Upon reception of this command, the server (updatee) will evaluate whether it is possible to begin the\n software update process. If that is deemed impossible, the command will be rejected with one of the\n error codes defined in the response section of this definition (e.g., BAD_STATE if the node is currently\n on-duty and a sudden interruption of its activities is considered unsafe, and so on).\n If an update process is already underway, the updatee should abort the process and restart with the new file,\n unless the updatee can determine that the specified file is the same file that is already being downloaded,\n in which case it is allowed to respond SUCCESS and continue the old update process.\n If there are no other conditions precluding the requested update, the updatee will return a SUCCESS and\n initiate the file transfer process by invoking the standard service uavcan.file.Read repeatedly until the file\n is transferred fully (please refer to the documentation for that data type for more information about its usage).\n\n While the software is being updated, the updatee should set its mode (the field \"mode\" in uavcan.node.Heartbeat)\n to MODE_SOFTWARE_UPDATE. Please refer to the documentation for uavcan.node.Heartbeat for more information.\n\n It is recognized that most systems will have to interrupt their normal services to perform the software update\n (unless some form of software hot swapping is implemented, as is the case in some high-availability systems).\n\n Microcontrollers that are requested to update their firmware may need to stop execution of their current firmware\n and start the embedded bootloader (although other approaches are possible as well). In that case,\n while the embedded bootloader is running, the mode reported via the message uavcan.node.Heartbeat should be\n MODE_SOFTWARE_UPDATE as long as the bootloader is runing, even if no update-related activities\n are currently underway. For example, if the update process failed and the bootloader cannot load the software,\n the same mode MODE_SOFTWARE_UPDATE will be reported.\n It is also recognized that in a microcontroller setting, the application that served the update request will have\n to pass the update-related metadata (such as the node-ID of the server and the firmware image file path) to\n the embedded bootloader. The tactics of that transaction lie outside of the scope of this specification."
                )]
                pub const COMMAND_BEGIN_SOFTWARE_UPDATE: u16 = 65533;
                #[cfg_attr(
                    not(doctest),
                    doc = " Return the node's configuration back to the factory default settings (may require restart).\n Due to the uncertainty whether a restart is required, generic interfaces should always force a restart."
                )]
                pub const COMMAND_FACTORY_RESET: u16 = 65532;
                #[cfg_attr(
                    not(doctest),
                    doc = " Cease activities immediately, enter a safe state until restarted.\n Further operation may no longer be possible until a restart command is executed."
                )]
                pub const COMMAND_EMERGENCY_STOP: u16 = 65531;
                #[cfg_attr(
                    not(doctest),
                    doc = " This command instructs the node to store the current configuration parameter values and other persistent states\n to the non-volatile storage. Nodes are allowed to manage persistent states automatically, obviating the need for\n this command by committing all such data to the non-volatile memory automatically as necessary. However, some\n nodes may lack this functionality, in which case this parameter should be used. Generic interfaces should always\n invoke this command in order to ensure that the data is stored even if the node doesn't implement automatic\n persistence management."
                )]
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
            ///
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            #[deprecated]
            pub struct ExecuteCommandResponse {
                #[cfg_attr(not(doctest), doc = " The result of the request.")]
                ///
                /// `saturated uint8`
                ///
                /// Always aligned,
                /// size 8 bits
                pub status: u8,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandResponse {
                /// This type is delimited with an extent of 48 bytes.
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ExecuteCommandResponse {}
            impl ExecuteCommandResponse {
                #[cfg_attr(not(doctest), doc = " Started or executed successfully")]
                pub const STATUS_SUCCESS: u8 = 0;
                #[cfg_attr(
                    not(doctest),
                    doc = " Could not start or the desired outcome could not be reached"
                )]
                pub const STATUS_FAILURE: u8 = 1;
                #[cfg_attr(not(doctest), doc = " Denied due to lack of authorization")]
                pub const STATUS_NOT_AUTHORIZED: u8 = 2;
                #[cfg_attr(
                    not(doctest),
                    doc = " The requested command is not known or not supported"
                )]
                pub const STATUS_BAD_COMMAND: u8 = 3;
                #[cfg_attr(
                    not(doctest),
                    doc = " The supplied parameter cannot be used with the selected command"
                )]
                pub const STATUS_BAD_PARAMETER: u8 = 4;
                #[cfg_attr(
                    not(doctest),
                    doc = " The current state of the node does not permit execution of this command"
                )]
                pub const STATUS_BAD_STATE: u8 = 5;
                #[cfg_attr(
                    not(doctest),
                    doc = " The operation should have succeeded but an unexpected failure occurred"
                )]
                pub const STATUS_INTERNAL_ERROR: u8 = 6;
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(435);

            /// `uavcan.node.ExecuteCommand.1.1`
            ///
            /// Size ranges from 3 to 258 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Instructs the server node to execute or commence execution of a simple predefined command.\n All standard commands are optional; i.e., not guaranteed to be supported by all nodes."
            )]
            pub struct ExecuteCommandRequest {
                #[cfg_attr(
                    not(doctest),
                    doc = " Standard pre-defined commands are at the top of the range (defined below).\n Vendors can define arbitrary, vendor-specific commands in the bottom part of the range (starting from zero).\n Vendor-specific commands shall not use identifiers above 32767."
                )]
                ///
                /// `saturated uint16`
                ///
                /// Always aligned,
                /// size 16 bits
                pub command: u16,
                #[cfg_attr(
                    not(doctest),
                    doc = " A string parameter supplied to the command. The format and interpretation is command-specific.\n The standard commands do not use this field (ignore it), excepting the following:\n   - COMMAND_BEGIN_SOFTWARE_UPDATE"
                )]
                ///
                /// `saturated uint8[<=255]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 2040 bits
                pub parameter: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandRequest {
                /// This type is delimited with an extent of 300 bytes.
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ExecuteCommandRequest {}
            impl ExecuteCommandRequest {
                #[cfg_attr(
                    not(doctest),
                    doc = " Reboot the node.\n Note that some standard commands may or may not require a restart in order to take effect; e.g., factory reset."
                )]
                pub const COMMAND_RESTART: u16 = 65535;
                #[cfg_attr(
                    not(doctest),
                    doc = " Shut down the node; further access will not be possible until the power is turned back on."
                )]
                pub const COMMAND_POWER_OFF: u16 = 65534;
                #[cfg_attr(
                    not(doctest),
                    doc = " Begin the software update process using uavcan.file.Read. This command makes use of the \"parameter\" field below.\n The parameter contains the path to the new software image file to be downloaded by the server from the client\n using the standard service uavcan.file.Read. Observe that this operation swaps the roles of the client and\n the server.\n\n Upon reception of this command, the server (updatee) will evaluate whether it is possible to begin the\n software update process. If that is deemed impossible, the command will be rejected with one of the\n error codes defined in the response section of this definition (e.g., BAD_STATE if the node is currently\n on-duty and a sudden interruption of its activities is considered unsafe, and so on).\n If an update process is already underway, the updatee should abort the process and restart with the new file,\n unless the updatee can determine that the specified file is the same file that is already being downloaded,\n in which case it is allowed to respond SUCCESS and continue the old update process.\n If there are no other conditions precluding the requested update, the updatee will return a SUCCESS and\n initiate the file transfer process by invoking the standard service uavcan.file.Read repeatedly until the file\n is transferred fully (please refer to the documentation for that data type for more information about its usage).\n\n While the software is being updated, the updatee should set its mode (the field \"mode\" in uavcan.node.Heartbeat)\n to MODE_SOFTWARE_UPDATE. Please refer to the documentation for uavcan.node.Heartbeat for more information.\n\n It is recognized that most systems will have to interrupt their normal services to perform the software update\n (unless some form of software hot swapping is implemented, as is the case in some high-availability systems).\n\n Microcontrollers that are requested to update their firmware may need to stop execution of their current firmware\n and start the embedded bootloader (although other approaches are possible as well). In that case,\n while the embedded bootloader is running, the mode reported via the message uavcan.node.Heartbeat should be\n MODE_SOFTWARE_UPDATE as long as the bootloader is runing, even if no update-related activities\n are currently underway. For example, if the update process failed and the bootloader cannot load the software,\n the same mode MODE_SOFTWARE_UPDATE will be reported.\n It is also recognized that in a microcontroller setting, the application that served the update request will have\n to pass the update-related metadata (such as the node-ID of the server and the firmware image file path) to\n the embedded bootloader. The tactics of that transaction lie outside of the scope of this specification."
                )]
                pub const COMMAND_BEGIN_SOFTWARE_UPDATE: u16 = 65533;
                #[cfg_attr(
                    not(doctest),
                    doc = " Return the node's configuration back to the factory default settings (may require restart).\n Due to the uncertainty whether a restart is required, generic interfaces should always force a restart."
                )]
                pub const COMMAND_FACTORY_RESET: u16 = 65532;
                #[cfg_attr(
                    not(doctest),
                    doc = " Cease activities immediately, enter a safe state until restarted.\n Further operation may no longer be possible until a restart command is executed."
                )]
                pub const COMMAND_EMERGENCY_STOP: u16 = 65531;
                #[cfg_attr(
                    not(doctest),
                    doc = " This command instructs the node to store the current configuration parameter values and other persistent states\n to the non-volatile storage. Nodes are allowed to manage persistent states automatically, obviating the need for\n this command by committing all such data to the non-volatile memory automatically as necessary. However, some\n nodes may lack this functionality, in which case this parameter should be used. Generic interfaces should always\n invoke this command in order to ensure that the data is stored even if the node doesn't implement automatic\n persistence management."
                )]
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
            ///
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ExecuteCommandResponse {
                #[cfg_attr(not(doctest), doc = " The result of the request.")]
                ///
                /// `saturated uint8`
                ///
                /// Always aligned,
                /// size 8 bits
                pub status: u8,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandResponse {
                /// This type is delimited with an extent of 48 bytes.
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ExecuteCommandResponse {}
            impl ExecuteCommandResponse {
                #[cfg_attr(not(doctest), doc = " Started or executed successfully")]
                pub const STATUS_SUCCESS: u8 = 0;
                #[cfg_attr(
                    not(doctest),
                    doc = " Could not start or the desired outcome could not be reached"
                )]
                pub const STATUS_FAILURE: u8 = 1;
                #[cfg_attr(not(doctest), doc = " Denied due to lack of authorization")]
                pub const STATUS_NOT_AUTHORIZED: u8 = 2;
                #[cfg_attr(
                    not(doctest),
                    doc = " The requested command is not known or not supported"
                )]
                pub const STATUS_BAD_COMMAND: u8 = 3;
                #[cfg_attr(
                    not(doctest),
                    doc = " The supplied parameter cannot be used with the selected command"
                )]
                pub const STATUS_BAD_PARAMETER: u8 = 4;
                #[cfg_attr(
                    not(doctest),
                    doc = " The current state of the node does not permit execution of this command"
                )]
                pub const STATUS_BAD_STATE: u8 = 5;
                #[cfg_attr(
                    not(doctest),
                    doc = " The operation should have succeeded but an unexpected failure occurred"
                )]
                pub const STATUS_INTERNAL_ERROR: u8 = 6;
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(430);

            /// `uavcan.node.GetInfo.1.0`
            ///
            /// Fixed size 0 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Full node info request.\n All of the returned information shall be static (unchanged) while the node is running.\n It is highly recommended to support this service on all nodes."
            )]
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct GetInfoRequest {}
            impl ::canadensis_encoding::DataType for GetInfoRequest {
                /// This type is sealed.
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
            ///
            pub struct GetInfoResponse {
                #[cfg_attr(
                    not(doctest),
                    doc = " The Cyphal protocol version implemented on this node, both major and minor.\n Not to be changed while the node is running."
                )]
                ///
                /// `uavcan.node.Version.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub protocol_version: crate::uavcan::node::version_1_0::Version,
                ///
                /// `uavcan.node.Version.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub hardware_version: crate::uavcan::node::version_1_0::Version,
                #[cfg_attr(
                    not(doctest),
                    doc = " The version information shall not be changed while the node is running.\n The correct hardware version shall be reported at all times, excepting software-only nodes, in which\n case it should be set to zeros.\n If the node is equipped with a Cyphal-capable bootloader, the bootloader should report the software\n version of the installed application, if there is any; if no application is found, zeros should be reported."
                )]
                ///
                /// `uavcan.node.Version.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub software_version: crate::uavcan::node::version_1_0::Version,
                #[cfg_attr(
                    not(doctest),
                    doc = " A version control system (VCS) revision number or hash. Not to be changed while the node is running.\n For example, this field can be used for reporting the short git commit hash of the current\n software revision.\n Set to zero if not used."
                )]
                ///
                /// `saturated uint64`
                ///
                /// Always aligned,
                /// size 64 bits
                pub software_vcs_revision_id: u64,
                #[cfg_attr(
                    not(doctest),
                    doc = " The unique-ID (UID) is a 128-bit long sequence that is likely to be globally unique per node.\n The vendor shall ensure that the probability of a collision with any other node UID globally is negligibly low.\n UID is defined once per hardware unit and should never be changed.\n All zeros is not a valid UID.\n If the node is equipped with a Cyphal-capable bootloader, the bootloader shall use the same UID.\n Manual serialization note: only fixed-size fields up to this point. The following fields are dynamically sized."
                )]
                ///
                /// `saturated uint8[16]`
                ///
                /// Always aligned,
                /// size 128 bits
                pub unique_id: [u8; 16],
                #[cfg_attr(
                    not(doctest),
                    doc = " Human-readable non-empty ASCII node name. An empty name is not permitted.\n The name shall not be changed while the node is running.\n Allowed characters are: a-z (lowercase ASCII letters) 0-9 (decimal digits) . (dot) - (dash) _ (underscore).\n Node name is a reversed Internet domain name (like Java packages), e.g. \"com.manufacturer.project.product\"."
                )]
                ///
                /// `saturated uint8[<=50]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 400 bits
                pub name: ::heapless::Vec<u8, 50>,
                #[cfg_attr(
                    not(doctest),
                    doc = " The value of an arbitrary hash function applied to the software image. Not to be changed while the node is running.\n This field can be used to detect whether the software or firmware running on the node is an exact\n same version as a certain specific revision. This field provides a very strong identity guarantee,\n unlike the version fields above, which can be the same for different builds of the software.\n As can be seen from its definition, this field is optional.\n\n The exact hash function and the methods of its application are implementation-defined.\n However, implementations are recommended to adhere to the following guidelines, fully or partially:\n   - The hash function should be CRC-64-WE.\n   - The hash function should be applied to the entire application image padded to 8 bytes.\n   - If the computed image CRC is stored within the software image itself, the value of\n     the hash function becomes ill-defined, because it becomes recursively dependent on itself.\n     In order to circumvent this issue, while computing or checking the CRC, its value stored\n     within the image should be zeroed out."
                )]
                ///
                /// `saturated uint64[<=1]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 64 bits
                pub software_image_crc: ::heapless::Vec<u64, 1>,
                #[cfg_attr(
                    not(doctest),
                    doc = " The certificate of authenticity (COA) of the node, 222 bytes max, optional. This field can be used for\n reporting digital signatures (e.g., RSA-1776, or ECDSA if a higher degree of cryptographic strength is desired).\n Leave empty if not used. Not to be changed while the node is running.\n At most five CAN FD frames"
                )]
                ///
                /// `saturated uint8[<=222]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 1776 bits
                pub certificate_of_authenticity: ::heapless::Vec<u8, 222>,
            }
            impl ::canadensis_encoding::DataType for GetInfoResponse {
                /// This type is delimited with an extent of 448 bytes.
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(434);

            /// `uavcan.node.GetTransportStatistics.0.1`
            ///
            /// Fixed size 0 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Returns a set of general low-level transport statistical counters.\n Servers are encouraged but not required to sample the data atomically."
            )]
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct GetTransportStatisticsRequest {}
            impl ::canadensis_encoding::DataType for GetTransportStatisticsRequest {
                /// This type is sealed.
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
            ///
            pub struct GetTransportStatisticsResponse {
                #[cfg_attr(
                    not(doctest),
                    doc = " Cyphal transfer performance statistics:\n the number of Cyphal transfers successfully sent, successfully received, and failed.\n The methods of error counting are implementation-defined."
                )]
                ///
                /// `uavcan.node.IOStatistics.0.1`
                ///
                /// Always aligned,
                /// size 120 bits
                pub transfer_statistics: crate::uavcan::node::io_statistics_0_1::IOStatistics,
                #[cfg_attr(
                    not(doctest),
                    doc = " Network interface statistics, separate per interface.\n E.g., for a doubly redundant transport, this array would contain two elements,\n the one at the index zero would apply to the first interface, the other to the second interface.\n The methods of counting are implementation-defined.\n One CAN FD frame"
                )]
                ///
                /// `uavcan.node.IOStatistics.0.1[<=3]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 360 bits
                pub network_interface_statistics:
                    ::heapless::Vec<crate::uavcan::node::io_statistics_0_1::IOStatistics, 3>,
            }
            impl ::canadensis_encoding::DataType for GetTransportStatisticsResponse {
                /// This type is delimited with an extent of 192 bytes.
                const EXTENT_BYTES: Option<u32> = Some(192);
            }
            impl ::canadensis_encoding::Response for GetTransportStatisticsResponse {}
            impl GetTransportStatisticsResponse {
                #[cfg_attr(
                    not(doctest),
                    doc = " Cyphal supports up to triply modular redundant interfaces."
                )]
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Abstract component health information. If the node performs multiple activities (provides multiple network services),\n its health status should reflect the status of the worst-performing activity (network service).\n Follows:\n   https://www.law.cornell.edu/cfr/text/14/23.1322\n   https://www.faa.gov/documentLibrary/media/Advisory_Circular/AC_25.1322-1.pdf section 6"
            )]
            pub struct Health {
                ///
                /// `saturated uint2`
                ///
                /// Always aligned,
                /// size 2 bits
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for Health {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Health {}
            impl Health {
                #[cfg_attr(
                    not(doctest),
                    doc = " The component is functioning properly (nominal)."
                )]
                pub const NOMINAL: u8 = 0;
                #[cfg_attr(
                    not(doctest),
                    doc = " A critical parameter went out of range or the component encountered a minor failure that does not prevent\n the subsystem from performing any of its real-time functions."
                )]
                pub const ADVISORY: u8 = 1;
                #[cfg_attr(
                    not(doctest),
                    doc = " The component encountered a major failure and is performing in a degraded mode or outside of its designed limitations."
                )]
                pub const CAUTION: u8 = 2;
                #[cfg_attr(
                    not(doctest),
                    doc = " The component suffered a fatal malfunction and is unable to perform its intended function."
                )]
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
            #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
            pub const SUBJECT: ::canadensis_core::SubjectId =
                ::canadensis_core::SubjectId::from_truncating(7509);

            /// `uavcan.node.Heartbeat.1.0`
            ///
            /// Fixed size 7 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Abstract node status information.\n This is the only high-level function that shall be implemented by all nodes.\n\n All Cyphal nodes that have a node-ID are required to publish this message to its fixed subject periodically.\n Nodes that do not have a node-ID (also known as \"anonymous nodes\") shall not publish to this subject.\n\n The default subject-ID 7509 is 1110101010101 in binary. The alternating bit pattern at the end helps transceiver\n synchronization (e.g., on CAN-based networks) and on some transports permits automatic bit rate detection.\n\n Network-wide health monitoring can be implemented by subscribing to the fixed subject."
            )]
            pub struct Heartbeat {
                #[cfg_attr(
                    not(doctest),
                    doc = " [second]\n The uptime seconds counter should never overflow. The counter will reach the upper limit in ~136 years,\n upon which time it should stay at 0xFFFFFFFF until the node is restarted.\n Other nodes may detect that a remote node has restarted when this value leaps backwards."
                )]
                ///
                /// `saturated uint32`
                ///
                /// Always aligned,
                /// size 32 bits
                pub uptime: u32,
                #[cfg_attr(not(doctest), doc = " The abstract health status of this node.")]
                ///
                /// `uavcan.node.Health.1.0`
                ///
                /// Always aligned,
                /// size 8 bits
                pub health: crate::uavcan::node::health_1_0::Health,
                #[cfg_attr(
                    not(doctest),
                    doc = " The abstract operating mode of the publishing node.\n This field indicates the general level of readiness that can be further elaborated on a per-activity basis\n using various specialized interfaces."
                )]
                ///
                /// `uavcan.node.Mode.1.0`
                ///
                /// Always aligned,
                /// size 8 bits
                pub mode: crate::uavcan::node::mode_1_0::Mode,
                #[cfg_attr(
                    not(doctest),
                    doc = " Optional, vendor-specific node status code, e.g. a fault code or a status bitmask.\n Fits into a single-frame Classic CAN transfer (least capable transport, smallest MTU)."
                )]
                ///
                /// `saturated uint8`
                ///
                /// Always aligned,
                /// size 8 bits
                pub vendor_specific_status_code: u8,
            }
            impl ::canadensis_encoding::DataType for Heartbeat {
                /// This type is delimited with an extent of 12 bytes.
                const EXTENT_BYTES: Option<u32> = Some(12);
            }
            impl ::canadensis_encoding::Message for Heartbeat {}
            impl Heartbeat {
                #[cfg_attr(
                    not(doctest),
                    doc = " [second]\n The publication period shall not exceed this limit.\n The period should not change while the node is running."
                )]
                pub const MAX_PUBLICATION_PERIOD: u16 = 1;
                #[cfg_attr(
                    not(doctest),
                    doc = " [second]\n If the last message from the node was received more than this amount of time ago, it should be considered offline."
                )]
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Defines a node-ID.\n The maximum valid value is dependent on the underlying transport layer.\n Values lower than 128 are always valid for all transports.\n Refer to the specification for more info."
            )]
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ID {
                ///
                /// `saturated uint16`
                ///
                /// Always aligned,
                /// size 16 bits
                pub value: u16,
            }
            impl ::canadensis_encoding::DataType for ID {
                /// This type is sealed.
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " A standard set of generic input/output statistical counters that generally should not overflow.\n If a 40-bit counter is incremented every millisecond, it will overflow in ~35 years.\n If an overflow occurs, the value will wrap over to zero.\n\n The values should not be reset while the node is running."
            )]
            pub struct IOStatistics {
                #[cfg_attr(not(doctest), doc = " The number of successfully emitted entities.")]
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub num_emitted: u64,
                #[cfg_attr(not(doctest), doc = " The number of successfully received entities.")]
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub num_received: u64,
                #[cfg_attr(
                    not(doctest),
                    doc = " How many errors have occurred.\n The exact definition of \"error\" and how they are counted are implementation-defined,\n unless specifically defined otherwise."
                )]
                ///
                /// `truncated uint40`
                ///
                /// Always aligned,
                /// size 40 bits
                pub num_errored: u64,
            }
            impl ::canadensis_encoding::DataType for IOStatistics {
                /// This type is sealed.
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " The operating mode of a node.\n Reserved values can be used in future revisions of the specification."
            )]
            pub struct Mode {
                ///
                /// `saturated uint3`
                ///
                /// Always aligned,
                /// size 3 bits
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for Mode {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Mode {}
            impl Mode {
                #[cfg_attr(not(doctest), doc = " Normal operating mode.")]
                pub const OPERATIONAL: u8 = 0;
                #[cfg_attr(
                    not(doctest),
                    doc = " Initialization is in progress; this mode is entered immediately after startup."
                )]
                pub const INITIALIZATION: u8 = 1;
                #[cfg_attr(not(doctest), doc = " E.g., calibration, self-test, etc.")]
                pub const MAINTENANCE: u8 = 2;
                #[cfg_attr(
                    not(doctest),
                    doc = " New software/firmware is being loaded or the bootloader is running."
                )]
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " Used to refer either to a Service or to a Subject.\n The chosen tag identifies the kind of the port, then the numerical ID identifies the port within the kind."
                )]
                pub enum ID {
                    ///
                    /// uavcan.node.port.SubjectID.1.0
                    SubjectId(crate::uavcan::node::port::subject_id_1_0::SubjectID),
                    ///
                    /// uavcan.node.port.ServiceID.1.0
                    ServiceId(crate::uavcan::node::port::service_id_1_0::ServiceID),
                }
                impl ::canadensis_encoding::DataType for ID {
                    /// This type is sealed.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod list_0_1 {
                #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
                #[deprecated]
                pub const SUBJECT: ::canadensis_core::SubjectId =
                    ::canadensis_core::SubjectId::from_truncating(7510);

                /// `uavcan.node.port.List.0.1`
                ///
                /// Size ranges from 146 to 2194 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A list of ports that this node is using:\n - Subjects published by this node (whether periodically or ad-hoc).\n - Subjects that this node is subscribed to (a datalogger or a debugger would typically subscribe to all subjects).\n - RPC services consumed by this node (i.e., service clients).\n - RPC services provided by this node (i.e., service servers).\n\n All nodes should implement this capability to provide network introspection and diagnostic capabilities.\n This message should be published using the fixed subject-ID as follows:\n - At the OPTIONAL priority level at least every MAX_PUBLICATION_PERIOD seconds.\n - At the OPTIONAL or SLOW priority level within MAX_PUBLICATION_PERIOD after the port configuration is changed.\n Replaced with v1."
                )]
                #[deprecated]
                pub struct List {
                    ///
                    /// `uavcan.node.port.SubjectIDList.0.1`
                    ///
                    /// Always aligned,
                    /// size ranges from 8 to 8200 bits
                    pub publishers: crate::uavcan::node::port::subject_id_list_0_1::SubjectIDList,
                    ///
                    /// `uavcan.node.port.SubjectIDList.0.1`
                    ///
                    /// Always aligned,
                    /// size ranges from 8 to 8200 bits
                    pub subscribers: crate::uavcan::node::port::subject_id_list_0_1::SubjectIDList,
                    ///
                    /// `uavcan.node.port.ServiceIDList.0.1`
                    ///
                    /// Always aligned,
                    /// size 512 bits
                    pub clients: crate::uavcan::node::port::service_id_list_0_1::ServiceIDList,
                    ///
                    /// `uavcan.node.port.ServiceIDList.0.1`
                    ///
                    /// Always aligned,
                    /// size 512 bits
                    pub servers: crate::uavcan::node::port::service_id_list_0_1::ServiceIDList,
                }
                impl ::canadensis_encoding::DataType for List {
                    /// This type is sealed.
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for List {}
                impl List {
                    #[cfg_attr(
                        not(doctest),
                        doc = " [seconds]\n If the port configuration is not updated in this amount of time, the node should publish this message anyway."
                    )]
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
            pub mod list_1_0 {
                #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
                pub const SUBJECT: ::canadensis_core::SubjectId =
                    ::canadensis_core::SubjectId::from_truncating(7510);

                /// `uavcan.node.port.List.1.0`
                ///
                /// Size ranges from 146 to 2194 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A list of ports that this node is using:\n - Subjects published by this node (whether periodically or ad-hoc).\n - Subjects that this node is subscribed to (a datalogger or a debugger would typically subscribe to all subjects).\n - RPC services consumed by this node (i.e., service clients).\n - RPC services provided by this node (i.e., service servers).\n\n All nodes should implement this capability to provide network introspection and diagnostic capabilities.\n This message should be published using the fixed subject-ID as follows:\n - At the OPTIONAL priority level at least every MAX_PUBLICATION_PERIOD seconds.\n - At the OPTIONAL or SLOW priority level within MAX_PUBLICATION_PERIOD after the port configuration is changed."
                )]
                pub struct List {
                    ///
                    /// `uavcan.node.port.SubjectIDList.1.0`
                    ///
                    /// Always aligned,
                    /// size ranges from 8 to 8200 bits
                    pub publishers: crate::uavcan::node::port::subject_id_list_1_0::SubjectIDList,
                    ///
                    /// `uavcan.node.port.SubjectIDList.1.0`
                    ///
                    /// Always aligned,
                    /// size ranges from 8 to 8200 bits
                    pub subscribers: crate::uavcan::node::port::subject_id_list_1_0::SubjectIDList,
                    ///
                    /// `uavcan.node.port.ServiceIDList.1.0`
                    ///
                    /// Always aligned,
                    /// size 512 bits
                    pub clients: crate::uavcan::node::port::service_id_list_1_0::ServiceIDList,
                    ///
                    /// `uavcan.node.port.ServiceIDList.1.0`
                    ///
                    /// Always aligned,
                    /// size 512 bits
                    pub servers: crate::uavcan::node::port::service_id_list_1_0::ServiceIDList,
                }
                impl ::canadensis_encoding::DataType for List {
                    /// This type is sealed.
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for List {}
                impl List {
                    #[cfg_attr(
                        not(doctest),
                        doc = " [seconds]\n If the port configuration is not updated in this amount of time, the node should publish this message anyway."
                    )]
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " Service-ID. The ranges are defined by the specification."
                )]
                pub struct ServiceID {
                    ///
                    /// `saturated uint9`
                    ///
                    /// Always aligned,
                    /// size 9 bits
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for ServiceID {
                    /// This type is sealed.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod service_id_list_0_1 {
                /// `uavcan.node.port.ServiceIDList.0.1`
                ///
                /// Fixed size 64 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A list of service identifiers.\n This is a trivial constant-size bitmask with some reserved space in case the range of service-ID is increased\n in a future revision of the protocol.\n Replaced with v1."
                )]
                #[deprecated]
                pub struct ServiceIDList {
                    #[cfg_attr(
                        not(doctest),
                        doc = " The index represents the identifier value. True -- present/used. False -- absent/unused."
                    )]
                    ///
                    /// `saturated bool[512]`
                    ///
                    /// Always aligned,
                    /// size 512 bits
                    pub mask: ::canadensis_encoding::bits::BitArray<64>,
                }
                impl ::canadensis_encoding::DataType for ServiceIDList {
                    /// This type is delimited with an extent of 128 bytes.
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
            pub mod service_id_list_1_0 {
                /// `uavcan.node.port.ServiceIDList.1.0`
                ///
                /// Fixed size 64 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A list of service identifiers.\n This is a trivial constant-size bitmask with some reserved space in case the range of service-ID is increased\n in a future revision of the protocol."
                )]
                pub struct ServiceIDList {
                    #[cfg_attr(
                        not(doctest),
                        doc = " The index represents the identifier value. True -- present/used. False -- absent/unused."
                    )]
                    ///
                    /// `saturated bool[512]`
                    ///
                    /// Always aligned,
                    /// size 512 bits
                    pub mask: ::canadensis_encoding::bits::BitArray<64>,
                }
                impl ::canadensis_encoding::DataType for ServiceIDList {
                    /// This type is delimited with an extent of 128 bytes.
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " Subject-ID. The ranges are defined by the specification."
                )]
                pub struct SubjectID {
                    ///
                    /// `saturated uint13`
                    ///
                    /// Always aligned,
                    /// size 13 bits
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for SubjectID {
                    /// This type is sealed.
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
            #[allow(deprecated)]
            #[deprecated]
            pub mod subject_id_list_0_1 {
                /// `uavcan.node.port.SubjectIDList.0.1`
                ///
                /// Size ranges from 1 to 1025 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A list of subject identifiers.\n The range of subject-ID is large, so using a fixed-size bitmask would make this type difficult to handle on\n resource-constrained systems. To address that, we provide two extra options: a simple variable-length list,\n and a special case that indicates that every subject-ID is in use.\n Replaced with v1."
                )]
                #[deprecated]
                pub enum SubjectIDList {
                    #[cfg_attr(
                        not(doctest),
                        doc = " The index represents the identifier value. True -- present/used. False -- absent/unused."
                    )]
                    ///
                    /// saturated bool[8192]
                    Mask(::canadensis_encoding::bits::BitArray<1024>),
                    #[cfg_attr(
                        not(doctest),
                        doc = " A list of identifiers that can be used instead of the mask if most of the identifiers are unused."
                    )]
                    ///
                    /// uavcan.node.port.SubjectID.1.0[<=255]
                    SparseList(
                        ::heapless::Vec<crate::uavcan::node::port::subject_id_1_0::SubjectID, 255>,
                    ),
                    #[cfg_attr(
                        not(doctest),
                        doc = " A special case indicating that all identifiers are in use.\n Reserve space in case the range is extended in the future."
                    )]
                    ///
                    /// uavcan.primitive.Empty.1.0
                    Total(crate::uavcan::primitive::empty_1_0::Empty),
                }
                impl ::canadensis_encoding::DataType for SubjectIDList {
                    /// This type is delimited with an extent of 4097 bytes.
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
                                (*inner).serialize(cursor);
                            }
                            SubjectIDList::SparseList(inner) => {
                                cursor.write_aligned_u8(1);
                                cursor.write_aligned_u8((*inner).len() as u8);
                                for value in (*inner).iter() {
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
            pub mod subject_id_list_1_0 {
                /// `uavcan.node.port.SubjectIDList.1.0`
                ///
                /// Size ranges from 1 to 1025 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " A list of subject identifiers.\n The range of subject-ID is large, so using a fixed-size bitmask would make this type difficult to handle on\n resource-constrained systems. To address that, we provide two extra options: a simple variable-length list,\n and a special case that indicates that every subject-ID is in use."
                )]
                pub enum SubjectIDList {
                    #[cfg_attr(
                        not(doctest),
                        doc = " The index represents the identifier value. True -- present/used. False -- absent/unused."
                    )]
                    ///
                    /// saturated bool[8192]
                    Mask(::canadensis_encoding::bits::BitArray<1024>),
                    #[cfg_attr(
                        not(doctest),
                        doc = " A list of identifiers that can be used instead of the mask if most of the identifiers are unused."
                    )]
                    ///
                    /// uavcan.node.port.SubjectID.1.0[<=255]
                    SparseList(
                        ::heapless::Vec<crate::uavcan::node::port::subject_id_1_0::SubjectID, 255>,
                    ),
                    #[cfg_attr(
                        not(doctest),
                        doc = " A special case indicating that all identifiers are in use.\n Reserve space in case the range is extended in the future."
                    )]
                    ///
                    /// uavcan.primitive.Empty.1.0
                    Total(crate::uavcan::primitive::empty_1_0::Empty),
                }
                impl ::canadensis_encoding::DataType for SubjectIDList {
                    /// This type is delimited with an extent of 4097 bytes.
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
                                (*inner).serialize(cursor);
                            }
                            SubjectIDList::SparseList(inner) => {
                                cursor.write_aligned_u8(1);
                                cursor.write_aligned_u8((*inner).len() as u8);
                                for value in (*inner).iter() {
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " A shortened semantic version representation: only major and minor.\n The protocol generally does not concern itself with the patch version."
            )]
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct Version {
                ///
                /// `saturated uint8`
                ///
                /// Always aligned,
                /// size 8 bits
                pub major: u8,
                ///
                /// `saturated uint8`
                ///
                /// Always aligned,
                /// size 8 bits
                pub minor: u8,
            }
            impl ::canadensis_encoding::DataType for Version {
                /// This type is sealed.
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
                #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
                pub const SERVICE: ::canadensis_core::ServiceId =
                    ::canadensis_core::ServiceId::from_truncating(390);

                /// `uavcan.pnp.cluster.AppendEntries.1.0`
                ///
                /// Size ranges from 13 to 35 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " This type is a part of the Raft consensus algorithm. The Raft consensus is used for the maintenance of the\n distributed allocation table between redundant allocators. The following description is focused on the exchanges\n between redundant PnP node-ID allocators. It does not apply to the case of non-redundant allocators, because\n in that case the allocation table is stored locally and the process of node-ID allocation is trivial and fully local.\n Exchanges between allocatees and allocators are documented in the appropriate message type definition.\n\n The algorithm used for replication of the allocation table across redundant allocators is a fairly direct\n implementation of the Raft consensus algorithm, as published in the paper\n \"In Search of an Understandable Consensus Algorithm (Extended Version)\" by Diego Ongaro and John Ousterhout.\n The following text assumes that the reader is familiar with the paper.\n\n The Raft log contains entries of type Entry (in the same namespace), where every entry contains the Raft term\n number, the unique-ID, and the corresponding node-ID value (or zeros if it could not be requested from a static\n node). Therefore, the Raft log is the allocation table itself.\n\n Since the maximum number of entries in the allocation table is limited by the range of node-ID values, the log\n capacity is bounded. Therefore, the snapshot transfer and log compaction functions are not required,\n so they are not used in this implementation of the Raft algorithm.\n\n When an allocator becomes the leader of the Raft cluster, it checks if the Raft log contains an entry for its own\n node-ID, and if it doesn't, the leader adds its own allocation entry to the log (the unique-ID can be replaced with\n zeros at the discretion of the implementer). This behavior guarantees that the Raft log always contains at least\n one entry, therefore it is not necessary to support negative log indices, as proposed by the Raft paper.\n\n Since the log is write-only and limited in growth, all allocations are permanent. This restriction is acceptable,\n since Cyphal is a vehicle bus, and configuration of vehicle's components is not expected to change frequently.\n Old allocations can be removed in order to free node-IDs for new allocations by clearing the Raft log on all\n allocators; such clearing shall be performed simultaneously while the network is down, otherwise the Raft cluster\n will automatically attempt to restore the lost state on the allocators where the table was cleared.\n\n The allocators need to be aware of each other's node-ID in order to form a cluster. In order to learn each other's\n node-ID values, the allocators broadcast messages of type Discovery (in the same namespace) until the cluster is\n fully discovered and all allocators know of each other's node-ID. This extension to the Raft algorithm makes the\n cluster almost configuration-free - the only parameter that shall be configured on all allocators of the cluster\n is the number of nodes in the cluster (everything else will be auto-detected).\n\n Runtime cluster membership changes are not supported, since they are not needed for a vehicle bus.\n\n As has been explained in the general description of the PnP node-ID allocation feature, allocators shall watch for\n unknown static nodes appearing on the bus. In the case of a non-redundant allocator, the task is trivial, since the\n allocation table can be updated locally. In the case of a Raft cluster, however, the network monitoring task shall\n be performed by the leader only, since other cluster members cannot commit to the shared allocation table (i.e.,\n the Raft log) anyway. Redundant allocators should not attempt to obtain the true unique-ID of the newly detected\n static nodes (use zeros instead), because the allocation table is write-only: if the unique-ID of a static node\n ever changes (e.g., a replacement unit is installed, or network configuration is changed manually), the change\n will be impossible to reflect in the allocation table.\n\n Only the current Raft leader can process allocation requests and engage in communication with allocatees.\n An allocator is allowed to send allocation responses only if both conditions are met:\n\n   - The allocator is currently the Raft leader.\n   - Its replica of the Raft log does not contain uncommitted entries (i.e. the last allocation request has been\n     completed successfully).\n\n All cluster maintenance traffic should normally use either the lowest or the next-to-lowest transfer priority level."
                )]
                pub struct AppendEntriesRequest {
                    ///
                    /// `saturated uint32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub term: u32,
                    ///
                    /// `saturated uint32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub prev_log_term: u32,
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub prev_log_index: u16,
                    #[cfg_attr(not(doctest), doc = " Refer to the Raft paper for explanation.")]
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub leader_commit: u16,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Worst case replication time per Follower can be computed as:\n\n   worst replication time = (node-ID capacity) * (2 trips of next_index) * (request interval per Follower)\n\n E.g., given the request interval of 0.5 seconds, the worst case replication time for CAN bus is:\n\n   128 nodes * 2 trips * 0.5 seconds = 128 seconds.\n\n This is the amount of time it will take for a new Follower to reconstruct a full replica of the distributed log."
                    )]
                    ///
                    /// `uavcan.pnp.cluster.Entry.1.0[<=1]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 176 bits
                    pub entries: ::heapless::Vec<crate::uavcan::pnp::cluster::entry_1_0::Entry, 1>,
                }
                impl ::canadensis_encoding::DataType for AppendEntriesRequest {
                    /// This type is delimited with an extent of 96 bytes.
                    const EXTENT_BYTES: Option<u32> = Some(96);
                }
                impl ::canadensis_encoding::Request for AppendEntriesRequest {}
                impl AppendEntriesRequest {
                    #[cfg_attr(not(doctest), doc = " [second]")]
                    pub const DEFAULT_MIN_ELECTION_TIMEOUT: u8 = 2;
                    #[cfg_attr(
                        not(doctest),
                        doc = " [second]\n Given the minimum election timeout and the cluster size,\n the maximum recommended request interval can be derived as follows:\n\n   max recommended request interval = (min election timeout) / 2 requests / (cluster size - 1)\n\n The equation assumes that the Leader requests one Follower at a time, so that there's at most one pending call\n at any moment. Such behavior is optimal as it creates a uniform bus load, although it is implementation-specific.\n Obviously, the request interval can be lower than that if needed, but higher values are not recommended as they may\n cause Followers to initiate premature elections in case of frame losses or delays.\n\n The timeout value is randomized in the range (MIN, MAX], according to the Raft paper. The randomization granularity\n should be at least one millisecond or higher."
                    )]
                    pub const DEFAULT_MAX_ELECTION_TIMEOUT: u8 = 4;
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
                ///
                pub struct AppendEntriesResponse {
                    ///
                    /// `saturated uint32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub term: u32,
                    #[cfg_attr(not(doctest), doc = " Refer to the Raft paper for explanation.")]
                    ///
                    /// `saturated bool`
                    ///
                    /// Always aligned,
                    /// size 1 bits
                    pub success: bool,
                }
                impl ::canadensis_encoding::DataType for AppendEntriesResponse {
                    /// This type is delimited with an extent of 48 bytes.
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
                #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
                pub const SUBJECT: ::canadensis_core::SubjectId =
                    ::canadensis_core::SubjectId::from_truncating(8164);

                /// `uavcan.pnp.cluster.Discovery.1.0`
                ///
                /// Size ranges from 2 to 12 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " This message is used by redundant allocators to find each other's node-ID.\n Please refer to the type AppendEntries for details.\n\n An allocator should stop publishing this message as soon as it has discovered all other allocators in the cluster.\n\n An exception applies: when an allocator receives a Discovery message where the list of known nodes is incomplete\n (i.e. len(known_nodes) < configured_cluster_size), it shall publish a Discovery message once. This condition\n allows other allocators to quickly re-discover the cluster after a restart."
                )]
                pub struct Discovery {
                    #[cfg_attr(
                        not(doctest),
                        doc = " The number of allocators in the cluster as configured on the sender.\n This value shall be the same across all allocators."
                    )]
                    ///
                    /// `saturated uint3`
                    ///
                    /// Always aligned,
                    /// size 3 bits
                    pub configured_cluster_size: u8,
                    // 5 bits of padding
                    #[cfg_attr(
                        not(doctest),
                        doc = " Node-IDs of the allocators that are known to the publishing allocator, including the publishing allocator itself."
                    )]
                    ///
                    /// `uavcan.node.ID.1.0[<=5]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 80 bits
                    pub known_nodes: ::heapless::Vec<crate::uavcan::node::id_1_0::ID, 5>,
                }
                impl ::canadensis_encoding::DataType for Discovery {
                    /// This type is delimited with an extent of 96 bytes.
                    const EXTENT_BYTES: Option<u32> = Some(96);
                }
                impl ::canadensis_encoding::Message for Discovery {}
                impl Discovery {
                    #[cfg_attr(
                        not(doctest),
                        doc = " [second]\n This message should be broadcasted by the allocator at this interval until all other allocators are discovered."
                    )]
                    pub const BROADCASTING_PERIOD: u8 = 1;
                    #[cfg_attr(
                        not(doctest),
                        doc = " The redundant allocator cluster cannot contain more than 5 allocators."
                    )]
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " One PnP node-ID allocation entry.\n This type is a part of the Raft consensus algorithm. Please refer to the type AppendEntries for details."
                )]
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Entry {
                    #[cfg_attr(not(doctest), doc = " Refer to the Raft paper for explanation.")]
                    ///
                    /// `saturated uint32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub term: u32,
                    #[cfg_attr(
                        not(doctest),
                        doc = " Unique-ID of this allocation; zero if unknown."
                    )]
                    ///
                    /// `saturated uint8[16]`
                    ///
                    /// Always aligned,
                    /// size 128 bits
                    pub unique_id: [u8; 16],
                    #[cfg_attr(not(doctest), doc = " Node-ID of this allocation.")]
                    ///
                    /// `uavcan.node.ID.1.0`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub node_id: crate::uavcan::node::id_1_0::ID,
                }
                impl ::canadensis_encoding::DataType for Entry {
                    /// This type is sealed.
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
                #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
                pub const SERVICE: ::canadensis_core::ServiceId =
                    ::canadensis_core::ServiceId::from_truncating(391);

                /// `uavcan.pnp.cluster.RequestVote.1.0`
                ///
                /// Fixed size 10 bytes
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " This type is a part of the Raft consensus algorithm. Please refer to the type AppendEntries for details."
                )]
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct RequestVoteRequest {
                    ///
                    /// `saturated uint32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub term: u32,
                    ///
                    /// `saturated uint32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub last_log_term: u32,
                    #[cfg_attr(not(doctest), doc = " Refer to the Raft paper for explanation.")]
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub last_log_index: u16,
                }
                impl ::canadensis_encoding::DataType for RequestVoteRequest {
                    /// This type is delimited with an extent of 48 bytes.
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
                ///
                pub struct RequestVoteResponse {
                    ///
                    /// `saturated uint32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub term: u32,
                    #[cfg_attr(not(doctest), doc = " Refer to the Raft paper for explanation.")]
                    ///
                    /// `saturated bool`
                    ///
                    /// Always aligned,
                    /// size 1 bits
                    pub vote_granted: bool,
                }
                impl ::canadensis_encoding::DataType for RequestVoteResponse {
                    /// This type is delimited with an extent of 48 bytes.
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
            #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
            pub const SUBJECT: ::canadensis_core::SubjectId =
                ::canadensis_core::SubjectId::from_truncating(8166);

            /// `uavcan.pnp.NodeIDAllocationData.1.0`
            ///
            /// Size ranges from 7 to 9 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " This definition of the allocation message is intended for use with transports where anonymous transfers are limited\n to 7 bytes of payload, such as Classic CAN. The definition is carried over from the original UAVCAN v0 specification\n with some modifications. For transports other than Classic CAN (e.g., CAN FD, serial, etc.) there is a more\n general, more capable definition NodeIDAllocationData v2.0. The PnP protocol itself is described in the documentation\n for the v2 definition. The documentation provided here builds upon the general case, so read that first please.\n\n The full 128-bit unique-ID can't be accommodated in a single-frame anonymous message transfer over Classic CAN, so\n this definition substitutes the full 128-bit ID with a smaller 48-bit hash of it. The 48-bit hash is obtained by\n applying an arbitrary hash function to the unique-ID that outputs at least 48 bit of data. The recommended hash\n function is the standard CRC-64WE where only the lowest 48 bit of the result are used.\n\n Allocators that support allocation messages of different versions should maintain a shared allocation table for all.\n Requests received via the v1 message obviously do not contain the full unique-ID; the allocators are recommended\n to left-zero-pad the small 48-bit hash in order to obtain a \"pseudo unique-ID\", and use this value in the\n allocation table as a substitute for the real unique-ID. It is recognized that this behavior will have certain\n side effects, such as the same allocatee obtaining different allocated node-ID values depending on which version\n of the message is used, but they are considered tolerable.\n\n Allocatees that may need to operate over Classic CAN along with high-MTU transports may choose to use\n only this constrained method of allocation for consistency and simplification.\n\n In order to save space for the hash, the preferred node-ID is removed from the request. The allocated node-ID\n is provided in the response, however; this is achieved by means of an optional field that is not populated in\n the request but is populated in the response. This implies that the response may be a multi-frame transfer,\n which is acceptable since responses are sent by allocators, which are regular nodes, and therefore they are\n allowed to use regular message transfers rather than being limited to anonymous message transfers as allocatees are.\n\n On the allocatee's side the protocol is defined through the following set of rules:\n\n   Rule A. On initialization:\n     1. The allocatee subscribes to this message.\n     2. The allocatee starts the Request Timer with a random interval of Trequest.\n\n   Rule B. On expiration of the Request Timer (started as per rules A, B, or C):\n     1. Request Timer restarts with a random interval of Trequest (chosen anew).\n     2. The allocatee broadcasts an allocation request message, where the fields are populated as follows:\n        unique_id_hash    - a 48-bit hash of the unique-ID of the allocatee.\n        allocated_node_id - empty (not populated).\n\n   Rule C. On any allocation message, even if other rules also match:\n     1. Request Timer restarts with a random interval of Trequest (chosen anew).\n\n   Rule D. On an allocation message WHERE (source node-ID is non-anonymous, i.e., regular allocation response)\n                                    AND   (the field unique_id_hash matches the allocatee's 48-bit unique-ID hash)\n                                    AND   (the field allocated_node_id is populated):\n     1. Request Timer stops.\n     2. The allocatee initializes its node-ID with the received value.\n     3. The allocatee terminates its subscription to allocation messages.\n     4. Exit."
            )]
            pub struct NodeIDAllocationData {
                #[cfg_attr(
                    not(doctest),
                    doc = " An arbitrary 48-bit hash of the unique-ID of the local node."
                )]
                ///
                /// `truncated uint48`
                ///
                /// Always aligned,
                /// size 48 bits
                pub unique_id_hash: u64,
                #[cfg_attr(
                    not(doctest),
                    doc = " Shall be empty in request messages.\n Shall be populated in response messages."
                )]
                ///
                /// `uavcan.node.ID.1.0[<=1]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 16 bits
                pub allocated_node_id: ::heapless::Vec<crate::uavcan::node::id_1_0::ID, 1>,
            }
            impl ::canadensis_encoding::DataType for NodeIDAllocationData {
                /// This type is sealed.
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
            #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
            pub const SUBJECT: ::canadensis_core::SubjectId =
                ::canadensis_core::SubjectId::from_truncating(8165);

            /// `uavcan.pnp.NodeIDAllocationData.2.0`
            ///
            /// Fixed size 18 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " In order to be able to operate in a Cyphal network, a node shall have a node-ID that is unique within the network.\n Typically, a valid node-ID can be configured manually for each node; however, in certain use cases the manual\n approach is either undesirable or impossible, therefore Cyphal defines the high-level feature of plug-and-play\n nodes that allows nodes to obtain a node-ID value automatically upon connection to the network. When combined\n with automatic physical layer configuration (such as auto bit rate detection), this feature allows one to implement\n nodes that can join a Cyphal network without any prior manual configuration whatsoever. Such nodes are referred to\n as \"plug-and-play nodes\" (or \"PnP nodes\" for brevity).\n\n The feature is fundamentally non-deterministic and is likely to be unfit for some high-reliability systems;\n the designers need to carefully consider the trade-offs involved before deciding to rely on this feature.\n Normally, static node-ID settings should be preferred.\n\n This feature relies on the concept of \"anonymous message transfers\", please consult with the Cyphal transport\n layer specification for details.\n\n The process of plug-and-play node-ID allocation always involves two types of nodes: \"allocators\", which serve\n allocation requests; and \"allocatees\", which request PnP node-ID from allocators. A Cyphal network may implement\n the following configurations of allocators:\n\n   - Zero allocators, in which case plug-and-play node-ID allocation cannot be used, only nodes with statically\n     configured node-ID can communicate.\n\n   - One allocator, in which case the feature of plug-and-play node-ID allocation will become unavailable\n     if the allocator fails. In this configuration, the role of the allocator can be performed even by a very\n     resource-constrained system, e.g., a low-end microcontroller.\n\n   - Three allocators, in which case the allocators will be using a replicated allocation table via a\n     distributed consensus algorithm. In this configuration, the network can tolerate the loss of one\n     allocator and continue to serve allocation requests. This configuration requires the allocators to\n     maintain large data structures for the needs of the distributed consensus algorithm, and may therefore\n     require a slightly more sophisticated computational platform, e.g., a high-end microcontroller.\n\n   - Five allocators, it is the same as the three allocator configuration reviewed above except that the network\n     can tolerate the loss of two allocators and still continue to serve allocation requests.\n\n In order to get a PnP node-ID, an allocatee shall have a globally unique 128-bit integer identifier, known as\n unique-ID (where \"globally unique\" means that the probability of having two nodes anywhere in the world that share\n the same unique-ID is negligibly low). This is the same value that is used in the field unique_id of the data type\n uavcan.node.GetInfo. All PnP nodes shall support the service uavcan.node.GetInfo, and they shall use the same\n unique-ID value when requesting node-ID allocation and when responding to the GetInfo requests (there may exist\n other usages of the unique-ID value, but they lie outside of the scope of the PnP protocol).\n\n During allocation, the allocatee communicates its unique-ID to the allocator (or allocators in the case of a\n redundant allocator configuration), which then use it to produce an appropriate allocation response. Unique-ID\n values are kept by allocators in the \"allocation table\" - a data structure that contains the mapping between\n unique-ID and the corresponding node-ID values. The allocation table is a write-only data structure that can\n only expand. When a new allocatee requests a PnP node-ID, its unique-ID is recorded in the allocation table,\n and all subsequent allocation requests from the same allocatee will be served with the same node-ID value.\n\n In configurations with redundant allocators, every allocator maintains a replica of the same allocation table\n (a Cyphal network cannot contain more than one allocation table, regardless of the number of allocators employed).\n While the allocation table is a write-only data structure that can only grow, it is still possible to wipe the\n table completely (as long as it is removed from all redundant allocators on the network simultaneously),\n forcing the allocators to forget known nodes and perform all future allocations anew.\n\n In the context of the following description, nodes that use a manually-configured node-ID will be referred to as\n \"static nodes\". It is assumed that allocators are always static nodes themselves since there is no other authority\n on the network that can grant a PnP node-ID, so allocators are unable to request a PnP node-ID for themselves.\n Excepting allocators, it is not recommended to mix PnP and static nodes on the same network; i.e., normally,\n a Cyphal network should contain either all static nodes, or all PnP nodes (excepting allocators). If this\n recommendation cannot be followed, the following rules of safe co-existence of PnP nodes with static nodes should\n be adopted:\n   - It is safe to connect PnP nodes to the bus at any time.\n   - A static node can be connected to the bus if the allocator (allocators) is (are) already aware of it.\n     I.e., the static node is already listed in the allocation table.\n   - A new static node (i.e., a node that does not meet the above criterion) can be connected to the bus only if\n     no PnP allocation requests are happening at the moment.\n\n Due to the possibility of coexistence of static nodes with PnP nodes, allocators are tasked with monitoring\n the nodes present in the network. If the allocator detects an online node in the network the node-ID of which is\n not found in the allocation table (or the local copy thereof in the case of redundant allocators), the allocator\n shall create a new mock entry where the node-ID matches that of the newly detected node and the unique-ID is set to\n zero (i.e., a 128-bit long sequence of zero bits). This behavior ensures that PnP nodes will never be granted\n node-ID values that are already taken by static nodes. Allocators are allowed to request the true unique-ID of the\n newly detected nodes by issuing requests uavcan.node.GetInfo instead of using mock zero unique-IDs, but this is not\n required for the sake of simplicity and determinism (some nodes may fail to respond to the GetInfo request, e.g.,\n if this service is not supported). Note that in the case of redundant allocators, some of them may be relieved of\n this task due to the intrinsic properties of the distributed consensus algorithm; please refer to the documentation\n for the data type uavcan.pnp.cluster.AppendEntries for more information.\n\n The unique-ID & node-ID pair of each allocator shall be kept in the allocation table as well. It is allowed to replace\n the unique-ID values of allocators with zeros at the discretion of the implementer.\n\n As can be inferred from the above, the process of PnP node-ID allocation involves up to two types of communications:\n\n   - \"Allocatee-allocator exchange\" - this communication is used when an allocatee requests a PnP node-ID from the\n     allocator (or redundant allocators), and also when the allocator transmits a response back to the allocatee.\n     This communication is invariant to the allocator configuration used, i.e., the allocatees are not aware of\n     how many allocators are available on the network and how they are configured. In configurations with\n     non-redundant (i.e., single) allocator, this is the only type of PnP allocation exchanges.\n\n   - \"Allocator-allocator exchange\" - this communication is used by redundant allocators for the maintenance of\n     the replicated allocation table and for other needs of the distributed consensus algorithm. Allocatees are\n     completely isolated and are unaware of these exchanges. This communication is not used with the single-allocator\n     configuration, since there is only one server and the allocation table is not distributed. The data types\n     used for the allocator-allocator exchanges are defined in the namespace uavcan.pnp.cluster.\n\n As has been said earlier, the logic used for communication between allocators (for the needs of the maintenance of\n the distributed allocation table) is completely unrelated to the allocatees. The allocatees are unaware of these\n exchanges, and they are also unaware of the allocator configuration used on the network: single or redundant.\n As such, the documentation you're currently reading does not describe the logic and requirements of the\n allocator-allocator exchanges for redundant configurations; for that, please refer to the documentation for the\n data type uavcan.pnp.cluster.AppendEntries.\n\n Allocatee-allocator exchanges are performed using only this message type uavcan.pnp.NodeIDAllocationData. Allocators\n use it with regular message transfers; allocatees use it with anonymous message transfers. The specification and\n usage info for this data type is provided below.\n\n The general idea of the allocatee-allocator exchanges is that the allocatee communicates to the allocator its\n unique-ID and, if applicable, the preferred node-ID value that it would like to have. The allocatee uses\n anonymous message transfers of this type. The allocator performs the allocation and sends a response using\n the same message type, where the field for unique-ID is populated with the unique-ID of the requesting node\n and the field for node-ID is populated with the allocated node-ID. All exchanges from allocatee to allocator use\n single-frame transfers only (see the specification for more information on the limitations of anonymous messages).\n\n The allocatee-allocator exchange logic differs between allocators and allocatees. For allocators, the logic is\n trivial: upon reception of a request, the allocator performs an allocation and sends a response back. If the\n allocation could not be performed for any reason (e.g., the allocation table is full, or there was a failure),\n no response is sent back (i.e., the request is simply ignored); the recommended strategy for the allocatee is to\n continue sending new allocation requests until a response is granted or a higher-level system (e.g., a maintenance\n technician or some automation) intervenes to rectify the problem (e.g., by purging the allocation table).\n The allocator that could not complete an allocation for any reason is recommended to emit a diagnostic message\n with a human-readable description of the problem. For allocatees, the logic is described below.\n\n This message is used for PnP node-ID allocation on all transports where the maximum transmission unit size is\n sufficiently large. For low-MTU transports such as Classic CAN there is an older version of the definition (v1)\n that takes the low MTU into account (the unique-ID value is replaced with a short hash in order to fit the data\n into one 7-byte-long transfer).\n\n Generally, the randomly chosen values of the request period (Trequest) should be in the range from 0 to 1 seconds.\n Applications that are not concerned about the allocation time are recommended to pick higher values, as it will\n reduce interference with other nodes where faster allocations may be desirable. The random interval shall be chosen\n anew per transmission, whereas the pseudo node-ID value is allowed to stay constant per node.\n\n The source of random data for Trequest shall be likely to yield different values for participating nodes, avoiding\n common sequences. This implies that the time since boot alone is not a sufficiently robust source of randomness,\n as that would be probable to cause nodes powered up at the same time to emit colliding messages repeatedly.\n\n The response timeout is not explicitly defined for this protocol, as the allocatee will request a new allocation\n Trequest units of time later again, unless an allocation has been granted. Since the request and response messages\n are fully idempotent, accidentally repeated messages (e.g., due to benign race conditions that are inherent to this\n protocol) are harmless.\n\n On the allocatee's side the protocol is defined through the following set of rules:\n\n   Rule A. On initialization:\n     1. The allocatee subscribes to this message.\n     2. The allocatee starts the Request Timer with a random interval of Trequest.\n\n   Rule B. On expiration of the Request Timer:\n     1. Request Timer restarts with a random interval of Trequest (chosen anew).\n     2. The allocatee broadcasts an allocation request message, where the fields are populated as follows:\n        node_id   - the preferred node-ID, or the highest valid value if the allocatee doesn't have any preference.\n        unique_id - the 128-bit unique-ID of the allocatee, same value that is reported via uavcan.node.GetInfo.\n\n   Rule C. On an allocation message WHERE (source node-ID is non-anonymous, i.e., regular allocation response)\n                                    AND   (the field unique_id matches the allocatee's unique-ID):\n     1. Request Timer stops.\n     2. The allocatee initializes its node-ID with the received value.\n     3. The allocatee terminates its subscription to allocation messages.\n     4. Exit.\n\n As can be seen, the algorithm assumes that the allocatee will continue to emit requests at random intervals\n until an allocation is granted or the allocatee is disconnected."
            )]
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct NodeIDAllocationData {
                #[cfg_attr(
                    not(doctest),
                    doc = " If the message transfer is anonymous (i.e., allocation request), this is the preferred ID.\n If the message transfer is non-anonymous (i.e., allocation response), this is the allocated ID.\n\n If the allocatee does not have any preference, it should request the highest possible node-ID. Keep in mind that\n the two highest node-ID values are reserved for network maintenance tools; requesting those is not prohibited,\n but the allocator is recommended to avoid granting these node-ID, using nearest available lower value instead.\n The allocator will traverse the allocation table starting from the preferred node-ID upward,\n until a free node-ID is found (or the first ID reserved for network maintenance tools is reached).\n If a free node-ID could not be found, the allocator will restart the search from the preferred node-ID\n downward, until a free node-ID is found."
                )]
                ///
                /// `uavcan.node.ID.1.0`
                ///
                /// Always aligned,
                /// size 16 bits
                pub node_id: crate::uavcan::node::id_1_0::ID,
                #[cfg_attr(
                    not(doctest),
                    doc = " The unique-ID of the allocatee. This is the SAME value that is reported via uavcan.node.GetInfo.\n The value is subjected to the same set of constraints; e.g., it can't be changed while the node is running,\n and the same value should be unlikely to be used by any two different nodes anywhere in the world.\n\n If this is a non-anonymous transfer (i.e., allocation response), allocatees will match this value against their\n own unique-ID, and ignore the message if there is no match. If the IDs match, then the field node_id contains\n the allocated node-ID value for this node."
                )]
                ///
                /// `saturated uint8[16]`
                ///
                /// Always aligned,
                /// size 128 bits
                pub unique_id: [u8; 16],
            }
            impl ::canadensis_encoding::DataType for NodeIDAllocationData {
                /// This type is delimited with an extent of 48 bytes.
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " 2048 bits + 11 bit length + 4 bit padding = 2064 bits = 258 bytes"
                )]
                pub struct Bit {
                    ///
                    /// `saturated bool[<=2048]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::canadensis_encoding::bits::BitArray<256>,
                }
                impl ::canadensis_encoding::DataType for Bit {
                    /// This type is sealed.
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Bit {}
                impl Bit {}
                impl ::canadensis_encoding::Serialize for Bit {
                    fn size_bits(&self) -> usize {
                        16 + (self.value).len() * 1 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16((self.value).len() as u16);
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
                ///
                pub struct Integer16 {
                    ///
                    /// `saturated int16[<=128]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<i16, 128>,
                }
                impl ::canadensis_encoding::DataType for Integer16 {
                    /// This type is sealed.
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
                ///
                pub struct Integer32 {
                    ///
                    /// `saturated int32[<=64]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<i32, 64>,
                }
                impl ::canadensis_encoding::DataType for Integer32 {
                    /// This type is sealed.
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
                ///
                pub struct Integer64 {
                    ///
                    /// `saturated int64[<=32]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<i64, 32>,
                }
                impl ::canadensis_encoding::DataType for Integer64 {
                    /// This type is sealed.
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
                ///
                pub struct Integer8 {
                    ///
                    /// `saturated int8[<=256]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<i8, 256>,
                }
                impl ::canadensis_encoding::DataType for Integer8 {
                    /// This type is sealed.
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
                ///
                pub struct Natural16 {
                    ///
                    /// `saturated uint16[<=128]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<u16, 128>,
                }
                impl ::canadensis_encoding::DataType for Natural16 {
                    /// This type is sealed.
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
                ///
                pub struct Natural32 {
                    ///
                    /// `saturated uint32[<=64]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<u32, 64>,
                }
                impl ::canadensis_encoding::DataType for Natural32 {
                    /// This type is sealed.
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
                ///
                pub struct Natural64 {
                    ///
                    /// `saturated uint64[<=32]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<u64, 32>,
                }
                impl ::canadensis_encoding::DataType for Natural64 {
                    /// This type is sealed.
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
                ///
                pub struct Natural8 {
                    ///
                    /// `saturated uint8[<=256]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<u8, 256>,
                }
                impl ::canadensis_encoding::DataType for Natural8 {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(not(doctest), doc = " Exactly representable integers: [-2048, +2048]")]
                pub struct Real16 {
                    ///
                    /// `saturated float16[<=128]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<::half::f16, 128>,
                }
                impl ::canadensis_encoding::DataType for Real16 {
                    /// This type is sealed.
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
                            cursor.write_f16(*value);
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
                                        let _ = elements.push(cursor.read_f16());
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " Exactly representable integers: [-16777216, +16777216]"
                )]
                pub struct Real32 {
                    ///
                    /// `saturated float32[<=64]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<f32, 64>,
                }
                impl ::canadensis_encoding::DataType for Real32 {
                    /// This type is sealed.
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
                ///
                #[cfg_attr(
                    not(doctest),
                    doc = " Exactly representable integers: [-2**53, +2**53]"
                )]
                pub struct Real64 {
                    ///
                    /// `saturated float64[<=32]`
                    ///
                    /// Always aligned,
                    /// size ranges from 0 to 2048 bits
                    pub value: ::heapless::Vec<f64, 32>,
                }
                impl ::canadensis_encoding::DataType for Real64 {
                    /// This type is sealed.
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
            ///
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct Empty {}
            impl ::canadensis_encoding::DataType for Empty {
                /// This type is sealed.
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
                ///
                pub struct Bit {
                    ///
                    /// `saturated bool`
                    ///
                    /// Always aligned,
                    /// size 1 bits
                    pub value: bool,
                }
                impl ::canadensis_encoding::DataType for Bit {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Integer16 {
                    ///
                    /// `saturated int16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub value: i16,
                }
                impl ::canadensis_encoding::DataType for Integer16 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Integer32 {
                    ///
                    /// `saturated int32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub value: i32,
                }
                impl ::canadensis_encoding::DataType for Integer32 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Integer64 {
                    ///
                    /// `saturated int64`
                    ///
                    /// Always aligned,
                    /// size 64 bits
                    pub value: i64,
                }
                impl ::canadensis_encoding::DataType for Integer64 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Integer8 {
                    ///
                    /// `saturated int8`
                    ///
                    /// Always aligned,
                    /// size 8 bits
                    pub value: i8,
                }
                impl ::canadensis_encoding::DataType for Integer8 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Natural16 {
                    ///
                    /// `saturated uint16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for Natural16 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Natural32 {
                    ///
                    /// `saturated uint32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub value: u32,
                }
                impl ::canadensis_encoding::DataType for Natural32 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Natural64 {
                    ///
                    /// `saturated uint64`
                    ///
                    /// Always aligned,
                    /// size 64 bits
                    pub value: u64,
                }
                impl ::canadensis_encoding::DataType for Natural64 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Natural8 {
                    ///
                    /// `saturated uint8`
                    ///
                    /// Always aligned,
                    /// size 8 bits
                    pub value: u8,
                }
                impl ::canadensis_encoding::DataType for Natural8 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Real16 {
                    #[cfg_attr(
                        not(doctest),
                        doc = " Exactly representable integers: [-2048, +2048]"
                    )]
                    ///
                    /// `saturated float16`
                    ///
                    /// Always aligned,
                    /// size 16 bits
                    pub value: ::half::f16,
                }
                impl ::canadensis_encoding::DataType for Real16 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Real32 {
                    #[cfg_attr(
                        not(doctest),
                        doc = " Exactly representable integers: [-16777216, +16777216]"
                    )]
                    ///
                    /// `saturated float32`
                    ///
                    /// Always aligned,
                    /// size 32 bits
                    pub value: f32,
                }
                impl ::canadensis_encoding::DataType for Real32 {
                    /// This type is sealed.
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
                ///
                #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                #[repr(C, packed)]
                pub struct Real64 {
                    #[cfg_attr(
                        not(doctest),
                        doc = " Exactly representable integers: [-2**53, +2**53]"
                    )]
                    ///
                    /// `saturated float64`
                    ///
                    /// Always aligned,
                    /// size 64 bits
                    pub value: f64,
                }
                impl ::canadensis_encoding::DataType for Real64 {
                    /// This type is sealed.
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " A UTF8-encoded string of text.\n Since the string is represented as a dynamic array of bytes, it is not null-terminated. Like Pascal string."
            )]
            pub struct String {
                ///
                /// `saturated uint8[<=256]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 2048 bits
                pub value: ::heapless::Vec<u8, 256>,
            }
            impl ::canadensis_encoding::DataType for String {
                /// This type is sealed.
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " An unstructured collection of bytes, e.g., raw binary image."
            )]
            pub struct Unstructured {
                ///
                /// `saturated uint8[<=256]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 2048 bits
                pub value: ::heapless::Vec<u8, 256>,
            }
            impl ::canadensis_encoding::DataType for Unstructured {
                /// This type is sealed.
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(384);

            /// `uavcan.register.Access.1.0`
            ///
            /// Size ranges from 2 to 515 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Registers are strongly-typed named values used to store the configuration parameters of a node.\n This service is used to write and read a register.\n\n\n   READ/WRITE BEHAVIORS\n\n The write operation is performed first, unless skipped by sending an empty value in the request.\n The server may attempt to convert the type of the supplied value to the correct type if there is a type mismatch\n (e.g. uint8 may be converted to uint16); however, servers are not required to perform implicit type conversion,\n and the rules of such conversion are not explicitly specified, so this behavior should not be relied upon.\n\n On the next step the register will be read regardless of the outcome of the write operation. As such, if the write\n operation could not be performed (e.g. due to a type mismatch or any other issue), the register will retain its old\n value. By evaluating the response the caller can determine whether the register was written successfully.\n\n The write-read sequence is not guaranteed to be atomic, meaning that external influences may cause the register to\n change its value between the write and the subsequent read operation. The caller is responsible for handling that\n case properly.\n\n The timestamp provided in the response corresponds to the time when the register was read. The timestamp may\n be empty if the server does not support timestamping or its clock is not (yet) synchronized with the network.\n\n If only read is desired, but not write, the caller shall provide a value of type 'empty'. That will signal the server\n that the write operation shall be skipped, and it will proceed to read the register immediately.\n\n If the requested register does not exist, the write operation will have no effect and the returned value will be\n empty. Existing registers should not return 'empty' when read since that would make them indistinguishable from\n nonexistent registers.\n\n\n   REGISTER DEFINITION REQUIREMENTS\n\n Registers shall never change their type or flags as long as the server is running. Meaning that:\n   - Mutability and persistence flags cannot change their states.\n   - Read operations shall always return values of the same type and same dimensionality.\n     The dimensionality requirement does not apply to inherently variable-length values such as strings and\n     unstructured chunks.\n\n Register name should contain only:\n   - Lowercase ASCII alphanumeric characters (a-z, 0-9)\n   - Full stop (.)\n   - Low line (underscore) (_)\n With the following limitations/recommendations:\n   - The name shall not begin with a decimal digit (0-9).\n   - The name shall neither begin nor end with a full stop.\n   - A low line shall not be followed by a non-alphanumeric character.\n   - The name should contain at least one full stop character.\n Other patterns and ASCII characters are reserved for special function registers (introduced below).\n\n\n   ENVIRONMENT VARIABLES\n\n This section applies only to software nodes executed in a high-level operating system that supports environment\n variables or an equivalent mechanism.\n\n When a software node is launched, it is usually necessary to provide some of its configuration information early,\n particularly that which is related to Cyphal networking, before the node is started. Environment variables offer\n a convenient way of addressing this. Software nodes that support the register interface should evaluate the\n available environment variables during initialization and update their registers (whether they are stored in\n a persistent storage or in memory) accoringly. This should be completed before the first register read access.\n\n A register name is mapped to an environment variable name as follows:\n   - the name is upper-cased;\n   - full stop characters are replaced with double low line characters.\n For example: 'motor.inductance_dq' is mapped to 'MOTOR__INDUCTANCE_DQ'.\n\n Register values are represented in environment variables as follows:\n   - string:                         utf-8 or platform-specific\n   - unstructured:                   as-is\n   - bit, integer*, natural*, real*: space-separated decimals\n\n If an environment variable matches the name of an existing register but its value cannot be converted to the\n register's type, an error should be raised.\n\n If an environment variable does not match the name of any register, it may be ignored. However, if the implementation\n can reliably deduce the type and purpose of the register, it may create one automatically. This provision is to\n support applications where the register schema may be altered by configuration.\n\n\n   SPECIAL FUNCTION REGISTERS\n\n The following optional special function register names are defined:\n   - suffix '<' is used to define an immutable persistent value that contains the maximum value\n     of the respective register.\n   - suffix '>' is like above, used to define the minimum value of the respective register.\n   - suffix '=' is like above, used to define the default value of the respective register.\n   - prefix '*' is reserved for raw memory access (to be defined later).\n Examples:\n   - register name \"system.parameter\"\n   - maximum value is contained in the register named \"system.parameter<\" (optional)\n   - minimum value is contained in the register named \"system.parameter>\" (optional)\n   - default value is contained in the register named \"system.parameter=\" (optional)\n\n The type and dimensionality of the special function registers containing the minimum, maximum, and the default\n value of a register shall be the same as those of the register they relate to.\n\n If a written value exceeds the minimum/maximum specified by the respective special function registers,\n the server may either adjust the value automatically, or to retain the old value, depending on which behavior\n suits the objectives of the application better.\n The values of registers containing non-scalar numerical entities should be compared elementwise.\n\n\n   STANDARD REGISTERS\n\n The following table specifies the register name patterns that are reserved by the specification for\n common functions. These conventions are not mandatory to follow, but implementers are recommended to adhere because\n they enable enhanced introspection capabilities and simplify device configuration and diagnostics.\n\n   REGISTER NAME PATTERN                               TYPE            FLAGS                   RECOMMENDED DEFAULT\n =====================================================================================================================\n\n   uavcan.node.id                                      natural16[1]    mutable, persistent     65535 (unset/PnP)\n\n Contains the node-ID of the local node. Values above the maximum valid node-ID for the current transport\n indicate that the node-ID is not set; if plug-and-play is supported, it will be used by the node to obtain an\n automatic node-ID. Invalid values other than 65535 should be avoided for consistency.\n\n ---------------------------------------------------------------------------------------------------------------------\n\n   uavcan.node.description                             string          mutable, persistent     (empty)\n\n User/integrator-defined, human-readable description of this specific node.\n This is intended for use by a system integrator and should not be set by the manufacturer of a component.\n For example: on a quad-rotor drone this might read \"motor 2\" for one of the ESC nodes.\n\n ---------------------------------------------------------------------------------------------------------------------\n\n   uavcan.pub.PORT_NAME.id                             natural16[1]    mutable, persistent     65535 (unset, invalid)\n   uavcan.sub.PORT_NAME.id                             ditto           ditto                   ditto\n   uavcan.cln.PORT_NAME.id                             ditto           ditto                   ditto\n   uavcan.srv.PORT_NAME.id                             ditto           ditto                   ditto\n\n Publication/subscription/client/server port-ID, respectively. These registers are configured by the system integrator\n or an autoconfiguration authority when the node is first connected to a network.\n\n The \"PORT_NAME\" defines the human-friendly name of the port, which is related to the corresponding function\n or a network service supported by the node. The name shall match the following POSIX ERE expression:\n\n   [a-zA-Z_][a-zA-Z0-9_.]*\n\n The names are defined by the vendor of the node. The user/integrator is expected to understand their meaning and\n relation to the functional capabilities of the node by reading the technical documentation provided by the vendor.\n\n A port whose port-ID register is unset (invalid value) remains inactive (unused); the corresponding function may\n be disabled. For example, a register named \"uavcan.pub.measurement.id\" defines the subject-ID of a measurement\n published by this node; if the register contains an invalid value (above the maximum valid subject-ID),\n said measurement is not published.\n\n The same name is used in other similar registers defined below. Network introspection and autoconfiguration tools\n will expect to find a register of this form for every configurable port supported by the node.\n\n ---------------------------------------------------------------------------------------------------------------------\n\n   uavcan.pub.PORT_NAME.type                           string          immutable, persistent   N/A\n   uavcan.sub.PORT_NAME.type                           ditto           ditto                   ditto\n   uavcan.cln.PORT_NAME.type                           ditto           ditto                   ditto\n   uavcan.srv.PORT_NAME.type                           ditto           ditto                   ditto\n\n Publication/subscription/client/server full data type name and dot-separated version numbers, respectively.\n These registers are set by the vendor once and typically they are to remain unchanged (hence \"immutable\").\n The \"PORT_NAME\" defines the human-friendly name of the port as specified above.\n For example, a register named \"uavcan.pub.measurement.type\" may contain \"uavcan.si.sample.angle.Quaternion.1.0\".\n\n ---------------------------------------------------------------------------------------------------------------------\n\n   uavcan.diagnostic.*\n\n Prefix reserved for future use.\n\n ---------------------------------------------------------------------------------------------------------------------\n\n   uavcan.can.bitrate                                  natural32[2]    implementation-defined  implementation-defined\n   uavcan.can.iface                                    string          mutable, persistent     implementation-defined\n\n These registers are only relevant for nodes that support Cyphal/CAN.\n\n uavcan.can.bitrate defines the CAN bus bit rate: the first value is the arbitration bit rate, the second is the\n data phase bit rate. Nodes that support only Classic CAN should ignore the second value. Nodes that support CAN FD\n should initialize in the Classic CAN mode (MTU 8 bytes, BRS flag not set) if the values are equal. If CAN bitrate\n is not configurable or is always auto-detected, this register may be omitted or made immutable; otherwise it should\n be mutable and persistent.\n\n uavcan.can.iface is only relevant for software nodes or nodes that are capable of using different CAN interfaces.\n The value is a space-separated list of CAN interface names to use. The name format is implementation-defined\n (for example, \"can0\").\n\n ---------------------------------------------------------------------------------------------------------------------\n\n   uavcan.udp.*\n\n Prefix reserved for future use.\n\n ---------------------------------------------------------------------------------------------------------------------#\n\n   uavcan.serial.*\n\n Prefix reserved for future use.\n\n ---------------------------------------------------------------------------------------------------------------------"
            )]
            pub struct AccessRequest {
                #[cfg_attr(
                    not(doctest),
                    doc = " The name of the accessed register. Shall not be empty.\n Use the List service to obtain the list of registers on the node."
                )]
                ///
                /// `uavcan.register.Name.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2048 bits
                pub name: crate::uavcan::register::name_1_0::Name,
                #[cfg_attr(
                    not(doctest),
                    doc = " Value to be written. Empty if no write is required."
                )]
                ///
                /// `uavcan.register.Value.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2072 bits
                pub value: crate::uavcan::register::value_1_0::Value,
            }
            impl ::canadensis_encoding::DataType for AccessRequest {
                /// This type is sealed.
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
            ///
            pub struct AccessResponse {
                #[cfg_attr(
                    not(doctest),
                    doc = " The moment of time when the register was read (not written).\n Zero if the server does not support timestamping."
                )]
                ///
                /// `uavcan.time.SynchronizedTimestamp.1.0`
                ///
                /// Always aligned,
                /// size 56 bits
                pub timestamp:
                    crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                #[cfg_attr(
                    not(doctest),
                    doc = " Mutable means that the register can be written using this service.\n Immutable registers cannot be written, but that doesn't imply that their values are constant (unchanging)."
                )]
                ///
                /// `saturated bool`
                ///
                /// Always aligned,
                /// size 1 bits
                pub mutable: bool,
                #[cfg_attr(
                    not(doctest),
                    doc = " Persistence means that the register retains its value permanently across power cycles or any other changes\n in the state of the server, until it is explicitly overwritten (either via Cyphal, any other interface,\n or by the device itself).\n\n The server is recommended to manage persistence automatically by committing changed register values to a\n non-volatile storage automatically as necessary. If automatic persistence management is not implemented, it\n can be controlled manually via the standard service uavcan.node.ExecuteCommand. The same service can be used\n to return the configuration to a factory-default state. Please refer to its definition for more information.\n\n Consider the following examples:\n   - Configuration parameters are usually both mutable and persistent.\n   - Diagnostic values are usually immutable and non-persisient.\n   - Registers that trigger an activity when written are typically mutable but non-persisient.\n   - Registers that contain factory-programmed values such as calibration coefficients that can't\n     be changed are typically immutable but persistent."
                )]
                ///
                /// `saturated bool`
                ///
                /// Not always aligned,
                /// size 1 bits
                pub persistent: bool,
                // 6 bits of padding
                #[cfg_attr(
                    not(doctest),
                    doc = " The value of the register when it was read (beware of race conditions).\n Registers never change their type and dimensionality while the node is running.\n Empty value means that the register does not exist (in this case the flags should be cleared/ignored).\n By comparing the returned value against the write request the caller can determine whether the register\n was written successfully, unless write was not requested.\n An empty value shall never be returned for an existing register."
                )]
                ///
                /// `uavcan.register.Value.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2072 bits
                pub value: crate::uavcan::register::value_1_0::Value,
            }
            impl ::canadensis_encoding::DataType for AccessResponse {
                /// This type is sealed.
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(385);

            /// `uavcan.register.List.1.0`
            ///
            /// Fixed size 2 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " This service allows the caller to discover the names of all registers available on the server\n by iterating the index field from zero until an empty name is returned.\n\n The ordering of the registers shall remain constant while the server is running.\n The ordering is not guaranteed to remain unchanged when the server node is restarted."
            )]
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct ListRequest {
                ///
                /// `saturated uint16`
                ///
                /// Always aligned,
                /// size 16 bits
                pub index: u16,
            }
            impl ::canadensis_encoding::DataType for ListRequest {
                /// This type is sealed.
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
            ///
            pub struct ListResponse {
                #[cfg_attr(
                    not(doctest),
                    doc = " Empty name in response means that the index is out of bounds, i.e., discovery is finished."
                )]
                ///
                /// `uavcan.register.Name.1.0`
                ///
                /// Always aligned,
                /// size ranges from 8 to 2048 bits
                pub name: crate::uavcan::register::name_1_0::Name,
            }
            impl ::canadensis_encoding::DataType for ListResponse {
                /// This type is sealed.
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
            ///
            #[cfg_attr(not(doctest), doc = " An UTF8-encoded register name.")]
            pub struct Name {
                ///
                /// `saturated uint8[<=255]`
                ///
                /// Always aligned,
                /// size ranges from 0 to 2040 bits
                pub name: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for Name {
                /// This type is sealed.
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " This union contains all possible value types supported by the register protocol.\n Numeric types can be either scalars or arrays; the former is a special case of the latter."
            )]
            pub enum Value {
                #[cfg_attr(not(doctest), doc = " Tag 0     Used to represent an undefined value")]
                ///
                /// uavcan.primitive.Empty.1.0
                Empty(crate::uavcan::primitive::empty_1_0::Empty),
                #[cfg_attr(not(doctest), doc = " Tag 1     UTF-8 encoded text")]
                ///
                /// uavcan.primitive.String.1.0
                String(crate::uavcan::primitive::string_1_0::String),
                #[cfg_attr(not(doctest), doc = " Tag 2     Raw unstructured binary image")]
                ///
                /// uavcan.primitive.Unstructured.1.0
                Unstructured(crate::uavcan::primitive::unstructured_1_0::Unstructured),
                #[cfg_attr(not(doctest), doc = " Tag 3     Bit array")]
                ///
                /// uavcan.primitive.array.Bit.1.0
                Bit(crate::uavcan::primitive::array::bit_1_0::Bit),
                #[cfg_attr(not(doctest), doc = " Tag 4")]
                ///
                /// uavcan.primitive.array.Integer64.1.0
                Integer64(crate::uavcan::primitive::array::integer64_1_0::Integer64),
                #[cfg_attr(not(doctest), doc = " Tag 5")]
                ///
                /// uavcan.primitive.array.Integer32.1.0
                Integer32(crate::uavcan::primitive::array::integer32_1_0::Integer32),
                #[cfg_attr(not(doctest), doc = " Tag 6")]
                ///
                /// uavcan.primitive.array.Integer16.1.0
                Integer16(crate::uavcan::primitive::array::integer16_1_0::Integer16),
                #[cfg_attr(not(doctest), doc = " Tag 7")]
                ///
                /// uavcan.primitive.array.Integer8.1.0
                Integer8(crate::uavcan::primitive::array::integer8_1_0::Integer8),
                #[cfg_attr(not(doctest), doc = " Tag 8")]
                ///
                /// uavcan.primitive.array.Natural64.1.0
                Natural64(crate::uavcan::primitive::array::natural64_1_0::Natural64),
                #[cfg_attr(not(doctest), doc = " Tag 9")]
                ///
                /// uavcan.primitive.array.Natural32.1.0
                Natural32(crate::uavcan::primitive::array::natural32_1_0::Natural32),
                #[cfg_attr(not(doctest), doc = " Tag 10")]
                ///
                /// uavcan.primitive.array.Natural16.1.0
                Natural16(crate::uavcan::primitive::array::natural16_1_0::Natural16),
                #[cfg_attr(not(doctest), doc = " Tag 11")]
                ///
                /// uavcan.primitive.array.Natural8.1.0
                Natural8(crate::uavcan::primitive::array::natural8_1_0::Natural8),
                #[cfg_attr(
                    not(doctest),
                    doc = " Tag 12    Exactly representable integers: [-2**53,    +2**53]"
                )]
                ///
                /// uavcan.primitive.array.Real64.1.0
                Real64(crate::uavcan::primitive::array::real64_1_0::Real64),
                #[cfg_attr(
                    not(doctest),
                    doc = " Tag 13    Exactly representable integers: [-16777216, +16777216]"
                )]
                ///
                /// uavcan.primitive.array.Real32.1.0
                Real32(crate::uavcan::primitive::array::real32_1_0::Real32),
                #[cfg_attr(
                    not(doctest),
                    doc = " Tag 14    Exactly representable integers: [-2048,     +2048]\n Empty and the tag\n 258 bytes per field max and the tag"
                )]
                ///
                /// uavcan.primitive.array.Real16.1.0
                Real16(crate::uavcan::primitive::array::real16_1_0::Real16),
            }
            impl ::canadensis_encoding::DataType for Value {
                /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub meter_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub meter_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    pub struct Quaternion {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[4]`
                        ///
                        /// Always aligned,
                        /// size 128 bits
                        pub wxyz: [f32; 4],
                    }
                    impl ::canadensis_encoding::DataType for Quaternion {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub radian: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub radian_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub radian_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub radian_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub radian_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct WideScalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float64`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub second: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub coulomb: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub ampere: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub joule: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub newton: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub newton: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub hertz: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    pub struct WideScalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float64`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub meter: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        /// This type is sealed.
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
                    ///
                    pub struct WideVector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float64[3]`
                        ///
                        /// Always aligned,
                        /// size 192 bits
                        pub meter: [f64; 3],
                    }
                    impl ::canadensis_encoding::DataType for WideVector3 {
                        /// This type is sealed.
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
            pub mod luminance {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.luminance.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub candela_per_square_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                            cursor.write_f32(self.candela_per_square_meter);
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
                                candela_per_square_meter: { cursor.read_f32() },
                            })
                        }
                    }
                }
            }
            pub mod magnetic_field_strength {
                #[allow(deprecated)]
                #[deprecated]
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.magnetic_field_strength.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Use v1.1 instead where the unit of measure is named correctly."
                    )]
                    #[deprecated]
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub tesla: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                pub mod scalar_1_1 {
                    /// `uavcan.si.sample.magnetic_field_strength.Scalar.1.1`
                    ///
                    /// Fixed size 11 bytes
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub ampere_per_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                            cursor.write_f32(self.ampere_per_meter);
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
                                ampere_per_meter: { cursor.read_f32() },
                            })
                        }
                    }
                }
                #[allow(deprecated)]
                #[deprecated]
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.magnetic_field_strength.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Use v1.1 instead where the unit of measure is named correctly."
                    )]
                    #[deprecated]
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub tesla: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                pub mod vector3_1_1 {
                    /// `uavcan.si.sample.magnetic_field_strength.Vector3.1.1`
                    ///
                    /// Fixed size 19 bytes
                    ///
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub ampere_per_meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                            for value in (self.ampere_per_meter).iter() {
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
                                ampere_per_meter: {
                                    [cursor.read_f32(), cursor.read_f32(), cursor.read_f32()]
                                },
                            })
                        }
                    }
                }
            }
            pub mod magnetic_flux_density {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.magnetic_flux_density.Scalar.1.0`
                    ///
                    /// Fixed size 11 bytes
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub tesla: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    /// `uavcan.si.sample.magnetic_flux_density.Vector3.1.0`
                    ///
                    /// Fixed size 19 bytes
                    ///
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub tesla: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub kilogram: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub watt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub pascal: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub kelvin: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub newton_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub newton_meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Vector3 {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub meter_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub volt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub cubic_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    pub struct Scalar {
                        ///
                        /// `uavcan.time.SynchronizedTimestamp.1.0`
                        ///
                        /// Always aligned,
                        /// size 56 bits
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub cubic_meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub meter_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub meter_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Quaternion {
                        ///
                        /// `saturated float32[4]`
                        ///
                        /// Always aligned,
                        /// size 128 bits
                        pub wxyz: [f32; 4],
                    }
                    impl ::canadensis_encoding::DataType for Quaternion {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub radian: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub radian_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub radian_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub radian_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub radian_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct WideScalar {
                        ///
                        /// `saturated float64`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub second: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub coulomb: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub ampere: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub joule: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub newton: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub newton: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub hertz: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct WideScalar {
                        ///
                        /// `saturated float64`
                        ///
                        /// Always aligned,
                        /// size 64 bits
                        pub meter: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct WideVector3 {
                        ///
                        /// `saturated float64[3]`
                        ///
                        /// Always aligned,
                        /// size 192 bits
                        pub meter: [f64; 3],
                    }
                    impl ::canadensis_encoding::DataType for WideVector3 {
                        /// This type is sealed.
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
            pub mod luminance {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.luminance.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub candela_per_square_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                            ::memoffset::offset_of!(Scalar, candela_per_square_meter) * 8,
                            0
                        );
                    }
                }
            }
            pub mod magnetic_field_strength {
                #[allow(deprecated)]
                #[deprecated]
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.magnetic_field_strength.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Use v1.1 instead where the unit of measure is named correctly."
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    #[deprecated]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub tesla: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                pub mod scalar_1_1 {
                    /// `uavcan.si.unit.magnetic_field_strength.Scalar.1.1`
                    ///
                    /// Fixed size 4 bytes
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub ampere_per_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                        assert_eq!(::memoffset::offset_of!(Scalar, ampere_per_meter) * 8, 0);
                    }
                }
                #[allow(deprecated)]
                #[deprecated]
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.magnetic_field_strength.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    ///
                    #[cfg_attr(
                        not(doctest),
                        doc = " Use v1.1 instead where the unit of measure is named correctly."
                    )]
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    #[deprecated]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub tesla: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                pub mod vector3_1_1 {
                    /// `uavcan.si.unit.magnetic_field_strength.Vector3.1.1`
                    ///
                    /// Fixed size 12 bytes
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub ampere_per_meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                        assert_eq!(::memoffset::offset_of!(Vector3, ampere_per_meter) * 8, 0);
                    }
                }
            }
            pub mod magnetic_flux_density {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.magnetic_flux_density.Scalar.1.0`
                    ///
                    /// Fixed size 4 bytes
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub tesla: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    /// `uavcan.si.unit.magnetic_flux_density.Vector3.1.0`
                    ///
                    /// Fixed size 12 bytes
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub tesla: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub kilogram: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub watt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub pascal: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub kelvin: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub newton_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub newton_meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Vector3 {
                        ///
                        /// `saturated float32[3]`
                        ///
                        /// Always aligned,
                        /// size 96 bits
                        pub meter_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub volt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub cubic_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
                    ///
                    #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
                    #[repr(C, packed)]
                    pub struct Scalar {
                        ///
                        /// `saturated float32`
                        ///
                        /// Always aligned,
                        /// size 32 bits
                        pub cubic_meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        /// This type is sealed.
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
            #[cfg_attr(not(doctest), doc = "The fixed ID of this service")]
            pub const SERVICE: ::canadensis_core::ServiceId =
                ::canadensis_core::ServiceId::from_truncating(510);

            /// `uavcan.time.GetSynchronizationMasterInfo.0.1`
            ///
            /// Fixed size 0 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Every node that acts as a time synchronization master, or is capable of acting as such,\n should support this service.\n Its objective is to provide information about which time system is currently used in the network.\n\n Once a time system is chosen, it cannot be changed as long as at least one node on the network is running.\n In other words, the time system cannot be changed while the network is operating.\n An implication of this is that if there are redundant time synchronization masters, they all shall\n use the same time system always."
            )]
            #[derive(::zerocopy::FromBytes, ::zerocopy::AsBytes)]
            #[repr(C, packed)]
            pub struct GetSynchronizationMasterInfoRequest {}
            impl ::canadensis_encoding::DataType for GetSynchronizationMasterInfoRequest {
                /// This type is delimited with an extent of 48 bytes.
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
            ///
            pub struct GetSynchronizationMasterInfoResponse {
                #[cfg_attr(
                    not(doctest),
                    doc = " [second^2]\n Error variance, in second^2, of the time value reported by this master.\n This value is allowed to change freely while the master is running.\n For example, if the master's own clock is synchronized with a GNSS, the error variance is expected to increase\n as signal reception deteriorates. If the signal is lost, this value is expected to grow steadily, the rate of\n growth would be dependent on the quality of the time keeping hardware available locally (bad hardware yields\n faster growth). Once the signal is regained, this value would drop back to nominal."
                )]
                ///
                /// `saturated float32`
                ///
                /// Always aligned,
                /// size 32 bits
                pub error_variance: f32,
                #[cfg_attr(
                    not(doctest),
                    doc = " Time system currently in use by the master.\n Cannot be changed while the network is operating."
                )]
                ///
                /// `uavcan.time.TimeSystem.0.1`
                ///
                /// Always aligned,
                /// size 8 bits
                pub time_system: crate::uavcan::time::time_system_0_1::TimeSystem,
                #[cfg_attr(
                    not(doctest),
                    doc = " Actual information about TAI provided by this master, if supported.\n The fields in this data type are optional."
                )]
                ///
                /// `uavcan.time.TAIInfo.0.1`
                ///
                /// Always aligned,
                /// size 16 bits
                pub tai_info: crate::uavcan::time::tai_info_0_1::TAIInfo,
            }
            impl ::canadensis_encoding::DataType for GetSynchronizationMasterInfoResponse {
                /// This type is delimited with an extent of 192 bytes.
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
            #[cfg_attr(not(doctest), doc = "The fixed subject ID for this message type")]
            pub const SUBJECT: ::canadensis_core::SubjectId =
                ::canadensis_core::SubjectId::from_truncating(7168);

            /// `uavcan.time.Synchronization.1.0`
            ///
            /// Fixed size 7 bytes
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Network-wide time synchronization message.\n Any node that publishes timestamped data should use this time reference.\n\n The time synchronization algorithm is based on the work\n \"Implementing a Distributed High-Resolution Real-Time Clock using the CAN-Bus\" by M. Gergeleit and H. Streich.\n The general idea of the algorithm is to have one or more nodes that periodically publish a message of this type\n containing the exact timestamp of the PREVIOUS transmission of this message.\n A node that publishes this message periodically is referred to as a \"time synchronization master\",\n whereas nodes that synchronize their clocks with the master are referred to as \"time synchronization slaves\".\n\n Once a time base is chosen, it cannot be changed as long as at least one node on the network is running.\n In other words, the time base cannot be changed while the network is operating.\n An implication of this is that if there are redundant time synchronization masters, they all shall\n use the same time base.\n\n The resolution is dependent on the transport and its physical layer, but generally it can be assumed\n to be close to one bit time but not better than one microsecond (e.g., for a 500 kbps CAN bus,\n the resolution is two microseconds). The maximum accuracy is achievable only if the transport layer\n supports precise timestamping in hardware; otherwise, the accuracy may be degraded.\n\n This algorithm allows the slaves to precisely estimate the difference (i.e., phase error) between their\n local time and the master clock they are synchronized with. The algorithm for clock rate adjustment\n is entirely implementation-defined (for example, a simple phase-locked loop or a PID rate controller can be used).\n\n The network can accommodate more than one time synchronization master for purposes of increased reliability:\n if one master fails, the others will continue to provide the network with accurate and consistent time information.\n The risk of undesirable transients while the masters are swapped is mitigated by the requirement that all masters\n use the same time base at all times, as described above.\n\n The master with the lowest node-ID is called the \"dominant master\". The current dominant master ceases to be one\n if its last synchronization message was published more than 3X seconds ago, where X is the time interval\n between the last and the previous messages published by it. In this case, the master with the next-higher node-ID\n will take over as the new dominant master. The current dominant master will be displaced immediately as soon as\n the first message from a new master with a lower node-ID is seen on the bus.\n\n In the presence of multiple masters, they all publish their time synchronization messages concurrently at all times.\n The slaves shall listen to the master with the lowest node-ID and ignore the messages published by masters with\n higher node-ID values.\n\n Currently, there is a work underway to develop and validate a highly robust fault-operational time synchronization\n algorithm where the slaves select the median time base among all available masters rather than using only the\n one with the lowest node-ID value. Follow the work at https://forum.opencyphal.org. When complete, this algorithm\n will be added in a backward-compatible way as an option for high-reliability systems.\n\n For networks with redundant transports, the timestamp value published on different interfaces is likely to be\n different, since different transports are generally not expected to be synchronized. Synchronization slaves\n are allowed to use any of the available redundant interfaces for synchronization at their discretion.\n\n The following pseudocode shows the logic of a time synchronization master. This example assumes that the master\n does not need to synchronize its own clock with other masters on the bus, which is the case if the current master\n is the only master, or if all masters synchronize their clocks with a robust external source, e.g., a GNSS system.\n If several masters need to synchronize their clock through the bus, their logic will be extended with the\n slave-side behavior explained later.\n\n       // State variables\n       transfer_id := 0;\n       previous_tx_timestamp_per_iface[NUM_IFACES] := {0};\n\n       // This function publishes a message with a specified transfer-ID using only one transport interface.\n       function publishMessage(transfer_id, iface_index, msg);\n\n       // This callback is invoked when the transport layer completes the transmission of a time sync message.\n       // Observe that the time sync message is always a single-frame message by virtue of its small size.\n       // The tx_timestamp argument contains the exact timestamp when the transport frame was delivered to the bus.\n       function messageTxTimestampCallback(iface_index, tx_timestamp)\n       {\n           previous_tx_timestamp_per_iface[iface_index] := tx_timestamp;\n       }\n\n       // Publishes messages of type uavcan.time.Synchronization to each available transport interface.\n       // It is assumed that this function is invoked with a fixed frequency not lower than 1 hertz.\n       function publishTimeSync()\n       {\n           for (i := 0; i < NUM_IFACES; i++)\n           {\n               message := uavcan.time.Synchronization();\n               message.previous_transmission_timestamp_usec := previous_tx_timestamp_per_iface[i];\n               previous_tx_timestamp_per_iface[i] := 0;\n               publishMessage(transfer_id, i, message);\n           }\n           transfer_id++; // Overflow shall be handled correctly\n       }\n\n (end of the master-side logic pseudocode)\n The following pseudocode describes the logic of a time synchronization slave.\n\n       // State variables:\n       previous_rx_real_timestamp := 0;            // This clock is being synchronized\n       previous_rx_monotonic_timestamp := 0;       // Monotonic time -- doesn't leap or change rate\n       previous_transfer_id := 0;\n       state := STATE_UPDATE;                      // Variants: STATE_UPDATE, STATE_ADJUST\n       master_node_id := -1;                       // Invalid value\n       iface_index := -1;                          // Invalid value\n\n       // This function adjusts the local clock by the specified amount\n       function adjustLocalTime(phase_error);\n\n       function adjust(message)\n       {\n           // Clock adjustment will be performed every second message\n           local_time_phase_error := previous_rx_real_timestamp - msg.previous_transmission_timestamp_microsecond;\n           adjustLocalTime(local_time_phase_error);\n           state := STATE_UPDATE;\n       }\n\n       function update(message)\n       {\n           // A message is assumed to have two timestamps:\n           //   Real      - sampled from the clock that is being synchronized\n           //   Monotonic - clock that never leaps and never changes rate\n           previous_rx_real_timestamp := message.rx_real_timestamp;\n           previous_rx_monotonic_timestamp := message.rx_monotonic_timestamp;\n           master_node_id := message.source_node_id;\n           iface_index := message.iface_index;\n           previous_transfer_id := message.transfer_id;\n           state := STATE_ADJUST;\n       }\n\n       // Accepts the message of type uavcan.time.Synchronization\n       function handleReceivedTimeSyncMessage(message)\n       {\n           time_since_previous_msg := message.monotonic_timestamp - previous_rx_monotonic_timestamp;\n\n           needs_init := (master_node_id < 0) or (iface_index < 0);\n           switch_master := message.source_node_id < master_node_id;\n\n           // The value publisher_timeout is computed as described in the specification (3x interval)\n           publisher_timed_out := time_since_previous_msg > publisher_timeout;\n\n           if (needs_init or switch_master or publisher_timed_out)\n           {\n               update(message);\n           }\n           else if ((message.iface_index == iface_index) and (message.source_node_id == master_node_id))\n           {\n               // Revert the state to STATE_UPDATE if needed\n               if (state == STATE_ADJUST)\n               {\n                   msg_invalid := message.previous_transmission_timestamp_microsecond == 0;\n                   // Overflow shall be handled correctly\n                   wrong_tid := message.transfer_id != (previous_transfer_id + 1);\n                   wrong_timing := time_since_previous_msg > MAX_PUBLICATION_PERIOD;\n                   if (msg_invalid or wrong_tid or wrong_timing)\n                   {\n                       state := STATE_UPDATE;\n                   }\n               }\n               // Handle the current state\n               if (state == STATE_ADJUST)\n               {\n                   adjust(message);\n               }\n               else\n               {\n                   update(message);\n               }\n           }   // else ignore\n       }\n\n (end of the slave-side logic pseudocode)"
            )]
            pub struct Synchronization {
                #[cfg_attr(
                    not(doctest),
                    doc = " The time when the PREVIOUS message was transmitted from the current publisher, in microseconds.\n If this message is published for the first time, or if the previous transmission was more than\n one second ago, this field shall be zero."
                )]
                ///
                /// `truncated uint56`
                ///
                /// Always aligned,
                /// size 56 bits
                pub previous_transmission_timestamp_microsecond: u64,
            }
            impl ::canadensis_encoding::DataType for Synchronization {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Synchronization {}
            impl Synchronization {
                #[cfg_attr(
                    not(doctest),
                    doc = " [second]\n Publication period limits.\n A master should not change its publication period while running."
                )]
                pub const MAX_PUBLICATION_PERIOD: u8 = 1;
                #[cfg_attr(
                    not(doctest),
                    doc = " Synchronization slaves should normally switch to a new master if the current master was silent\n for thrice the interval between the reception of the last two messages published by it.\n For example, imagine that the last message was received at the time X, and the previous message\n was received at the time (X - 0.5 seconds); the period is 0.5 seconds, and therefore the publisher\n timeout is (0.5 seconds * 3) = 1.5 seconds. If there was no message from the current master in\n this amount of time, all slaves will synchronize with another master with the next-higher node-ID."
                )]
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Nested data type used for representing a network-wide synchronized timestamp with microsecond resolution.\n This data type is highly recommended for use both in standard and vendor-specific messages alike."
            )]
            pub struct SynchronizedTimestamp {
                #[cfg_attr(
                    not(doctest),
                    doc = " The number of microseconds that have passed since some arbitrary moment in the past.\n The moment of origin (i.e., the time base) is defined per-application. The current time base in use\n can be requested from the time synchronization master, see the corresponding service definition.\n\n This value is to never overflow. The value is 56-bit wide because:\n\n   - 2^56 microseconds is about 2285 years, which is plenty. A 64-bit microsecond counter would be\n     unnecessarily wide and its overflow interval of 585 thousand years induces a mild existential crisis.\n\n   - Classic-CAN (not FD) transports carry up to 7 bytes of payload per frame.\n     Time sync messages shall use single-frame transfers, which means that the value can't be wider than 56 bits."
                )]
                ///
                /// `truncated uint56`
                ///
                /// Always aligned,
                /// size 56 bits
                pub microsecond: u64,
            }
            impl ::canadensis_encoding::DataType for SynchronizedTimestamp {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for SynchronizedTimestamp {}
            impl SynchronizedTimestamp {
                #[cfg_attr(not(doctest), doc = " Zero means that the time is not known.")]
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " This data types defines constants and runtime values pertaining to the International Atomic Time, also known as TAI.\n See https://en.wikipedia.org/wiki/International_Atomic_Time.\n\n The relationship between the three major time systems -- TAI, GPS, and UTC -- is as follows:\n\n   TAI = GPS + 19 seconds\n   TAI = UTC + LS + 10 seconds\n\n Where \"LS\" is the current number of leap seconds: https://en.wikipedia.org/wiki/Leap_second.\n\n Cyphal applications should only rely on TAI whenever a global time system is needed.\n GPS time is strongly discouraged for reasons of consistency across different positioning systems and applications."
            )]
            pub struct TAIInfo {
                #[cfg_attr(
                    not(doctest),
                    doc = " The current difference between TAI and UTC, if known. If unknown, set to zero.\n\n This value may change states between known and unknown while the master is running,\n depending on its ability to obtain robust values from external sources.\n\n This value may change twice a year, possibly while the system is running; https://en.wikipedia.org/wiki/Leap_second.\n Since the rotation of Earth is decelerating, this value may only be positive. Do not use outside Earth.\n\n For reference, here is the full list of recorded TAI-UTC difference values, valid at the time of writing:\n\n     Date     | TAI-UTC difference [second]\n    ----------|-----------------------------\n     Jan 1972 | 10\n     Jul 1972 | 11\n     Jan 1973 | 12\n     Jan 1974 | 13\n     Jan 1975 | 14\n     Jan 1976 | 15\n     Jan 1977 | 16\n     Jan 1978 | 17\n     Jan 1979 | 18\n     Jan 1980 | 19\n     Jul 1981 | 20\n     Jul 1982 | 21\n     Jul 1983 | 22\n     Jul 1985 | 23\n     Jan 1988 | 24\n     Jan 1990 | 25\n     Jan 1991 | 26\n     Jul 1992 | 27\n     Jul 1993 | 28\n     Jul 1994 | 29\n     Jan 1996 | 30\n     Jul 1997 | 31\n     Jan 1999 | 32\n     Jan 2006 | 33\n     Jan 2009 | 34\n     Jul 2012 | 35\n     Jul 2015 | 36\n     Jan 2017 | 37\n\n As of 2020, the future of the leap second and the relation between UTC and TAI remains uncertain."
                )]
                ///
                /// `saturated uint10`
                ///
                /// Always aligned,
                /// size 10 bits
                pub difference_tai_minus_utc: u16,
            }
            impl ::canadensis_encoding::DataType for TAIInfo {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for TAIInfo {}
            impl TAIInfo {
                #[cfg_attr(
                    not(doctest),
                    doc = " [second]\n The fixed difference, in seconds, between TAI and GPS time. Does not change ever.\n Systems that use GPS time as a reference should convert that to TAI by adding this difference."
                )]
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
            ///
            #[cfg_attr(
                not(doctest),
                doc = " Time system enumeration.\n The time system shall be the same for all masters in the network.\n It cannot be changed while the network is running."
            )]
            pub struct TimeSystem {
                ///
                /// `truncated uint4`
                ///
                /// Always aligned,
                /// size 4 bits
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for TimeSystem {
                /// This type is sealed.
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for TimeSystem {}
            impl TimeSystem {
                #[cfg_attr(
                    not(doctest),
                    doc = " Monotonic time since boot.\n Monotonic time is a time reference that doesn't change rate or make leaps."
                )]
                pub const MONOTONIC_SINCE_BOOT: u8 = 0;
                #[cfg_attr(
                    not(doctest),
                    doc = " International Atomic Time; https://en.wikipedia.org/wiki/International_Atomic_Time.\n The timestamp value contains the number of microseconds elapsed since 1970-01-01T00:00:00Z TAI.\n TAI is always a fixed integer number of seconds ahead of GPS time.\n Systems that use GPS time as a reference should convert that to TAI by adding the fixed difference.\n GPS time is not supported for reasons of consistency across different positioning systems and applications."
                )]
                pub const TAI: u8 = 1;
                #[cfg_attr(
                    not(doctest),
                    doc = " Application-specific time system of unknown properties."
                )]
                pub const APPLICATION_SPECIFIC: u8 = 15;
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
