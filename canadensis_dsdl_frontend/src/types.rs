pub mod constant;
pub mod directive;
pub mod expression;
pub mod keywords;
pub(crate) mod set;
mod string;

use crate::compile::CompileContext;
use crate::compiled::{DsdlKind, Extent, Message};
use crate::error::Error;
use crate::types::expression::evaluate_expression;
use crate::types::set::Set;
use crate::types::string::StringValue;
use crate::TypeKey;
use canadensis_bit_length_set::BitLengthSet;
use canadensis_dsdl_parser::{CastMode, Span};
use num_rational::BigRational;
use std::convert::TryInto;

/// A DSDL expression value
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Value {
    Rational(BigRational),
    String(StringValue),
    Set(Set),
    Boolean(bool),
    Type(Type),
    Identifier(String),
}

impl Value {
    pub fn ty(&self) -> ExprType {
        match self {
            Value::Rational(_) => ExprType::Rational,
            Value::String(_) => ExprType::String,
            Value::Set(set) => ExprType::Set(set.ty().map(Box::new)),
            Value::Boolean(_) => ExprType::Boolean,
            Value::Type(_) => ExprType::Type,
            Value::Identifier(_) => ExprType::Identifier,
        }
    }
}

/// A type, which can be the value of a DSDL expression (also called a serializable metatype)
///
/// This is separate from `ExprType`, which is the type of a DSDL expression.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type {
    Scalar(ScalarType),
    FixedArray {
        inner: ScalarType,
        len: u64,
    },
    VariableArray {
        inner: ScalarType,
        /// Maximum length of the array, inclusive
        max_len: u64,
    },
}

impl Type {
    /// Resolves this type, looking up a versioned type (if any) and replacing it with a Message
    pub(crate) fn resolve(
        self,
        cx: &mut CompileContext<'_>,
        span: Span<'_>,
    ) -> Result<ResolvedType, Error> {
        match self {
            Type::Scalar(scalar) => Ok(ResolvedType::Scalar(scalar.resolve(cx, span)?)),
            Type::FixedArray { inner, len } => Ok(ResolvedType::FixedArray {
                inner: inner.resolve(cx, span)?,
                len,
            }),
            Type::VariableArray { inner, max_len } => Ok(ResolvedType::VariableArray {
                inner: inner.resolve(cx, span)?,
                max_len,
            }),
        }
    }

    /// Returns a bit length set representing the possible lengths of this type
    pub(crate) fn bit_length_set(
        &self,
        cx: &mut CompileContext<'_>,
        span: Span<'_>,
    ) -> Result<BitLengthSet, Error> {
        match self {
            Type::Scalar(scalar) => scalar.bit_length_set(cx, span),
            Type::FixedArray { inner, len } => {
                let element_length = inner.bit_length_set(cx, span)?;
                Ok(element_length.repeat(*len))
            }
            Type::VariableArray { inner, max_len } => {
                let element_length = inner.bit_length_set(cx, span)?;
                Ok(element_length.repeat_range(..=*max_len))
            }
        }
    }

    /// Returns the required alignment of this type
    pub fn alignment(&self) -> u32 {
        match self {
            Type::Scalar(scalar) => scalar.alignment(),
            // The alignment of an array equals the alignment of its element type
            Type::FixedArray { inner, .. } | Type::VariableArray { inner, .. } => inner.alignment(),
        }
    }
}

/// Returns the number of bits needed for the array size field to store up to `max_items` values
/// (inclusive) in a variable-length array, or `max_items` variants of a union
pub(crate) fn array_length_bits(max_items: u64) -> u32 {
    round_up_length(bit_length(max_items))
}

fn bit_length(value: u64) -> u32 {
    u64::BITS - value.leading_zeros()
}
fn round_up_length(value: u32) -> u32 {
    std::cmp::max(value, 8).next_power_of_two()
}

#[cfg(test)]
mod test {
    use super::{bit_length, round_up_length};
    fn length_bits(max_length: u64) -> u32 {
        round_up_length(bit_length(max_length))
    }

