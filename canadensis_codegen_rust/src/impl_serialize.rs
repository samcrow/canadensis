use crate::size_bits::SizeBitsExpr;
use crate::{
    round_up_integer_size, GeneratedField, GeneratedType, GeneratedTypeKind, GeneratedVariant,
};
use canadensis_dsdl_frontend::types::{
    ImplicitField, PrimitiveType, ResolvedScalarType, ResolvedType,
};
use std::fmt::{Display, Formatter, Result};

/// Implements Serialize for a type
pub(crate) struct ImplementSerialize<'t, 'c> {
    pub ty: &'t GeneratedType<'c>,
    pub zero_copy: bool,
}

impl Display for ImplementSerialize<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            "impl ::canadensis_encoding::Serialize for {} {{",
            self.ty.name.type_name
        )?;

        // Size
        writeln!(
            f,
            "fn size_bits(&self) -> usize {{ {} }}",
            SizeBitsExpr(self.ty)
        )?;

        // Serialize
        writeln!(
            f,
            "fn serialize(&self, cursor: &mut ::canadensis_encoding::WriteCursor<'_>) {{"
        )?;

        match &self.ty.kind {
            GeneratedTypeKind::Struct(gstruct) => {
                if self.zero_copy {
                    writeln!(
                        f,
                        "cursor.write_aligned_bytes(::zerocopy::AsBytes::as_bytes(self));"
                    )?;
                } else {
                    for field in &gstruct.fields {
                        Display::fmt(&SerializeField(field), f)?;
                    }
                }
            }
            GeneratedTypeKind::Enum(genum) => {
                writeln!(f, "match self {{")?;

                for (i, variant) in genum.variants.iter().enumerate() {
                    // Match arm (inner value is called `inner`)
                    writeln!(
                        f,
                        "{}::{}(inner) => {{",
                        self.ty.name.type_name, variant.name
                    )?;
                    // Write discriminant
                    writeln!(
                        f,
                        "cursor.write_aligned_u{}({});",
                        genum.discriminant_bits, i
                    )?;
                    // Write the content of this variant
                    Display::fmt(&SerializeVariant(variant), f)?;

                    // End match arm
                    writeln!(f, "}}")?;
                }

                // End match
                writeln!(f, "}}")?;
            }
        }

        // End of serialize function
        writeln!(f, "}}")?;
        // End of impl
        writeln!(f, "}}")?;
        Ok(())
    }
}

struct SerializeField<'f, 'c>(&'f GeneratedField<'c>);

impl Display for SerializeField<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.0 {
            GeneratedField::Data(data) => {
                if data.always_aligned {
                    Display::fmt(
                        &WriteAlignedField {
                            field_expr: &format!("self.{}", data.name),
                            ty: data.cyphal_ty,
                        },
                        f,
                    )
                } else {
                    Display::fmt(
                        &WriteUnalignedField {
                            field_expr: &format!("self.{}", data.name),
                            ty: data.cyphal_ty,
                        },
                        f,
                    )
                }
            }
            GeneratedField::Padding(bits) => writeln!(f, "cursor.skip_{}();", *bits),
        }
    }
}

struct SerializeVariant<'v, 'c>(&'v GeneratedVariant<'c>);

impl Display for SerializeVariant<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(
            &WriteVariant {
                field_expr: "inner",
                ty: &self.0.cyphal_ty,
            },
            f,
        )
    }
}

struct WriteVariant<'t> {
    field_expr: &'t str,
    ty: &'t ResolvedType,
}

impl<'t> Display for WriteVariant<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.ty {
            ResolvedType::Scalar(ResolvedScalarType::Composite { .. }) => {
                writeln!(f, "cursor.write_composite({});", self.field_expr)
            }
            other => Display::fmt(
                &WriteAlignedField {
                    // Add a dereference to get the primitive type value
                    field_expr: &format!("(*{})", self.field_expr),
                    ty: other,
                },
                f,
            ),
        }
    }
}

struct WriteAlignedField<'t> {
    field_expr: &'t str,
    ty: &'t ResolvedType,
}

