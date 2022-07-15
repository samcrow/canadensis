//! Information about constants

use crate::compile::CompileContext;
use crate::error::Error;
use crate::types::keywords::is_reserved_keyword;
use crate::types::{evaluate_expression, PrimitiveType, Value};
use canadensis_dsdl_parser::num_bigint::BigInt;
use canadensis_dsdl_parser::{Expression, Identifier, Span};
use half::f16;
use num_traits::ToPrimitive;
use std::ops::Range;

/// A constant declared in a data type
#[derive(Debug, Clone)]
pub struct Constant {
    /// The declared type of the constant
    ty: PrimitiveType,
    /// The value of the constant, used to evaluate DSDL expressions
    ///
    /// The type of this value may not be equal to its declared type.
    dsdl_value: Value,
    /// The possibly-simplified value of this constant that is exposed to client code in other
    /// crates and can be used to generate code
    value: ConstantValue,
    /// Documentation comments for this constant
    comments: String,
    /// The offset, in bytes into the file, of the end of this constant definition
    end_offset: usize,
}

impl Constant {
    /// Evaluates an expression from a declaration
    pub(crate) fn evaluate(
        cx: &mut CompileContext<'_>,
        ty: canadensis_dsdl_parser::PrimitiveType,
        name: Identifier,
        value: Expression,
    ) -> Result<Self, Error> {
        let ty: PrimitiveType = ty.into();
        let value_span = value.span.clone();
        let end_offset = value_span.end();
        if is_reserved_keyword(name.name) {
            return Err(span_error!(
                name.span,
                "Use of reserved keyword {}",
                name.name
            ));
        }
        let dsdl_value = evaluate_expression(cx, value)?;
        let value = check_type_compatibility(&ty, &dsdl_value, value_span)?;

        Ok(Constant {
            ty,
            dsdl_value,
            value,
            comments: String::new(),
            end_offset,
        })
    }

    /// Returns the declared type of this constant
    pub fn ty(&self) -> &PrimitiveType {
        &self.ty
    }

    /// Returns the value of this constant in a form used to evaluate DSDL expressions
    pub(crate) fn dsdl_value(&self) -> &Value {
        &self.dsdl_value
    }

    /// Returns the value of the constant
    pub fn value(&self) -> &ConstantValue {
        &self.value
    }

    /// Returns the documentation comments for this constant
    pub fn comments(&self) -> &str {
        &self.comments
    }

    /// Appends a newline if self.comments is not empty,
    /// followed by the provided string, to the comments of this constant
    pub(crate) fn append_comment(&mut self, comment: &str) {
        if !self.comments.is_empty() {
            self.comments.push('\n');
        }
        self.comments.push_str(comment);
    }
    /// Returns the offset in bytes from the beginning of the file of the end of the declaration
    /// of this constant
    pub(crate) fn end_offset(&self) -> usize {
        self.end_offset
    }
}

