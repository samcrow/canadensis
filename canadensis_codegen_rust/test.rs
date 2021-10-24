#[cfg(not(target_endian = "little"))]
compile_error!("Zero-copy serialization requires a little-endian target");
#[allow(unused_variables, unused_braces, unused_parens)]
#[deny(unaligned_references)]
pub mod canadensis {
    pub mod test {
        pub mod contains_health_0_1 {
            /// `canadensis.test.ContainsHealth.0.1`
            ///
            /// Fixed size 2 bytes
            pub struct ContainsHealth {
                /// `uavcan.node.Health.1.0`
                ///
                /// Always aligned
                /// Size 8 bits
                pub health0: ::canadensis_data_types::uavcan::node::health_1_0::Health,
                /// `uavcan.node.Health.1.0`
                ///
                /// Always aligned
                /// Size 8 bits
                pub health1: ::canadensis_data_types::uavcan::node::health_1_0::Health,
            }
            impl ::canadensis_encoding::DataType for ContainsHealth {
                const EXTENT_BYTES: Option<u32> = None;
            }
            impl ::canadensis_encoding::Message for ContainsHealth {}
            impl ContainsHealth {}
            impl ::canadensis_encoding::Serialize for ContainsHealth {
                fn size_bits(&self) -> usize {
                    16
                }
                fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {
                    cursor.write_composite(&self.health0);
                    cursor.write_composite(&self.health1);
                }
            }
            impl ::canadensis_encoding::Deserialize for ContainsHealth {
                fn deserialize(
                    cursor: &mut ::canadensis_encoding::ReadCursor<'_>,
                ) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError>
                where
                    Self: Sized,
                {
                    Ok(ContainsHealth {
                        health0: { cursor.read_composite()? },
                        health1: { cursor.read_composite()? },
                    })
                }
            }
        }
    }
}