    #[test]
    fn test_array_size_length() {
        assert_eq!(8, length_bits(0));
        assert_eq!(8, length_bits(1));
        assert_eq!(8, length_bits(2));
        // ...
        assert_eq!(8, length_bits(254));
        assert_eq!(8, length_bits(255));
        assert_eq!(16, length_bits(256));
        assert_eq!(16, length_bits(257));
        // ...
        assert_eq!(16, length_bits(65535));
        assert_eq!(32, length_bits(65536));
        assert_eq!(32, length_bits(65537));
    }
}

/// An implicit field that may be inserted before another field
pub(crate) enum ImplicitField {
    /// A u32 header for an enclosed delimited type
    DelimiterHeader,
    /// An unsigned integer length for a variable-length array
    ArrayLength { bits: u8 },
}

impl From<ImplicitField> for PrimitiveType {
    fn from(implicit: ImplicitField) -> Self {
        match implicit {
            ImplicitField::DelimiterHeader => PrimitiveType::UInt {
                bits: 32,
                mode: CastMode::Saturated,
            },
            ImplicitField::ArrayLength { bits } => PrimitiveType::UInt {
                bits,
                mode: CastMode::Saturated,
            },
        }
    }
}

impl From<canadensis_dsdl_parser::ScalarType<'_>> for ScalarType {
    fn from(ty: canadensis_dsdl_parser::ScalarType) -> Self {
        match ty {
            canadensis_dsdl_parser::ScalarType::Versioned(versioned) => {
                ScalarType::Versioned(versioned.into())
            }
            canadensis_dsdl_parser::ScalarType::Primitive(primitive) => {
                ScalarType::Primitive(primitive.into())
            }
            canadensis_dsdl_parser::ScalarType::Void { bits } => ScalarType::Void { bits },
        }
    }
}

