#![allow(unused_variables)]

pub mod reg {
    pub mod drone {
        pub mod physics {
            pub mod acoustics {
                pub mod note_0_1 {
                    /// `reg.drone.physics.acoustics.Note.0.1`
                    /// Fixed size 12 bytes
                    pub struct Note {
                        // uavcan.si.unit.frequency.Scalar.1.0
                        // Always aligned
                        pub frequency: crate::uavcan::si::unit::frequency::scalar_1_0::Scalar,
                        // uavcan.si.unit.duration.Scalar.1.0
                        // Always aligned
                        pub duration: crate::uavcan::si::unit::duration::scalar_1_0::Scalar,
                        // uavcan.si.unit.power.Scalar.1.0
                        // Always aligned
                        pub acoustic_power: crate::uavcan::si::unit::power::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Note {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Note {}
                    impl ::canadensis_encoding::Serialize for Note {
                        fn size_bits(&self) -> usize {
                            32 + 32 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.frequency);
                            cursor.write_composite(&self.duration);
                            cursor.write_composite(&self.acoustic_power);
                        }
                    }
                }
            }
            pub mod dynamics {
                pub mod rotation {
                    pub mod planar_0_1 {
                        /// `reg.drone.physics.dynamics.rotation.Planar.0.1`
                        /// Fixed size 16 bytes
                        pub struct Planar {
// reg.drone.physics.kinematics.rotation.Planar.0.1
// Always aligned
pub kinematics: crate::reg::drone::physics::kinematics::rotation::planar_0_1::Planar,
// uavcan.si.unit.torque.Scalar.1.0
// Always aligned
pub torque: crate::uavcan::si::unit::torque::scalar_1_0::Scalar,
}
                        impl ::canadensis_encoding::DataType for Planar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Planar {}
                        impl ::canadensis_encoding::Serialize for Planar {
                            fn size_bits(&self) -> usize {
                                96 + 32 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.kinematics);
                                cursor.write_composite(&self.torque);
                            }
                        }
                    }
                    pub mod planar_ts_0_1 {
                        /// `reg.drone.physics.dynamics.rotation.PlanarTs.0.1`
                        /// Fixed size 23 bytes
                        pub struct PlanarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.dynamics.rotation.Planar.0.1
// Always aligned
pub value: crate::reg::drone::physics::dynamics::rotation::planar_0_1::Planar,
}
                        impl ::canadensis_encoding::DataType for PlanarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PlanarTs {}
                        impl ::canadensis_encoding::Serialize for PlanarTs {
                            fn size_bits(&self) -> usize {
                                56 + 128 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                }
                pub mod translation {
                    pub mod linear_0_1 {
                        /// `reg.drone.physics.dynamics.translation.Linear.0.1`
                        /// Fixed size 16 bytes
                        pub struct Linear {
// reg.drone.physics.kinematics.translation.Linear.0.1
// Always aligned
pub kinematics: crate::reg::drone::physics::kinematics::translation::linear_0_1::Linear,
// uavcan.si.unit.force.Scalar.1.0
// Always aligned
pub force: crate::uavcan::si::unit::force::scalar_1_0::Scalar,
}
                        impl ::canadensis_encoding::DataType for Linear {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Linear {}
                        impl ::canadensis_encoding::Serialize for Linear {
                            fn size_bits(&self) -> usize {
                                96 + 32 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.kinematics);
                                cursor.write_composite(&self.force);
                            }
                        }
                    }
                    pub mod linear_ts_0_1 {
                        /// `reg.drone.physics.dynamics.translation.LinearTs.0.1`
                        /// Fixed size 23 bytes
                        pub struct LinearTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.dynamics.translation.Linear.0.1
// Always aligned
pub value: crate::reg::drone::physics::dynamics::translation::linear_0_1::Linear,
}
                        impl ::canadensis_encoding::DataType for LinearTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for LinearTs {}
                        impl ::canadensis_encoding::Serialize for LinearTs {
                            fn size_bits(&self) -> usize {
                                56 + 128 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                }
            }
            pub mod electricity {
                pub mod power_0_1 {
                    /// `reg.drone.physics.electricity.Power.0.1`
                    /// Fixed size 8 bytes
                    pub struct Power {
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub current: crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.voltage.Scalar.1.0
                        // Always aligned
                        pub voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Power {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Power {}
                    impl ::canadensis_encoding::Serialize for Power {
                        fn size_bits(&self) -> usize {
                            32 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.current);
                            cursor.write_composite(&self.voltage);
                        }
                    }
                }
                pub mod power_ts_0_1 {
                    /// `reg.drone.physics.electricity.PowerTs.0.1`
                    /// Fixed size 15 bytes
                    pub struct PowerTs {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // reg.drone.physics.electricity.Power.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::electricity::power_0_1::Power,
                    }
                    impl ::canadensis_encoding::DataType for PowerTs {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for PowerTs {}
                    impl ::canadensis_encoding::Serialize for PowerTs {
                        fn size_bits(&self) -> usize {
                            56 + 64 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_composite(&self.value);
                        }
                    }
                }
                pub mod source_0_1 {
                    /// `reg.drone.physics.electricity.Source.0.1`
                    /// Fixed size 16 bytes
                    pub struct Source {
                        // reg.drone.physics.electricity.Power.0.1
                        // Always aligned
                        pub power: crate::reg::drone::physics::electricity::power_0_1::Power,
                        // uavcan.si.unit.energy.Scalar.1.0
                        // Always aligned
                        pub energy: crate::uavcan::si::unit::energy::scalar_1_0::Scalar,
                        // uavcan.si.unit.energy.Scalar.1.0
                        // Always aligned
                        pub full_energy: crate::uavcan::si::unit::energy::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Source {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Source {}
                    impl ::canadensis_encoding::Serialize for Source {
                        fn size_bits(&self) -> usize {
                            64 + 32 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.power);
                            cursor.write_composite(&self.energy);
                            cursor.write_composite(&self.full_energy);
                        }
                    }
                }
                pub mod source_ts_0_1 {
                    /// `reg.drone.physics.electricity.SourceTs.0.1`
                    /// Fixed size 23 bytes
                    pub struct SourceTs {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // reg.drone.physics.electricity.Source.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::electricity::source_0_1::Source,
                    }
                    impl ::canadensis_encoding::DataType for SourceTs {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for SourceTs {}
                    impl ::canadensis_encoding::Serialize for SourceTs {
                        fn size_bits(&self) -> usize {
                            56 + 128 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_composite(&self.value);
                        }
                    }
                }
            }
            pub mod kinematics {
                pub mod cartesian {
                    pub mod point_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.Point.0.1`
                        /// Fixed size 24 bytes
                        pub struct Point {
                            // uavcan.si.unit.length.WideVector3.1.0
                            // Always aligned
                            pub value:
                                crate::uavcan::si::unit::length::wide_vector3_1_0::WideVector3,
                        }
                        impl ::canadensis_encoding::DataType for Point {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Point {}
                        impl ::canadensis_encoding::Serialize for Point {
                            fn size_bits(&self) -> usize {
                                192 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                    pub mod point_state_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointState.0.1`
                        /// Fixed size 36 bytes
                        pub struct PointState {
                            // reg.drone.physics.kinematics.cartesian.Point.0.1
                            // Always aligned
                            pub position:
                                crate::reg::drone::physics::kinematics::cartesian::point_0_1::Point,
                            // uavcan.si.unit.velocity.Vector3.1.0
                            // Always aligned
                            pub velocity: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                        }
                        impl ::canadensis_encoding::DataType for PointState {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointState {}
                        impl ::canadensis_encoding::Serialize for PointState {
                            fn size_bits(&self) -> usize {
                                192 + 96 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.position);
                                cursor.write_composite(&self.velocity);
                            }
                        }
                    }
                    pub mod point_state_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointStateVar.0.1`
                        /// Fixed size 60 bytes
                        pub struct PointStateVar {
// reg.drone.physics.kinematics.cartesian.PointVar.0.1
// Always aligned
pub position: crate::reg::drone::physics::kinematics::cartesian::point_var_0_1::PointVar,
// reg.drone.physics.kinematics.translation.Velocity3Var.0.2
// Always aligned
pub velocity: crate::reg::drone::physics::kinematics::translation::velocity3_var_0_2::Velocity3Var,
}
                        impl ::canadensis_encoding::DataType for PointStateVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointStateVar {}
                        impl ::canadensis_encoding::Serialize for PointStateVar {
                            fn size_bits(&self) -> usize {
                                288 + 192 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.position);
                                cursor.write_composite(&self.velocity);
                            }
                        }
                    }
                    pub mod point_state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointStateVarTs.0.1`
                        /// Fixed size 67 bytes
                        pub struct PointStateVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.cartesian.PointStateVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::cartesian::point_state_var_0_1::PointStateVar,
}
                        impl ::canadensis_encoding::DataType for PointStateVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointStateVarTs {}
                        impl ::canadensis_encoding::Serialize for PointStateVarTs {
                            fn size_bits(&self) -> usize {
                                56 + 480 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                    pub mod point_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointVar.0.1`
                        /// Fixed size 36 bytes
                        pub struct PointVar {
                            // reg.drone.physics.kinematics.cartesian.Point.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::cartesian::point_0_1::Point,
                            // saturated float16[6]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 6],
                        }
                        impl ::canadensis_encoding::DataType for PointVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointVar {}
                        impl ::canadensis_encoding::Serialize for PointVar {
                            fn size_bits(&self) -> usize {
                                192 + (self.covariance_urt)
                                    .iter()
                                    .map(|element| 16)
                                    .sum::<usize>()
                                    + 0
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
                    }
                    pub mod pose_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.Pose.0.1`
                        /// Fixed size 40 bytes
                        pub struct Pose {
                            // reg.drone.physics.kinematics.cartesian.Point.0.1
                            // Always aligned
                            pub position:
                                crate::reg::drone::physics::kinematics::cartesian::point_0_1::Point,
                            // uavcan.si.unit.angle.Quaternion.1.0
                            // Always aligned
                            pub orientation:
                                crate::uavcan::si::unit::angle::quaternion_1_0::Quaternion,
                        }
                        impl ::canadensis_encoding::DataType for Pose {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Pose {}
                        impl ::canadensis_encoding::Serialize for Pose {
                            fn size_bits(&self) -> usize {
                                192 + 128 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.position);
                                cursor.write_composite(&self.orientation);
                            }
                        }
                    }
                    pub mod pose_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PoseVar.0.1`
                        /// Fixed size 82 bytes
                        pub struct PoseVar {
                            // reg.drone.physics.kinematics.cartesian.Pose.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::cartesian::pose_0_1::Pose,
                            // saturated float16[21]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 21],
                        }
                        impl ::canadensis_encoding::DataType for PoseVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PoseVar {}
                        impl ::canadensis_encoding::Serialize for PoseVar {
                            fn size_bits(&self) -> usize {
                                320 + (self.covariance_urt)
                                    .iter()
                                    .map(|element| 16)
                                    .sum::<usize>()
                                    + 0
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
                    }
                    pub mod pose_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PoseVarTs.0.1`
                        /// Fixed size 89 bytes
                        pub struct PoseVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.cartesian.PoseVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::cartesian::pose_var_0_1::PoseVar,
}
                        impl ::canadensis_encoding::DataType for PoseVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PoseVarTs {}
                        impl ::canadensis_encoding::Serialize for PoseVarTs {
                            fn size_bits(&self) -> usize {
                                56 + 656 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                    pub mod state_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.State.0.1`
                        /// Fixed size 64 bytes
                        pub struct State {
                            // reg.drone.physics.kinematics.cartesian.Pose.0.1
                            // Always aligned
                            pub pose:
                                crate::reg::drone::physics::kinematics::cartesian::pose_0_1::Pose,
                            // reg.drone.physics.kinematics.cartesian.Twist.0.1
                            // Always aligned
                            pub twist:
                                crate::reg::drone::physics::kinematics::cartesian::twist_0_1::Twist,
                        }
                        impl ::canadensis_encoding::DataType for State {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for State {}
                        impl ::canadensis_encoding::Serialize for State {
                            fn size_bits(&self) -> usize {
                                320 + 192 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.pose);
                                cursor.write_composite(&self.twist);
                            }
                        }
                    }
                    pub mod state_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.StateVar.0.1`
                        /// Fixed size 148 bytes
                        pub struct StateVar {
// reg.drone.physics.kinematics.cartesian.PoseVar.0.1
// Always aligned
pub pose: crate::reg::drone::physics::kinematics::cartesian::pose_var_0_1::PoseVar,
// reg.drone.physics.kinematics.cartesian.TwistVar.0.1
// Always aligned
pub twist: crate::reg::drone::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                        impl ::canadensis_encoding::DataType for StateVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for StateVar {}
                        impl ::canadensis_encoding::Serialize for StateVar {
                            fn size_bits(&self) -> usize {
                                656 + 528 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.pose);
                                cursor.write_composite(&self.twist);
                            }
                        }
                    }
                    pub mod state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.StateVarTs.0.1`
                        /// Fixed size 155 bytes
                        pub struct StateVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.cartesian.StateVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::cartesian::state_var_0_1::StateVar,
}
                        impl ::canadensis_encoding::DataType for StateVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for StateVarTs {}
                        impl ::canadensis_encoding::Serialize for StateVarTs {
                            fn size_bits(&self) -> usize {
                                56 + 1184 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                    pub mod twist_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.Twist.0.1`
                        /// Fixed size 24 bytes
                        pub struct Twist {
                            // uavcan.si.unit.velocity.Vector3.1.0
                            // Always aligned
                            pub linear: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                            // uavcan.si.unit.angular_velocity.Vector3.1.0
                            // Always aligned
                            pub angular:
                                crate::uavcan::si::unit::angular_velocity::vector3_1_0::Vector3,
                        }
                        impl ::canadensis_encoding::DataType for Twist {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Twist {}
                        impl ::canadensis_encoding::Serialize for Twist {
                            fn size_bits(&self) -> usize {
                                96 + 96 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.linear);
                                cursor.write_composite(&self.angular);
                            }
                        }
                    }
                    pub mod twist_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.TwistVar.0.1`
                        /// Fixed size 66 bytes
                        pub struct TwistVar {
                            // reg.drone.physics.kinematics.cartesian.Twist.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::cartesian::twist_0_1::Twist,
                            // saturated float16[21]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 21],
                        }
                        impl ::canadensis_encoding::DataType for TwistVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for TwistVar {}
                        impl ::canadensis_encoding::Serialize for TwistVar {
                            fn size_bits(&self) -> usize {
                                192 + (self.covariance_urt)
                                    .iter()
                                    .map(|element| 16)
                                    .sum::<usize>()
                                    + 0
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
                    }
                    pub mod twist_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.TwistVarTs.0.1`
                        /// Fixed size 73 bytes
                        pub struct TwistVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.cartesian.TwistVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                        impl ::canadensis_encoding::DataType for TwistVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for TwistVarTs {}
                        impl ::canadensis_encoding::Serialize for TwistVarTs {
                            fn size_bits(&self) -> usize {
                                56 + 528 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                }
                pub mod geodetic {
                    pub mod point_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.Point.0.1`
                        /// Fixed size 24 bytes
                        pub struct Point {
                            // saturated float64
                            // Always aligned
                            pub latitude: f64,
                            // saturated float64
                            // Always aligned
                            pub longitude: f64,
                            // uavcan.si.unit.length.WideScalar.1.0
                            // Always aligned
                            pub altitude:
                                crate::uavcan::si::unit::length::wide_scalar_1_0::WideScalar,
                        }
                        impl ::canadensis_encoding::DataType for Point {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Point {}
                        impl ::canadensis_encoding::Serialize for Point {
                            fn size_bits(&self) -> usize {
                                64 + 64 + 64 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_f64(self.latitude);
                                cursor.write_f64(self.longitude);
                                cursor.write_composite(&self.altitude);
                            }
                        }
                    }
                    pub mod point_state_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointState.0.1`
                        /// Fixed size 36 bytes
                        pub struct PointState {
                            // reg.drone.physics.kinematics.geodetic.Point.0.1
                            // Always aligned
                            pub position:
                                crate::reg::drone::physics::kinematics::geodetic::point_0_1::Point,
                            // uavcan.si.unit.velocity.Vector3.1.0
                            // Always aligned
                            pub velocity: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                        }
                        impl ::canadensis_encoding::DataType for PointState {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointState {}
                        impl ::canadensis_encoding::Serialize for PointState {
                            fn size_bits(&self) -> usize {
                                192 + 96 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.position);
                                cursor.write_composite(&self.velocity);
                            }
                        }
                    }
                    pub mod point_state_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointStateVar.0.1`
                        /// Fixed size 60 bytes
                        pub struct PointStateVar {
// reg.drone.physics.kinematics.geodetic.PointVar.0.1
// Always aligned
pub position: crate::reg::drone::physics::kinematics::geodetic::point_var_0_1::PointVar,
// reg.drone.physics.kinematics.translation.Velocity3Var.0.2
// Always aligned
pub velocity: crate::reg::drone::physics::kinematics::translation::velocity3_var_0_2::Velocity3Var,
}
                        impl ::canadensis_encoding::DataType for PointStateVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointStateVar {}
                        impl ::canadensis_encoding::Serialize for PointStateVar {
                            fn size_bits(&self) -> usize {
                                288 + 192 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.position);
                                cursor.write_composite(&self.velocity);
                            }
                        }
                    }
                    pub mod point_state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointStateVarTs.0.1`
                        /// Fixed size 67 bytes
                        pub struct PointStateVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.geodetic.PointStateVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::geodetic::point_state_var_0_1::PointStateVar,
}
                        impl ::canadensis_encoding::DataType for PointStateVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointStateVarTs {}
                        impl ::canadensis_encoding::Serialize for PointStateVarTs {
                            fn size_bits(&self) -> usize {
                                56 + 480 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                    pub mod point_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointVar.0.1`
                        /// Fixed size 36 bytes
                        pub struct PointVar {
                            // reg.drone.physics.kinematics.geodetic.Point.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::geodetic::point_0_1::Point,
                            // saturated float16[6]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 6],
                        }
                        impl ::canadensis_encoding::DataType for PointVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PointVar {}
                        impl ::canadensis_encoding::Serialize for PointVar {
                            fn size_bits(&self) -> usize {
                                192 + (self.covariance_urt)
                                    .iter()
                                    .map(|element| 16)
                                    .sum::<usize>()
                                    + 0
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
                    }
                    pub mod pose_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.Pose.0.1`
                        /// Fixed size 40 bytes
                        pub struct Pose {
                            // reg.drone.physics.kinematics.geodetic.Point.0.1
                            // Always aligned
                            pub position:
                                crate::reg::drone::physics::kinematics::geodetic::point_0_1::Point,
                            // uavcan.si.unit.angle.Quaternion.1.0
                            // Always aligned
                            pub orientation:
                                crate::uavcan::si::unit::angle::quaternion_1_0::Quaternion,
                        }
                        impl ::canadensis_encoding::DataType for Pose {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Pose {}
                        impl ::canadensis_encoding::Serialize for Pose {
                            fn size_bits(&self) -> usize {
                                192 + 128 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.position);
                                cursor.write_composite(&self.orientation);
                            }
                        }
                    }
                    pub mod pose_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PoseVar.0.1`
                        /// Fixed size 82 bytes
                        pub struct PoseVar {
                            // reg.drone.physics.kinematics.geodetic.Pose.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::geodetic::pose_0_1::Pose,
                            // saturated float16[21]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 21],
                        }
                        impl ::canadensis_encoding::DataType for PoseVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PoseVar {}
                        impl ::canadensis_encoding::Serialize for PoseVar {
                            fn size_bits(&self) -> usize {
                                320 + (self.covariance_urt)
                                    .iter()
                                    .map(|element| 16)
                                    .sum::<usize>()
                                    + 0
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
                    }
                    pub mod state_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.State.0.1`
                        /// Fixed size 64 bytes
                        pub struct State {
                            // reg.drone.physics.kinematics.geodetic.Pose.0.1
                            // Always aligned
                            pub pose:
                                crate::reg::drone::physics::kinematics::geodetic::pose_0_1::Pose,
                            // reg.drone.physics.kinematics.cartesian.Twist.0.1
                            // Always aligned
                            pub twist:
                                crate::reg::drone::physics::kinematics::cartesian::twist_0_1::Twist,
                        }
                        impl ::canadensis_encoding::DataType for State {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for State {}
                        impl ::canadensis_encoding::Serialize for State {
                            fn size_bits(&self) -> usize {
                                320 + 192 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.pose);
                                cursor.write_composite(&self.twist);
                            }
                        }
                    }
                    pub mod state_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.StateVar.0.1`
                        /// Fixed size 148 bytes
                        pub struct StateVar {
// reg.drone.physics.kinematics.geodetic.PoseVar.0.1
// Always aligned
pub pose: crate::reg::drone::physics::kinematics::geodetic::pose_var_0_1::PoseVar,
// reg.drone.physics.kinematics.cartesian.TwistVar.0.1
// Always aligned
pub twist: crate::reg::drone::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                        impl ::canadensis_encoding::DataType for StateVar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for StateVar {}
                        impl ::canadensis_encoding::Serialize for StateVar {
                            fn size_bits(&self) -> usize {
                                656 + 528 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.pose);
                                cursor.write_composite(&self.twist);
                            }
                        }
                    }
                    pub mod state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.StateVarTs.0.1`
                        /// Fixed size 155 bytes
                        pub struct StateVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.geodetic.StateVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::geodetic::state_var_0_1::StateVar,
}
                        impl ::canadensis_encoding::DataType for StateVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for StateVarTs {}
                        impl ::canadensis_encoding::Serialize for StateVarTs {
                            fn size_bits(&self) -> usize {
                                56 + 1184 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                }
                pub mod rotation {
                    pub mod planar_0_1 {
                        /// `reg.drone.physics.kinematics.rotation.Planar.0.1`
                        /// Fixed size 12 bytes
                        pub struct Planar {
                            // uavcan.si.unit.angle.Scalar.1.0
                            // Always aligned
                            pub angular_position:
                                crate::uavcan::si::unit::angle::scalar_1_0::Scalar,
                            // uavcan.si.unit.angular_velocity.Scalar.1.0
                            // Always aligned
                            pub angular_velocity:
                                crate::uavcan::si::unit::angular_velocity::scalar_1_0::Scalar,
                            // uavcan.si.unit.angular_acceleration.Scalar.1.0
                            // Always aligned
                            pub angular_acceleration:
                                crate::uavcan::si::unit::angular_acceleration::scalar_1_0::Scalar,
                        }
                        impl ::canadensis_encoding::DataType for Planar {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Planar {}
                        impl ::canadensis_encoding::Serialize for Planar {
                            fn size_bits(&self) -> usize {
                                32 + 32 + 32 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.angular_position);
                                cursor.write_composite(&self.angular_velocity);
                                cursor.write_composite(&self.angular_acceleration);
                            }
                        }
                    }
                    pub mod planar_ts_0_1 {
                        /// `reg.drone.physics.kinematics.rotation.PlanarTs.0.1`
                        /// Fixed size 19 bytes
                        pub struct PlanarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.rotation.Planar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::rotation::planar_0_1::Planar,
}
                        impl ::canadensis_encoding::DataType for PlanarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for PlanarTs {}
                        impl ::canadensis_encoding::Serialize for PlanarTs {
                            fn size_bits(&self) -> usize {
                                56 + 96 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                }
                pub mod translation {
                    pub mod linear_0_1 {
                        /// `reg.drone.physics.kinematics.translation.Linear.0.1`
                        /// Fixed size 12 bytes
                        pub struct Linear {
                            // uavcan.si.unit.length.Scalar.1.0
                            // Always aligned
                            pub position: crate::uavcan::si::unit::length::scalar_1_0::Scalar,
                            // uavcan.si.unit.velocity.Scalar.1.0
                            // Always aligned
                            pub velocity: crate::uavcan::si::unit::velocity::scalar_1_0::Scalar,
                            // uavcan.si.unit.acceleration.Scalar.1.0
                            // Always aligned
                            pub acceleration:
                                crate::uavcan::si::unit::acceleration::scalar_1_0::Scalar,
                        }
                        impl ::canadensis_encoding::DataType for Linear {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Linear {}
                        impl ::canadensis_encoding::Serialize for Linear {
                            fn size_bits(&self) -> usize {
                                32 + 32 + 32 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.position);
                                cursor.write_composite(&self.velocity);
                                cursor.write_composite(&self.acceleration);
                            }
                        }
                    }
                    pub mod linear_ts_0_1 {
                        /// `reg.drone.physics.kinematics.translation.LinearTs.0.1`
                        /// Fixed size 19 bytes
                        pub struct LinearTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.translation.Linear.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::translation::linear_0_1::Linear,
}
                        impl ::canadensis_encoding::DataType for LinearTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for LinearTs {}
                        impl ::canadensis_encoding::Serialize for LinearTs {
                            fn size_bits(&self) -> usize {
                                56 + 96 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.timestamp);
                                cursor.write_composite(&self.value);
                            }
                        }
                    }
                    pub mod linear_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.translation.LinearVarTs.0.1`
                        /// Fixed size 25 bytes
                        pub struct LinearVarTs {
// reg.drone.physics.kinematics.translation.LinearTs.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::translation::linear_ts_0_1::LinearTs,
// saturated float16
// Always aligned
pub position_error_variance: ::half::f16,
// saturated float16
// Always aligned
pub velocity_error_variance: ::half::f16,
// saturated float16
// Always aligned
pub acceleration_error_variance: ::half::f16,
}
                        impl ::canadensis_encoding::DataType for LinearVarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for LinearVarTs {}
                        impl ::canadensis_encoding::Serialize for LinearVarTs {
                            fn size_bits(&self) -> usize {
                                152 + 16 + 16 + 16 + 0
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
                    }
                    pub mod velocity1_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.translation.Velocity1VarTs.0.1`
                        /// Fixed size 13 bytes
                        pub struct Velocity1VarTs {
                            // uavcan.si.sample.velocity.Scalar.1.0
                            // Always aligned
                            pub value: crate::uavcan::si::sample::velocity::scalar_1_0::Scalar,
                            // saturated float16
                            // Always aligned
                            pub error_variance: ::half::f16,
                        }
                        impl ::canadensis_encoding::DataType for Velocity1VarTs {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Velocity1VarTs {}
                        impl ::canadensis_encoding::Serialize for Velocity1VarTs {
                            fn size_bits(&self) -> usize {
                                88 + 16 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.value);
                                cursor.write_f16(self.error_variance);
                            }
                        }
                    }
                    pub mod velocity3_var_0_1 {
                        /// `reg.drone.physics.kinematics.translation.Velocity3Var.0.1`
                        /// Fixed size 31 bytes
                        pub struct Velocity3Var {
                            // uavcan.si.sample.velocity.Vector3.1.0
                            // Always aligned
                            pub value: crate::uavcan::si::sample::velocity::vector3_1_0::Vector3,
                            // saturated float16[6]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 6],
                        }
                        impl ::canadensis_encoding::DataType for Velocity3Var {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Velocity3Var {}
                        impl ::canadensis_encoding::Serialize for Velocity3Var {
                            fn size_bits(&self) -> usize {
                                152 + (self.covariance_urt)
                                    .iter()
                                    .map(|element| 16)
                                    .sum::<usize>()
                                    + 0
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
                    }
                    pub mod velocity3_var_0_2 {
                        /// `reg.drone.physics.kinematics.translation.Velocity3Var.0.2`
                        /// Fixed size 24 bytes
                        pub struct Velocity3Var {
                            // uavcan.si.unit.velocity.Vector3.1.0
                            // Always aligned
                            pub value: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                            // saturated float16[6]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 6],
                        }
                        impl ::canadensis_encoding::DataType for Velocity3Var {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for Velocity3Var {}
                        impl ::canadensis_encoding::Serialize for Velocity3Var {
                            fn size_bits(&self) -> usize {
                                96 + (self.covariance_urt)
                                    .iter()
                                    .map(|element| 16)
                                    .sum::<usize>()
                                    + 0
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
                    }
                }
            }
            pub mod optics {
                pub mod high_color_0_1 {
                    /// `reg.drone.physics.optics.HighColor.0.1`
                    /// Fixed size 2 bytes
                    pub struct HighColor {
                        // saturated uint5
                        // Always aligned
                        pub red: u8,
                        // saturated uint6
                        // Not always aligned
                        pub green: u8,
                        // saturated uint5
                        // Not always aligned
                        pub blue: u8,
                    }
                    impl ::canadensis_encoding::DataType for HighColor {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for HighColor {}
                    impl ::canadensis_encoding::Serialize for HighColor {
                        fn size_bits(&self) -> usize {
                            5 + 6 + 5 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_u5(self.red);
                            cursor.write_u6(self.green);
                            cursor.write_u5(self.blue);
                        }
                    }
                }
            }
            pub mod thermodynamics {
                pub mod pressure_temp_var_ts_0_1 {
                    /// `reg.drone.physics.thermodynamics.PressureTempVarTs.0.1`
                    /// Fixed size 21 bytes
                    pub struct PressureTempVarTs {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // uavcan.si.unit.pressure.Scalar.1.0
                        // Always aligned
                        pub pressure: crate::uavcan::si::unit::pressure::scalar_1_0::Scalar,
                        // uavcan.si.unit.temperature.Scalar.1.0
                        // Always aligned
                        pub temperature: crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
                        // saturated float16[3]
                        // Always aligned
                        pub covariance_urt: [::half::f16; 3],
                    }
                    impl ::canadensis_encoding::DataType for PressureTempVarTs {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for PressureTempVarTs {}
                    impl ::canadensis_encoding::Serialize for PressureTempVarTs {
                        fn size_bits(&self) -> usize {
                            56 + 32
                                + 32
                                + (self.covariance_urt)
                                    .iter()
                                    .map(|element| 16)
                                    .sum::<usize>()
                                + 0
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
                }
            }
            pub mod time {
                pub mod tai64_0_1 {
                    /// `reg.drone.physics.time.TAI64.0.1`
                    /// Fixed size 8 bytes
                    pub struct TAI64 {
                        // saturated int64
                        // Always aligned
                        pub tai64n: i64,
                    }
                    impl ::canadensis_encoding::DataType for TAI64 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for TAI64 {}
                    impl ::canadensis_encoding::Serialize for TAI64 {
                        fn size_bits(&self) -> usize {
                            64 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_u64(self.tai64n as u64);
                        }
                    }
                }
                pub mod tai64_var_0_1 {
                    /// `reg.drone.physics.time.TAI64Var.0.1`
                    /// Fixed size 12 bytes
                    pub struct TAI64Var {
                        // reg.drone.physics.time.TAI64.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::time::tai64_0_1::TAI64,
                        // saturated float32
                        // Always aligned
                        pub error_variance: f32,
                    }
                    impl ::canadensis_encoding::DataType for TAI64Var {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for TAI64Var {}
                    impl ::canadensis_encoding::Serialize for TAI64Var {
                        fn size_bits(&self) -> usize {
                            64 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.value);
                            cursor.write_f32(self.error_variance);
                        }
                    }
                }
                pub mod tai64_var_ts_0_1 {
                    /// `reg.drone.physics.time.TAI64VarTs.0.1`
                    /// Fixed size 19 bytes
                    pub struct TAI64VarTs {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // reg.drone.physics.time.TAI64Var.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::time::tai64_var_0_1::TAI64Var,
                    }
                    impl ::canadensis_encoding::DataType for TAI64VarTs {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for TAI64VarTs {}
                    impl ::canadensis_encoding::Serialize for TAI64VarTs {
                        fn size_bits(&self) -> usize {
                            56 + 96 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_composite(&self.value);
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
                        /// Fixed size 0 bytes
                        pub struct _0 {}
                        impl ::canadensis_encoding::DataType for _0 {
                            const EXTENT_BYTES: Option<u32> = Some(0);
                        }
                        impl ::canadensis_encoding::Message for _0 {}
                        impl ::canadensis_encoding::Serialize for _0 {
                            fn size_bits(&self) -> usize {
                                0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                            }
                        }
                    }
                    pub mod fault_flags_0_1 {
                        /// `reg.drone.service.actuator.common.FaultFlags.0.1`
                        /// Fixed size 2 bytes
                        pub struct FaultFlags {
                            // saturated bool
                            // Always aligned
                            pub overload: bool,
                            // saturated bool
                            // Not always aligned
                            pub voltage: bool,
                            // saturated bool
                            // Not always aligned
                            pub motor_temperature: bool,
                            // saturated bool
                            // Not always aligned
                            pub controller_temperature: bool,
                            // saturated bool
                            // Not always aligned
                            pub velocity: bool,
                            // saturated bool
                            // Not always aligned
                            pub mechanical: bool,
                            // saturated bool
                            // Not always aligned
                            pub vibration: bool,
                            // saturated bool
                            // Not always aligned
                            pub configuration: bool,
                            // saturated bool
                            // Always aligned
                            pub control_mode: bool,
                            // saturated bool
                            // Not always aligned
                            pub other: bool,
                        }
                        impl ::canadensis_encoding::DataType for FaultFlags {
                            const EXTENT_BYTES: Option<u32> = None;
                        }
                        impl ::canadensis_encoding::Message for FaultFlags {}
                        impl ::canadensis_encoding::Serialize for FaultFlags {
                            fn size_bits(&self) -> usize {
                                1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 0
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
                                cursor.write_bool(self.other);
                            }
                        }
                    }
                    pub mod feedback_0_1 {
                        /// `reg.drone.service.actuator.common.Feedback.0.1`
                        /// Fixed size 3 bytes
                        pub struct Feedback {
                            // reg.drone.service.common.Heartbeat.0.1
                            // Always aligned
                            pub heartbeat:
                                crate::reg::drone::service::common::heartbeat_0_1::Heartbeat,
                            // saturated int8
                            // Always aligned
                            pub demand_factor_pct: i8,
                        }
                        impl ::canadensis_encoding::DataType for Feedback {
                            const EXTENT_BYTES: Option<u32> = Some(63);
                        }
                        impl ::canadensis_encoding::Message for Feedback {}
                        impl ::canadensis_encoding::Serialize for Feedback {
                            fn size_bits(&self) -> usize {
                                16 + 8 + 0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                                cursor.write_composite(&self.heartbeat);
                                cursor.write_aligned_u8(self.demand_factor_pct as u8);
                            }
                        }
                    }
                    pub mod sp {
                        pub mod _0_1 {
                            /// `reg.drone.service.actuator.common.sp._.0.1`
                            /// Fixed size 0 bytes
                            pub struct _0 {}
                            impl ::canadensis_encoding::DataType for _0 {
                                const EXTENT_BYTES: Option<u32> = Some(0);
                            }
                            impl ::canadensis_encoding::Message for _0 {}
                            impl ::canadensis_encoding::Serialize for _0 {
                                fn size_bits(&self) -> usize {
                                    0
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                }
                            }
                        }
                        pub mod scalar_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Scalar.0.1`
                            /// Fixed size 2 bytes
                            pub struct Scalar {
                                // saturated float16
                                // Always aligned
                                pub value: ::half::f16,
                            }
                            impl ::canadensis_encoding::DataType for Scalar {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Scalar {}
                            impl ::canadensis_encoding::Serialize for Scalar {
                                fn size_bits(&self) -> usize {
                                    16 + 0
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    cursor.write_f16(self.value);
                                }
                            }
                        }
                        pub mod vector2_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector2.0.1`
                            /// Fixed size 4 bytes
                            pub struct Vector2 {
                                // saturated float16[2]
                                // Always aligned
                                pub value: [::half::f16; 2],
                            }
                            impl ::canadensis_encoding::DataType for Vector2 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector2 {}
                            impl ::canadensis_encoding::Serialize for Vector2 {
                                fn size_bits(&self) -> usize {
                                    (self.value).iter().map(|element| 16).sum::<usize>() + 0
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    for value in (self.value).iter() {
                                        cursor.write_f16(*value);
                                    }
                                }
                            }
                        }
                        pub mod vector31_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector31.0.1`
                            /// Fixed size 62 bytes
                            pub struct Vector31 {
                                // saturated float16[31]
                                // Always aligned
                                pub value: [::half::f16; 31],
                            }
                            impl ::canadensis_encoding::DataType for Vector31 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector31 {}
                            impl ::canadensis_encoding::Serialize for Vector31 {
                                fn size_bits(&self) -> usize {
                                    (self.value).iter().map(|element| 16).sum::<usize>() + 0
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    for value in (self.value).iter() {
                                        cursor.write_f16(*value);
                                    }
                                }
                            }
                        }
                        pub mod vector3_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector3.0.1`
                            /// Fixed size 6 bytes
                            pub struct Vector3 {
                                // saturated float16[3]
                                // Always aligned
                                pub value: [::half::f16; 3],
                            }
                            impl ::canadensis_encoding::DataType for Vector3 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector3 {}
                            impl ::canadensis_encoding::Serialize for Vector3 {
                                fn size_bits(&self) -> usize {
                                    (self.value).iter().map(|element| 16).sum::<usize>() + 0
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    for value in (self.value).iter() {
                                        cursor.write_f16(*value);
                                    }
                                }
                            }
                        }
                        pub mod vector4_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector4.0.1`
                            /// Fixed size 8 bytes
                            pub struct Vector4 {
                                // saturated float16[4]
                                // Always aligned
                                pub value: [::half::f16; 4],
                            }
                            impl ::canadensis_encoding::DataType for Vector4 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector4 {}
                            impl ::canadensis_encoding::Serialize for Vector4 {
                                fn size_bits(&self) -> usize {
                                    (self.value).iter().map(|element| 16).sum::<usize>() + 0
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    for value in (self.value).iter() {
                                        cursor.write_f16(*value);
                                    }
                                }
                            }
                        }
                        pub mod vector6_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector6.0.1`
                            /// Fixed size 12 bytes
                            pub struct Vector6 {
                                // saturated float16[6]
                                // Always aligned
                                pub value: [::half::f16; 6],
                            }
                            impl ::canadensis_encoding::DataType for Vector6 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector6 {}
                            impl ::canadensis_encoding::Serialize for Vector6 {
                                fn size_bits(&self) -> usize {
                                    (self.value).iter().map(|element| 16).sum::<usize>() + 0
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    for value in (self.value).iter() {
                                        cursor.write_f16(*value);
                                    }
                                }
                            }
                        }
                        pub mod vector8_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector8.0.1`
                            /// Fixed size 16 bytes
                            pub struct Vector8 {
                                // saturated float16[8]
                                // Always aligned
                                pub value: [::half::f16; 8],
                            }
                            impl ::canadensis_encoding::DataType for Vector8 {
                                const EXTENT_BYTES: Option<u32> = Some(512);
                            }
                            impl ::canadensis_encoding::Message for Vector8 {}
                            impl ::canadensis_encoding::Serialize for Vector8 {
                                fn size_bits(&self) -> usize {
                                    (self.value).iter().map(|element| 16).sum::<usize>() + 0
                                }
                                fn serialize(
                                    &self,
                                    cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                                ) {
                                    for value in (self.value).iter() {
                                        cursor.write_f16(*value);
                                    }
                                }
                            }
                        }
                    }
                    pub mod status_0_1 {
                        /// `reg.drone.service.actuator.common.Status.0.1`
                        /// Fixed size 14 bytes
                        pub struct Status {
// uavcan.si.unit.temperature.Scalar.1.0
// Always aligned
pub motor_temperature: crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
// uavcan.si.unit.temperature.Scalar.1.0
// Always aligned
pub controller_temperature: crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
// saturated uint32
// Always aligned
pub error_count: u32,
// reg.drone.service.actuator.common.FaultFlags.0.1
// Always aligned
pub fault_flags: crate::reg::drone::service::actuator::common::fault_flags_0_1::FaultFlags,
}
                        impl ::canadensis_encoding::DataType for Status {
                            const EXTENT_BYTES: Option<u32> = Some(63);
                        }
                        impl ::canadensis_encoding::Message for Status {}
                        impl ::canadensis_encoding::Serialize for Status {
                            fn size_bits(&self) -> usize {
                                32 + 32 + 32 + 16 + 0
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
                    }
                }
                pub mod esc {
                    pub mod _0_1 {
                        /// `reg.drone.service.actuator.esc._.0.1`
                        /// Fixed size 0 bytes
                        pub struct _0 {}
                        impl ::canadensis_encoding::DataType for _0 {
                            const EXTENT_BYTES: Option<u32> = Some(0);
                        }
                        impl ::canadensis_encoding::Message for _0 {}
                        impl ::canadensis_encoding::Serialize for _0 {
                            fn size_bits(&self) -> usize {
                                0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                            }
                        }
                    }
                }
                pub mod servo {
                    pub mod _0_1 {
                        /// `reg.drone.service.actuator.servo._.0.1`
                        /// Fixed size 0 bytes
                        pub struct _0 {}
                        impl ::canadensis_encoding::DataType for _0 {
                            const EXTENT_BYTES: Option<u32> = Some(0);
                        }
                        impl ::canadensis_encoding::Message for _0 {}
                        impl ::canadensis_encoding::Serialize for _0 {
                            fn size_bits(&self) -> usize {
                                0
                            }
                            fn serialize(
                                &self,
                                cursor: &mut ::canadensis_encoding::WriteCursor<'_>,
                            ) {
                            }
                        }
                    }
                }
            }
            pub mod air_data_computer {
                pub mod _0_1 {
                    /// `reg.drone.service.air_data_computer._.0.1`
                    /// Fixed size 0 bytes
                    pub struct _0 {}
                    impl ::canadensis_encoding::DataType for _0 {
                        const EXTENT_BYTES: Option<u32> = Some(0);
                    }
                    impl ::canadensis_encoding::Message for _0 {}
                    impl ::canadensis_encoding::Serialize for _0 {
                        fn size_bits(&self) -> usize {
                            0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
                    }
                }
            }
            pub mod battery {
                pub mod _0_1 {
                    /// `reg.drone.service.battery._.0.1`
                    /// Fixed size 0 bytes
                    pub struct _0 {}
                    impl ::canadensis_encoding::DataType for _0 {
                        const EXTENT_BYTES: Option<u32> = Some(0);
                    }
                    impl ::canadensis_encoding::Message for _0 {}
                    impl ::canadensis_encoding::Serialize for _0 {
                        fn size_bits(&self) -> usize {
                            0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
                    }
                }
                pub mod error_0_1 {
                    /// `reg.drone.service.battery.Error.0.1`
                    /// Fixed size 1 bytes
                    pub struct Error {
                        // saturated uint8
                        // Always aligned
                        pub value: u8,
                    }
                    impl ::canadensis_encoding::DataType for Error {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Error {}
                    impl ::canadensis_encoding::Serialize for Error {
                        fn size_bits(&self) -> usize {
                            8 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_u8(self.value);
                        }
                    }
                }
                pub mod parameters_0_1 {
                    /// `reg.drone.service.battery.Parameters.0.1`
                    /// Fixed size 54 bytes
                    pub struct Parameters {
                        // truncated uint64
                        // Always aligned
                        pub unique_id: u64,
                        // uavcan.si.unit.mass.Scalar.1.0
                        // Always aligned
                        pub mass: crate::uavcan::si::unit::mass::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_charge.Scalar.1.0
                        // Always aligned
                        pub design_capacity:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        // uavcan.si.unit.voltage.Scalar.1.0[2]
                        // Always aligned
                        pub design_cell_voltage_min_max:
                            [crate::uavcan::si::unit::voltage::scalar_1_0::Scalar; 2],
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub discharge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub discharge_current_burst:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub charge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub charge_current_fast:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub charge_termination_treshold:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.voltage.Scalar.1.0
                        // Always aligned
                        pub charge_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                        // saturated uint16
                        // Always aligned
                        pub cycle_count: u16,
                        // saturated uint8
                        // Always aligned
                        pub series_cell_count: u8,
                        // saturated uint7
                        // Always aligned
                        pub state_of_health_pct: u8,
                        // reg.drone.service.battery.Technology.0.1
                        // Always aligned
                        pub technology:
                            crate::reg::drone::service::battery::technology_0_1::Technology,
                    }
                    impl ::canadensis_encoding::DataType for Parameters {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Parameters {}
                    impl ::canadensis_encoding::Serialize for Parameters {
                        fn size_bits(&self) -> usize {
                            64 + 32
                                + 32
                                + (self.design_cell_voltage_min_max)
                                    .iter()
                                    .map(|element| 32)
                                    .sum::<usize>()
                                + 32
                                + 32
                                + 32
                                + 32
                                + 32
                                + 32
                                + 16
                                + 8
                                + 7
                                + 8
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
                            cursor.write_composite(&self.charge_termination_treshold);
                            cursor.write_composite(&self.charge_voltage);
                            cursor.write_aligned_u16(self.cycle_count);
                            cursor.write_aligned_u8(self.series_cell_count);
                            cursor.write_u7(self.state_of_health_pct);
                            cursor.write_composite(&self.technology);
                        }
                    }
                }
                pub mod parameters_0_2 {
                    /// `reg.drone.service.battery.Parameters.0.2`
                    /// Fixed size 54 bytes
                    pub struct Parameters {
                        // truncated uint64
                        // Always aligned
                        pub unique_id: u64,
                        // uavcan.si.unit.mass.Scalar.1.0
                        // Always aligned
                        pub mass: crate::uavcan::si::unit::mass::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_charge.Scalar.1.0
                        // Always aligned
                        pub design_capacity:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        // uavcan.si.unit.voltage.Scalar.1.0[2]
                        // Always aligned
                        pub design_cell_voltage_min_max:
                            [crate::uavcan::si::unit::voltage::scalar_1_0::Scalar; 2],
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub discharge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub discharge_current_burst:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub charge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub charge_current_fast:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub charge_termination_threshold:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.voltage.Scalar.1.0
                        // Always aligned
                        pub charge_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                        // saturated uint16
                        // Always aligned
                        pub cycle_count: u16,
                        // saturated uint7
                        // Always aligned
                        pub state_of_health_pct: u8,
                        // reg.drone.service.battery.Technology.0.1
                        // Always aligned
                        pub technology:
                            crate::reg::drone::service::battery::technology_0_1::Technology,
                    }
                    impl ::canadensis_encoding::DataType for Parameters {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Parameters {}
                    impl ::canadensis_encoding::Serialize for Parameters {
                        fn size_bits(&self) -> usize {
                            64 + 32
                                + 32
                                + (self.design_cell_voltage_min_max)
                                    .iter()
                                    .map(|element| 32)
                                    .sum::<usize>()
                                + 32
                                + 32
                                + 32
                                + 32
                                + 32
                                + 32
                                + 16
                                + 7
                                + 8
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
                            cursor.write_u7(self.state_of_health_pct);
                            cursor.write_composite(&self.technology);
                        }
                    }
                }
                pub mod parameters_0_3 {
                    /// `reg.drone.service.battery.Parameters.0.3`
                    /// Fixed size 58 bytes
                    pub struct Parameters {
                        // truncated uint64
                        // Always aligned
                        pub unique_id: u64,
                        // uavcan.si.unit.mass.Scalar.1.0
                        // Always aligned
                        pub mass: crate::uavcan::si::unit::mass::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_charge.Scalar.1.0
                        // Always aligned
                        pub design_capacity:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        // uavcan.si.unit.voltage.Scalar.1.0[2]
                        // Always aligned
                        pub design_cell_voltage_min_max:
                            [crate::uavcan::si::unit::voltage::scalar_1_0::Scalar; 2],
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub discharge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub discharge_current_burst:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub charge_current:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub charge_current_fast:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub charge_termination_threshold:
                            crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.voltage.Scalar.1.0
                        // Always aligned
                        pub charge_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                        // saturated uint16
                        // Always aligned
                        pub cycle_count: u16,
                        // saturated uint7
                        // Always aligned
                        pub state_of_health_pct: u8,
                        // reg.drone.service.battery.Technology.0.1
                        // Always aligned
                        pub technology:
                            crate::reg::drone::service::battery::technology_0_1::Technology,
                        // uavcan.si.unit.voltage.Scalar.1.0
                        // Always aligned
                        pub nominal_voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Parameters {
                        const EXTENT_BYTES: Option<u32> = Some(67);
                    }
                    impl ::canadensis_encoding::Message for Parameters {}
                    impl ::canadensis_encoding::Serialize for Parameters {
                        fn size_bits(&self) -> usize {
                            64 + 32
                                + 32
                                + (self.design_cell_voltage_min_max)
                                    .iter()
                                    .map(|element| 32)
                                    .sum::<usize>()
                                + 32
                                + 32
                                + 32
                                + 32
                                + 32
                                + 32
                                + 16
                                + 7
                                + 8
                                + 32
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
                            cursor.write_u7(self.state_of_health_pct);
                            cursor.write_composite(&self.technology);
                            cursor.write_composite(&self.nominal_voltage);
                        }
                    }
                }
                pub mod status_0_1 {
                    /// `reg.drone.service.battery.Status.0.1`
                    /// Fixed size 23 bytes
                    pub struct Status {
                        // reg.drone.service.common.Heartbeat.0.1
                        // Always aligned
                        pub heartbeat: crate::reg::drone::service::common::heartbeat_0_1::Heartbeat,
                        // uavcan.si.unit.temperature.Scalar.1.0[2]
                        // Always aligned
                        pub temperature_min_max:
                            [crate::uavcan::si::unit::temperature::scalar_1_0::Scalar; 2],
                        // uavcan.si.unit.voltage.Scalar.1.0[2]
                        // Always aligned
                        pub cell_voltage_min_max:
                            [crate::uavcan::si::unit::voltage::scalar_1_0::Scalar; 2],
                        // uavcan.si.unit.electric_charge.Scalar.1.0
                        // Always aligned
                        pub available_charge:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        // reg.drone.service.battery.Error.0.1
                        // Always aligned
                        pub error: crate::reg::drone::service::battery::error_0_1::Error,
                    }
                    impl ::canadensis_encoding::DataType for Status {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Status {}
                    impl ::canadensis_encoding::Serialize for Status {
                        fn size_bits(&self) -> usize {
                            16 + (self.temperature_min_max)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + (self.cell_voltage_min_max)
                                    .iter()
                                    .map(|element| 32)
                                    .sum::<usize>()
                                + 32
                                + 8
                                + 0
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
                }
                pub mod status_0_2 {
                    /// `reg.drone.service.battery.Status.0.2`
                    /// Size ranges from 24 to 534 bytes
                    pub struct Status {
                        // reg.drone.service.common.Heartbeat.0.1
                        // Always aligned
                        pub heartbeat: crate::reg::drone::service::common::heartbeat_0_1::Heartbeat,
                        // uavcan.si.unit.temperature.Scalar.1.0[2]
                        // Always aligned
                        pub temperature_min_max:
                            [crate::uavcan::si::unit::temperature::scalar_1_0::Scalar; 2],
                        // uavcan.si.unit.electric_charge.Scalar.1.0
                        // Always aligned
                        pub available_charge:
                            crate::uavcan::si::unit::electric_charge::scalar_1_0::Scalar,
                        // reg.drone.service.battery.Error.0.1
                        // Always aligned
                        pub error: crate::reg::drone::service::battery::error_0_1::Error,
                        // saturated float16[<=255]
                        // Always aligned
                        pub cell_voltages: ::heapless::Vec<::half::f16, 255>,
                    }
                    impl ::canadensis_encoding::DataType for Status {
                        const EXTENT_BYTES: Option<u32> = Some(600);
                    }
                    impl ::canadensis_encoding::Message for Status {}
                    impl ::canadensis_encoding::Serialize for Status {
                        fn size_bits(&self) -> usize {
                            16 + (self.temperature_min_max)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + 32
                                + 8
                                + 8
                                + (self.cell_voltages).iter().map(|element| 16).sum::<usize>()
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
                }
                pub mod technology_0_1 {
                    /// `reg.drone.service.battery.Technology.0.1`
                    /// Fixed size 1 bytes
                    pub struct Technology {
                        // saturated uint8
                        // Always aligned
                        pub value: u8,
                    }
                    impl ::canadensis_encoding::DataType for Technology {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Technology {}
                    impl ::canadensis_encoding::Serialize for Technology {
                        fn size_bits(&self) -> usize {
                            8 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_aligned_u8(self.value);
                        }
                    }
                }
            }
            pub mod common {
                pub mod heartbeat_0_1 {
                    /// `reg.drone.service.common.Heartbeat.0.1`
                    /// Fixed size 2 bytes
                    pub struct Heartbeat {
                        // reg.drone.service.common.Readiness.0.1
                        // Always aligned
                        pub readiness: crate::reg::drone::service::common::readiness_0_1::Readiness,
                        // uavcan.node.Health.1.0
                        // Always aligned
                        pub health: crate::uavcan::node::health_1_0::Health,
                    }
                    impl ::canadensis_encoding::DataType for Heartbeat {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Heartbeat {}
                    impl ::canadensis_encoding::Serialize for Heartbeat {
                        fn size_bits(&self) -> usize {
                            8 + 8 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.readiness);
                            cursor.write_composite(&self.health);
                        }
                    }
                }
                pub mod readiness_0_1 {
                    /// `reg.drone.service.common.Readiness.0.1`
                    /// Fixed size 1 bytes
                    pub struct Readiness {
                        // truncated uint2
                        // Always aligned
                        pub value: u8,
                    }
                    impl ::canadensis_encoding::DataType for Readiness {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Readiness {}
                    impl ::canadensis_encoding::Serialize for Readiness {
                        fn size_bits(&self) -> usize {
                            2 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_u2(self.value);
                        }
                    }
                }
            }
            pub mod gnss {
                pub mod _0_1 {
                    /// `reg.drone.service.gnss._.0.1`
                    /// Fixed size 0 bytes
                    pub struct _0 {}
                    impl ::canadensis_encoding::DataType for _0 {
                        const EXTENT_BYTES: Option<u32> = Some(0);
                    }
                    impl ::canadensis_encoding::Message for _0 {}
                    impl ::canadensis_encoding::Serialize for _0 {
                        fn size_bits(&self) -> usize {
                            0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
                    }
                }
                pub mod dilution_of_precision_0_1 {
                    /// `reg.drone.service.gnss.DilutionOfPrecision.0.1`
                    /// Fixed size 14 bytes
                    pub struct DilutionOfPrecision {
                        // saturated float16
                        // Always aligned
                        pub geometric: ::half::f16,
                        // saturated float16
                        // Always aligned
                        pub position: ::half::f16,
                        // saturated float16
                        // Always aligned
                        pub horizontal: ::half::f16,
                        // saturated float16
                        // Always aligned
                        pub vertical: ::half::f16,
                        // saturated float16
                        // Always aligned
                        pub time: ::half::f16,
                        // saturated float16
                        // Always aligned
                        pub northing: ::half::f16,
                        // saturated float16
                        // Always aligned
                        pub easting: ::half::f16,
                    }
                    impl ::canadensis_encoding::DataType for DilutionOfPrecision {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for DilutionOfPrecision {}
                    impl ::canadensis_encoding::Serialize for DilutionOfPrecision {
                        fn size_bits(&self) -> usize {
                            16 + 16 + 16 + 16 + 16 + 16 + 16 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f16(self.geometric);
                            cursor.write_f16(self.position);
                            cursor.write_f16(self.horizontal);
                            cursor.write_f16(self.vertical);
                            cursor.write_f16(self.time);
                            cursor.write_f16(self.northing);
                            cursor.write_f16(self.easting);
                        }
                    }
                }
                pub mod heartbeat_0_1 {
                    /// `reg.drone.service.gnss.Heartbeat.0.1`
                    /// Fixed size 25 bytes
                    pub struct Heartbeat {
// reg.drone.service.common.Heartbeat.0.1
// Always aligned
pub heartbeat: crate::reg::drone::service::common::heartbeat_0_1::Heartbeat,
// reg.drone.service.gnss.Sources.0.1
// Always aligned
pub sources: crate::reg::drone::service::gnss::sources_0_1::Sources,
// reg.drone.service.gnss.DilutionOfPrecision.0.1
// Always aligned
pub dop: crate::reg::drone::service::gnss::dilution_of_precision_0_1::DilutionOfPrecision,
// saturated uint8
// Always aligned
pub num_visible_satellites: u8,
// saturated uint8
// Always aligned
pub num_used_satellites: u8,
// saturated bool
// Always aligned
pub fix: bool,
// saturated bool
// Not always aligned
pub rtk_fix: bool,
}
                    impl ::canadensis_encoding::DataType for Heartbeat {
                        const EXTENT_BYTES: Option<u32> = Some(124);
                    }
                    impl ::canadensis_encoding::Message for Heartbeat {}
                    impl ::canadensis_encoding::Serialize for Heartbeat {
                        fn size_bits(&self) -> usize {
                            16 + 48 + 112 + 8 + 8 + 1 + 1 + 0
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
                }
                pub mod sources_0_1 {
                    /// `reg.drone.service.gnss.Sources.0.1`
                    /// Fixed size 6 bytes
                    pub struct Sources {
                        // saturated bool
                        // Always aligned
                        pub gps_l1: bool,
                        // saturated bool
                        // Not always aligned
                        pub gps_l2: bool,
                        // saturated bool
                        // Not always aligned
                        pub gps_l5: bool,
                        // saturated bool
                        // Not always aligned
                        pub glonass_l1: bool,
                        // saturated bool
                        // Not always aligned
                        pub glonass_l2: bool,
                        // saturated bool
                        // Not always aligned
                        pub glonass_l3: bool,
                        // saturated bool
                        // Not always aligned
                        pub galileo_e1: bool,
                        // saturated bool
                        // Not always aligned
                        pub galileo_e5a: bool,
                        // saturated bool
                        // Always aligned
                        pub galileo_e5b: bool,
                        // saturated bool
                        // Not always aligned
                        pub galileo_e6: bool,
                        // saturated bool
                        // Not always aligned
                        pub beidou_b1: bool,
                        // saturated bool
                        // Not always aligned
                        pub beidou_b2: bool,
                        // saturated bool
                        // Not always aligned
                        pub sbas: bool,
                        // saturated bool
                        // Not always aligned
                        pub gbas: bool,
                        // saturated bool
                        // Not always aligned
                        pub rtk_base: bool,
                        // saturated bool
                        // Not always aligned
                        pub imu: bool,
                        // saturated bool
                        // Always aligned
                        pub visual_odometry: bool,
                        // saturated bool
                        // Not always aligned
                        pub dead_reckoning: bool,
                        // saturated bool
                        // Not always aligned
                        pub uwb: bool,
                        // saturated bool
                        // Not always aligned
                        pub magnetic_compass: bool,
                        // saturated bool
                        // Always aligned
                        pub gyro_compass: bool,
                        // saturated bool
                        // Not always aligned
                        pub other_compass: bool,
                    }
                    impl ::canadensis_encoding::DataType for Sources {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Sources {}
                    impl ::canadensis_encoding::Serialize for Sources {
                        fn size_bits(&self) -> usize {
                            1 + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 1
                                + 0
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
                            cursor.write_bool(self.sbas);
                            cursor.write_bool(self.gbas);
                            cursor.write_bool(self.rtk_base);
                            cursor.write_bool(self.imu);
                            cursor.write_bool(self.visual_odometry);
                            cursor.write_bool(self.dead_reckoning);
                            cursor.write_bool(self.uwb);
                            cursor.write_bool(self.magnetic_compass);
                            cursor.write_bool(self.gyro_compass);
                            cursor.write_bool(self.other_compass);
                        }
                    }
                }
                pub mod time_0_1 {
                    /// `reg.drone.service.gnss.Time.0.1`
                    /// Fixed size 21 bytes
                    pub struct Time {
                        // reg.drone.physics.time.TAI64VarTs.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::time::tai64_var_ts_0_1::TAI64VarTs,
                        // uavcan.time.TAIInfo.0.1
                        // Always aligned
                        pub info: crate::uavcan::time::tai_info_0_1::TAIInfo,
                    }
                    impl ::canadensis_encoding::DataType for Time {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Time {}
                    impl ::canadensis_encoding::Serialize for Time {
                        fn size_bits(&self) -> usize {
                            152 + 16 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.value);
                            cursor.write_composite(&self.info);
                        }
                    }
                }
            }
            pub mod sensor {
                pub mod status_0_1 {
                    /// `reg.drone.service.sensor.Status.0.1`
                    /// Fixed size 12 bytes
                    pub struct Status {
                        // uavcan.si.unit.duration.Scalar.1.0
                        // Always aligned
                        pub data_validity_period:
                            crate::uavcan::si::unit::duration::scalar_1_0::Scalar,
                        // saturated uint32
                        // Always aligned
                        pub error_count: u32,
                        // uavcan.si.unit.temperature.Scalar.1.0
                        // Always aligned
                        pub sensor_temperature:
                            crate::uavcan::si::unit::temperature::scalar_1_0::Scalar,
                    }
                    impl ::canadensis_encoding::DataType for Status {
                        const EXTENT_BYTES: Option<u32> = Some(63);
                    }
                    impl ::canadensis_encoding::Message for Status {}
                    impl ::canadensis_encoding::Serialize for Status {
                        fn size_bits(&self) -> usize {
                            32 + 32 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.data_validity_period);
                            cursor.write_aligned_u32(self.error_count);
                            cursor.write_composite(&self.sensor_temperature);
                        }
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
            /// Size ranges from 9 to 121 bytes
            pub struct Record {
                // uavcan.time.SynchronizedTimestamp.1.0
                // Always aligned
                pub timestamp:
                    crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                // uavcan.diagnostic.Severity.1.0
                // Always aligned
                pub severity: crate::uavcan::diagnostic::severity_1_0::Severity,
                // saturated uint8[<=112]
                // Always aligned
                pub text: ::heapless::Vec<u8, 112>,
            }
            impl ::canadensis_encoding::DataType for Record {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Message for Record {}
            impl ::canadensis_encoding::Serialize for Record {
                fn size_bits(&self) -> usize {
                    56 + 8 + 8 + (self.text).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.timestamp);
                    cursor.write_composite(&self.severity);
                    cursor.write_aligned_u8((self.text).len() as u8);
                    cursor.write_bytes(&(self.text)[..]);
                }
            }
        }
        pub mod record_1_1 {
            /// `uavcan.diagnostic.Record.1.1`
            /// Size ranges from 9 to 264 bytes
            pub struct Record {
                // uavcan.time.SynchronizedTimestamp.1.0
                // Always aligned
                pub timestamp:
                    crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                // uavcan.diagnostic.Severity.1.0
                // Always aligned
                pub severity: crate::uavcan::diagnostic::severity_1_0::Severity,
                // saturated uint8[<=255]
                // Always aligned
                pub text: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for Record {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Message for Record {}
            impl ::canadensis_encoding::Serialize for Record {
                fn size_bits(&self) -> usize {
                    56 + 8 + 8 + (self.text).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.timestamp);
                    cursor.write_composite(&self.severity);
                    cursor.write_aligned_u8((self.text).len() as u8);
                    cursor.write_bytes(&(self.text)[..]);
                }
            }
        }
        pub mod severity_1_0 {
            /// `uavcan.diagnostic.Severity.1.0`
            /// Fixed size 1 bytes
            pub struct Severity {
                // saturated uint3
                // Always aligned
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for Severity {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Severity {}
            impl ::canadensis_encoding::Serialize for Severity {
                fn size_bits(&self) -> usize {
                    3 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u3(self.value);
                }
            }
        }
    }
    pub mod file {
        pub mod error_1_0 {
            /// `uavcan.file.Error.1.0`
            /// Fixed size 2 bytes
            pub struct Error {
                // saturated uint16
                // Always aligned
                pub value: u16,
            }
            impl ::canadensis_encoding::DataType for Error {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Error {}
            impl ::canadensis_encoding::Serialize for Error {
                fn size_bits(&self) -> usize {
                    16 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16(self.value);
                }
            }
        }
        pub mod get_info_0_1 {
            /// `uavcan.file.GetInfo.0.1`
            /// Size ranges from 1 to 113 bytes
            pub struct GetInfoRequest {
                // uavcan.file.Path.1.0
                // Always aligned
                pub path: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for GetInfoRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for GetInfoRequest {}
            impl ::canadensis_encoding::Serialize for GetInfoRequest {
                fn size_bits(&self) -> usize {
                    (self.path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.path);
                }
            }

            /// `uavcan.file.GetInfo.0.1`
            /// Fixed size 13 bytes
            pub struct GetInfoResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
                // truncated uint40
                // Always aligned
                pub size: u64,
                // truncated uint40
                // Always aligned
                pub unix_timestamp_of_last_modification: u64,
                // saturated bool
                // Always aligned
                pub is_file_not_directory: bool,
                // saturated bool
                // Not always aligned
                pub is_link: bool,
                // saturated bool
                // Not always aligned
                pub is_readable: bool,
                // saturated bool
                // Not always aligned
                pub is_writeable: bool,
            }
            impl ::canadensis_encoding::DataType for GetInfoResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for GetInfoResponse {}
            impl ::canadensis_encoding::Serialize for GetInfoResponse {
                fn size_bits(&self) -> usize {
                    16 + 40 + 40 + 1 + 1 + 1 + 1 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                    cursor.write_u40(self.size);
                    cursor.write_u40(self.unix_timestamp_of_last_modification);
                    cursor.write_bool(self.is_file_not_directory);
                    cursor.write_bool(self.is_link);
                    cursor.write_bool(self.is_readable);
                    cursor.write_bool(self.is_writeable);
                }
            }
        }
        pub mod get_info_0_2 {
            /// `uavcan.file.GetInfo.0.2`
            /// Size ranges from 1 to 256 bytes
            pub struct GetInfoRequest {
                // uavcan.file.Path.2.0
                // Always aligned
                pub path: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for GetInfoRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for GetInfoRequest {}
            impl ::canadensis_encoding::Serialize for GetInfoRequest {
                fn size_bits(&self) -> usize {
                    (self.path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.path);
                }
            }

            /// `uavcan.file.GetInfo.0.2`
            /// Fixed size 13 bytes
            pub struct GetInfoResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
                // truncated uint40
                // Always aligned
                pub size: u64,
                // truncated uint40
                // Always aligned
                pub unix_timestamp_of_last_modification: u64,
                // saturated bool
                // Always aligned
                pub is_file_not_directory: bool,
                // saturated bool
                // Not always aligned
                pub is_link: bool,
                // saturated bool
                // Not always aligned
                pub is_readable: bool,
                // saturated bool
                // Not always aligned
                pub is_writeable: bool,
            }
            impl ::canadensis_encoding::DataType for GetInfoResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for GetInfoResponse {}
            impl ::canadensis_encoding::Serialize for GetInfoResponse {
                fn size_bits(&self) -> usize {
                    16 + 40 + 40 + 1 + 1 + 1 + 1 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                    cursor.write_u40(self.size);
                    cursor.write_u40(self.unix_timestamp_of_last_modification);
                    cursor.write_bool(self.is_file_not_directory);
                    cursor.write_bool(self.is_link);
                    cursor.write_bool(self.is_readable);
                    cursor.write_bool(self.is_writeable);
                }
            }
        }
        pub mod list_0_1 {
            /// `uavcan.file.List.0.1`
            /// Size ranges from 9 to 121 bytes
            pub struct ListRequest {
                // saturated uint32
                // Always aligned
                pub entry_index: u32,
                // uavcan.file.Path.1.0
                // Always aligned
                pub directory_path: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ListRequest {}
            impl ::canadensis_encoding::Serialize for ListRequest {
                fn size_bits(&self) -> usize {
                    32 + (self.directory_path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u32(self.entry_index);
                    cursor.write_composite(&self.directory_path);
                }
            }

            /// `uavcan.file.List.0.1`
            /// Size ranges from 5 to 117 bytes
            pub struct ListResponse {
                // uavcan.file.Path.1.0
                // Always aligned
                pub entry_base_name: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListResponse {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Response for ListResponse {}
            impl ::canadensis_encoding::Serialize for ListResponse {
                fn size_bits(&self) -> usize {
                    (self.entry_base_name).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.entry_base_name);
                }
            }
        }
        pub mod list_0_2 {
            /// `uavcan.file.List.0.2`
            /// Size ranges from 9 to 264 bytes
            pub struct ListRequest {
                // saturated uint32
                // Always aligned
                pub entry_index: u32,
                // uavcan.file.Path.2.0
                // Always aligned
                pub directory_path: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ListRequest {}
            impl ::canadensis_encoding::Serialize for ListRequest {
                fn size_bits(&self) -> usize {
                    32 + (self.directory_path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u32(self.entry_index);
                    cursor.write_composite(&self.directory_path);
                }
            }

            /// `uavcan.file.List.0.2`
            /// Size ranges from 5 to 260 bytes
            pub struct ListResponse {
                // uavcan.file.Path.2.0
                // Always aligned
                pub entry_base_name: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ListResponse {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Response for ListResponse {}
            impl ::canadensis_encoding::Serialize for ListResponse {
                fn size_bits(&self) -> usize {
                    (self.entry_base_name).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.entry_base_name);
                }
            }
        }
        pub mod modify_1_0 {
            /// `uavcan.file.Modify.1.0`
            /// Size ranges from 6 to 230 bytes
            pub struct ModifyRequest {
                // saturated bool
                // Always aligned
                pub preserve_source: bool,
                // saturated bool
                // Not always aligned
                pub overwrite_destination: bool,
                // uavcan.file.Path.1.0
                // Always aligned
                pub source: crate::uavcan::file::path_1_0::Path,
                // uavcan.file.Path.1.0
                // Always aligned
                pub destination: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ModifyRequest {
                const EXTENT_BYTES: Option<u32> = Some(600);
            }
            impl ::canadensis_encoding::Request for ModifyRequest {}
            impl ::canadensis_encoding::Serialize for ModifyRequest {
                fn size_bits(&self) -> usize {
                    1 + 1 + (self.source).size_bits() + (self.destination).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_bool(self.preserve_source);
                    cursor.write_bool(self.overwrite_destination);
                    cursor.write_composite(&self.source);
                    cursor.write_composite(&self.destination);
                }
            }

            /// `uavcan.file.Modify.1.0`
            /// Fixed size 2 bytes
            pub struct ModifyResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for ModifyResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ModifyResponse {}
            impl ::canadensis_encoding::Serialize for ModifyResponse {
                fn size_bits(&self) -> usize {
                    16 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                }
            }
        }
        pub mod modify_1_1 {
            /// `uavcan.file.Modify.1.1`
            /// Size ranges from 6 to 516 bytes
            pub struct ModifyRequest {
                // saturated bool
                // Always aligned
                pub preserve_source: bool,
                // saturated bool
                // Not always aligned
                pub overwrite_destination: bool,
                // uavcan.file.Path.2.0
                // Always aligned
                pub source: crate::uavcan::file::path_2_0::Path,
                // uavcan.file.Path.2.0
                // Always aligned
                pub destination: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ModifyRequest {
                const EXTENT_BYTES: Option<u32> = Some(600);
            }
            impl ::canadensis_encoding::Request for ModifyRequest {}
            impl ::canadensis_encoding::Serialize for ModifyRequest {
                fn size_bits(&self) -> usize {
                    1 + 1 + (self.source).size_bits() + (self.destination).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_bool(self.preserve_source);
                    cursor.write_bool(self.overwrite_destination);
                    cursor.write_composite(&self.source);
                    cursor.write_composite(&self.destination);
                }
            }

            /// `uavcan.file.Modify.1.1`
            /// Fixed size 2 bytes
            pub struct ModifyResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for ModifyResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ModifyResponse {}
            impl ::canadensis_encoding::Serialize for ModifyResponse {
                fn size_bits(&self) -> usize {
                    16 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                }
            }
        }
        pub mod path_1_0 {
            /// `uavcan.file.Path.1.0`
            /// Size ranges from 1 to 113 bytes
            pub struct Path {
                // saturated uint8[<=112]
                // Always aligned
                pub path: ::heapless::Vec<u8, 112>,
            }
            impl ::canadensis_encoding::DataType for Path {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Path {}
            impl ::canadensis_encoding::Serialize for Path {
                fn size_bits(&self) -> usize {
                    8 + (self.path).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u8((self.path).len() as u8);
                    cursor.write_bytes(&(self.path)[..]);
                }
            }
        }
        pub mod path_2_0 {
            /// `uavcan.file.Path.2.0`
            /// Size ranges from 1 to 256 bytes
            pub struct Path {
                // saturated uint8[<=255]
                // Always aligned
                pub path: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for Path {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Path {}
            impl ::canadensis_encoding::Serialize for Path {
                fn size_bits(&self) -> usize {
                    8 + (self.path).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u8((self.path).len() as u8);
                    cursor.write_bytes(&(self.path)[..]);
                }
            }
        }
        pub mod read_1_0 {
            /// `uavcan.file.Read.1.0`
            /// Size ranges from 6 to 118 bytes
            pub struct ReadRequest {
                // truncated uint40
                // Always aligned
                pub offset: u64,
                // uavcan.file.Path.1.0
                // Always aligned
                pub path: crate::uavcan::file::path_1_0::Path,
            }
            impl ::canadensis_encoding::DataType for ReadRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ReadRequest {}
            impl ::canadensis_encoding::Serialize for ReadRequest {
                fn size_bits(&self) -> usize {
                    40 + (self.path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u40(self.offset);
                    cursor.write_composite(&self.path);
                }
            }

            /// `uavcan.file.Read.1.0`
            /// Size ranges from 4 to 260 bytes
            pub struct ReadResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
                // saturated uint8[<=256]
                // Always aligned
                pub data: ::heapless::Vec<u8, 256>,
            }
            impl ::canadensis_encoding::DataType for ReadResponse {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Response for ReadResponse {}
            impl ::canadensis_encoding::Serialize for ReadResponse {
                fn size_bits(&self) -> usize {
                    16 + 16 + (self.data).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                    cursor.write_aligned_u16((self.data).len() as u16);
                    cursor.write_bytes(&(self.data)[..]);
                }
            }
        }
        pub mod read_1_1 {
            /// `uavcan.file.Read.1.1`
            /// Size ranges from 6 to 261 bytes
            pub struct ReadRequest {
                // truncated uint40
                // Always aligned
                pub offset: u64,
                // uavcan.file.Path.2.0
                // Always aligned
                pub path: crate::uavcan::file::path_2_0::Path,
            }
            impl ::canadensis_encoding::DataType for ReadRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ReadRequest {}
            impl ::canadensis_encoding::Serialize for ReadRequest {
                fn size_bits(&self) -> usize {
                    40 + (self.path).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u40(self.offset);
                    cursor.write_composite(&self.path);
                }
            }

            /// `uavcan.file.Read.1.1`
            /// Size ranges from 4 to 260 bytes
            pub struct ReadResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
                // uavcan.primitive.Unstructured.1.0
                // Always aligned
                pub data: crate::uavcan::primitive::unstructured_1_0::Unstructured,
            }
            impl ::canadensis_encoding::DataType for ReadResponse {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Response for ReadResponse {}
            impl ::canadensis_encoding::Serialize for ReadResponse {
                fn size_bits(&self) -> usize {
                    16 + (self.data).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                    cursor.write_composite(&self.data);
                }
            }
        }
        pub mod write_1_0 {
            /// `uavcan.file.Write.1.0`
            /// Size ranges from 7 to 311 bytes
            pub struct WriteRequest {
                // truncated uint40
                // Always aligned
                pub offset: u64,
                // uavcan.file.Path.1.0
                // Always aligned
                pub path: crate::uavcan::file::path_1_0::Path,
                // saturated uint8[<=192]
                // Always aligned
                pub data: ::heapless::Vec<u8, 192>,
            }
            impl ::canadensis_encoding::DataType for WriteRequest {
                const EXTENT_BYTES: Option<u32> = Some(600);
            }
            impl ::canadensis_encoding::Request for WriteRequest {}
            impl ::canadensis_encoding::Serialize for WriteRequest {
                fn size_bits(&self) -> usize {
                    40 + (self.path).size_bits()
                        + 8
                        + (self.data).iter().map(|element| 8).sum::<usize>()
                        + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u40(self.offset);
                    cursor.write_composite(&self.path);
                    cursor.write_aligned_u8((self.data).len() as u8);
                    cursor.write_bytes(&(self.data)[..]);
                }
            }

            /// `uavcan.file.Write.1.0`
            /// Fixed size 2 bytes
            pub struct WriteResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for WriteResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for WriteResponse {}
            impl ::canadensis_encoding::Serialize for WriteResponse {
                fn size_bits(&self) -> usize {
                    16 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                }
            }
        }
        pub mod write_1_1 {
            /// `uavcan.file.Write.1.1`
            /// Size ranges from 8 to 519 bytes
            pub struct WriteRequest {
                // truncated uint40
                // Always aligned
                pub offset: u64,
                // uavcan.file.Path.2.0
                // Always aligned
                pub path: crate::uavcan::file::path_2_0::Path,
                // uavcan.primitive.Unstructured.1.0
                // Always aligned
                pub data: crate::uavcan::primitive::unstructured_1_0::Unstructured,
            }
            impl ::canadensis_encoding::DataType for WriteRequest {
                const EXTENT_BYTES: Option<u32> = Some(600);
            }
            impl ::canadensis_encoding::Request for WriteRequest {}
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

            /// `uavcan.file.Write.1.1`
            /// Fixed size 2 bytes
            pub struct WriteResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
            }
            impl ::canadensis_encoding::DataType for WriteResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for WriteResponse {}
            impl ::canadensis_encoding::Serialize for WriteResponse {
                fn size_bits(&self) -> usize {
                    16 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.error);
                }
            }
        }
    }
    pub mod internet {
        pub mod udp {
            pub mod handle_incoming_packet_0_1 {
                /// `uavcan.internet.udp.HandleIncomingPacket.0.1`
                /// Size ranges from 4 to 313 bytes
                pub struct HandleIncomingPacketRequest {
                    // saturated uint16
                    // Always aligned
                    pub session_id: u16,
                    // saturated uint8[<=309]
                    // Always aligned
                    pub payload: ::heapless::Vec<u8, 309>,
                }
                impl ::canadensis_encoding::DataType for HandleIncomingPacketRequest {
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Request for HandleIncomingPacketRequest {}
                impl ::canadensis_encoding::Serialize for HandleIncomingPacketRequest {
                    fn size_bits(&self) -> usize {
                        16 + 16 + (self.payload).iter().map(|element| 8).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.session_id);
                        cursor.write_aligned_u16((self.payload).len() as u16);
                        cursor.write_bytes(&(self.payload)[..]);
                    }
                }

                /// `uavcan.internet.udp.HandleIncomingPacket.0.1`
                /// Fixed size 0 bytes
                pub struct HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::DataType for HandleIncomingPacketResponse {
                    const EXTENT_BYTES: Option<u32> = Some(63);
                }
                impl ::canadensis_encoding::Response for HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::Serialize for HandleIncomingPacketResponse {
                    fn size_bits(&self) -> usize {
                        0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
                }
            }
            pub mod handle_incoming_packet_0_2 {
                /// `uavcan.internet.udp.HandleIncomingPacket.0.2`
                /// Size ranges from 4 to 512 bytes
                pub struct HandleIncomingPacketRequest {
                    // saturated uint16
                    // Always aligned
                    pub session_id: u16,
                    // saturated uint8[<=508]
                    // Always aligned
                    pub payload: ::heapless::Vec<u8, 508>,
                }
                impl ::canadensis_encoding::DataType for HandleIncomingPacketRequest {
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Request for HandleIncomingPacketRequest {}
                impl ::canadensis_encoding::Serialize for HandleIncomingPacketRequest {
                    fn size_bits(&self) -> usize {
                        16 + 16 + (self.payload).iter().map(|element| 8).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.session_id);
                        cursor.write_aligned_u16((self.payload).len() as u16);
                        cursor.write_bytes(&(self.payload)[..]);
                    }
                }

                /// `uavcan.internet.udp.HandleIncomingPacket.0.2`
                /// Fixed size 0 bytes
                pub struct HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::DataType for HandleIncomingPacketResponse {
                    const EXTENT_BYTES: Option<u32> = Some(63);
                }
                impl ::canadensis_encoding::Response for HandleIncomingPacketResponse {}
                impl ::canadensis_encoding::Serialize for HandleIncomingPacketResponse {
                    fn size_bits(&self) -> usize {
                        0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
                }
            }
            pub mod outgoing_packet_0_1 {
                /// `uavcan.internet.udp.OutgoingPacket.0.1`
                /// Size ranges from 8 to 313 bytes
                pub struct OutgoingPacket {
                    // saturated uint16
                    // Always aligned
                    pub session_id: u16,
                    // saturated uint16
                    // Always aligned
                    pub destination_port: u16,
                    // saturated uint8[<=45]
                    // Always aligned
                    pub destination_address: ::heapless::Vec<u8, 45>,
                    // saturated bool
                    // Always aligned
                    pub use_masquerading: bool,
                    // saturated bool
                    // Not always aligned
                    pub use_dtls: bool,
                    // saturated uint8[<=260]
                    // Always aligned
                    pub payload: ::heapless::Vec<u8, 260>,
                }
                impl ::canadensis_encoding::DataType for OutgoingPacket {
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Message for OutgoingPacket {}
                impl ::canadensis_encoding::Serialize for OutgoingPacket {
                    fn size_bits(&self) -> usize {
                        16 + 16
                            + 8
                            + (self.destination_address)
                                .iter()
                                .map(|element| 8)
                                .sum::<usize>()
                            + 1
                            + 1
                            + 16
                            + (self.payload).iter().map(|element| 8).sum::<usize>()
                            + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.session_id);
                        cursor.write_aligned_u16(self.destination_port);
                        cursor.write_aligned_u8((self.destination_address).len() as u8);
                        cursor.write_bytes(&(self.destination_address)[..]);
                        cursor.write_bool(self.use_masquerading);
                        cursor.write_bool(self.use_dtls);
                        cursor.write_aligned_u16((self.payload).len() as u16);
                        cursor.write_bytes(&(self.payload)[..]);
                    }
                }
            }
            pub mod outgoing_packet_0_2 {
                /// `uavcan.internet.udp.OutgoingPacket.0.2`
                /// Size ranges from 8 to 561 bytes
                pub struct OutgoingPacket {
                    // saturated uint16
                    // Always aligned
                    pub session_id: u16,
                    // saturated uint16
                    // Always aligned
                    pub destination_port: u16,
                    // saturated uint8[<=45]
                    // Always aligned
                    pub destination_address: ::heapless::Vec<u8, 45>,
                    // saturated bool
                    // Always aligned
                    pub use_masquerading: bool,
                    // saturated bool
                    // Not always aligned
                    pub use_dtls: bool,
                    // saturated uint8[<=508]
                    // Always aligned
                    pub payload: ::heapless::Vec<u8, 508>,
                }
                impl ::canadensis_encoding::DataType for OutgoingPacket {
                    const EXTENT_BYTES: Option<u32> = Some(600);
                }
                impl ::canadensis_encoding::Message for OutgoingPacket {}
                impl ::canadensis_encoding::Serialize for OutgoingPacket {
                    fn size_bits(&self) -> usize {
                        16 + 16
                            + 8
                            + (self.destination_address)
                                .iter()
                                .map(|element| 8)
                                .sum::<usize>()
                            + 1
                            + 1
                            + 16
                            + (self.payload).iter().map(|element| 8).sum::<usize>()
                            + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.session_id);
                        cursor.write_aligned_u16(self.destination_port);
                        cursor.write_aligned_u8((self.destination_address).len() as u8);
                        cursor.write_bytes(&(self.destination_address)[..]);
                        cursor.write_bool(self.use_masquerading);
                        cursor.write_bool(self.use_dtls);
                        cursor.write_aligned_u16((self.payload).len() as u16);
                        cursor.write_bytes(&(self.payload)[..]);
                    }
                }
            }
        }
    }
    pub mod metatransport {
        pub mod can {
            pub mod arbitration_id_0_1 {
                /// `uavcan.metatransport.can.ArbitrationID.0.1`
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
                impl ::canadensis_encoding::Serialize for ArbitrationID {
                    fn size_bits(&self) -> usize {
                        8 + match self {
                            ArbitrationID::Base(inner) => 32,
                            ArbitrationID::Extended(inner) => 32,
                        }
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
            }
            pub mod base_arbitration_id_0_1 {
                /// `uavcan.metatransport.can.BaseArbitrationID.0.1`
                /// Fixed size 4 bytes
                pub struct BaseArbitrationID {
                    // truncated uint11
                    // Always aligned
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for BaseArbitrationID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for BaseArbitrationID {}
                impl ::canadensis_encoding::Serialize for BaseArbitrationID {
                    fn size_bits(&self) -> usize {
                        11 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u11(self.value);
                    }
                }
            }
            pub mod data_classic_0_1 {
                /// `uavcan.metatransport.can.DataClassic.0.1`
                /// Size ranges from 6 to 14 bytes
                pub struct DataClassic {
                    // uavcan.metatransport.can.ArbitrationID.0.1
                    // Always aligned
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                    // saturated uint8[<=8]
                    // Always aligned
                    pub data: ::heapless::Vec<u8, 8>,
                }
                impl ::canadensis_encoding::DataType for DataClassic {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for DataClassic {}
                impl ::canadensis_encoding::Serialize for DataClassic {
                    fn size_bits(&self) -> usize {
                        40 + 8 + (self.data).iter().map(|element| 8).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.arbitration_id);
                        cursor.write_aligned_u8((self.data).len() as u8);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
            }
            pub mod data_fd_0_1 {
                /// `uavcan.metatransport.can.DataFD.0.1`
                /// Size ranges from 6 to 70 bytes
                pub struct DataFD {
                    // uavcan.metatransport.can.ArbitrationID.0.1
                    // Always aligned
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                    // saturated uint8[<=64]
                    // Always aligned
                    pub data: ::heapless::Vec<u8, 64>,
                }
                impl ::canadensis_encoding::DataType for DataFD {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for DataFD {}
                impl ::canadensis_encoding::Serialize for DataFD {
                    fn size_bits(&self) -> usize {
                        40 + 8 + (self.data).iter().map(|element| 8).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.arbitration_id);
                        cursor.write_aligned_u8((self.data).len() as u8);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
            }
            pub mod error_0_1 {
                /// `uavcan.metatransport.can.Error.0.1`
                /// Fixed size 4 bytes
                pub struct Error {}
                impl ::canadensis_encoding::DataType for Error {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Error {}
                impl ::canadensis_encoding::Serialize for Error {
                    fn size_bits(&self) -> usize {
                        0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
                }
            }
            pub mod extended_arbitration_id_0_1 {
                /// `uavcan.metatransport.can.ExtendedArbitrationID.0.1`
                /// Fixed size 4 bytes
                pub struct ExtendedArbitrationID {
                    // truncated uint29
                    // Always aligned
                    pub value: u32,
                }
                impl ::canadensis_encoding::DataType for ExtendedArbitrationID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for ExtendedArbitrationID {}
                impl ::canadensis_encoding::Serialize for ExtendedArbitrationID {
                    fn size_bits(&self) -> usize {
                        29 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u29(self.value);
                    }
                }
            }
            pub mod frame_0_1 {
                /// `uavcan.metatransport.can.Frame.0.1`
                /// Size ranges from 12 to 78 bytes
                pub struct Frame {
                    // uavcan.time.SynchronizedTimestamp.1.0
                    // Always aligned
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    // uavcan.metatransport.can.Manifestation.0.1
                    // Always aligned
                    pub manifestation:
                        crate::uavcan::metatransport::can::manifestation_0_1::Manifestation,
                }
                impl ::canadensis_encoding::DataType for Frame {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Frame {}
                impl ::canadensis_encoding::Serialize for Frame {
                    fn size_bits(&self) -> usize {
                        56 + (self.manifestation).size_bits() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.timestamp);
                        cursor.write_composite(&self.manifestation);
                    }
                }
            }
            pub mod frame_0_2 {
                /// `uavcan.metatransport.can.Frame.0.2`
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
            }
            pub mod manifestation_0_1 {
                /// `uavcan.metatransport.can.Manifestation.0.1`
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
            }
            pub mod rtr_0_1 {
                /// `uavcan.metatransport.can.RTR.0.1`
                /// Fixed size 5 bytes
                pub struct RTR {
                    // uavcan.metatransport.can.ArbitrationID.0.1
                    // Always aligned
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                }
                impl ::canadensis_encoding::DataType for RTR {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for RTR {}
                impl ::canadensis_encoding::Serialize for RTR {
                    fn size_bits(&self) -> usize {
                        40 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.arbitration_id);
                    }
                }
            }
        }
        pub mod ethernet {
            pub mod ether_type_0_1 {
                /// `uavcan.metatransport.ethernet.EtherType.0.1`
                /// Fixed size 2 bytes
                pub struct EtherType {
                    // saturated uint16
                    // Always aligned
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for EtherType {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for EtherType {}
                impl ::canadensis_encoding::Serialize for EtherType {
                    fn size_bits(&self) -> usize {
                        16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.value);
                    }
                }
            }
            pub mod frame_0_1 {
                /// `uavcan.metatransport.ethernet.Frame.0.1`
                /// Size ranges from 16 to 9232 bytes
                pub struct Frame {
                    // saturated uint8[6]
                    // Always aligned
                    pub destination: [u8; 6],
                    // saturated uint8[6]
                    // Always aligned
                    pub source: [u8; 6],
                    // uavcan.metatransport.ethernet.EtherType.0.1
                    // Always aligned
                    pub ethertype:
                        crate::uavcan::metatransport::ethernet::ether_type_0_1::EtherType,
                    // saturated uint8[<=9216]
                    // Always aligned
                    pub payload: ::heapless::Vec<u8, 9216>,
                }
                impl ::canadensis_encoding::DataType for Frame {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Frame {}
                impl ::canadensis_encoding::Serialize for Frame {
                    fn size_bits(&self) -> usize {
                        (self.destination).iter().map(|element| 8).sum::<usize>()
                            + (self.source).iter().map(|element| 8).sum::<usize>()
                            + 16
                            + 16
                            + (self.payload).iter().map(|element| 8).sum::<usize>()
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
            }
        }
        pub mod serial {
            pub mod fragment_0_1 {
                /// `uavcan.metatransport.serial.Fragment.0.1`
                /// Size ranges from 9 to 265 bytes
                pub struct Fragment {
                    // uavcan.time.SynchronizedTimestamp.1.0
                    // Always aligned
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    // saturated uint8[<=256]
                    // Always aligned
                    pub data: ::heapless::Vec<u8, 256>,
                }
                impl ::canadensis_encoding::DataType for Fragment {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Fragment {}
                impl ::canadensis_encoding::Serialize for Fragment {
                    fn size_bits(&self) -> usize {
                        56 + 16 + (self.data).iter().map(|element| 8).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.timestamp);
                        cursor.write_aligned_u16((self.data).len() as u16);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
            }
            pub mod fragment_0_2 {
                /// `uavcan.metatransport.serial.Fragment.0.2`
                /// Size ranges from 2 to 2050 bytes
                pub struct Fragment {
                    // saturated uint8[<=2048]
                    // Always aligned
                    pub data: ::heapless::Vec<u8, 2048>,
                }
                impl ::canadensis_encoding::DataType for Fragment {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Fragment {}
                impl ::canadensis_encoding::Serialize for Fragment {
                    fn size_bits(&self) -> usize {
                        16 + (self.data).iter().map(|element| 8).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16((self.data).len() as u16);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
            }
        }
        pub mod udp {
            pub mod endpoint_0_1 {
                /// `uavcan.metatransport.udp.Endpoint.0.1`
                /// Fixed size 32 bytes
                pub struct Endpoint {
                    // saturated uint8[16]
                    // Always aligned
                    pub ip_address: [u8; 16],
                    // saturated uint8[6]
                    // Always aligned
                    pub mac_address: [u8; 6],
                    // saturated uint16
                    // Always aligned
                    pub port: u16,
                }
                impl ::canadensis_encoding::DataType for Endpoint {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Endpoint {}
                impl ::canadensis_encoding::Serialize for Endpoint {
                    fn size_bits(&self) -> usize {
                        (self.ip_address).iter().map(|element| 8).sum::<usize>()
                            + (self.mac_address).iter().map(|element| 8).sum::<usize>()
                            + 16
                            + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_bytes(&(self.ip_address)[..]);
                        cursor.write_bytes(&(self.mac_address)[..]);
                        cursor.write_aligned_u16(self.port);
                    }
                }
            }
            pub mod frame_0_1 {
                /// `uavcan.metatransport.udp.Frame.0.1`
                /// Size ranges from 74 to 9262 bytes
                pub struct Frame {
                    // uavcan.time.SynchronizedTimestamp.1.0
                    // Always aligned
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    // uavcan.metatransport.udp.Endpoint.0.1
                    // Always aligned
                    pub source: crate::uavcan::metatransport::udp::endpoint_0_1::Endpoint,
                    // uavcan.metatransport.udp.Endpoint.0.1
                    // Always aligned
                    pub destination: crate::uavcan::metatransport::udp::endpoint_0_1::Endpoint,
                    // saturated uint8[<=9188]
                    // Always aligned
                    pub data: ::heapless::Vec<u8, 9188>,
                }
                impl ::canadensis_encoding::DataType for Frame {
                    const EXTENT_BYTES: Option<u32> = Some(10240);
                }
                impl ::canadensis_encoding::Message for Frame {}
                impl ::canadensis_encoding::Serialize for Frame {
                    fn size_bits(&self) -> usize {
                        56 + 256 + 256 + 16 + (self.data).iter().map(|element| 8).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_composite(&self.timestamp);
                        cursor.write_composite(&self.source);
                        cursor.write_composite(&self.destination);
                        cursor.write_aligned_u16((self.data).len() as u16);
                        cursor.write_bytes(&(self.data)[..]);
                    }
                }
            }
        }
    }
    pub mod node {
        pub mod execute_command_1_0 {
            /// `uavcan.node.ExecuteCommand.1.0`
            /// Size ranges from 3 to 115 bytes
            pub struct ExecuteCommandRequest {
                // saturated uint16
                // Always aligned
                pub command: u16,
                // saturated uint8[<=112]
                // Always aligned
                pub parameter: ::heapless::Vec<u8, 112>,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ExecuteCommandRequest {}
            impl ::canadensis_encoding::Serialize for ExecuteCommandRequest {
                fn size_bits(&self) -> usize {
                    16 + 8 + (self.parameter).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16(self.command);
                    cursor.write_aligned_u8((self.parameter).len() as u8);
                    cursor.write_bytes(&(self.parameter)[..]);
                }
            }

            /// `uavcan.node.ExecuteCommand.1.0`
            /// Fixed size 1 bytes
            pub struct ExecuteCommandResponse {
                // saturated uint8
                // Always aligned
                pub status: u8,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ExecuteCommandResponse {}
            impl ::canadensis_encoding::Serialize for ExecuteCommandResponse {
                fn size_bits(&self) -> usize {
                    8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u8(self.status);
                }
            }
        }
        pub mod execute_command_1_1 {
            /// `uavcan.node.ExecuteCommand.1.1`
            /// Size ranges from 3 to 258 bytes
            pub struct ExecuteCommandRequest {
                // saturated uint16
                // Always aligned
                pub command: u16,
                // saturated uint8[<=255]
                // Always aligned
                pub parameter: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandRequest {
                const EXTENT_BYTES: Option<u32> = Some(300);
            }
            impl ::canadensis_encoding::Request for ExecuteCommandRequest {}
            impl ::canadensis_encoding::Serialize for ExecuteCommandRequest {
                fn size_bits(&self) -> usize {
                    16 + 8 + (self.parameter).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16(self.command);
                    cursor.write_aligned_u8((self.parameter).len() as u8);
                    cursor.write_bytes(&(self.parameter)[..]);
                }
            }

            /// `uavcan.node.ExecuteCommand.1.1`
            /// Fixed size 1 bytes
            pub struct ExecuteCommandResponse {
                // saturated uint8
                // Always aligned
                pub status: u8,
            }
            impl ::canadensis_encoding::DataType for ExecuteCommandResponse {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Response for ExecuteCommandResponse {}
            impl ::canadensis_encoding::Serialize for ExecuteCommandResponse {
                fn size_bits(&self) -> usize {
                    8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u8(self.status);
                }
            }
        }
        pub mod get_info_1_0 {
            /// `uavcan.node.GetInfo.1.0`
            /// Fixed size 0 bytes
            pub struct GetInfoRequest {}
            impl ::canadensis_encoding::DataType for GetInfoRequest {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Request for GetInfoRequest {}
            impl ::canadensis_encoding::Serialize for GetInfoRequest {
                fn size_bits(&self) -> usize {
                    0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
            }

            /// `uavcan.node.GetInfo.1.0`
            /// Size ranges from 33 to 313 bytes
            pub struct GetInfoResponse {
                // uavcan.node.Version.1.0
                // Always aligned
                pub protocol_version: crate::uavcan::node::version_1_0::Version,
                // uavcan.node.Version.1.0
                // Always aligned
                pub hardware_version: crate::uavcan::node::version_1_0::Version,
                // uavcan.node.Version.1.0
                // Always aligned
                pub software_version: crate::uavcan::node::version_1_0::Version,
                // saturated uint64
                // Always aligned
                pub software_vcs_revision_id: u64,
                // saturated uint8[16]
                // Always aligned
                pub unique_id: [u8; 16],
                // saturated uint8[<=50]
                // Always aligned
                pub name: ::heapless::Vec<u8, 50>,
                // saturated uint64[<=1]
                // Always aligned
                pub software_image_crc: ::heapless::Vec<u64, 1>,
                // saturated uint8[<=222]
                // Always aligned
                pub certificate_of_authenticity: ::heapless::Vec<u8, 222>,
            }
            impl ::canadensis_encoding::DataType for GetInfoResponse {
                const EXTENT_BYTES: Option<u32> = Some(448);
            }
            impl ::canadensis_encoding::Response for GetInfoResponse {}
            impl ::canadensis_encoding::Serialize for GetInfoResponse {
                fn size_bits(&self) -> usize {
                    16 + 16
                        + 16
                        + 64
                        + (self.unique_id).iter().map(|element| 8).sum::<usize>()
                        + 8
                        + (self.name).iter().map(|element| 8).sum::<usize>()
                        + 8
                        + (self.software_image_crc)
                            .iter()
                            .map(|element| 64)
                            .sum::<usize>()
                        + 8
                        + (self.certificate_of_authenticity)
                            .iter()
                            .map(|element| 8)
                            .sum::<usize>()
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
        }
        pub mod get_transport_statistics_0_1 {
            /// `uavcan.node.GetTransportStatistics.0.1`
            /// Fixed size 0 bytes
            pub struct GetTransportStatisticsRequest {}
            impl ::canadensis_encoding::DataType for GetTransportStatisticsRequest {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Request for GetTransportStatisticsRequest {}
            impl ::canadensis_encoding::Serialize for GetTransportStatisticsRequest {
                fn size_bits(&self) -> usize {
                    0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
            }

            /// `uavcan.node.GetTransportStatistics.0.1`
            /// Size ranges from 16 to 61 bytes
            pub struct GetTransportStatisticsResponse {
                // uavcan.node.IOStatistics.0.1
                // Always aligned
                pub transfer_statistics: crate::uavcan::node::io_statistics_0_1::IOStatistics,
                // uavcan.node.IOStatistics.0.1[<=3]
                // Always aligned
                pub network_interface_statistics:
                    ::heapless::Vec<crate::uavcan::node::io_statistics_0_1::IOStatistics, 3>,
            }
            impl ::canadensis_encoding::DataType for GetTransportStatisticsResponse {
                const EXTENT_BYTES: Option<u32> = Some(192);
            }
            impl ::canadensis_encoding::Response for GetTransportStatisticsResponse {}
            impl ::canadensis_encoding::Serialize for GetTransportStatisticsResponse {
                fn size_bits(&self) -> usize {
                    120 + 8
                        + (self.network_interface_statistics)
                            .iter()
                            .map(|element| 120)
                            .sum::<usize>()
                        + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.transfer_statistics);
                    cursor.write_aligned_u8((self.network_interface_statistics).len() as u8);
                    for value in (self.network_interface_statistics).iter() {
                        cursor.write_composite(value);
                    }
                }
            }
        }
        pub mod health_1_0 {
            /// `uavcan.node.Health.1.0`
            /// Fixed size 1 bytes
            pub struct Health {
                // saturated uint2
                // Always aligned
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for Health {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Health {}
            impl ::canadensis_encoding::Serialize for Health {
                fn size_bits(&self) -> usize {
                    2 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u2(self.value);
                }
            }
        }
        pub mod heartbeat_1_0 {
            /// `uavcan.node.Heartbeat.1.0`
            /// Fixed size 7 bytes
            pub struct Heartbeat {
                // saturated uint32
                // Always aligned
                pub uptime: u32,
                // uavcan.node.Health.1.0
                // Always aligned
                pub health: crate::uavcan::node::health_1_0::Health,
                // uavcan.node.Mode.1.0
                // Always aligned
                pub mode: crate::uavcan::node::mode_1_0::Mode,
                // saturated uint8
                // Always aligned
                pub vendor_specific_status_code: u8,
            }
            impl ::canadensis_encoding::DataType for Heartbeat {
                const EXTENT_BYTES: Option<u32> = Some(12);
            }
            impl ::canadensis_encoding::Message for Heartbeat {}
            impl ::canadensis_encoding::Serialize for Heartbeat {
                fn size_bits(&self) -> usize {
                    32 + 8 + 8 + 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u32(self.uptime);
                    cursor.write_composite(&self.health);
                    cursor.write_composite(&self.mode);
                    cursor.write_aligned_u8(self.vendor_specific_status_code);
                }
            }
        }
        pub mod id_1_0 {
            /// `uavcan.node.ID.1.0`
            /// Fixed size 2 bytes
            pub struct ID {
                // saturated uint16
                // Always aligned
                pub value: u16,
            }
            impl ::canadensis_encoding::DataType for ID {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for ID {}
            impl ::canadensis_encoding::Serialize for ID {
                fn size_bits(&self) -> usize {
                    16 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16(self.value);
                }
            }
        }
        pub mod io_statistics_0_1 {
            /// `uavcan.node.IOStatistics.0.1`
            /// Fixed size 15 bytes
            pub struct IOStatistics {
                // truncated uint40
                // Always aligned
                pub num_emitted: u64,
                // truncated uint40
                // Always aligned
                pub num_received: u64,
                // truncated uint40
                // Always aligned
                pub num_errored: u64,
            }
            impl ::canadensis_encoding::DataType for IOStatistics {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for IOStatistics {}
            impl ::canadensis_encoding::Serialize for IOStatistics {
                fn size_bits(&self) -> usize {
                    40 + 40 + 40 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u40(self.num_emitted);
                    cursor.write_u40(self.num_received);
                    cursor.write_u40(self.num_errored);
                }
            }
        }
        pub mod mode_1_0 {
            /// `uavcan.node.Mode.1.0`
            /// Fixed size 1 bytes
            pub struct Mode {
                // saturated uint3
                // Always aligned
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for Mode {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Mode {}
            impl ::canadensis_encoding::Serialize for Mode {
                fn size_bits(&self) -> usize {
                    3 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u3(self.value);
                }
            }
        }
        pub mod port {
            pub mod id_1_0 {
                /// `uavcan.node.port.ID.1.0`
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
                impl ::canadensis_encoding::Serialize for ID {
                    fn size_bits(&self) -> usize {
                        8 + match self {
                            ID::SubjectId(inner) => 16,
                            ID::ServiceId(inner) => 16,
                        }
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
            }
            pub mod list_0_1 {
                /// `uavcan.node.port.List.0.1`
                /// Size ranges from 146 to 2194 bytes
                pub struct List {
                    // uavcan.node.port.SubjectIDList.0.1
                    // Always aligned
                    pub publishers: crate::uavcan::node::port::subject_id_list_0_1::SubjectIDList,
                    // uavcan.node.port.SubjectIDList.0.1
                    // Always aligned
                    pub subscribers: crate::uavcan::node::port::subject_id_list_0_1::SubjectIDList,
                    // uavcan.node.port.ServiceIDList.0.1
                    // Always aligned
                    pub clients: crate::uavcan::node::port::service_id_list_0_1::ServiceIDList,
                    // uavcan.node.port.ServiceIDList.0.1
                    // Always aligned
                    pub servers: crate::uavcan::node::port::service_id_list_0_1::ServiceIDList,
                }
                impl ::canadensis_encoding::DataType for List {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for List {}
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
            }
            pub mod service_id_1_0 {
                /// `uavcan.node.port.ServiceID.1.0`
                /// Fixed size 2 bytes
                pub struct ServiceID {
                    // saturated uint9
                    // Always aligned
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for ServiceID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for ServiceID {}
                impl ::canadensis_encoding::Serialize for ServiceID {
                    fn size_bits(&self) -> usize {
                        9 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u9(self.value);
                    }
                }
            }
            pub mod service_id_list_0_1 {
                /// `uavcan.node.port.ServiceIDList.0.1`
                /// Fixed size 64 bytes
                pub struct ServiceIDList {
                    // saturated bool[512]
                    // Always aligned
                    pub mask: [bool; 512],
                }
                impl ::canadensis_encoding::DataType for ServiceIDList {
                    const EXTENT_BYTES: Option<u32> = Some(128);
                }
                impl ::canadensis_encoding::Message for ServiceIDList {}
                impl ::canadensis_encoding::Serialize for ServiceIDList {
                    fn size_bits(&self) -> usize {
                        (self.mask).iter().map(|element| 1).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        for value in (self.mask).iter() {
                            cursor.write_bool(*value);
                        }
                    }
                }
            }
            pub mod subject_id_1_0 {
                /// `uavcan.node.port.SubjectID.1.0`
                /// Fixed size 2 bytes
                pub struct SubjectID {
                    // saturated uint13
                    // Always aligned
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for SubjectID {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for SubjectID {}
                impl ::canadensis_encoding::Serialize for SubjectID {
                    fn size_bits(&self) -> usize {
                        13 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u13(self.value);
                    }
                }
            }
            pub mod subject_id_list_0_1 {
                /// `uavcan.node.port.SubjectIDList.0.1`
                /// Size ranges from 1 to 1025 bytes
                pub enum SubjectIDList {
                    // saturated bool[8192]
                    Mask([bool; 8192]),
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
                impl ::canadensis_encoding::Serialize for SubjectIDList {
                    fn size_bits(&self) -> usize {
                        8 + match self {
                            SubjectIDList::Mask(inner) => {
                                (inner).iter().map(|element| 1).sum::<usize>()
                            }
                            SubjectIDList::SparseList(inner) => {
                                8 + (inner).iter().map(|element| 16).sum::<usize>()
                            }
                            SubjectIDList::Total(inner) => 0,
                        }
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        match self {
                            SubjectIDList::Mask(inner) => {
                                cursor.write_aligned_u8(0);
                                for value in (inner).iter() {
                                    cursor.write_bool(*value);
                                }
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
            }
        }
        pub mod version_1_0 {
            /// `uavcan.node.Version.1.0`
            /// Fixed size 2 bytes
            pub struct Version {
                // saturated uint8
                // Always aligned
                pub major: u8,
                // saturated uint8
                // Always aligned
                pub minor: u8,
            }
            impl ::canadensis_encoding::DataType for Version {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Version {}
            impl ::canadensis_encoding::Serialize for Version {
                fn size_bits(&self) -> usize {
                    8 + 8 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u8(self.major);
                    cursor.write_aligned_u8(self.minor);
                }
            }
        }
    }
    pub mod pnp {
        pub mod cluster {
            pub mod append_entries_1_0 {
                /// `uavcan.pnp.cluster.AppendEntries.1.0`
                /// Size ranges from 13 to 35 bytes
                pub struct AppendEntriesRequest {
                    // saturated uint32
                    // Always aligned
                    pub term: u32,
                    // saturated uint32
                    // Always aligned
                    pub prev_log_term: u32,
                    // saturated uint16
                    // Always aligned
                    pub prev_log_index: u16,
                    // saturated uint16
                    // Always aligned
                    pub leader_commit: u16,
                    // uavcan.pnp.cluster.Entry.1.0[<=1]
                    // Always aligned
                    pub entries: ::heapless::Vec<crate::uavcan::pnp::cluster::entry_1_0::Entry, 1>,
                }
                impl ::canadensis_encoding::DataType for AppendEntriesRequest {
                    const EXTENT_BYTES: Option<u32> = Some(96);
                }
                impl ::canadensis_encoding::Request for AppendEntriesRequest {}
                impl ::canadensis_encoding::Serialize for AppendEntriesRequest {
                    fn size_bits(&self) -> usize {
                        32 + 32
                            + 16
                            + 16
                            + 8
                            + (self.entries).iter().map(|element| 176).sum::<usize>()
                            + 0
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

                /// `uavcan.pnp.cluster.AppendEntries.1.0`
                /// Fixed size 5 bytes
                pub struct AppendEntriesResponse {
                    // saturated uint32
                    // Always aligned
                    pub term: u32,
                    // saturated bool
                    // Always aligned
                    pub success: bool,
                }
                impl ::canadensis_encoding::DataType for AppendEntriesResponse {
                    const EXTENT_BYTES: Option<u32> = Some(48);
                }
                impl ::canadensis_encoding::Response for AppendEntriesResponse {}
                impl ::canadensis_encoding::Serialize for AppendEntriesResponse {
                    fn size_bits(&self) -> usize {
                        32 + 1 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u32(self.term);
                        cursor.write_bool(self.success);
                    }
                }
            }
            pub mod discovery_1_0 {
                /// `uavcan.pnp.cluster.Discovery.1.0`
                /// Size ranges from 2 to 12 bytes
                pub struct Discovery {
                    // saturated uint3
                    // Always aligned
                    pub configured_cluster_size: u8,
                    // uavcan.node.ID.1.0[<=5]
                    // Always aligned
                    pub known_nodes: ::heapless::Vec<crate::uavcan::node::id_1_0::ID, 5>,
                }
                impl ::canadensis_encoding::DataType for Discovery {
                    const EXTENT_BYTES: Option<u32> = Some(96);
                }
                impl ::canadensis_encoding::Message for Discovery {}
                impl ::canadensis_encoding::Serialize for Discovery {
                    fn size_bits(&self) -> usize {
                        3 + 8 + (self.known_nodes).iter().map(|element| 16).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_u3(self.configured_cluster_size);
                        cursor.write_aligned_u8((self.known_nodes).len() as u8);
                        for value in (self.known_nodes).iter() {
                            cursor.write_composite(value);
                        }
                    }
                }
            }
            pub mod entry_1_0 {
                /// `uavcan.pnp.cluster.Entry.1.0`
                /// Fixed size 22 bytes
                pub struct Entry {
                    // saturated uint32
                    // Always aligned
                    pub term: u32,
                    // saturated uint8[16]
                    // Always aligned
                    pub unique_id: [u8; 16],
                    // uavcan.node.ID.1.0
                    // Always aligned
                    pub node_id: crate::uavcan::node::id_1_0::ID,
                }
                impl ::canadensis_encoding::DataType for Entry {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Entry {}
                impl ::canadensis_encoding::Serialize for Entry {
                    fn size_bits(&self) -> usize {
                        32 + (self.unique_id).iter().map(|element| 8).sum::<usize>() + 16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u32(self.term);
                        cursor.write_bytes(&(self.unique_id)[..]);
                        cursor.write_composite(&self.node_id);
                    }
                }
            }
            pub mod request_vote_1_0 {
                /// `uavcan.pnp.cluster.RequestVote.1.0`
                /// Fixed size 10 bytes
                pub struct RequestVoteRequest {
                    // saturated uint32
                    // Always aligned
                    pub term: u32,
                    // saturated uint32
                    // Always aligned
                    pub last_log_term: u32,
                    // saturated uint16
                    // Always aligned
                    pub last_log_index: u16,
                }
                impl ::canadensis_encoding::DataType for RequestVoteRequest {
                    const EXTENT_BYTES: Option<u32> = Some(48);
                }
                impl ::canadensis_encoding::Request for RequestVoteRequest {}
                impl ::canadensis_encoding::Serialize for RequestVoteRequest {
                    fn size_bits(&self) -> usize {
                        32 + 32 + 16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u32(self.term);
                        cursor.write_aligned_u32(self.last_log_term);
                        cursor.write_aligned_u16(self.last_log_index);
                    }
                }

                /// `uavcan.pnp.cluster.RequestVote.1.0`
                /// Fixed size 5 bytes
                pub struct RequestVoteResponse {
                    // saturated uint32
                    // Always aligned
                    pub term: u32,
                    // saturated bool
                    // Always aligned
                    pub vote_granted: bool,
                }
                impl ::canadensis_encoding::DataType for RequestVoteResponse {
                    const EXTENT_BYTES: Option<u32> = Some(48);
                }
                impl ::canadensis_encoding::Response for RequestVoteResponse {}
                impl ::canadensis_encoding::Serialize for RequestVoteResponse {
                    fn size_bits(&self) -> usize {
                        32 + 1 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u32(self.term);
                        cursor.write_bool(self.vote_granted);
                    }
                }
            }
        }
        pub mod node_id_allocation_data_1_0 {
            /// `uavcan.pnp.NodeIDAllocationData.1.0`
            /// Size ranges from 7 to 9 bytes
            pub struct NodeIDAllocationData {
                // truncated uint48
                // Always aligned
                pub unique_id_hash: u64,
                // uavcan.node.ID.1.0[<=1]
                // Always aligned
                pub allocated_node_id: ::heapless::Vec<crate::uavcan::node::id_1_0::ID, 1>,
            }
            impl ::canadensis_encoding::DataType for NodeIDAllocationData {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for NodeIDAllocationData {}
            impl ::canadensis_encoding::Serialize for NodeIDAllocationData {
                fn size_bits(&self) -> usize {
                    48 + 8
                        + (self.allocated_node_id)
                            .iter()
                            .map(|element| 16)
                            .sum::<usize>()
                        + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u48(self.unique_id_hash);
                    cursor.write_aligned_u8((self.allocated_node_id).len() as u8);
                    for value in (self.allocated_node_id).iter() {
                        cursor.write_composite(value);
                    }
                }
            }
        }
        pub mod node_id_allocation_data_2_0 {
            /// `uavcan.pnp.NodeIDAllocationData.2.0`
            /// Fixed size 18 bytes
            pub struct NodeIDAllocationData {
                // uavcan.node.ID.1.0
                // Always aligned
                pub node_id: crate::uavcan::node::id_1_0::ID,
                // saturated uint8[16]
                // Always aligned
                pub unique_id: [u8; 16],
            }
            impl ::canadensis_encoding::DataType for NodeIDAllocationData {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Message for NodeIDAllocationData {}
            impl ::canadensis_encoding::Serialize for NodeIDAllocationData {
                fn size_bits(&self) -> usize {
                    16 + (self.unique_id).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.node_id);
                    cursor.write_bytes(&(self.unique_id)[..]);
                }
            }
        }
    }
    pub mod primitive {
        pub mod array {
            pub mod bit_1_0 {
                /// `uavcan.primitive.array.Bit.1.0`
                /// Size ranges from 2 to 258 bytes
                pub struct Bit {
                    // saturated bool[<=2048]
                    // Always aligned
                    pub value: ::heapless::Vec<bool, 2048>,
                }
                impl ::canadensis_encoding::DataType for Bit {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Bit {}
                impl ::canadensis_encoding::Serialize for Bit {
                    fn size_bits(&self) -> usize {
                        16 + (self.value).iter().map(|element| 1).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16((self.value).len() as u16);
                        for value in (self.value).iter() {
                            cursor.write_bool(*value);
                        }
                    }
                }
            }
            pub mod integer16_1_0 {
                /// `uavcan.primitive.array.Integer16.1.0`
                /// Size ranges from 1 to 257 bytes
                pub struct Integer16 {
                    // saturated int16[<=128]
                    // Always aligned
                    pub value: ::heapless::Vec<i16, 128>,
                }
                impl ::canadensis_encoding::DataType for Integer16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer16 {}
                impl ::canadensis_encoding::Serialize for Integer16 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).iter().map(|element| 16).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u16(*value as u16);
                        }
                    }
                }
            }
            pub mod integer32_1_0 {
                /// `uavcan.primitive.array.Integer32.1.0`
                /// Size ranges from 1 to 257 bytes
                pub struct Integer32 {
                    // saturated int32[<=64]
                    // Always aligned
                    pub value: ::heapless::Vec<i32, 64>,
                }
                impl ::canadensis_encoding::DataType for Integer32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer32 {}
                impl ::canadensis_encoding::Serialize for Integer32 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).iter().map(|element| 32).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u32(*value as u32);
                        }
                    }
                }
            }
            pub mod integer64_1_0 {
                /// `uavcan.primitive.array.Integer64.1.0`
                /// Size ranges from 1 to 257 bytes
                pub struct Integer64 {
                    // saturated int64[<=32]
                    // Always aligned
                    pub value: ::heapless::Vec<i64, 32>,
                }
                impl ::canadensis_encoding::DataType for Integer64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer64 {}
                impl ::canadensis_encoding::Serialize for Integer64 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).iter().map(|element| 64).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u64(*value as u64);
                        }
                    }
                }
            }
            pub mod integer8_1_0 {
                /// `uavcan.primitive.array.Integer8.1.0`
                /// Size ranges from 2 to 258 bytes
                pub struct Integer8 {
                    // saturated int8[<=256]
                    // Always aligned
                    pub value: ::heapless::Vec<i8, 256>,
                }
                impl ::canadensis_encoding::DataType for Integer8 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer8 {}
                impl ::canadensis_encoding::Serialize for Integer8 {
                    fn size_bits(&self) -> usize {
                        16 + (self.value).iter().map(|element| 8).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16((self.value).len() as u16);
                        for value in (self.value).iter() {
                            cursor.write_u8(*value as u8);
                        }
                    }
                }
            }
            pub mod natural16_1_0 {
                /// `uavcan.primitive.array.Natural16.1.0`
                /// Size ranges from 1 to 257 bytes
                pub struct Natural16 {
                    // saturated uint16[<=128]
                    // Always aligned
                    pub value: ::heapless::Vec<u16, 128>,
                }
                impl ::canadensis_encoding::DataType for Natural16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural16 {}
                impl ::canadensis_encoding::Serialize for Natural16 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).iter().map(|element| 16).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u16(*value);
                        }
                    }
                }
            }
            pub mod natural32_1_0 {
                /// `uavcan.primitive.array.Natural32.1.0`
                /// Size ranges from 1 to 257 bytes
                pub struct Natural32 {
                    // saturated uint32[<=64]
                    // Always aligned
                    pub value: ::heapless::Vec<u32, 64>,
                }
                impl ::canadensis_encoding::DataType for Natural32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural32 {}
                impl ::canadensis_encoding::Serialize for Natural32 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).iter().map(|element| 32).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u32(*value);
                        }
                    }
                }
            }
            pub mod natural64_1_0 {
                /// `uavcan.primitive.array.Natural64.1.0`
                /// Size ranges from 1 to 257 bytes
                pub struct Natural64 {
                    // saturated uint64[<=32]
                    // Always aligned
                    pub value: ::heapless::Vec<u64, 32>,
                }
                impl ::canadensis_encoding::DataType for Natural64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural64 {}
                impl ::canadensis_encoding::Serialize for Natural64 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).iter().map(|element| 64).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_u64(*value);
                        }
                    }
                }
            }
            pub mod natural8_1_0 {
                /// `uavcan.primitive.array.Natural8.1.0`
                /// Size ranges from 2 to 258 bytes
                pub struct Natural8 {
                    // saturated uint8[<=256]
                    // Always aligned
                    pub value: ::heapless::Vec<u8, 256>,
                }
                impl ::canadensis_encoding::DataType for Natural8 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural8 {}
                impl ::canadensis_encoding::Serialize for Natural8 {
                    fn size_bits(&self) -> usize {
                        16 + (self.value).iter().map(|element| 8).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16((self.value).len() as u16);
                        cursor.write_bytes(&(self.value)[..]);
                    }
                }
            }
            pub mod real16_1_0 {
                /// `uavcan.primitive.array.Real16.1.0`
                /// Size ranges from 1 to 257 bytes
                pub struct Real16 {
                    // saturated float16[<=128]
                    // Always aligned
                    pub value: ::heapless::Vec<::half::f16, 128>,
                }
                impl ::canadensis_encoding::DataType for Real16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real16 {}
                impl ::canadensis_encoding::Serialize for Real16 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).iter().map(|element| 16).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_f16(*value);
                        }
                    }
                }
            }
            pub mod real32_1_0 {
                /// `uavcan.primitive.array.Real32.1.0`
                /// Size ranges from 1 to 257 bytes
                pub struct Real32 {
                    // saturated float32[<=64]
                    // Always aligned
                    pub value: ::heapless::Vec<f32, 64>,
                }
                impl ::canadensis_encoding::DataType for Real32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real32 {}
                impl ::canadensis_encoding::Serialize for Real32 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).iter().map(|element| 32).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_f32(*value);
                        }
                    }
                }
            }
            pub mod real64_1_0 {
                /// `uavcan.primitive.array.Real64.1.0`
                /// Size ranges from 1 to 257 bytes
                pub struct Real64 {
                    // saturated float64[<=32]
                    // Always aligned
                    pub value: ::heapless::Vec<f64, 32>,
                }
                impl ::canadensis_encoding::DataType for Real64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real64 {}
                impl ::canadensis_encoding::Serialize for Real64 {
                    fn size_bits(&self) -> usize {
                        8 + (self.value).iter().map(|element| 64).sum::<usize>() + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8((self.value).len() as u8);
                        for value in (self.value).iter() {
                            cursor.write_f64(*value);
                        }
                    }
                }
            }
        }
        pub mod empty_1_0 {
            /// `uavcan.primitive.Empty.1.0`
            /// Fixed size 0 bytes
            pub struct Empty {}
            impl ::canadensis_encoding::DataType for Empty {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Empty {}
            impl ::canadensis_encoding::Serialize for Empty {
                fn size_bits(&self) -> usize {
                    0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
            }
        }
        pub mod scalar {
            pub mod bit_1_0 {
                /// `uavcan.primitive.scalar.Bit.1.0`
                /// Fixed size 1 bytes
                pub struct Bit {
                    // saturated bool
                    // Always aligned
                    pub value: bool,
                }
                impl ::canadensis_encoding::DataType for Bit {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Bit {}
                impl ::canadensis_encoding::Serialize for Bit {
                    fn size_bits(&self) -> usize {
                        1 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_bool(self.value);
                    }
                }
            }
            pub mod integer16_1_0 {
                /// `uavcan.primitive.scalar.Integer16.1.0`
                /// Fixed size 2 bytes
                pub struct Integer16 {
                    // saturated int16
                    // Always aligned
                    pub value: i16,
                }
                impl ::canadensis_encoding::DataType for Integer16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer16 {}
                impl ::canadensis_encoding::Serialize for Integer16 {
                    fn size_bits(&self) -> usize {
                        16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.value as u16);
                    }
                }
            }
            pub mod integer32_1_0 {
                /// `uavcan.primitive.scalar.Integer32.1.0`
                /// Fixed size 4 bytes
                pub struct Integer32 {
                    // saturated int32
                    // Always aligned
                    pub value: i32,
                }
                impl ::canadensis_encoding::DataType for Integer32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer32 {}
                impl ::canadensis_encoding::Serialize for Integer32 {
                    fn size_bits(&self) -> usize {
                        32 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u32(self.value as u32);
                    }
                }
            }
            pub mod integer64_1_0 {
                /// `uavcan.primitive.scalar.Integer64.1.0`
                /// Fixed size 8 bytes
                pub struct Integer64 {
                    // saturated int64
                    // Always aligned
                    pub value: i64,
                }
                impl ::canadensis_encoding::DataType for Integer64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer64 {}
                impl ::canadensis_encoding::Serialize for Integer64 {
                    fn size_bits(&self) -> usize {
                        64 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u64(self.value as u64);
                    }
                }
            }
            pub mod integer8_1_0 {
                /// `uavcan.primitive.scalar.Integer8.1.0`
                /// Fixed size 1 bytes
                pub struct Integer8 {
                    // saturated int8
                    // Always aligned
                    pub value: i8,
                }
                impl ::canadensis_encoding::DataType for Integer8 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Integer8 {}
                impl ::canadensis_encoding::Serialize for Integer8 {
                    fn size_bits(&self) -> usize {
                        8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8(self.value as u8);
                    }
                }
            }
            pub mod natural16_1_0 {
                /// `uavcan.primitive.scalar.Natural16.1.0`
                /// Fixed size 2 bytes
                pub struct Natural16 {
                    // saturated uint16
                    // Always aligned
                    pub value: u16,
                }
                impl ::canadensis_encoding::DataType for Natural16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural16 {}
                impl ::canadensis_encoding::Serialize for Natural16 {
                    fn size_bits(&self) -> usize {
                        16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u16(self.value);
                    }
                }
            }
            pub mod natural32_1_0 {
                /// `uavcan.primitive.scalar.Natural32.1.0`
                /// Fixed size 4 bytes
                pub struct Natural32 {
                    // saturated uint32
                    // Always aligned
                    pub value: u32,
                }
                impl ::canadensis_encoding::DataType for Natural32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural32 {}
                impl ::canadensis_encoding::Serialize for Natural32 {
                    fn size_bits(&self) -> usize {
                        32 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u32(self.value);
                    }
                }
            }
            pub mod natural64_1_0 {
                /// `uavcan.primitive.scalar.Natural64.1.0`
                /// Fixed size 8 bytes
                pub struct Natural64 {
                    // saturated uint64
                    // Always aligned
                    pub value: u64,
                }
                impl ::canadensis_encoding::DataType for Natural64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural64 {}
                impl ::canadensis_encoding::Serialize for Natural64 {
                    fn size_bits(&self) -> usize {
                        64 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u64(self.value);
                    }
                }
            }
            pub mod natural8_1_0 {
                /// `uavcan.primitive.scalar.Natural8.1.0`
                /// Fixed size 1 bytes
                pub struct Natural8 {
                    // saturated uint8
                    // Always aligned
                    pub value: u8,
                }
                impl ::canadensis_encoding::DataType for Natural8 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Natural8 {}
                impl ::canadensis_encoding::Serialize for Natural8 {
                    fn size_bits(&self) -> usize {
                        8 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_aligned_u8(self.value);
                    }
                }
            }
            pub mod real16_1_0 {
                /// `uavcan.primitive.scalar.Real16.1.0`
                /// Fixed size 2 bytes
                pub struct Real16 {
                    // saturated float16
                    // Always aligned
                    pub value: ::half::f16,
                }
                impl ::canadensis_encoding::DataType for Real16 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real16 {}
                impl ::canadensis_encoding::Serialize for Real16 {
                    fn size_bits(&self) -> usize {
                        16 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_f16(self.value);
                    }
                }
            }
            pub mod real32_1_0 {
                /// `uavcan.primitive.scalar.Real32.1.0`
                /// Fixed size 4 bytes
                pub struct Real32 {
                    // saturated float32
                    // Always aligned
                    pub value: f32,
                }
                impl ::canadensis_encoding::DataType for Real32 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real32 {}
                impl ::canadensis_encoding::Serialize for Real32 {
                    fn size_bits(&self) -> usize {
                        32 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_f32(self.value);
                    }
                }
            }
            pub mod real64_1_0 {
                /// `uavcan.primitive.scalar.Real64.1.0`
                /// Fixed size 8 bytes
                pub struct Real64 {
                    // saturated float64
                    // Always aligned
                    pub value: f64,
                }
                impl ::canadensis_encoding::DataType for Real64 {
                    const EXTENT_BYTES: Option<u32> = None;
                }
                impl ::canadensis_encoding::Message for Real64 {}
                impl ::canadensis_encoding::Serialize for Real64 {
                    fn size_bits(&self) -> usize {
                        64 + 0
                    }
                    fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                        cursor.write_f64(self.value);
                    }
                }
            }
        }
        pub mod string_1_0 {
            /// `uavcan.primitive.String.1.0`
            /// Size ranges from 2 to 258 bytes
            pub struct String {
                // saturated uint8[<=256]
                // Always aligned
                pub value: ::heapless::Vec<u8, 256>,
            }
            impl ::canadensis_encoding::DataType for String {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for String {}
            impl ::canadensis_encoding::Serialize for String {
                fn size_bits(&self) -> usize {
                    16 + (self.value).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16((self.value).len() as u16);
                    cursor.write_bytes(&(self.value)[..]);
                }
            }
        }
        pub mod unstructured_1_0 {
            /// `uavcan.primitive.Unstructured.1.0`
            /// Size ranges from 2 to 258 bytes
            pub struct Unstructured {
                // saturated uint8[<=256]
                // Always aligned
                pub value: ::heapless::Vec<u8, 256>,
            }
            impl ::canadensis_encoding::DataType for Unstructured {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Unstructured {}
            impl ::canadensis_encoding::Serialize for Unstructured {
                fn size_bits(&self) -> usize {
                    16 + (self.value).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16((self.value).len() as u16);
                    cursor.write_bytes(&(self.value)[..]);
                }
            }
        }
    }
    pub mod register {
        pub mod access_1_0 {
            /// `uavcan.register.Access.1.0`
            /// Size ranges from 2 to 515 bytes
            pub struct AccessRequest {
                // uavcan.register.Name.1.0
                // Always aligned
                pub name: crate::uavcan::register::name_1_0::Name,
                // uavcan.register.Value.1.0
                // Always aligned
                pub value: crate::uavcan::register::value_1_0::Value,
            }
            impl ::canadensis_encoding::DataType for AccessRequest {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Request for AccessRequest {}
            impl ::canadensis_encoding::Serialize for AccessRequest {
                fn size_bits(&self) -> usize {
                    (self.name).size_bits() + (self.value).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.name);
                    cursor.write_composite(&self.value);
                }
            }

            /// `uavcan.register.Access.1.0`
            /// Size ranges from 9 to 267 bytes
            pub struct AccessResponse {
                // uavcan.time.SynchronizedTimestamp.1.0
                // Always aligned
                pub timestamp:
                    crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                // saturated bool
                // Always aligned
                pub mutable: bool,
                // saturated bool
                // Not always aligned
                pub persistent: bool,
                // uavcan.register.Value.1.0
                // Always aligned
                pub value: crate::uavcan::register::value_1_0::Value,
            }
            impl ::canadensis_encoding::DataType for AccessResponse {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Response for AccessResponse {}
            impl ::canadensis_encoding::Serialize for AccessResponse {
                fn size_bits(&self) -> usize {
                    56 + 1 + 1 + (self.value).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.timestamp);
                    cursor.write_bool(self.mutable);
                    cursor.write_bool(self.persistent);
                    cursor.write_composite(&self.value);
                }
            }
        }
        pub mod list_1_0 {
            /// `uavcan.register.List.1.0`
            /// Fixed size 2 bytes
            pub struct ListRequest {
                // saturated uint16
                // Always aligned
                pub index: u16,
            }
            impl ::canadensis_encoding::DataType for ListRequest {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Request for ListRequest {}
            impl ::canadensis_encoding::Serialize for ListRequest {
                fn size_bits(&self) -> usize {
                    16 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u16(self.index);
                }
            }

            /// `uavcan.register.List.1.0`
            /// Size ranges from 1 to 256 bytes
            pub struct ListResponse {
                // uavcan.register.Name.1.0
                // Always aligned
                pub name: crate::uavcan::register::name_1_0::Name,
            }
            impl ::canadensis_encoding::DataType for ListResponse {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Response for ListResponse {}
            impl ::canadensis_encoding::Serialize for ListResponse {
                fn size_bits(&self) -> usize {
                    (self.name).size_bits() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.name);
                }
            }
        }
        pub mod name_1_0 {
            /// `uavcan.register.Name.1.0`
            /// Size ranges from 1 to 256 bytes
            pub struct Name {
                // saturated uint8[<=255]
                // Always aligned
                pub name: ::heapless::Vec<u8, 255>,
            }
            impl ::canadensis_encoding::DataType for Name {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Name {}
            impl ::canadensis_encoding::Serialize for Name {
                fn size_bits(&self) -> usize {
                    8 + (self.name).iter().map(|element| 8).sum::<usize>() + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_aligned_u8((self.name).len() as u8);
                    cursor.write_bytes(&(self.name)[..]);
                }
            }
        }
        pub mod value_1_0 {
            /// `uavcan.register.Value.1.0`
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
        }
    }
    pub mod si {
        pub mod sample {
            pub mod acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.acceleration.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub meter_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.meter_per_second_per_second);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.acceleration.Vector3.1.0`
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub meter_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            56 + (self.meter_per_second_per_second)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.meter_per_second_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod angle {
                pub mod quaternion_1_0 {
                    /// `uavcan.si.sample.angle.Quaternion.1.0`
                    /// Fixed size 23 bytes
                    pub struct Quaternion {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[4]
                        // Always aligned
                        pub wxyz: [f32; 4],
                    }
                    impl ::canadensis_encoding::DataType for Quaternion {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Quaternion {}
                    impl ::canadensis_encoding::Serialize for Quaternion {
                        fn size_bits(&self) -> usize {
                            56 + (self.wxyz).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.wxyz).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.angle.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub radian: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.radian);
                        }
                    }
                }
            }
            pub mod angular_acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.angular_acceleration.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub radian_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.radian_per_second_per_second);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.angular_acceleration.Vector3.1.0`
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub radian_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            56 + (self.radian_per_second_per_second)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.radian_per_second_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod angular_velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.angular_velocity.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub radian_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.radian_per_second);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.angular_velocity.Vector3.1.0`
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub radian_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            56 + (self.radian_per_second)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.radian_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod duration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.duration.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.second);
                        }
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.sample.duration.WideScalar.1.0`
                    /// Fixed size 15 bytes
                    pub struct WideScalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float64
                        // Always aligned
                        pub second: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideScalar {}
                    impl ::canadensis_encoding::Serialize for WideScalar {
                        fn size_bits(&self) -> usize {
                            56 + 64 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f64(self.second);
                        }
                    }
                }
            }
            pub mod electric_charge {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.electric_charge.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub coulomb: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.coulomb);
                        }
                    }
                }
            }
            pub mod electric_current {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.electric_current.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub ampere: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.ampere);
                        }
                    }
                }
            }
            pub mod energy {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.energy.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub joule: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.joule);
                        }
                    }
                }
            }
            pub mod force {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.force.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub newton: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.newton);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.force.Vector3.1.0`
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub newton: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            56 + (self.newton).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.newton).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod frequency {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.frequency.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub hertz: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.hertz);
                        }
                    }
                }
            }
            pub mod length {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.length.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.meter);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.length.Vector3.1.0`
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            56 + (self.meter).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.meter).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.sample.length.WideScalar.1.0`
                    /// Fixed size 15 bytes
                    pub struct WideScalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float64
                        // Always aligned
                        pub meter: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideScalar {}
                    impl ::canadensis_encoding::Serialize for WideScalar {
                        fn size_bits(&self) -> usize {
                            56 + 64 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f64(self.meter);
                        }
                    }
                }
                pub mod wide_vector3_1_0 {
                    /// `uavcan.si.sample.length.WideVector3.1.0`
                    /// Fixed size 31 bytes
                    pub struct WideVector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float64[3]
                        // Always aligned
                        pub meter: [f64; 3],
                    }
                    impl ::canadensis_encoding::DataType for WideVector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideVector3 {}
                    impl ::canadensis_encoding::Serialize for WideVector3 {
                        fn size_bits(&self) -> usize {
                            56 + (self.meter).iter().map(|element| 64).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.meter).iter() {
                                cursor.write_f64(*value);
                            }
                        }
                    }
                }
            }
            pub mod magnetic_field_strength {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.magnetic_field_strength.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub tesla: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.tesla);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.magnetic_field_strength.Vector3.1.0`
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub tesla: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            56 + (self.tesla).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.tesla).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod mass {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.mass.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub kilogram: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.kilogram);
                        }
                    }
                }
            }
            pub mod power {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.power.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub watt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.watt);
                        }
                    }
                }
            }
            pub mod pressure {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.pressure.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub pascal: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.pascal);
                        }
                    }
                }
            }
            pub mod temperature {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.temperature.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub kelvin: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.kelvin);
                        }
                    }
                }
            }
            pub mod torque {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.torque.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub newton_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.newton_meter);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.torque.Vector3.1.0`
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub newton_meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            56 + (self.newton_meter).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.newton_meter).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.velocity.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.meter_per_second);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.velocity.Vector3.1.0`
                    /// Fixed size 19 bytes
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub meter_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            56 + (self.meter_per_second)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            for value in (self.meter_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod voltage {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.voltage.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub volt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.volt);
                        }
                    }
                }
            }
            pub mod volume {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.volume.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub cubic_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.cubic_meter);
                        }
                    }
                }
            }
            pub mod volumetric_flow_rate {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.volumetric_flow_rate.Scalar.1.0`
                    /// Fixed size 11 bytes
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub cubic_meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            56 + 32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_composite(&self.timestamp);
                            cursor.write_f32(self.cubic_meter_per_second);
                        }
                    }
                }
            }
        }
        pub mod unit {
            pub mod acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.acceleration.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub meter_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.meter_per_second_per_second);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.acceleration.Vector3.1.0`
                    /// Fixed size 12 bytes
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub meter_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            (self.meter_per_second_per_second)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.meter_per_second_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod angle {
                pub mod quaternion_1_0 {
                    /// `uavcan.si.unit.angle.Quaternion.1.0`
                    /// Fixed size 16 bytes
                    pub struct Quaternion {
                        // saturated float32[4]
                        // Always aligned
                        pub wxyz: [f32; 4],
                    }
                    impl ::canadensis_encoding::DataType for Quaternion {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Quaternion {}
                    impl ::canadensis_encoding::Serialize for Quaternion {
                        fn size_bits(&self) -> usize {
                            (self.wxyz).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.wxyz).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.angle.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub radian: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.radian);
                        }
                    }
                }
            }
            pub mod angular_acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.angular_acceleration.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub radian_per_second_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.radian_per_second_per_second);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.angular_acceleration.Vector3.1.0`
                    /// Fixed size 12 bytes
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub radian_per_second_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            (self.radian_per_second_per_second)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.radian_per_second_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod angular_velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.angular_velocity.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub radian_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.radian_per_second);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.angular_velocity.Vector3.1.0`
                    /// Fixed size 12 bytes
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub radian_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            (self.radian_per_second)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.radian_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod duration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.duration.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.second);
                        }
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.unit.duration.WideScalar.1.0`
                    /// Fixed size 8 bytes
                    pub struct WideScalar {
                        // saturated float64
                        // Always aligned
                        pub second: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideScalar {}
                    impl ::canadensis_encoding::Serialize for WideScalar {
                        fn size_bits(&self) -> usize {
                            64 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f64(self.second);
                        }
                    }
                }
            }
            pub mod electric_charge {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub coulomb: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.coulomb);
                        }
                    }
                }
            }
            pub mod electric_current {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.electric_current.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub ampere: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.ampere);
                        }
                    }
                }
            }
            pub mod energy {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.energy.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub joule: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.joule);
                        }
                    }
                }
            }
            pub mod force {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.force.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub newton: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.newton);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.force.Vector3.1.0`
                    /// Fixed size 12 bytes
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub newton: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            (self.newton).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.newton).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod frequency {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.frequency.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub hertz: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.hertz);
                        }
                    }
                }
            }
            pub mod length {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.length.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.meter);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.length.Vector3.1.0`
                    /// Fixed size 12 bytes
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            (self.meter).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.meter).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.unit.length.WideScalar.1.0`
                    /// Fixed size 8 bytes
                    pub struct WideScalar {
                        // saturated float64
                        // Always aligned
                        pub meter: f64,
                    }
                    impl ::canadensis_encoding::DataType for WideScalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideScalar {}
                    impl ::canadensis_encoding::Serialize for WideScalar {
                        fn size_bits(&self) -> usize {
                            64 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f64(self.meter);
                        }
                    }
                }
                pub mod wide_vector3_1_0 {
                    /// `uavcan.si.unit.length.WideVector3.1.0`
                    /// Fixed size 24 bytes
                    pub struct WideVector3 {
                        // saturated float64[3]
                        // Always aligned
                        pub meter: [f64; 3],
                    }
                    impl ::canadensis_encoding::DataType for WideVector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for WideVector3 {}
                    impl ::canadensis_encoding::Serialize for WideVector3 {
                        fn size_bits(&self) -> usize {
                            (self.meter).iter().map(|element| 64).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.meter).iter() {
                                cursor.write_f64(*value);
                            }
                        }
                    }
                }
            }
            pub mod magnetic_field_strength {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.magnetic_field_strength.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub tesla: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.tesla);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.magnetic_field_strength.Vector3.1.0`
                    /// Fixed size 12 bytes
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub tesla: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            (self.tesla).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.tesla).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod mass {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.mass.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub kilogram: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.kilogram);
                        }
                    }
                }
            }
            pub mod power {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.power.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub watt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.watt);
                        }
                    }
                }
            }
            pub mod pressure {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.pressure.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub pascal: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.pascal);
                        }
                    }
                }
            }
            pub mod temperature {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.temperature.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub kelvin: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.kelvin);
                        }
                    }
                }
            }
            pub mod torque {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.torque.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub newton_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.newton_meter);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.torque.Vector3.1.0`
                    /// Fixed size 12 bytes
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub newton_meter: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            (self.newton_meter).iter().map(|element| 32).sum::<usize>() + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.newton_meter).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.velocity.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.meter_per_second);
                        }
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.velocity.Vector3.1.0`
                    /// Fixed size 12 bytes
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub meter_per_second: [f32; 3],
                    }
                    impl ::canadensis_encoding::DataType for Vector3 {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Vector3 {}
                    impl ::canadensis_encoding::Serialize for Vector3 {
                        fn size_bits(&self) -> usize {
                            (self.meter_per_second)
                                .iter()
                                .map(|element| 32)
                                .sum::<usize>()
                                + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            for value in (self.meter_per_second).iter() {
                                cursor.write_f32(*value);
                            }
                        }
                    }
                }
            }
            pub mod voltage {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.voltage.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub volt: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.volt);
                        }
                    }
                }
            }
            pub mod volume {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.volume.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub cubic_meter: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.cubic_meter);
                        }
                    }
                }
            }
            pub mod volumetric_flow_rate {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.volumetric_flow_rate.Scalar.1.0`
                    /// Fixed size 4 bytes
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub cubic_meter_per_second: f32,
                    }
                    impl ::canadensis_encoding::DataType for Scalar {
                        const EXTENT_BYTES: Option<u32> = None;
                    }
                    impl ::canadensis_encoding::Message for Scalar {}
                    impl ::canadensis_encoding::Serialize for Scalar {
                        fn size_bits(&self) -> usize {
                            32 + 0
                        }
                        fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                            cursor.write_f32(self.cubic_meter_per_second);
                        }
                    }
                }
            }
        }
    }
    pub mod time {
        pub mod get_synchronization_master_info_0_1 {
            /// `uavcan.time.GetSynchronizationMasterInfo.0.1`
            /// Fixed size 0 bytes
            pub struct GetSynchronizationMasterInfoRequest {}
            impl ::canadensis_encoding::DataType for GetSynchronizationMasterInfoRequest {
                const EXTENT_BYTES: Option<u32> = Some(48);
            }
            impl ::canadensis_encoding::Request for GetSynchronizationMasterInfoRequest {}
            impl ::canadensis_encoding::Serialize for GetSynchronizationMasterInfoRequest {
                fn size_bits(&self) -> usize {
                    0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {}
            }

            /// `uavcan.time.GetSynchronizationMasterInfo.0.1`
            /// Fixed size 7 bytes
            pub struct GetSynchronizationMasterInfoResponse {
                // saturated float32
                // Always aligned
                pub error_variance: f32,
                // uavcan.time.TimeSystem.0.1
                // Always aligned
                pub time_system: crate::uavcan::time::time_system_0_1::TimeSystem,
                // uavcan.time.TAIInfo.0.1
                // Always aligned
                pub tai_info: crate::uavcan::time::tai_info_0_1::TAIInfo,
            }
            impl ::canadensis_encoding::DataType for GetSynchronizationMasterInfoResponse {
                const EXTENT_BYTES: Option<u32> = Some(192);
            }
            impl ::canadensis_encoding::Response for GetSynchronizationMasterInfoResponse {}
            impl ::canadensis_encoding::Serialize for GetSynchronizationMasterInfoResponse {
                fn size_bits(&self) -> usize {
                    32 + 8 + 16 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_f32(self.error_variance);
                    cursor.write_composite(&self.time_system);
                    cursor.write_composite(&self.tai_info);
                }
            }
        }
        pub mod synchronization_1_0 {
            /// `uavcan.time.Synchronization.1.0`
            /// Fixed size 7 bytes
            pub struct Synchronization {
                // truncated uint56
                // Always aligned
                pub previous_transmission_timestamp_microsecond: u64,
            }
            impl ::canadensis_encoding::DataType for Synchronization {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for Synchronization {}
            impl ::canadensis_encoding::Serialize for Synchronization {
                fn size_bits(&self) -> usize {
                    56 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u56(self.previous_transmission_timestamp_microsecond);
                }
            }
        }
        pub mod synchronized_timestamp_1_0 {
            /// `uavcan.time.SynchronizedTimestamp.1.0`
            /// Fixed size 7 bytes
            pub struct SynchronizedTimestamp {
                // truncated uint56
                // Always aligned
                pub microsecond: u64,
            }
            impl ::canadensis_encoding::DataType for SynchronizedTimestamp {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for SynchronizedTimestamp {}
            impl ::canadensis_encoding::Serialize for SynchronizedTimestamp {
                fn size_bits(&self) -> usize {
                    56 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u56(self.microsecond);
                }
            }
        }
        pub mod tai_info_0_1 {
            /// `uavcan.time.TAIInfo.0.1`
            /// Fixed size 2 bytes
            pub struct TAIInfo {
                // saturated uint10
                // Always aligned
                pub difference_tai_minus_utc: u16,
            }
            impl ::canadensis_encoding::DataType for TAIInfo {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for TAIInfo {}
            impl ::canadensis_encoding::Serialize for TAIInfo {
                fn size_bits(&self) -> usize {
                    10 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u10(self.difference_tai_minus_utc);
                }
            }
        }
        pub mod time_system_0_1 {
            /// `uavcan.time.TimeSystem.0.1`
            /// Fixed size 1 bytes
            pub struct TimeSystem {
                // truncated uint4
                // Always aligned
                pub value: u8,
            }
            impl ::canadensis_encoding::DataType for TimeSystem {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for TimeSystem {}
            impl ::canadensis_encoding::Serialize for TimeSystem {
                fn size_bits(&self) -> usize {
                    4 + 0
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_u4(self.value);
                }
            }
        }
    }
}
