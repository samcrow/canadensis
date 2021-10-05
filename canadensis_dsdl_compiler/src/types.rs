pub mod constant;
mod expression;
pub mod keywords;
mod set;
mod string;

use crate::compile::CompileContext;
use crate::package::Error;
use crate::types::expression::evaluate_expression;
use crate::types::set::Set;
use crate::types::string::StringValue;
use crate::TypeKey;
use canadensis_dsdl_parser::num_bigint::BigInt;
use canadensis_dsdl_parser::Expression;
use canadensis_dsdl_parser::{make_error, CastMode, Identifier};
use num_rational::BigRational;
use std::collections::BTreeMap;

pub struct Definition {
    /// Values of the constants in this definition
    pub constants: BTreeMap<String, Value>,
}

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
        len: BigInt,
    },
    VariableArray {
        inner: ScalarType,
        /// Maximum length of the array, inclusive
        max_len: BigInt,
    },
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
            PrimitiveType::Boolean => CastMode::Saturated,
            PrimitiveType::Int { .. } => CastMode::Saturated,
            PrimitiveType::UInt { mode, .. } => mode.clone(),
            PrimitiveType::Float16 { mode } => mode.clone(),
            PrimitiveType::Float32 { mode } => mode.clone(),
            PrimitiveType::Float64 { mode } => mode.clone(),
        }
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
    use std::fmt::*;

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

pub(crate) fn evaluate_directive(
    cx: &mut CompileContext,
    name: Identifier<'_>,
    expression: Option<Expression<'_>>,
) -> Result<(), Error> {
    match name.name {
        "union" => todo!(),
        "extent" => todo!(),
        "sealed" => todo!(),
        "deprecated" => todo!(),
        "assert" => match expression {
            Some(expr) => {
                let expr_span = expr.span.clone();
                match evaluate_expression(cx, expr)? {
                    Value::Boolean(true) => Ok(()),
                    Value::Boolean(false) => Err(make_error("Assert failed", expr_span).into()),
                    other => Err(make_error(
                        format!("Assert expression evaluated to non-boolean value {}", other),
                        expr_span,
                    )
                    .into()),
                }
            }
            None => Err(make_error("Assert directive has no expression", name.span).into()),
        },
        "print" => {
            if let Some(expr) = expression {
                let value = evaluate_expression(cx, expr)?;
                // TODO: Should this be printed in some other way?
                println!("{}", value);
                Ok(())
            } else {
                // Print with no expression has no effect (perhaps it should print a new line)
                Ok(())
            }
        }
        _ => Err(make_error(format!("Unrecognized directive {}", name.name), name.span).into()),
    }
}