impl From<canadensis_dsdl_parser::PrimitiveType> for PrimitiveType {
    fn from(primitive: canadensis_dsdl_parser::PrimitiveType) -> Self {
        match primitive {
            canadensis_dsdl_parser::PrimitiveType::Boolean => PrimitiveType::Boolean,
            canadensis_dsdl_parser::PrimitiveType::Int { bits } => PrimitiveType::Int { bits },
            canadensis_dsdl_parser::PrimitiveType::UInt { bits, mode } => {
                PrimitiveType::UInt { bits, mode }
            }
            canadensis_dsdl_parser::PrimitiveType::Float16 { mode } => {
                PrimitiveType::Float16 { mode }
            }
            canadensis_dsdl_parser::PrimitiveType::Float32 { mode } => {
                PrimitiveType::Float32 { mode }
            }
            canadensis_dsdl_parser::PrimitiveType::Float64 { mode } => {
                PrimitiveType::Float64 { mode }
            }
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ScalarType {
    /// A composite type with a name and version
    Versioned(TypeKey),
    /// A primitive type
    Primitive(PrimitiveType),
    /// A void type
    Void { bits: u8 },
}

impl ScalarType {
    /// Resolves this type, looking up a versioned type (if any) and replacing it with a [[`Message`]]
    pub(crate) fn resolve(
        self,
        cx: &mut CompileContext<'_>,
        span: Span<'_>,
    ) -> Result<ResolvedScalarType, Error> {
        match self {
            ScalarType::Versioned(key) => {
                let referenced_type = cx.type_by_key(&key)?;
                match &referenced_type.kind {
                    DsdlKind::Message { message, .. } => Ok(ResolvedScalarType::Composite {
                        key,
                        inner: Box::new(message.clone()),
                    }),
                    DsdlKind::Service { .. } => {
                        Err(span_error!(span, "Can't refer to a service type"))
                    }
                }
            }
            ScalarType::Primitive(primitive) => Ok(ResolvedScalarType::Primitive(primitive)),
            ScalarType::Void { bits } => Ok(ResolvedScalarType::Void { bits }),
        }
    }

    /// Returns a bit length set representing the possible lengths of this type
    fn bit_length_set(
        &self,
        cx: &mut CompileContext<'_>,
        span: Span<'_>,
    ) -> Result<BitLengthSet, Error> {
        match self {
            ScalarType::Versioned(key) => {
                let referenced_type = cx.type_by_key(key)?;
                match &referenced_type.kind {
                    DsdlKind::Message { message, .. } => Ok(message.bit_length.clone()),
                    DsdlKind::Service { .. } => {
                        Err(span_error!(span, "Can't refer to a service type"))
                    }
                }
            }
            ScalarType::Primitive(primitive) => Ok(BitLengthSet::single(primitive.bit_length())),
            ScalarType::Void { bits } => Ok(BitLengthSet::single(u64::from(*bits))),
        }
    }
    /// Returns the required alignment of this type
    pub(crate) fn alignment(&self) -> u32 {
        match self {
            // Versioned implies a composite type and 8-bit alignment
            ScalarType::Versioned(_) => 8,
            // Primitive and void types have 1-bit alignment
            ScalarType::Primitive(_) | ScalarType::Void { .. } => 1,
        }
    }
}

/// A primitive type
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PrimitiveType {
    /// Boolean, always saturated
    Boolean,
    /// Signed integer, always saturated
    Int { bits: u8 },
    /// Unsigned integer
    UInt { bits: u8, mode: CastMode },
    /// 16-bit float
    Float16 { mode: CastMode },
    /// 32-bit float
    Float32 { mode: CastMode },
    /// 64-bit float
    Float64 { mode: CastMode },
}

impl PrimitiveType {
    /// Returns the cast mode of this type
    pub fn cast_mode(&self) -> CastMode {
        match self {
            PrimitiveType::Boolean | PrimitiveType::Int { .. } => CastMode::Saturated,
            PrimitiveType::UInt { mode, .. }
            | PrimitiveType::Float16 { mode }
            | PrimitiveType::Float32 { mode }
            | PrimitiveType::Float64 { mode } => mode.clone(),
        }
    }

    pub fn bit_length(&self) -> u64 {
        match self {
            PrimitiveType::Boolean => 1,
            PrimitiveType::Int { bits } | PrimitiveType::UInt { bits, .. } => u64::from(*bits),
            PrimitiveType::Float16 { .. } => 16,
            PrimitiveType::Float32 { .. } => 32,
            PrimitiveType::Float64 { .. } => 64,
        }
    }

    pub fn alignment(&self) -> u32 {
        // All primitive types have alignment 1
        1
    }
}

/// A DSDL expression type
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ExprType {
    Rational,
    String,
    Set(Option<Box<ExprType>>),
    Boolean,
    Type,
    Identifier,
}

mod fmt_impl {
    use super::{ExprType, Type};
    use crate::types::string::StringValue;
    use crate::types::{PrimitiveType, ScalarType, Value};
    use canadensis_dsdl_parser::CastMode;
    use std::fmt::{Display, Formatter, Result, Write};

    impl Display for ExprType {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                ExprType::Rational => write!(f, "rational"),
                ExprType::String => write!(f, "string"),
                ExprType::Set(Some(element)) => write!(f, "set<{}>", element),
                ExprType::Set(None) => write!(f, "set<_>"),
                ExprType::Boolean => write!(f, "bool"),
                // So this is the serializable metatype I've heard so much about
                ExprType::Type => write!(f, "metaserializable"),
                ExprType::Identifier => write!(f, "identifier"),
            }
        }
    }

    impl Display for PrimitiveType {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let mode_str = match self.cast_mode() {
                CastMode::Truncated => "truncated",
                CastMode::Saturated => "saturated",
            };
            match self {
                PrimitiveType::Boolean => write!(f, "{} bool", mode_str),
                PrimitiveType::Int { bits, .. } => {
                    write!(f, "{} int{}", mode_str, *bits)
                }
                PrimitiveType::UInt { bits, .. } => {
                    write!(f, "{} uint{}", mode_str, *bits)
                }
                PrimitiveType::Float16 { .. } => write!(f, "{} float16", mode_str),
                PrimitiveType::Float32 { .. } => write!(f, "{} float32", mode_str),
                PrimitiveType::Float64 { .. } => write!(f, "{} float64", mode_str),
            }
        }
    }

