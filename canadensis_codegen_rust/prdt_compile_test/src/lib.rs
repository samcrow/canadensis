pub mod reg {
    pub mod drone {
        pub mod physics {
            pub mod acoustics {
                pub mod note_0_1 {
                    /// `reg.drone.physics.acoustics.Note.0.1`
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
                }
            }
            pub mod dynamics {
                pub mod rotation {
                    pub mod planar_0_1 {
                        /// `reg.drone.physics.dynamics.rotation.Planar.0.1`
                        pub struct Planar {
// reg.drone.physics.kinematics.rotation.Planar.0.1
// Always aligned
pub kinematics: crate::reg::drone::physics::kinematics::rotation::planar_0_1::Planar,
// uavcan.si.unit.torque.Scalar.1.0
// Always aligned
pub torque: crate::uavcan::si::unit::torque::scalar_1_0::Scalar,
}
                    }
                    pub mod planar_ts_0_1 {
                        /// `reg.drone.physics.dynamics.rotation.PlanarTs.0.1`
                        pub struct PlanarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.dynamics.rotation.Planar.0.1
// Always aligned
pub value: crate::reg::drone::physics::dynamics::rotation::planar_0_1::Planar,
}
                    }
                }
                pub mod translation {
                    pub mod linear_0_1 {
                        /// `reg.drone.physics.dynamics.translation.Linear.0.1`
                        pub struct Linear {
// reg.drone.physics.kinematics.translation.Linear.0.1
// Always aligned
pub kinematics: crate::reg::drone::physics::kinematics::translation::linear_0_1::Linear,
// uavcan.si.unit.force.Scalar.1.0
// Always aligned
pub force: crate::uavcan::si::unit::force::scalar_1_0::Scalar,
}
                    }
                    pub mod linear_ts_0_1 {
                        /// `reg.drone.physics.dynamics.translation.LinearTs.0.1`
                        pub struct LinearTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.dynamics.translation.Linear.0.1
// Always aligned
pub value: crate::reg::drone::physics::dynamics::translation::linear_0_1::Linear,
}
                    }
                }
            }
            pub mod electricity {
                pub mod power_0_1 {
                    /// `reg.drone.physics.electricity.Power.0.1`
                    pub struct Power {
                        // uavcan.si.unit.electric_current.Scalar.1.0
                        // Always aligned
                        pub current: crate::uavcan::si::unit::electric_current::scalar_1_0::Scalar,
                        // uavcan.si.unit.voltage.Scalar.1.0
                        // Always aligned
                        pub voltage: crate::uavcan::si::unit::voltage::scalar_1_0::Scalar,
                    }
                }
                pub mod power_ts_0_1 {
                    /// `reg.drone.physics.electricity.PowerTs.0.1`
                    pub struct PowerTs {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // reg.drone.physics.electricity.Power.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::electricity::power_0_1::Power,
                    }
                }
                pub mod source_0_1 {
                    /// `reg.drone.physics.electricity.Source.0.1`
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
                }
                pub mod source_ts_0_1 {
                    /// `reg.drone.physics.electricity.SourceTs.0.1`
                    pub struct SourceTs {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // reg.drone.physics.electricity.Source.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::electricity::source_0_1::Source,
                    }
                }
            }
            pub mod kinematics {
                pub mod cartesian {
                    pub mod point_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.Point.0.1`
                        pub struct Point {
                            // uavcan.si.unit.length.WideVector3.1.0
                            // Always aligned
                            pub value:
                                crate::uavcan::si::unit::length::wide_vector3_1_0::WideVector3,
                        }
                    }
                    pub mod point_state_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointState.0.1`
                        pub struct PointState {
                            // reg.drone.physics.kinematics.cartesian.Point.0.1
                            // Always aligned
                            pub position:
                                crate::reg::drone::physics::kinematics::cartesian::point_0_1::Point,
                            // uavcan.si.unit.velocity.Vector3.1.0
                            // Always aligned
                            pub velocity: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                        }
                    }
                    pub mod point_state_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointStateVar.0.1`
                        pub struct PointStateVar {
// reg.drone.physics.kinematics.cartesian.PointVar.0.1
// Always aligned
pub position: crate::reg::drone::physics::kinematics::cartesian::point_var_0_1::PointVar,
// reg.drone.physics.kinematics.translation.Velocity3Var.0.2
// Always aligned
pub velocity: crate::reg::drone::physics::kinematics::translation::velocity3_var_0_2::Velocity3Var,
}
                    }
                    pub mod point_state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointStateVarTs.0.1`
                        pub struct PointStateVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.cartesian.PointStateVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::cartesian::point_state_var_0_1::PointStateVar,
}
                    }
                    pub mod point_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PointVar.0.1`
                        pub struct PointVar {
                            // reg.drone.physics.kinematics.cartesian.Point.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::cartesian::point_0_1::Point,
                            // saturated float16[6]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 6],
                        }
                    }
                    pub mod pose_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.Pose.0.1`
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
                    }
                    pub mod pose_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PoseVar.0.1`
                        pub struct PoseVar {
                            // reg.drone.physics.kinematics.cartesian.Pose.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::cartesian::pose_0_1::Pose,
                            // saturated float16[21]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 21],
                        }
                    }
                    pub mod pose_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.PoseVarTs.0.1`
                        pub struct PoseVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.cartesian.PoseVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::cartesian::pose_var_0_1::PoseVar,
}
                    }
                    pub mod state_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.State.0.1`
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
                    }
                    pub mod state_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.StateVar.0.1`
                        pub struct StateVar {
// reg.drone.physics.kinematics.cartesian.PoseVar.0.1
// Always aligned
pub pose: crate::reg::drone::physics::kinematics::cartesian::pose_var_0_1::PoseVar,
// reg.drone.physics.kinematics.cartesian.TwistVar.0.1
// Always aligned
pub twist: crate::reg::drone::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                    }
                    pub mod state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.StateVarTs.0.1`
                        pub struct StateVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.cartesian.StateVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::cartesian::state_var_0_1::StateVar,
}
                    }
                    pub mod twist_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.Twist.0.1`
                        pub struct Twist {
                            // uavcan.si.unit.velocity.Vector3.1.0
                            // Always aligned
                            pub linear: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                            // uavcan.si.unit.angular_velocity.Vector3.1.0
                            // Always aligned
                            pub angular:
                                crate::uavcan::si::unit::angular_velocity::vector3_1_0::Vector3,
                        }
                    }
                    pub mod twist_var_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.TwistVar.0.1`
                        pub struct TwistVar {
                            // reg.drone.physics.kinematics.cartesian.Twist.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::cartesian::twist_0_1::Twist,
                            // saturated float16[21]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 21],
                        }
                    }
                    pub mod twist_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.cartesian.TwistVarTs.0.1`
                        pub struct TwistVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.cartesian.TwistVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                    }
                }
                pub mod geodetic {
                    pub mod point_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.Point.0.1`
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
                    }
                    pub mod point_state_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointState.0.1`
                        pub struct PointState {
                            // reg.drone.physics.kinematics.geodetic.Point.0.1
                            // Always aligned
                            pub position:
                                crate::reg::drone::physics::kinematics::geodetic::point_0_1::Point,
                            // uavcan.si.unit.velocity.Vector3.1.0
                            // Always aligned
                            pub velocity: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                        }
                    }
                    pub mod point_state_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointStateVar.0.1`
                        pub struct PointStateVar {
// reg.drone.physics.kinematics.geodetic.PointVar.0.1
// Always aligned
pub position: crate::reg::drone::physics::kinematics::geodetic::point_var_0_1::PointVar,
// reg.drone.physics.kinematics.translation.Velocity3Var.0.2
// Always aligned
pub velocity: crate::reg::drone::physics::kinematics::translation::velocity3_var_0_2::Velocity3Var,
}
                    }
                    pub mod point_state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointStateVarTs.0.1`
                        pub struct PointStateVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.geodetic.PointStateVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::geodetic::point_state_var_0_1::PointStateVar,
}
                    }
                    pub mod point_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PointVar.0.1`
                        pub struct PointVar {
                            // reg.drone.physics.kinematics.geodetic.Point.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::geodetic::point_0_1::Point,
                            // saturated float16[6]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 6],
                        }
                    }
                    pub mod pose_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.Pose.0.1`
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
                    }
                    pub mod pose_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.PoseVar.0.1`
                        pub struct PoseVar {
                            // reg.drone.physics.kinematics.geodetic.Pose.0.1
                            // Always aligned
                            pub value:
                                crate::reg::drone::physics::kinematics::geodetic::pose_0_1::Pose,
                            // saturated float16[21]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 21],
                        }
                    }
                    pub mod state_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.State.0.1`
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
                    }
                    pub mod state_var_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.StateVar.0.1`
                        pub struct StateVar {
// reg.drone.physics.kinematics.geodetic.PoseVar.0.1
// Always aligned
pub pose: crate::reg::drone::physics::kinematics::geodetic::pose_var_0_1::PoseVar,
// reg.drone.physics.kinematics.cartesian.TwistVar.0.1
// Always aligned
pub twist: crate::reg::drone::physics::kinematics::cartesian::twist_var_0_1::TwistVar,
}
                    }
                    pub mod state_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.geodetic.StateVarTs.0.1`
                        pub struct StateVarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.geodetic.StateVar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::geodetic::state_var_0_1::StateVar,
}
                    }
                }
                pub mod rotation {
                    pub mod planar_0_1 {
                        /// `reg.drone.physics.kinematics.rotation.Planar.0.1`
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
                    }
                    pub mod planar_ts_0_1 {
                        /// `reg.drone.physics.kinematics.rotation.PlanarTs.0.1`
                        pub struct PlanarTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.rotation.Planar.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::rotation::planar_0_1::Planar,
}
                    }
                }
                pub mod translation {
                    pub mod linear_0_1 {
                        /// `reg.drone.physics.kinematics.translation.Linear.0.1`
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
                    }
                    pub mod linear_ts_0_1 {
                        /// `reg.drone.physics.kinematics.translation.LinearTs.0.1`
                        pub struct LinearTs {
// uavcan.time.SynchronizedTimestamp.1.0
// Always aligned
pub timestamp: crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
// reg.drone.physics.kinematics.translation.Linear.0.1
// Always aligned
pub value: crate::reg::drone::physics::kinematics::translation::linear_0_1::Linear,
}
                    }
                    pub mod linear_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.translation.LinearVarTs.0.1`
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
                    }
                    pub mod velocity1_var_ts_0_1 {
                        /// `reg.drone.physics.kinematics.translation.Velocity1VarTs.0.1`
                        pub struct Velocity1VarTs {
                            // uavcan.si.sample.velocity.Scalar.1.0
                            // Always aligned
                            pub value: crate::uavcan::si::sample::velocity::scalar_1_0::Scalar,
                            // saturated float16
                            // Always aligned
                            pub error_variance: ::half::f16,
                        }
                    }
                    pub mod velocity3_var_0_1 {
                        /// `reg.drone.physics.kinematics.translation.Velocity3Var.0.1`
                        pub struct Velocity3Var {
                            // uavcan.si.sample.velocity.Vector3.1.0
                            // Always aligned
                            pub value: crate::uavcan::si::sample::velocity::vector3_1_0::Vector3,
                            // saturated float16[6]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 6],
                        }
                    }
                    pub mod velocity3_var_0_2 {
                        /// `reg.drone.physics.kinematics.translation.Velocity3Var.0.2`
                        pub struct Velocity3Var {
                            // uavcan.si.unit.velocity.Vector3.1.0
                            // Always aligned
                            pub value: crate::uavcan::si::unit::velocity::vector3_1_0::Vector3,
                            // saturated float16[6]
                            // Always aligned
                            pub covariance_urt: [::half::f16; 6],
                        }
                    }
                }
            }
            pub mod optics {
                pub mod high_color_0_1 {
                    /// `reg.drone.physics.optics.HighColor.0.1`
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
                }
            }
            pub mod thermodynamics {
                pub mod pressure_temp_var_ts_0_1 {
                    /// `reg.drone.physics.thermodynamics.PressureTempVarTs.0.1`
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
                }
            }
            pub mod time {
                pub mod tai64_0_1 {
                    /// `reg.drone.physics.time.TAI64.0.1`
                    pub struct TAI64 {
                        // saturated int64
                        // Always aligned
                        pub tai64n: i64,
                    }
                }
                pub mod tai64_var_0_1 {
                    /// `reg.drone.physics.time.TAI64Var.0.1`
                    pub struct TAI64Var {
                        // reg.drone.physics.time.TAI64.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::time::tai64_0_1::TAI64,
                        // saturated float32
                        // Always aligned
                        pub error_variance: f32,
                    }
                }
                pub mod tai64_var_ts_0_1 {
                    /// `reg.drone.physics.time.TAI64VarTs.0.1`
                    pub struct TAI64VarTs {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // reg.drone.physics.time.TAI64Var.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::time::tai64_var_0_1::TAI64Var,
                    }
                }
            }
        }
        pub mod service {
            pub mod actuator {
                pub mod common {
                    pub mod _0_1 {
                        /// `reg.drone.service.actuator.common._.0.1`
                        pub struct _0 {}
                    }
                    pub mod fault_flags_0_1 {
                        /// `reg.drone.service.actuator.common.FaultFlags.0.1`
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
                    }
                    pub mod feedback_0_1 {
                        /// `reg.drone.service.actuator.common.Feedback.0.1`
                        pub struct Feedback {
                            // reg.drone.service.common.Heartbeat.0.1
                            // Always aligned
                            pub heartbeat:
                                crate::reg::drone::service::common::heartbeat_0_1::Heartbeat,
                            // saturated int8
                            // Always aligned
                            pub demand_factor_pct: i8,
                        }
                    }
                    pub mod sp {
                        pub mod _0_1 {
                            /// `reg.drone.service.actuator.common.sp._.0.1`
                            pub struct _0 {}
                        }
                        pub mod scalar_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Scalar.0.1`
                            pub struct Scalar {
                                // saturated float16
                                // Always aligned
                                pub value: ::half::f16,
                            }
                        }
                        pub mod vector2_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector2.0.1`
                            pub struct Vector2 {
                                // saturated float16[2]
                                // Always aligned
                                pub value: [::half::f16; 2],
                            }
                        }
                        pub mod vector31_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector31.0.1`
                            pub struct Vector31 {
                                // saturated float16[31]
                                // Always aligned
                                pub value: [::half::f16; 31],
                            }
                        }
                        pub mod vector3_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector3.0.1`
                            pub struct Vector3 {
                                // saturated float16[3]
                                // Always aligned
                                pub value: [::half::f16; 3],
                            }
                        }
                        pub mod vector4_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector4.0.1`
                            pub struct Vector4 {
                                // saturated float16[4]
                                // Always aligned
                                pub value: [::half::f16; 4],
                            }
                        }
                        pub mod vector6_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector6.0.1`
                            pub struct Vector6 {
                                // saturated float16[6]
                                // Always aligned
                                pub value: [::half::f16; 6],
                            }
                        }
                        pub mod vector8_0_1 {
                            /// `reg.drone.service.actuator.common.sp.Vector8.0.1`
                            pub struct Vector8 {
                                // saturated float16[8]
                                // Always aligned
                                pub value: [::half::f16; 8],
                            }
                        }
                    }
                    pub mod status_0_1 {
                        /// `reg.drone.service.actuator.common.Status.0.1`
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
                    }
                }
                pub mod esc {
                    pub mod _0_1 {
                        /// `reg.drone.service.actuator.esc._.0.1`
                        pub struct _0 {}
                    }
                }
                pub mod servo {
                    pub mod _0_1 {
                        /// `reg.drone.service.actuator.servo._.0.1`
                        pub struct _0 {}
                    }
                }
            }
            pub mod air_data_computer {
                pub mod _0_1 {
                    /// `reg.drone.service.air_data_computer._.0.1`
                    pub struct _0 {}
                }
            }
            pub mod battery {
                pub mod _0_1 {
                    /// `reg.drone.service.battery._.0.1`
                    pub struct _0 {}
                }
                pub mod error_0_1 {
                    /// `reg.drone.service.battery.Error.0.1`
                    pub struct Error {
                        // saturated uint8
                        // Always aligned
                        pub value: u8,
                    }
                }
                pub mod parameters_0_1 {
                    /// `reg.drone.service.battery.Parameters.0.1`
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
                }
                pub mod parameters_0_2 {
                    /// `reg.drone.service.battery.Parameters.0.2`
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
                }
                pub mod parameters_0_3 {
                    /// `reg.drone.service.battery.Parameters.0.3`
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
                }
                pub mod status_0_1 {
                    /// `reg.drone.service.battery.Status.0.1`
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
                }
                pub mod status_0_2 {
                    /// `reg.drone.service.battery.Status.0.2`
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
                }
                pub mod technology_0_1 {
                    /// `reg.drone.service.battery.Technology.0.1`
                    pub struct Technology {
                        // saturated uint8
                        // Always aligned
                        pub value: u8,
                    }
                }
            }
            pub mod common {
                pub mod heartbeat_0_1 {
                    /// `reg.drone.service.common.Heartbeat.0.1`
                    pub struct Heartbeat {
                        // reg.drone.service.common.Readiness.0.1
                        // Always aligned
                        pub readiness: crate::reg::drone::service::common::readiness_0_1::Readiness,
                        // uavcan.node.Health.1.0
                        // Always aligned
                        pub health: crate::uavcan::node::health_1_0::Health,
                    }
                }
                pub mod readiness_0_1 {
                    /// `reg.drone.service.common.Readiness.0.1`
                    pub struct Readiness {
                        // truncated uint2
                        // Always aligned
                        pub value: u8,
                    }
                }
            }
            pub mod gnss {
                pub mod _0_1 {
                    /// `reg.drone.service.gnss._.0.1`
                    pub struct _0 {}
                }
                pub mod dilution_of_precision_0_1 {
                    /// `reg.drone.service.gnss.DilutionOfPrecision.0.1`
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
                }
                pub mod heartbeat_0_1 {
                    /// `reg.drone.service.gnss.Heartbeat.0.1`
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
                }
                pub mod sources_0_1 {
                    /// `reg.drone.service.gnss.Sources.0.1`
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
                }
                pub mod time_0_1 {
                    /// `reg.drone.service.gnss.Time.0.1`
                    pub struct Time {
                        // reg.drone.physics.time.TAI64VarTs.0.1
                        // Always aligned
                        pub value: crate::reg::drone::physics::time::tai64_var_ts_0_1::TAI64VarTs,
                        // uavcan.time.TAIInfo.0.1
                        // Always aligned
                        pub info: crate::uavcan::time::tai_info_0_1::TAIInfo,
                    }
                }
            }
            pub mod sensor {
                pub mod status_0_1 {
                    /// `reg.drone.service.sensor.Status.0.1`
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
                }
            }
        }
    }
}
pub mod uavcan {
    pub mod diagnostic {
        pub mod record_1_0 {
            /// `uavcan.diagnostic.Record.1.0`
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
        }
        pub mod record_1_1 {
            /// `uavcan.diagnostic.Record.1.1`
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
        }
        pub mod severity_1_0 {
            /// `uavcan.diagnostic.Severity.1.0`
            pub struct Severity {
                // saturated uint3
                // Always aligned
                pub value: u8,
            }
        }
    }
    pub mod file {
        pub mod error_1_0 {
            /// `uavcan.file.Error.1.0`
            pub struct Error {
                // saturated uint16
                // Always aligned
                pub value: u16,
            }
        }
        pub mod get_info_0_1 {
            /// `uavcan.file.GetInfo.0.1`
            pub struct GetInfoRequest {
                // uavcan.file.Path.1.0
                // Always aligned
                pub path: crate::uavcan::file::path_1_0::Path,
            }

            /// `uavcan.file.GetInfo.0.1`
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
        }
        pub mod get_info_0_2 {
            /// `uavcan.file.GetInfo.0.2`
            pub struct GetInfoRequest {
                // uavcan.file.Path.2.0
                // Always aligned
                pub path: crate::uavcan::file::path_2_0::Path,
            }

            /// `uavcan.file.GetInfo.0.2`
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
        }
        pub mod list_0_1 {
            /// `uavcan.file.List.0.1`
            pub struct ListRequest {
                // saturated uint32
                // Always aligned
                pub entry_index: u32,
                // uavcan.file.Path.1.0
                // Always aligned
                pub directory_path: crate::uavcan::file::path_1_0::Path,
            }

            /// `uavcan.file.List.0.1`
            pub struct ListResponse {
                // uavcan.file.Path.1.0
                // Always aligned
                pub entry_base_name: crate::uavcan::file::path_1_0::Path,
            }
        }
        pub mod list_0_2 {
            /// `uavcan.file.List.0.2`
            pub struct ListRequest {
                // saturated uint32
                // Always aligned
                pub entry_index: u32,
                // uavcan.file.Path.2.0
                // Always aligned
                pub directory_path: crate::uavcan::file::path_2_0::Path,
            }

            /// `uavcan.file.List.0.2`
            pub struct ListResponse {
                // uavcan.file.Path.2.0
                // Always aligned
                pub entry_base_name: crate::uavcan::file::path_2_0::Path,
            }
        }
        pub mod modify_1_0 {
            /// `uavcan.file.Modify.1.0`
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

            /// `uavcan.file.Modify.1.0`
            pub struct ModifyResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
            }
        }
        pub mod modify_1_1 {
            /// `uavcan.file.Modify.1.1`
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

            /// `uavcan.file.Modify.1.1`
            pub struct ModifyResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
            }
        }
        pub mod path_1_0 {
            /// `uavcan.file.Path.1.0`
            pub struct Path {
                // saturated uint8[<=112]
                // Always aligned
                pub path: ::heapless::Vec<u8, 112>,
            }
        }
        pub mod path_2_0 {
            /// `uavcan.file.Path.2.0`
            pub struct Path {
                // saturated uint8[<=255]
                // Always aligned
                pub path: ::heapless::Vec<u8, 255>,
            }
        }
        pub mod read_1_0 {
            /// `uavcan.file.Read.1.0`
            pub struct ReadRequest {
                // truncated uint40
                // Always aligned
                pub offset: u64,
                // uavcan.file.Path.1.0
                // Always aligned
                pub path: crate::uavcan::file::path_1_0::Path,
            }

            /// `uavcan.file.Read.1.0`
            pub struct ReadResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
                // saturated uint8[<=256]
                // Always aligned
                pub data: ::heapless::Vec<u8, 256>,
            }
        }
        pub mod read_1_1 {
            /// `uavcan.file.Read.1.1`
            pub struct ReadRequest {
                // truncated uint40
                // Always aligned
                pub offset: u64,
                // uavcan.file.Path.2.0
                // Always aligned
                pub path: crate::uavcan::file::path_2_0::Path,
            }

            /// `uavcan.file.Read.1.1`
            pub struct ReadResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
                // uavcan.primitive.Unstructured.1.0
                // Always aligned
                pub data: crate::uavcan::primitive::unstructured_1_0::Unstructured,
            }
        }
        pub mod write_1_0 {
            /// `uavcan.file.Write.1.0`
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

            /// `uavcan.file.Write.1.0`
            pub struct WriteResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
            }
        }
        pub mod write_1_1 {
            /// `uavcan.file.Write.1.1`
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

            /// `uavcan.file.Write.1.1`
            pub struct WriteResponse {
                // uavcan.file.Error.1.0
                // Always aligned
                pub error: crate::uavcan::file::error_1_0::Error,
            }
        }
    }
    pub mod internet {
        pub mod udp {
            pub mod handle_incoming_packet_0_1 {
                /// `uavcan.internet.udp.HandleIncomingPacket.0.1`
                pub struct HandleIncomingPacketRequest {
                    // saturated uint16
                    // Always aligned
                    pub session_id: u16,
                    // saturated uint8[<=309]
                    // Always aligned
                    pub payload: ::heapless::Vec<u8, 309>,
                }

                /// `uavcan.internet.udp.HandleIncomingPacket.0.1`
                pub struct HandleIncomingPacketResponse {}
            }
            pub mod handle_incoming_packet_0_2 {
                /// `uavcan.internet.udp.HandleIncomingPacket.0.2`
                pub struct HandleIncomingPacketRequest {
                    // saturated uint16
                    // Always aligned
                    pub session_id: u16,
                    // saturated uint8[<=508]
                    // Always aligned
                    pub payload: ::heapless::Vec<u8, 508>,
                }

                /// `uavcan.internet.udp.HandleIncomingPacket.0.2`
                pub struct HandleIncomingPacketResponse {}
            }
            pub mod outgoing_packet_0_1 {
                /// `uavcan.internet.udp.OutgoingPacket.0.1`
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
            }
            pub mod outgoing_packet_0_2 {
                /// `uavcan.internet.udp.OutgoingPacket.0.2`
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
            }
        }
    }
    pub mod metatransport {
        pub mod can {
            pub mod arbitration_id_0_1 {
                /// `uavcan.metatransport.can.ArbitrationID.0.1`
                pub enum ArbitrationID {
                    // uavcan.metatransport.can.BaseArbitrationID.0.1
Base(crate::uavcan::metatransport::can::base_arbitration_id_0_1::BaseArbitrationID),
// uavcan.metatransport.can.ExtendedArbitrationID.0.1
Extended(crate::uavcan::metatransport::can::extended_arbitration_id_0_1::ExtendedArbitrationID),
}
            }
            pub mod base_arbitration_id_0_1 {
                /// `uavcan.metatransport.can.BaseArbitrationID.0.1`
                pub struct BaseArbitrationID {
                    // truncated uint11
                    // Always aligned
                    pub value: u16,
                }
            }
            pub mod data_classic_0_1 {
                /// `uavcan.metatransport.can.DataClassic.0.1`
                pub struct DataClassic {
                    // uavcan.metatransport.can.ArbitrationID.0.1
                    // Always aligned
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                    // saturated uint8[<=8]
                    // Always aligned
                    pub data: ::heapless::Vec<u8, 8>,
                }
            }
            pub mod data_fd_0_1 {
                /// `uavcan.metatransport.can.DataFD.0.1`
                pub struct DataFD {
                    // uavcan.metatransport.can.ArbitrationID.0.1
                    // Always aligned
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                    // saturated uint8[<=64]
                    // Always aligned
                    pub data: ::heapless::Vec<u8, 64>,
                }
            }
            pub mod error_0_1 {
                /// `uavcan.metatransport.can.Error.0.1`
                pub struct Error {}
            }
            pub mod extended_arbitration_id_0_1 {
                /// `uavcan.metatransport.can.ExtendedArbitrationID.0.1`
                pub struct ExtendedArbitrationID {
                    // truncated uint29
                    // Always aligned
                    pub value: u32,
                }
            }
            pub mod frame_0_1 {
                /// `uavcan.metatransport.can.Frame.0.1`
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
            }
            pub mod frame_0_2 {
                /// `uavcan.metatransport.can.Frame.0.2`
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
            }
            pub mod manifestation_0_1 {
                /// `uavcan.metatransport.can.Manifestation.0.1`
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
            }
            pub mod rtr_0_1 {
                /// `uavcan.metatransport.can.RTR.0.1`
                pub struct RTR {
                    // uavcan.metatransport.can.ArbitrationID.0.1
                    // Always aligned
                    pub arbitration_id:
                        crate::uavcan::metatransport::can::arbitration_id_0_1::ArbitrationID,
                }
            }
        }
        pub mod ethernet {
            pub mod ether_type_0_1 {
                /// `uavcan.metatransport.ethernet.EtherType.0.1`
                pub struct EtherType {
                    // saturated uint16
                    // Always aligned
                    pub value: u16,
                }
            }
            pub mod frame_0_1 {
                /// `uavcan.metatransport.ethernet.Frame.0.1`
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
            }
        }
        pub mod serial {
            pub mod fragment_0_1 {
                /// `uavcan.metatransport.serial.Fragment.0.1`
                pub struct Fragment {
                    // uavcan.time.SynchronizedTimestamp.1.0
                    // Always aligned
                    pub timestamp:
                        crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                    // saturated uint8[<=256]
                    // Always aligned
                    pub data: ::heapless::Vec<u8, 256>,
                }
            }
            pub mod fragment_0_2 {
                /// `uavcan.metatransport.serial.Fragment.0.2`
                pub struct Fragment {
                    // saturated uint8[<=2048]
                    // Always aligned
                    pub data: ::heapless::Vec<u8, 2048>,
                }
            }
        }
        pub mod udp {
            pub mod endpoint_0_1 {
                /// `uavcan.metatransport.udp.Endpoint.0.1`
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
            }
            pub mod frame_0_1 {
                /// `uavcan.metatransport.udp.Frame.0.1`
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
            }
        }
    }
    pub mod node {
        pub mod execute_command_1_0 {
            /// `uavcan.node.ExecuteCommand.1.0`
            pub struct ExecuteCommandRequest {
                // saturated uint16
                // Always aligned
                pub command: u16,
                // saturated uint8[<=112]
                // Always aligned
                pub parameter: ::heapless::Vec<u8, 112>,
            }

            /// `uavcan.node.ExecuteCommand.1.0`
            pub struct ExecuteCommandResponse {
                // saturated uint8
                // Always aligned
                pub status: u8,
            }
        }
        pub mod execute_command_1_1 {
            /// `uavcan.node.ExecuteCommand.1.1`
            pub struct ExecuteCommandRequest {
                // saturated uint16
                // Always aligned
                pub command: u16,
                // saturated uint8[<=255]
                // Always aligned
                pub parameter: ::heapless::Vec<u8, 255>,
            }

            /// `uavcan.node.ExecuteCommand.1.1`
            pub struct ExecuteCommandResponse {
                // saturated uint8
                // Always aligned
                pub status: u8,
            }
        }
        pub mod get_info_1_0 {
            /// `uavcan.node.GetInfo.1.0`
            pub struct GetInfoRequest {}

            /// `uavcan.node.GetInfo.1.0`
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
        }
        pub mod get_transport_statistics_0_1 {
            /// `uavcan.node.GetTransportStatistics.0.1`
            pub struct GetTransportStatisticsRequest {}

            /// `uavcan.node.GetTransportStatistics.0.1`
            pub struct GetTransportStatisticsResponse {
                // uavcan.node.IOStatistics.0.1
                // Always aligned
                pub transfer_statistics: crate::uavcan::node::io_statistics_0_1::IOStatistics,
                // uavcan.node.IOStatistics.0.1[<=3]
                // Always aligned
                pub network_interface_statistics:
                    ::heapless::Vec<crate::uavcan::node::io_statistics_0_1::IOStatistics, 3>,
            }
        }
        pub mod health_1_0 {
            /// `uavcan.node.Health.1.0`
            pub struct Health {
                // saturated uint2
                // Always aligned
                pub value: u8,
            }
        }
        pub mod heartbeat_1_0 {
            /// `uavcan.node.Heartbeat.1.0`
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
        }
        pub mod id_1_0 {
            /// `uavcan.node.ID.1.0`
            pub struct ID {
                // saturated uint16
                // Always aligned
                pub value: u16,
            }
        }
        pub mod io_statistics_0_1 {
            /// `uavcan.node.IOStatistics.0.1`
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
        }
        pub mod mode_1_0 {
            /// `uavcan.node.Mode.1.0`
            pub struct Mode {
                // saturated uint3
                // Always aligned
                pub value: u8,
            }
        }
        pub mod port {
            pub mod id_1_0 {
                /// `uavcan.node.port.ID.1.0`
                pub enum ID {
                    // uavcan.node.port.SubjectID.1.0
                    SubjectId(crate::uavcan::node::port::subject_id_1_0::SubjectID),
                    // uavcan.node.port.ServiceID.1.0
                    ServiceId(crate::uavcan::node::port::service_id_1_0::ServiceID),
                }
            }
            pub mod list_0_1 {
                /// `uavcan.node.port.List.0.1`
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
            }
            pub mod service_id_1_0 {
                /// `uavcan.node.port.ServiceID.1.0`
                pub struct ServiceID {
                    // saturated uint9
                    // Always aligned
                    pub value: u16,
                }
            }
            pub mod service_id_list_0_1 {
                /// `uavcan.node.port.ServiceIDList.0.1`
                pub struct ServiceIDList {
                    // saturated bool[512]
                    // Always aligned
                    pub mask: [bool; 512],
                }
            }
            pub mod subject_id_1_0 {
                /// `uavcan.node.port.SubjectID.1.0`
                pub struct SubjectID {
                    // saturated uint13
                    // Always aligned
                    pub value: u16,
                }
            }
            pub mod subject_id_list_0_1 {
                /// `uavcan.node.port.SubjectIDList.0.1`
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
            }
        }
        pub mod version_1_0 {
            /// `uavcan.node.Version.1.0`
            pub struct Version {
                // saturated uint8
                // Always aligned
                pub major: u8,
                // saturated uint8
                // Always aligned
                pub minor: u8,
            }
        }
    }
    pub mod pnp {
        pub mod cluster {
            pub mod append_entries_1_0 {
                /// `uavcan.pnp.cluster.AppendEntries.1.0`
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

                /// `uavcan.pnp.cluster.AppendEntries.1.0`
                pub struct AppendEntriesResponse {
                    // saturated uint32
                    // Always aligned
                    pub term: u32,
                    // saturated bool
                    // Always aligned
                    pub success: bool,
                }
            }
            pub mod discovery_1_0 {
                /// `uavcan.pnp.cluster.Discovery.1.0`
                pub struct Discovery {
                    // saturated uint3
                    // Always aligned
                    pub configured_cluster_size: u8,
                    // uavcan.node.ID.1.0[<=5]
                    // Always aligned
                    pub known_nodes: ::heapless::Vec<crate::uavcan::node::id_1_0::ID, 5>,
                }
            }
            pub mod entry_1_0 {
                /// `uavcan.pnp.cluster.Entry.1.0`
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
            }
            pub mod request_vote_1_0 {
                /// `uavcan.pnp.cluster.RequestVote.1.0`
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

                /// `uavcan.pnp.cluster.RequestVote.1.0`
                pub struct RequestVoteResponse {
                    // saturated uint32
                    // Always aligned
                    pub term: u32,
                    // saturated bool
                    // Always aligned
                    pub vote_granted: bool,
                }
            }
        }
        pub mod node_id_allocation_data_1_0 {
            /// `uavcan.pnp.NodeIDAllocationData.1.0`
            pub struct NodeIDAllocationData {
                // truncated uint48
                // Always aligned
                pub unique_id_hash: u64,
                // uavcan.node.ID.1.0[<=1]
                // Always aligned
                pub allocated_node_id: ::heapless::Vec<crate::uavcan::node::id_1_0::ID, 1>,
            }
        }
        pub mod node_id_allocation_data_2_0 {
            /// `uavcan.pnp.NodeIDAllocationData.2.0`
            pub struct NodeIDAllocationData {
                // uavcan.node.ID.1.0
                // Always aligned
                pub node_id: crate::uavcan::node::id_1_0::ID,
                // saturated uint8[16]
                // Always aligned
                pub unique_id: [u8; 16],
            }
        }
    }
    pub mod primitive {
        pub mod array {
            pub mod bit_1_0 {
                /// `uavcan.primitive.array.Bit.1.0`
                pub struct Bit {
                    // saturated bool[<=2048]
                    // Always aligned
                    pub value: ::heapless::Vec<bool, 2048>,
                }
            }
            pub mod integer16_1_0 {
                /// `uavcan.primitive.array.Integer16.1.0`
                pub struct Integer16 {
                    // saturated int16[<=128]
                    // Always aligned
                    pub value: ::heapless::Vec<i16, 128>,
                }
            }
            pub mod integer32_1_0 {
                /// `uavcan.primitive.array.Integer32.1.0`
                pub struct Integer32 {
                    // saturated int32[<=64]
                    // Always aligned
                    pub value: ::heapless::Vec<i32, 64>,
                }
            }
            pub mod integer64_1_0 {
                /// `uavcan.primitive.array.Integer64.1.0`
                pub struct Integer64 {
                    // saturated int64[<=32]
                    // Always aligned
                    pub value: ::heapless::Vec<i64, 32>,
                }
            }
            pub mod integer8_1_0 {
                /// `uavcan.primitive.array.Integer8.1.0`
                pub struct Integer8 {
                    // saturated int8[<=256]
                    // Always aligned
                    pub value: ::heapless::Vec<i8, 256>,
                }
            }
            pub mod natural16_1_0 {
                /// `uavcan.primitive.array.Natural16.1.0`
                pub struct Natural16 {
                    // saturated uint16[<=128]
                    // Always aligned
                    pub value: ::heapless::Vec<u16, 128>,
                }
            }
            pub mod natural32_1_0 {
                /// `uavcan.primitive.array.Natural32.1.0`
                pub struct Natural32 {
                    // saturated uint32[<=64]
                    // Always aligned
                    pub value: ::heapless::Vec<u32, 64>,
                }
            }
            pub mod natural64_1_0 {
                /// `uavcan.primitive.array.Natural64.1.0`
                pub struct Natural64 {
                    // saturated uint64[<=32]
                    // Always aligned
                    pub value: ::heapless::Vec<u64, 32>,
                }
            }
            pub mod natural8_1_0 {
                /// `uavcan.primitive.array.Natural8.1.0`
                pub struct Natural8 {
                    // saturated uint8[<=256]
                    // Always aligned
                    pub value: ::heapless::Vec<u8, 256>,
                }
            }
            pub mod real16_1_0 {
                /// `uavcan.primitive.array.Real16.1.0`
                pub struct Real16 {
                    // saturated float16[<=128]
                    // Always aligned
                    pub value: ::heapless::Vec<::half::f16, 128>,
                }
            }
            pub mod real32_1_0 {
                /// `uavcan.primitive.array.Real32.1.0`
                pub struct Real32 {
                    // saturated float32[<=64]
                    // Always aligned
                    pub value: ::heapless::Vec<f32, 64>,
                }
            }
            pub mod real64_1_0 {
                /// `uavcan.primitive.array.Real64.1.0`
                pub struct Real64 {
                    // saturated float64[<=32]
                    // Always aligned
                    pub value: ::heapless::Vec<f64, 32>,
                }
            }
        }
        pub mod empty_1_0 {
            /// `uavcan.primitive.Empty.1.0`
            pub struct Empty {}
        }
        pub mod scalar {
            pub mod bit_1_0 {
                /// `uavcan.primitive.scalar.Bit.1.0`
                pub struct Bit {
                    // saturated bool
                    // Always aligned
                    pub value: bool,
                }
            }
            pub mod integer16_1_0 {
                /// `uavcan.primitive.scalar.Integer16.1.0`
                pub struct Integer16 {
                    // saturated int16
                    // Always aligned
                    pub value: i16,
                }
            }
            pub mod integer32_1_0 {
                /// `uavcan.primitive.scalar.Integer32.1.0`
                pub struct Integer32 {
                    // saturated int32
                    // Always aligned
                    pub value: i32,
                }
            }
            pub mod integer64_1_0 {
                /// `uavcan.primitive.scalar.Integer64.1.0`
                pub struct Integer64 {
                    // saturated int64
                    // Always aligned
                    pub value: i64,
                }
            }
            pub mod integer8_1_0 {
                /// `uavcan.primitive.scalar.Integer8.1.0`
                pub struct Integer8 {
                    // saturated int8
                    // Always aligned
                    pub value: i8,
                }
            }
            pub mod natural16_1_0 {
                /// `uavcan.primitive.scalar.Natural16.1.0`
                pub struct Natural16 {
                    // saturated uint16
                    // Always aligned
                    pub value: u16,
                }
            }
            pub mod natural32_1_0 {
                /// `uavcan.primitive.scalar.Natural32.1.0`
                pub struct Natural32 {
                    // saturated uint32
                    // Always aligned
                    pub value: u32,
                }
            }
            pub mod natural64_1_0 {
                /// `uavcan.primitive.scalar.Natural64.1.0`
                pub struct Natural64 {
                    // saturated uint64
                    // Always aligned
                    pub value: u64,
                }
            }
            pub mod natural8_1_0 {
                /// `uavcan.primitive.scalar.Natural8.1.0`
                pub struct Natural8 {
                    // saturated uint8
                    // Always aligned
                    pub value: u8,
                }
            }
            pub mod real16_1_0 {
                /// `uavcan.primitive.scalar.Real16.1.0`
                pub struct Real16 {
                    // saturated float16
                    // Always aligned
                    pub value: ::half::f16,
                }
            }
            pub mod real32_1_0 {
                /// `uavcan.primitive.scalar.Real32.1.0`
                pub struct Real32 {
                    // saturated float32
                    // Always aligned
                    pub value: f32,
                }
            }
            pub mod real64_1_0 {
                /// `uavcan.primitive.scalar.Real64.1.0`
                pub struct Real64 {
                    // saturated float64
                    // Always aligned
                    pub value: f64,
                }
            }
        }
        pub mod string_1_0 {
            /// `uavcan.primitive.String.1.0`
            pub struct String {
                // saturated uint8[<=256]
                // Always aligned
                pub value: ::heapless::Vec<u8, 256>,
            }
        }
        pub mod unstructured_1_0 {
            /// `uavcan.primitive.Unstructured.1.0`
            pub struct Unstructured {
                // saturated uint8[<=256]
                // Always aligned
                pub value: ::heapless::Vec<u8, 256>,
            }
        }
    }
    pub mod register {
        pub mod access_1_0 {
            /// `uavcan.register.Access.1.0`
            pub struct AccessRequest {
                // uavcan.register.Name.1.0
                // Always aligned
                pub name: crate::uavcan::register::name_1_0::Name,
                // uavcan.register.Value.1.0
                // Always aligned
                pub value: crate::uavcan::register::value_1_0::Value,
            }

            /// `uavcan.register.Access.1.0`
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
        }
        pub mod list_1_0 {
            /// `uavcan.register.List.1.0`
            pub struct ListRequest {
                // saturated uint16
                // Always aligned
                pub index: u16,
            }

            /// `uavcan.register.List.1.0`
            pub struct ListResponse {
                // uavcan.register.Name.1.0
                // Always aligned
                pub name: crate::uavcan::register::name_1_0::Name,
            }
        }
        pub mod name_1_0 {
            /// `uavcan.register.Name.1.0`
            pub struct Name {
                // saturated uint8[<=255]
                // Always aligned
                pub name: ::heapless::Vec<u8, 255>,
            }
        }
        pub mod value_1_0 {
            /// `uavcan.register.Value.1.0`
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
        }
    }
    pub mod si {
        pub mod sample {
            pub mod acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.acceleration.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub meter_per_second_per_second: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.acceleration.Vector3.1.0`
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub meter_per_second_per_second: [f32; 3],
                    }
                }
            }
            pub mod angle {
                pub mod quaternion_1_0 {
                    /// `uavcan.si.sample.angle.Quaternion.1.0`
                    pub struct Quaternion {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[4]
                        // Always aligned
                        pub wxyz: [f32; 4],
                    }
                }
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.angle.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub radian: f32,
                    }
                }
            }
            pub mod angular_acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.angular_acceleration.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub radian_per_second_per_second: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.angular_acceleration.Vector3.1.0`
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub radian_per_second_per_second: [f32; 3],
                    }
                }
            }
            pub mod angular_velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.angular_velocity.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub radian_per_second: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.angular_velocity.Vector3.1.0`
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub radian_per_second: [f32; 3],
                    }
                }
            }
            pub mod duration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.duration.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub second: f32,
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.sample.duration.WideScalar.1.0`
                    pub struct WideScalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float64
                        // Always aligned
                        pub second: f64,
                    }
                }
            }
            pub mod electric_charge {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.electric_charge.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub coulomb: f32,
                    }
                }
            }
            pub mod electric_current {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.electric_current.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub ampere: f32,
                    }
                }
            }
            pub mod energy {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.energy.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub joule: f32,
                    }
                }
            }
            pub mod force {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.force.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub newton: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.force.Vector3.1.0`
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub newton: [f32; 3],
                    }
                }
            }
            pub mod frequency {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.frequency.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub hertz: f32,
                    }
                }
            }
            pub mod length {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.length.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub meter: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.length.Vector3.1.0`
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub meter: [f32; 3],
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.sample.length.WideScalar.1.0`
                    pub struct WideScalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float64
                        // Always aligned
                        pub meter: f64,
                    }
                }
                pub mod wide_vector3_1_0 {
                    /// `uavcan.si.sample.length.WideVector3.1.0`
                    pub struct WideVector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float64[3]
                        // Always aligned
                        pub meter: [f64; 3],
                    }
                }
            }
            pub mod magnetic_field_strength {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.magnetic_field_strength.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub tesla: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.magnetic_field_strength.Vector3.1.0`
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub tesla: [f32; 3],
                    }
                }
            }
            pub mod mass {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.mass.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub kilogram: f32,
                    }
                }
            }
            pub mod power {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.power.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub watt: f32,
                    }
                }
            }
            pub mod pressure {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.pressure.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub pascal: f32,
                    }
                }
            }
            pub mod temperature {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.temperature.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub kelvin: f32,
                    }
                }
            }
            pub mod torque {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.torque.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub newton_meter: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.torque.Vector3.1.0`
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub newton_meter: [f32; 3],
                    }
                }
            }
            pub mod velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.velocity.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub meter_per_second: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.sample.velocity.Vector3.1.0`
                    pub struct Vector3 {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32[3]
                        // Always aligned
                        pub meter_per_second: [f32; 3],
                    }
                }
            }
            pub mod voltage {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.voltage.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub volt: f32,
                    }
                }
            }
            pub mod volume {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.volume.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub cubic_meter: f32,
                    }
                }
            }
            pub mod volumetric_flow_rate {
                pub mod scalar_1_0 {
                    /// `uavcan.si.sample.volumetric_flow_rate.Scalar.1.0`
                    pub struct Scalar {
                        // uavcan.time.SynchronizedTimestamp.1.0
                        // Always aligned
                        pub timestamp:
                            crate::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp,
                        // saturated float32
                        // Always aligned
                        pub cubic_meter_per_second: f32,
                    }
                }
            }
        }
        pub mod unit {
            pub mod acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.acceleration.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub meter_per_second_per_second: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.acceleration.Vector3.1.0`
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub meter_per_second_per_second: [f32; 3],
                    }
                }
            }
            pub mod angle {
                pub mod quaternion_1_0 {
                    /// `uavcan.si.unit.angle.Quaternion.1.0`
                    pub struct Quaternion {
                        // saturated float32[4]
                        // Always aligned
                        pub wxyz: [f32; 4],
                    }
                }
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.angle.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub radian: f32,
                    }
                }
            }
            pub mod angular_acceleration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.angular_acceleration.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub radian_per_second_per_second: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.angular_acceleration.Vector3.1.0`
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub radian_per_second_per_second: [f32; 3],
                    }
                }
            }
            pub mod angular_velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.angular_velocity.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub radian_per_second: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.angular_velocity.Vector3.1.0`
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub radian_per_second: [f32; 3],
                    }
                }
            }
            pub mod duration {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.duration.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub second: f32,
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.unit.duration.WideScalar.1.0`
                    pub struct WideScalar {
                        // saturated float64
                        // Always aligned
                        pub second: f64,
                    }
                }
            }
            pub mod electric_charge {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.electric_charge.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub coulomb: f32,
                    }
                }
            }
            pub mod electric_current {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.electric_current.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub ampere: f32,
                    }
                }
            }
            pub mod energy {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.energy.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub joule: f32,
                    }
                }
            }
            pub mod force {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.force.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub newton: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.force.Vector3.1.0`
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub newton: [f32; 3],
                    }
                }
            }
            pub mod frequency {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.frequency.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub hertz: f32,
                    }
                }
            }
            pub mod length {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.length.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub meter: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.length.Vector3.1.0`
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub meter: [f32; 3],
                    }
                }
                pub mod wide_scalar_1_0 {
                    /// `uavcan.si.unit.length.WideScalar.1.0`
                    pub struct WideScalar {
                        // saturated float64
                        // Always aligned
                        pub meter: f64,
                    }
                }
                pub mod wide_vector3_1_0 {
                    /// `uavcan.si.unit.length.WideVector3.1.0`
                    pub struct WideVector3 {
                        // saturated float64[3]
                        // Always aligned
                        pub meter: [f64; 3],
                    }
                }
            }
            pub mod magnetic_field_strength {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.magnetic_field_strength.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub tesla: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.magnetic_field_strength.Vector3.1.0`
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub tesla: [f32; 3],
                    }
                }
            }
            pub mod mass {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.mass.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub kilogram: f32,
                    }
                }
            }
            pub mod power {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.power.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub watt: f32,
                    }
                }
            }
            pub mod pressure {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.pressure.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub pascal: f32,
                    }
                }
            }
            pub mod temperature {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.temperature.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub kelvin: f32,
                    }
                }
            }
            pub mod torque {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.torque.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub newton_meter: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.torque.Vector3.1.0`
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub newton_meter: [f32; 3],
                    }
                }
            }
            pub mod velocity {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.velocity.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub meter_per_second: f32,
                    }
                }
                pub mod vector3_1_0 {
                    /// `uavcan.si.unit.velocity.Vector3.1.0`
                    pub struct Vector3 {
                        // saturated float32[3]
                        // Always aligned
                        pub meter_per_second: [f32; 3],
                    }
                }
            }
            pub mod voltage {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.voltage.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub volt: f32,
                    }
                }
            }
            pub mod volume {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.volume.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub cubic_meter: f32,
                    }
                }
            }
            pub mod volumetric_flow_rate {
                pub mod scalar_1_0 {
                    /// `uavcan.si.unit.volumetric_flow_rate.Scalar.1.0`
                    pub struct Scalar {
                        // saturated float32
                        // Always aligned
                        pub cubic_meter_per_second: f32,
                    }
                }
            }
        }
    }
    pub mod time {
        pub mod get_synchronization_master_info_0_1 {
            /// `uavcan.time.GetSynchronizationMasterInfo.0.1`
            pub struct GetSynchronizationMasterInfoRequest {}

            /// `uavcan.time.GetSynchronizationMasterInfo.0.1`
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
        }
        pub mod synchronization_1_0 {
            /// `uavcan.time.Synchronization.1.0`
            pub struct Synchronization {
                // truncated uint56
                // Always aligned
                pub previous_transmission_timestamp_microsecond: u64,
            }
        }
        pub mod synchronized_timestamp_1_0 {
            /// `uavcan.time.SynchronizedTimestamp.1.0`
            pub struct SynchronizedTimestamp {
                // truncated uint56
                // Always aligned
                pub microsecond: u64,
            }
        }
        pub mod tai_info_0_1 {
            /// `uavcan.time.TAIInfo.0.1`
            pub struct TAIInfo {
                // saturated uint10
                // Always aligned
                pub difference_tai_minus_utc: u16,
            }
        }
        pub mod time_system_0_1 {
            /// `uavcan.time.TimeSystem.0.1`
            pub struct TimeSystem {
                // truncated uint4
                // Always aligned
                pub value: u8,
            }
        }
    }
}
