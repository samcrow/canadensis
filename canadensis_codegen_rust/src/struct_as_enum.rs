//! This module identifies Cyphal structure types that should be generated as Rust enums
//!
//! # DSDL requirements
//!
//! A DSDL file with the comment `#[canadensis(enum)]` before the first field will become an enum
//! if all the following apply:
//! * The type is a struct (not a union)
//! * The type has only one field, and that field has an unsigned integer type
//! * If the type has any constants, each constant must have the same type as the field
//! * No two constants in the type may have the same value
//!
//! A file with `#[canadensis(enum)]` that does not meet all the above requirements will trigger
//! an error.
//!

use crate::error::EnumError;
use crate::{
    GeneratedEnum, GeneratedType, GeneratedTypeKind, GeneratedVariant, MessageRole, RustTypeName,
};
use canadensis_dsdl_frontend::compiled::{Extent, FieldKind, Message, Struct};
use canadensis_dsdl_frontend::constants::{ConstantValue, Constants};
use canadensis_dsdl_frontend::types::{PrimitiveType, ResolvedScalarType, ResolvedType};
use canadensis_dsdl_frontend::TypeKey;
use num_bigint::BigInt;
use regex::RegexBuilder;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::convert::TryInto;

pub(crate) fn has_enum_directive(comments: &str) -> bool {
    // Find [canadensis(enum)] at the beginning of a line
    let pattern = RegexBuilder::new(r"^\[canadensis\(enum\)]")
        .multi_line(true)
        .build()
        .unwrap();
    pattern.is_match(comments)
}

pub(crate) fn generate_enum_from_struct<'a>(
    key: &TypeKey,
    rust_type: &RustTypeName,
    extent: Extent,
    role: MessageRole,
    message: &'a Message,
    cyphal_struct: &Struct,
    constants: &'a Constants,
    deprecated: bool,
    comments: &'a str,
    external_packages: &BTreeMap<Vec<String>, Vec<String>>,
) -> Result<GeneratedType<'a>, EnumError> {
    let field = match cyphal_struct.fields.as_slice() {
        [field] => field,
        _ => return Err(EnumError::EnumMultipleFields),
    };
    let (discriminant_bits, discriminant_mode) = match field.kind() {
        FieldKind::Data {
            ty:
                ResolvedType::Scalar(ResolvedScalarType::Primitive(PrimitiveType::UInt { bits, mode })),
            ..
        } => {
            // OK
            (*bits, mode.to_owned())
        }
        _ => {
            return Err(EnumError::FieldType(
                field.name().unwrap_or("<unknown>").to_owned(),
            ))
        }
    };

    // Constants must have the same type as the field, and have distinct values
    let mut values_used_and_names: BTreeMap<&BigInt, &str> = BTreeMap::new();
    for (name, value) in constants {
        match value.ty() {
            PrimitiveType::UInt { bits, .. } if *bits == discriminant_bits => {
                // OK
            }
            _ => {
                return Err(EnumError::ConstantType {
                    name: name.to_owned(),
                    actual: value.ty().to_owned(),
                    expected: PrimitiveType::UInt {
                        bits: discriminant_bits,
                        mode: discriminant_mode,
                    },
                })
            }
        }
        let constant_value = match value.value() {
            ConstantValue::Int(value) => value,
            _ => unreachable!("Integer-typed consant has a value that is not an integer"),
        };
        let entry = values_used_and_names.entry(constant_value);
        match entry {
            Entry::Vacant(entry) => {
                entry.insert(name);
            }
            Entry::Occupied(entry) => {
                return Err(EnumError::ConstantValue {
                    name: name.to_owned(),
                    value: constant_value.to_owned(),
                    already_used_by: String::from(*entry.get()),
                });
            }
        }
    }

    let variants = constants
        .iter()
        .map(|(name, value)| {
            let discriminant: u32 = match value.value() {
                ConstantValue::Int(value) => value.try_into().expect("Discriminant too large"),
                _ => panic!("Value not an integer"),
            };
            GeneratedVariant::new(
                discriminant,
                None,
                name.to_owned(),
                external_packages,
                value.comments(),
            )
        })
        .collect();

    Ok(GeneratedType::new(
        key,
        rust_type.clone(),
        message.bit_length(),
        extent,
        role,
        GeneratedTypeKind::Enum(GeneratedEnum {
            discriminant_bits,
            variants,
        }),
        // Constants were all consumed to make the variants
        Constants::default(),
        deprecated,
        comments,
    ))
}