    impl Display for Value {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Value::Rational(value) => {
                    if value.is_integer() {
                        write!(f, "{}", value.numer())
                    } else {
                        write!(f, "{}", value)
                    }
                }
                Value::String(value) => write!(f, "{}", value),
                Value::Set(set) => write!(f, "{}", set),
                Value::Boolean(value) => write!(f, "{}", *value),
                Value::Type(ty) => write!(f, "{}", ty),
                Value::Identifier(identifier) => write!(f, "{}", identifier),
            }
        }
    }

    impl Display for StringValue {
        /// Formats a string literal with enclosing quotation marks and some escape sequences
        /// in a way that can be parsed as a DSDL string
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_char('"')?;
            for c in self.chars() {
                // Escape backslashes, carriage returns, line feeds, and double quotes
                match c {
                    '\\' => f.write_str(r"\\")?,
                    '\r' => f.write_str(r"\r")?,
                    '\n' => f.write_str(r"\n")?,
                    '"' => f.write_str(r#"\""#)?,
                    _ => f.write_char(c)?,
                }
            }
            f.write_char('"')?;
            Ok(())
        }
    }

    impl Display for Type {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Type::Scalar(scalar) => write!(f, "{}", scalar),
                Type::FixedArray { inner, len } => write!(f, "{}[{}]", inner, len),
                Type::VariableArray { inner, max_len } => write!(f, "{}[<={}]", inner, max_len),
            }
        }
    }

    impl Display for ScalarType {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                ScalarType::Versioned(key) => write!(f, "{}", key),
                ScalarType::Primitive(primitive) => write!(f, "{}", primitive),
                ScalarType::Void { bits } => write!(f, "void{}", bits),
            }
        }
    }
}

/// A type of a DSDL field
///
/// If this is a composite type, the type name has been resolved to a Message.
#[derive(Debug, Clone)]
pub enum ResolvedType {
    Scalar(ResolvedScalarType),
    FixedArray {
        inner: ResolvedScalarType,
        len: u64,
    },
    VariableArray {
        inner: ResolvedScalarType,
        /// Maximum length of the array, inclusive
        max_len: u64,
    },
}

impl ResolvedType {
    pub fn size(&self) -> BitLengthSet {
        match self {
            ResolvedType::Scalar(scalar) => scalar.size(),
            ResolvedType::FixedArray { inner, len } => inner.size().repeat(*len),
            ResolvedType::VariableArray { inner, max_len } => {
                inner.size().repeat_range(..=*max_len)
            }
        }
    }
    pub fn alignment(&self) -> u32 {
        match self {
            ResolvedType::Scalar(scalar) => scalar.alignment(),
            ResolvedType::FixedArray { inner, .. } | ResolvedType::VariableArray { inner, .. } => {
                inner.alignment()
            }
        }
    }

    /// Returns the delimiter header or length field, if any, that this type requires
    pub(crate) fn implicit_field(&self) -> Option<ImplicitField> {
        match self {
            ResolvedType::Scalar(scalar) => scalar.implicit_field(),
            ResolvedType::FixedArray { .. } => None,
            ResolvedType::VariableArray { max_len, .. } => {
                let length_bits = array_length_bits(*max_len);
                Some(ImplicitField::ArrayLength {
                    bits: length_bits
                        .try_into()
                        .expect("Implicit length field length too large for u8"),
                })
            }
        }
    }
}

/// A scalar (non-array) type
///
/// If this is a composite type, the type name has been resolved to a Message.
#[derive(Debug, Clone)]
pub enum ResolvedScalarType {
    /// A composite message type
    Composite { key: TypeKey, inner: Box<Message> },
    /// A primitive type
    Primitive(PrimitiveType),
    /// A void type
    Void { bits: u8 },
}

impl ResolvedScalarType {
    pub fn size(&self) -> BitLengthSet {
        match self {
            ResolvedScalarType::Composite { inner, .. } => inner.bit_length.clone(),
            ResolvedScalarType::Primitive(primitive) => {
                BitLengthSet::single(primitive.bit_length())
            }
            ResolvedScalarType::Void { bits } => BitLengthSet::single(u64::from(*bits)),
        }
    }
    pub fn alignment(&self) -> u32 {
        match self {
            ResolvedScalarType::Composite { .. } => 8,
            ResolvedScalarType::Primitive(_) | ResolvedScalarType::Void { .. } => 1,
        }
    }

    /// Returns the delimiter header or length field, if any, that this type requires
    pub(crate) fn implicit_field(&self) -> Option<ImplicitField> {
        match self {
            ResolvedScalarType::Composite { inner, .. } => match inner.extent {
                Extent::Sealed => None,
                Extent::Delimited(_) => Some(ImplicitField::DelimiterHeader),
            },
            ResolvedScalarType::Primitive(_) | ResolvedScalarType::Void { .. } => None,
        }
    }
}