impl<'t> Display for WriteAlignedField<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.ty {
            ResolvedType::Scalar(scalar) => match scalar {
                ResolvedScalarType::Composite { .. } => {
                    writeln!(f, "cursor.write_composite(&{});", self.field_expr)?;
                }
                ResolvedScalarType::Primitive(primitive) => match primitive {
                    PrimitiveType::Boolean => {
                        writeln!(f, "cursor.write_bool({});", self.field_expr)?
                    }
                    PrimitiveType::Int { bits } => {
                        Display::fmt(
                            &CallWriteAligned {
                                bits: *bits,
                                name: self.field_expr,
                                as_uint: true,
                            },
                            f,
                        )?;
                    }
                    PrimitiveType::UInt { bits, .. } => {
                        Display::fmt(
                            &CallWriteAligned {
                                bits: *bits,
                                name: self.field_expr,
                                as_uint: false,
                            },
                            f,
                        )?;
                    }
                    PrimitiveType::Float16 { .. } => {
                        writeln!(f, "cursor.write_f16({});", self.field_expr)?
                    }
                    PrimitiveType::Float32 { .. } => {
                        writeln!(f, "cursor.write_f32({});", self.field_expr)?
                    }
                    PrimitiveType::Float64 { .. } => {
                        writeln!(f, "cursor.write_f64({});", self.field_expr)?
                    }
                },
                ResolvedScalarType::Void { bits } => writeln!(f, "cursor.skip_{}();", *bits)?,
            },
            ResolvedType::FixedArray {
                inner: ResolvedScalarType::Primitive(PrimitiveType::Boolean),
                ..
            } => {
                // Use BitArray with no length field
                writeln!(f, "({}).serialize(cursor);", self.field_expr)?;
            }
            ResolvedType::VariableArray {
                inner: ResolvedScalarType::Primitive(PrimitiveType::Boolean),
                ..
            } => {
                // Use BitArray with a length field
                if let Some(ImplicitField::ArrayLength { bits }) = self.ty.implicit_field() {
                    // Write length and then elements
                    Display::fmt(
                        &CallWriteAligned {
                            bits,
                            name: &format!("({}).len()", self.field_expr),
                            as_uint: true,
                        },
                        f,
                    )?;
                    writeln!(f, "({}).serialize(cursor);", self.field_expr)?;
                } else {
                    unreachable!("Variable-length array does not have an implicit length field");
                }
            }
            ResolvedType::FixedArray { inner, .. } => {
                Display::fmt(
                    &WriteArrayElements {
                        element_type: inner,
                        array_expr: self.field_expr,
                    },
                    f,
                )?;
            }
            ResolvedType::VariableArray { inner, .. } => {
                if let Some(ImplicitField::ArrayLength { bits }) = self.ty.implicit_field() {
                    // Write length and then elements
                    Display::fmt(
                        &CallWriteAligned {
                            bits,
                            name: &format!("({}).len()", self.field_expr),
                            as_uint: true,
                        },
                        f,
                    )?;
                    Display::fmt(
                        &WriteArrayElements {
                            element_type: inner,
                            array_expr: self.field_expr,
                        },
                        f,
                    )?;
                } else {
                    unreachable!("Variable-length array does not have an implicit length field");
                }
            }
        }

        Ok(())
    }
}

struct WriteUnalignedField<'t> {
    field_expr: &'t str,
    ty: &'t ResolvedType,
}

impl Display for WriteUnalignedField<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.ty {
            ResolvedType::Scalar(scalar) => match scalar {
                ResolvedScalarType::Composite { .. } => {
                    writeln!(f, "cursor.write_composite(&{});", self.field_expr)?;
                }
                ResolvedScalarType::Primitive(primitive) => match primitive {
                    PrimitiveType::Boolean => {
                        writeln!(f, "cursor.write_bool({});", self.field_expr)?
                    }
                    PrimitiveType::Int { bits } => Display::fmt(
                        &CallWrite {
                            bits: *bits,
                            name: self.field_expr,
                            as_uint: true,
                        },
                        f,
                    )?,
                    PrimitiveType::UInt { bits, .. } => Display::fmt(
                        &CallWrite {
                            bits: *bits,
                            name: self.field_expr,
                            as_uint: false,
                        },
                        f,
                    )?,
                    PrimitiveType::Float16 { .. } => {
                        writeln!(f, "cursor.write_f16({});", self.field_expr)?
                    }
                    PrimitiveType::Float32 { .. } => {
                        writeln!(f, "cursor.write_f32({});", self.field_expr)?
                    }
                    PrimitiveType::Float64 { .. } => {
                        writeln!(f, "cursor.write_f64({});", self.field_expr)?
                    }
                },
                ResolvedScalarType::Void { bits } => writeln!(f, "cursor.skip_{}();", *bits)?,
            },
            ResolvedType::FixedArray {
                inner: ResolvedScalarType::Primitive(PrimitiveType::Boolean),
                ..
            } => {
                // Use BitArray with no length field
                writeln!(f, "({}).serialize(cursor);", self.field_expr)?;
            }
            ResolvedType::VariableArray {
                inner: ResolvedScalarType::Primitive(PrimitiveType::Boolean),
                ..
            } => {
                // Use BitArray with a length field
                if let Some(ImplicitField::ArrayLength { bits }) = self.ty.implicit_field() {
                    // Write length and then elements
                    Display::fmt(
                        &CallWrite {
                            bits,
                            name: &format!("({}).len()", self.field_expr),
                            as_uint: true,
                        },
                        f,
                    )?;
                    writeln!(f, "({}).serialize(cursor);", self.field_expr)?;
                } else {
                    unreachable!("Variable-length array does not have an implicit length field");
                }
            }
            ResolvedType::FixedArray { inner, .. } => {
                Display::fmt(
                    &WriteArrayElements {
                        element_type: inner,
                        array_expr: self.field_expr,
                    },
                    f,
                )?;
            }
            ResolvedType::VariableArray { inner, .. } => {
                if let Some(ImplicitField::ArrayLength { bits }) = self.ty.implicit_field() {
                    // Write length and then elements
                    Display::fmt(
                        &CallWrite {
                            bits,
                            name: &format!("({}).len()", self.field_expr),
                            as_uint: true,
                        },
                        f,
                    )?;
                    Display::fmt(
                        &WriteArrayElements {
                            element_type: inner,
                            array_expr: self.field_expr,
                        },
                        f,
                    )?;
                } else {
                    unreachable!("Variable-length array does not have an implicit length field");
                }
            }
        }

        Ok(())
    }
}

