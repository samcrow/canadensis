//! Exposes constants declared in a DSDL file

use canadensis_dsdl_frontend::constants::ConstantValue;
use canadensis_dsdl_frontend::types::PrimitiveType;
use std::fmt::{Display, Formatter, Result};

use crate::{make_rust_identifier, round_up_integer_size, GeneratedType};

pub(crate) struct ImplementConstants<'t, 'c>(pub &'t GeneratedType<'c>);

impl Display for ImplementConstants<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "impl {} {{", self.0.name.type_name)?;
        for (name, constant) in self.0.constants {
            let name = make_rust_identifier(name.clone());
            let rust_type_name: String = match constant.ty() {
                PrimitiveType::Boolean => "bool".into(),
                PrimitiveType::Int { bits } => format!("i{}", round_up_integer_size(*bits)),
                PrimitiveType::UInt { bits, .. } => format!("u{}", round_up_integer_size(*bits)),
                PrimitiveType::Float16 { .. } => "::half::f16".into(),
                PrimitiveType::Float32 { .. } => "f32".into(),
                PrimitiveType::Float64 { .. } => "f64".into(),
            };

            if !constant.comments().is_empty() {
                writeln!(f, "#[doc = {:?}]", constant.comments())?;
            }
            writeln!(
                f,
                "pub const {}: {} = {};",
                name,
                rust_type_name,
                PrintLiteral(constant.value())
            )?
        }
        writeln!(f, "}}")
    }
}

struct PrintLiteral<'v>(&'v ConstantValue);

impl Display for PrintLiteral<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.0 {
            ConstantValue::Boolean(inner) => Display::fmt(inner, f),
            ConstantValue::Int(inner) => Display::fmt(inner, f),
            ConstantValue::Float16(inner) => {
                // Because f16 doesn't have literals, use to_bits and from_bits.
                // from_bits is a const function.
                write!(f, "::half::f16::from_bits({})", inner.to_bits())
            }
            ConstantValue::Float32(inner) => write!(f, "{}_f32", inner),
            ConstantValue::Float64(inner) => write!(f, "{}_f64", inner),
        }
    }
}
