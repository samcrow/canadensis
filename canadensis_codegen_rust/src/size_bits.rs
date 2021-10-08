//! Generates an expression that calculates the size (in bits) of a data type

use crate::{GeneratedType, GeneratedTypeKind};
use canadensis_dsdl_frontend::compiled::Extent;
use canadensis_dsdl_frontend::types::{
    ImplicitField, PrimitiveType, ResolvedScalarType, ResolvedType,
};
use std::fmt::{Display, Formatter, Result};

pub(crate) struct SizeBitsExpr<'t>(pub &'t GeneratedType);

impl Display for SizeBitsExpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // This is similar to generating the serialize implementation, but we have to manually
        // add 32 bits for delimiter headers of delimited composite types.

        match &self.0.kind {
            GeneratedTypeKind::Struct(gstruct) => {
                for field in &gstruct.fields {
                    Display::fmt(
                        &WriteFieldSize {
                            ty: &field.uavcan_ty,
                            expr: &format!("self.{}", field.name),
                        },
                        f,
                    )?;
                    // End of field
                    write!(f, " + ")?;
                }
                // Last field, make the expression end in + 0
                write!(f, "0")?;
            }
            GeneratedTypeKind::Enum(genum) => {
                write!(f, "{} + match self {{", genum.discriminant_bits)?;

                for variant in genum.variants.iter() {
                    // Match arm (inner value is called `inner`)
                    writeln!(
                        f,
                        "{}::{}(inner) => {{",
                        self.0.name.type_name, variant.name
                    )?;
                    Display::fmt(
                        &WriteFieldSize {
                            ty: &variant.uavcan_ty,
                            expr: "inner",
                        },
                        f,
                    )?;

                    // End match arm
                    writeln!(f, "}}")?;
                }

                // End match
                write!(f, "}}")?;
            }
        }

        Ok(())
    }
}

struct WriteFieldSize<'t> {
    ty: &'t ResolvedType,
    expr: &'t str,
}

impl Display for WriteFieldSize<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.ty {
            ResolvedType::Scalar(scalar) => Display::fmt(
                &WriteScalarSize {
                    ty: scalar,
                    expr: self.expr,
                },
                f,
            )?,
            ResolvedType::FixedArray { inner, .. } => {
                Display::fmt(
                    &WriteArrayElementSizes {
                        ty: inner,
                        expr: self.expr,
                    },
                    f,
                )?;
            }
            ResolvedType::VariableArray { inner, .. } => {
                // Add something for the length
                match self.ty.implicit_field() {
                    Some(ImplicitField::ArrayLength { bits }) => {
                        write!(f, "{} +", bits)?;
                    }
                    _ => panic!("Variable-length array has no implicit length"),
                }
                // Then the elements
                Display::fmt(
                    &WriteArrayElementSizes {
                        ty: inner,
                        expr: self.expr,
                    },
                    f,
                )?;
            }
        }
        Ok(())
    }
}

struct WriteArrayElementSizes<'t> {
    ty: &'t ResolvedScalarType,
    expr: &'t str,
}

impl Display for WriteArrayElementSizes<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // In general, we need to iterate over every element and add up the
        // lengths. If this doesn't call size_bits(), hopefully the compiler
        // will optimize it.
        write!(
            f,
            "({}).iter().map(|element| {}).sum::<usize>()",
            self.expr,
            WriteScalarSize {
                ty: self.ty,
                expr: "element"
            }
        )
    }
}

/// Writes the size of a scalar type
struct WriteScalarSize<'t> {
    ty: &'t ResolvedScalarType,
    expr: &'t str,
}

impl Display for WriteScalarSize<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.ty {
            ResolvedScalarType::Composite { inner, .. } => {
                if matches!(inner.extent(), Extent::Delimited(_)) {
                    // Add 32 bits for the delimiter header
                    write!(f, "32 + ")?;
                }

                let inner_min_size = inner.bit_length().min();
                let inner_max_size = inner.bit_length().max();
                if inner_min_size == inner_max_size {
                    // Fixed-size type, use a literal
                    write!(f, "{}", inner_min_size)?;
                } else {
                    // Call size_bits() on the inner type
                    write!(f, "({}).size_bits()", self.expr)?;
                }
            }
            ResolvedScalarType::Primitive(primitive) => match primitive {
                PrimitiveType::Boolean => write!(f, "1")?,
                PrimitiveType::Int { bits } => write!(f, "{}", *bits)?,
                PrimitiveType::UInt { bits, .. } => write!(f, "{}", *bits)?,
                PrimitiveType::Float16 { .. } => write!(f, "16")?,
                PrimitiveType::Float32 { .. } => write!(f, "32")?,
                PrimitiveType::Float64 { .. } => write!(f, "64")?,
            },
            ResolvedScalarType::Void { bits } => write!(f, "{}", *bits)?,
        }
        Ok(())
    }
}
