//! Implements Deserialize for a type

use canadensis_dsdl_frontend::types::{
    ImplicitField, PrimitiveType, ResolvedScalarType, ResolvedType,
};
use std::fmt::{Display, Formatter, Result};

use crate::{
    GeneratedEnum, GeneratedField, GeneratedStruct, GeneratedType, GeneratedTypeKind, RustTypeName,
};

pub(crate) struct ImplementDeserialize<'t, 'c> {
    pub ty: &'t GeneratedType<'c>,
    pub zero_copy: bool,
}

impl Display for ImplementDeserialize<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            "impl ::canadensis_encoding::Deserialize for {} {{",
            self.ty.name.type_name
        )?;
        writeln!(f, "fn deserialize(cursor: &mut ::canadensis_encoding::ReadCursor<'_>) -> ::core::result::Result<Self, ::canadensis_encoding::DeserializeError> where Self: Sized {{")?;

        match &self.ty.kind {
            GeneratedTypeKind::Struct(gstruct) => {
                deserialize_struct(f, &self.ty.name, gstruct, self.zero_copy)?
            }
            GeneratedTypeKind::Enum(genum) => deserialize_enum(f, &self.ty.name, genum)?,
        }

        // End function
        writeln!(f, "}}")?;

        // End impl
        writeln!(f, "}}")
    }
}

fn deserialize_struct(
    f: &mut Formatter,
    name: &RustTypeName,
    gstruct: &GeneratedStruct,
    zero_copy: bool,
) -> Result {
    if zero_copy {
        writeln!(f, "Ok(Self::deserialize_zero_copy(cursor))")
    } else {
        writeln!(f, "Ok( {} {{", name.type_name)?;

        // The padding from all padding fields before the next data field
        let mut padding_before_data: Vec<u8> = Vec::new();
        for field in &gstruct.fields {
            match field {
                GeneratedField::Data(field) => {
                    // Generate: field_name: { skip_bits(); field_value_expr }
                    write!(f, "{}: {{", field.name)?;
                    for padding in padding_before_data.drain(..) {
                        writeln!(f, "cursor.skip_{}();", padding)?;
                    }
                    // TODO: Use aligned if field is always aligned
                    writeln!(
                        f,
                        "{} }},",
                        ReadUnalignedField {
                            ty: field.cyphal_ty
                        }
                    )?;
                }
                GeneratedField::Padding(bits) => {
                    // Store the padding, which will be put before the next data field
                    padding_before_data.push(*bits);
                }
            }
        }

        writeln!(f, "}} )")?;
        Ok(())
    }
}

fn deserialize_enum(f: &mut Formatter<'_>, name: &RustTypeName, genum: &GeneratedEnum) -> Result {
    // Match on the discriminant
    writeln!(
        f,
        "match {} {{",
        CallReadAligned {
            bits: genum.discriminant_bits
        }
    )?;

    for variant in genum.variants.iter() {
        writeln!(f, "{} => {{", variant.discriminant)?;

        if let Some(ty) = &variant.ty {
            // Variant with data
            writeln!(
                f,
                "Ok({}::{}({{ {} }}))",
                name.type_name,
                variant.name,
                ReadUnalignedField { ty: &ty.cyphal_ty }
            )?;
        } else {
            // Variant with no data
            writeln!(f, "Ok({}::{})", name.type_name, variant.name)?;
        }

        // End match arm
        writeln!(f, "}}")?;
    }
    // Wildcard pattern
    writeln!(
        f,
        "_ => Err(::canadensis_encoding::DeserializeError::UnionTag),"
    )?;

    // End match
    writeln!(f, "}}")?;
    Ok(())
}

struct ReadUnalignedField<'t> {
    ty: &'t ResolvedType,
}