/// Checks a declared type and an expression value to determine if they are compatible
///
/// If the types are compatible, this function returns the value as a ConstantValue.
///
/// This is based on table 3.14 in the specification.
fn check_type_compatibility(
    declared: &PrimitiveType,
    value: &Value,
    value_span: Span<'_>,
) -> Result<ConstantValue, Error> {
    match (declared, value) {
        // bool = bool
        (PrimitiveType::Boolean, Value::Boolean(value)) => Ok(ConstantValue::Boolean(*value)),
        // [any cast mode] uint8 = integer-convertible-string
        (PrimitiveType::UInt { bits: 8, .. }, Value::String(string)) => {
            if let Some(int_value) = string.implicit_int() {
                Ok(ConstantValue::Int(int_value.into()))
            } else {
                Err(span_error!(
                    value_span,
                    "String {:?} cannot be assigned to a constant of type {}",
                    &**string,
                    declared
                ))
            }
        }
        // int[any bits] = rational that is an integer and fits into the int type
        (PrimitiveType::Int { bits }, Value::Rational(value)) => {
            if value.is_integer() {
                let value = value.numer();
                if signed_int_bounds(*bits).contains(value) {
                    Ok(ConstantValue::Int(value.clone()))
                } else {
                    Err(span_error!(
                        value_span,
                        "Integer {} cannot be assigned to a {} constant because it is too large",
                        value,
                        declared
                    ))
                }
            } else {
                Err(span_error!(
                    value_span,
                    "Non-integer {} cannot be assigned to an integer constant",
                    value
                ))
            }
        }
        // uint[any bits] = rational that is an integer and fits into the int type
        (PrimitiveType::UInt { bits, .. }, Value::Rational(value)) => {
            if value.is_integer() {
                let value = value.numer();
                if unsigned_int_bounds(*bits).contains(value) {
                    Ok(ConstantValue::Int(value.clone()))
                } else {
                    Err(span_error!(value_span,
                        "Integer {} cannot be assigned to a {} constant because it is too large or negative",
                        value,declared
                    ))
                }
            } else {
                Err(span_error!(
                    value_span,
                    "Non-integer {} cannot be assigned to an integer constant",
                    value
                ))
            }
        }
        // float16 = rational that fits into a float16
        (PrimitiveType::Float16 { .. }, Value::Rational(value)) => {
            match value.to_f32().map(f16::from_f32) {
                Some(float_value) if float_value.is_finite() => Ok(ConstantValue::Float16(float_value)),
                _ =>  Err(span_error!(value_span,
                        "Rational {} cannot be assigned to floating-point constant because it is too large",
                        value
                )),
            }
        }
        // float32 = rational that fits into a float32
        (PrimitiveType::Float32 { .. }, Value::Rational(value)) => match value.to_f32() {
            Some(float_value) if float_value.is_finite() => Ok(ConstantValue::Float32(float_value)),
            _ => Err(span_error!(
                value_span,
                "Value {} cannot be assigned to a floating-point constant because it is too large",
                value
            )),
        },
        // float64 = rational that fits into a float64
        (PrimitiveType::Float64 { .. }, Value::Rational(value)) => match value.to_f64() {
            Some(float_value) if float_value.is_finite() => Ok(ConstantValue::Float64(float_value)),
            _ => Err(span_error!(
                value_span,
                "Value {} cannot be assigned to a floating-point constant because it is too large",
                value
            )),
        },
        // Any other combination is not allowed
        (declared, actual) => Err(span_error!(
            value_span,
            "Declared type {} of constant is not compatible with value of type {}",
            declared,
            actual
        )),
    }
}

/// Returns the range of values that can be represented by a two's complement signed integer with
/// the specified number of bits
fn signed_int_bounds(bits: u8) -> Range<BigInt> {
    let lower = -(BigInt::from(2).pow(u32::from(bits - 1)));
    let upper = BigInt::from(2).pow(u32::from(bits - 1));
    lower..upper
}

/// Returns the range of values that can be represented by an unsigned integer with
/// the specified number of bits
fn unsigned_int_bounds(bits: u8) -> Range<BigInt> {
    let lower = BigInt::from(0);
    let upper = BigInt::from(2).pow(u32::from(bits));
    lower..upper
}

/// Values that a constant can hold
#[derive(Debug, Clone)]
pub enum ConstantValue {
    /// Boolean
    Boolean(bool),
    /// Signed or unsigned integer
    Int(BigInt),
    /// 16-bit float
    Float16(f16),
    /// 32-bit float
    Float32(f32),
    /// 64-bit float
    Float64(f64),
}

mod fmt_impl {
    use super::ConstantValue;
    use std::fmt::{Display, Formatter, Result};

    impl Display for ConstantValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                ConstantValue::Boolean(inner) => Display::fmt(inner, f),
                ConstantValue::Int(inner) => Display::fmt(inner, f),
                ConstantValue::Float16(inner) => Display::fmt(inner, f),
                ConstantValue::Float32(inner) => Display::fmt(inner, f),
                ConstantValue::Float64(inner) => Display::fmt(inner, f),
            }
        }
    }
}