struct WriteArrayElements<'t> {
    element_type: &'t ResolvedScalarType,
    array_expr: &'t str,
}

impl Display for WriteArrayElements<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.element_type {
            ResolvedScalarType::Composite { .. } => {
                // Regular composite case
                writeln!(
                    f,
                    "for value in ({}).iter() {{ cursor.write_composite(value); }}",
                    self.array_expr
                )
            }
            ResolvedScalarType::Primitive(primitive) => match primitive {
                PrimitiveType::Boolean => {
                    writeln!(
                        f,
                        "for value in ({}).iter() {{ cursor.write_bool(*value); }}",
                        self.array_expr
                    )
                }
                PrimitiveType::Int { bits, .. } => {
                    write!(f, "for value in ({}).iter() {{", self.array_expr)?;
                    Display::fmt(
                        &CallWrite {
                            bits: *bits,
                            name: "*value",
                            as_uint: true,
                        },
                        f,
                    )?;
                    writeln!(f, "}}")
                }
                PrimitiveType::UInt { bits: 8, .. } => {
                    // Special case for byte arrays
                    writeln!(f, "cursor.write_bytes(&({})[..]);", self.array_expr)
                }
                PrimitiveType::UInt { bits, .. } => {
                    write!(f, "for value in ({}).iter() {{", self.array_expr)?;
                    Display::fmt(
                        &CallWrite {
                            bits: *bits,
                            name: "*value",
                            as_uint: false,
                        },
                        f,
                    )?;
                    writeln!(f, "}}")
                }
                PrimitiveType::Float16 { .. } => {
                    writeln!(
                        f,
                        "for value in ({}).iter() {{ cursor.write_f16(*value); }}",
                        self.array_expr
                    )
                }
                PrimitiveType::Float32 { .. } => {
                    writeln!(
                        f,
                        "for value in ({}).iter() {{ cursor.write_f32(*value); }}",
                        self.array_expr
                    )
                }
                PrimitiveType::Float64 { .. } => {
                    writeln!(
                        f,
                        "for value in ({}).iter() {{ cursor.write_f64(*value); }}",
                        self.array_expr
                    )
                }
            },
            ResolvedScalarType::Void { .. } => {
                panic!("A void type can't be part of an array")
            }
        }
    }
}

struct CallWriteAligned<'n> {
    bits: u8,
    name: &'n str,
    as_uint: bool,
}

impl Display for CallWriteAligned<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let function_name = match self.bits {
            0 => unreachable!("Can't have a 0-bit integer"),
            8 => "write_aligned_u8".to_owned(),
            16 => "write_aligned_u16".to_owned(),
            32 => "write_aligned_u32".to_owned(),
            64 => "write_aligned_u64".to_owned(),
            1..=7 | 9..=15 | 17..=31 | 33..=63 => format!("write_u{}", self.bits),
            65..=u8::MAX => panic!("Integer too large"),
        };
        if self.as_uint {
            writeln!(
                f,
                "cursor.{}({} as u{});",
                function_name,
                self.name,
                round_up_integer_size(self.bits)
            )
        } else {
            writeln!(f, "cursor.{}({});", function_name, self.name)
        }
    }
}

struct CallWrite<'n> {
    bits: u8,
    name: &'n str,
    as_uint: bool,
}

impl Display for CallWrite<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let function_name = format!("write_u{}", self.bits);
        if self.as_uint {
            writeln!(
                f,
                "cursor.{}({} as u{});",
                function_name,
                self.name,
                round_up_integer_size(self.bits)
            )
        } else {
            writeln!(f, "cursor.{}({});", function_name, self.name)
        }
    }
}