impl Display for ReadUnalignedField<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.ty {
            ResolvedType::Scalar(scalar) => match scalar {
                ResolvedScalarType::Composite { .. } => {
                    write!(f, "cursor.read_composite()?")?;
                }
                ResolvedScalarType::Primitive(primitive) => match primitive {
                    PrimitiveType::Boolean => write!(f, "cursor.read_bool()")?,
                    PrimitiveType::Int { bits } => {
                        Display::fmt(&CallRead { bits: *bits }, f)?;
                    }
                    PrimitiveType::UInt { bits, .. } => {
                        Display::fmt(&CallRead { bits: *bits }, f)?;
                    }
                    PrimitiveType::Float16 { .. } => writeln!(f, "cursor.read_f16()")?,
                    PrimitiveType::Float32 { .. } => writeln!(f, "cursor.read_f32()")?,
                    PrimitiveType::Float64 { .. } => writeln!(f, "cursor.read_f64()")?,
                },
                ResolvedScalarType::Void { bits } => writeln!(f, "cursor.skip_{}();", *bits)?,
            },
            ResolvedType::FixedArray {
                inner: ResolvedScalarType::Primitive(PrimitiveType::Boolean),
                len,
            } => {
                // Use BitArray with no length field
                writeln!(
                    f,
                    "::canadensis_encoding::bits::BitArray::deserialize({}_usize, cursor)",
                    *len
                )?;
            }
            ResolvedType::VariableArray {
                inner: ResolvedScalarType::Primitive(PrimitiveType::Boolean),
                ..
            } => {
                // Use BitArray with a length field
                let length_bits = match &self.ty.implicit_field() {
                    Some(ImplicitField::ArrayLength { bits }) => *bits,
                    _ => unreachable!("Variable-length array does not have a length field"),
                };
                writeln!(f, "{{ let length = {};", CallRead { bits: length_bits })?;
                writeln!(
                    f,
                    "::canadensis_encoding::bits::BitArray::deserialize(length, cursor) }}"
                )?;
            }
            ResolvedType::FixedArray { inner, len } => {
                // Make an array literal
                writeln!(f, "[")?;
                for _ in 0..*len {
                    writeln!(f, "{},", ReadUnalignedScalar { ty: inner })?;
                }
                writeln!(f, "]")?;
            }
            ResolvedType::VariableArray { inner, max_len } => {
                // Read and check the length
                // Create a heapless::Vec (its element type and capacity will be inferred)
                // Read and push elements
                let length_bits = match &self.ty.implicit_field() {
                    Some(ImplicitField::ArrayLength { bits }) => *bits,
                    _ => unreachable!("Variable-length array does not have a length field"),
                };
                writeln!(f, "let length = {};", CallRead { bits: length_bits })?;
                writeln!(f, "if length <= {} {{", *max_len)?;

                writeln!(f, "let mut elements = ::heapless::Vec::new();")?;
                writeln!(f, "for _ in 0..length {{")?;

                // Don't use unwrap() because that requires inner to implement Debug
                writeln!(
                    f,
                    "let _ = elements.push({});",
                    ReadUnalignedScalar { ty: inner }
                )?;

                // End for
                writeln!(f, "}}")?;
                writeln!(f, "elements")?;

                writeln!(f, "}} else {{")?;
                // Length too large
                writeln!(
                    f,
                    "return Err(::canadensis_encoding::DeserializeError::ArrayLength)"
                )?;
                writeln!(f, "}}")?;
            }
        }

        Ok(())
    }
}
struct ReadUnalignedScalar<'t> {
    ty: &'t ResolvedScalarType,
}

impl Display for ReadUnalignedScalar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.ty {
            ResolvedScalarType::Composite { .. } => {
                write!(f, "cursor.read_composite()?")?;
            }
            ResolvedScalarType::Primitive(primitive) => match primitive {
                PrimitiveType::Boolean => write!(f, "cursor.read_bool()")?,
                PrimitiveType::Int { bits } => Display::fmt(&CallRead { bits: *bits }, f)?,
                PrimitiveType::UInt { bits, .. } => Display::fmt(&CallRead { bits: *bits }, f)?,
                PrimitiveType::Float16 { .. } => write!(f, "cursor.read_f16()")?,
                PrimitiveType::Float32 { .. } => write!(f, "cursor.read_f32()")?,
                PrimitiveType::Float64 { .. } => write!(f, "cursor.read_f64()")?,
            },
            ResolvedScalarType::Void { bits } => write!(f, "cursor.skip_{}()", *bits)?,
        }
        Ok(())
    }
}

struct CallReadAligned {
    bits: u8,
}

impl Display for CallReadAligned {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let function_name = match self.bits {
            0 => unreachable!("Can't have a 0-bit integer"),
            8 => "read_aligned_u8".to_owned(),
            16 => "read_aligned_u16".to_owned(),
            32 => "read_aligned_u32".to_owned(),
            64 => "read_aligned_u64".to_owned(),
            1..=7 | 9..=15 | 17..=31 | 33..=63 => format!("read_u{}", self.bits),
            65..=u8::MAX => panic!("Integer too large"),
        };
        write!(f, "cursor.{}() as _", function_name,)
    }
}

struct CallRead {
    bits: u8,
}

impl Display for CallRead {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let function_name = format!("read_u{}", self.bits);
        write!(f, "cursor.{}() as _", function_name,)
    }
}
