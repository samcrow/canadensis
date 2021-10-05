//! Information about constants

use crate::compile::CompileContext;
use crate::error::Error;
use crate::types::keywords::is_reserved_keyword;
use crate::types::{evaluate_expression, PrimitiveType, Value};
use canadensis_dsdl_parser::num_bigint::BigInt;
use canadensis_dsdl_parser::{make_error, Expression, Identifier, Span};
use half::f16;
use num_traits::ToPrimitive;
use std::ops::{Deref, Range};

/// A constant declared in a data type
#[derive(Debug)]
pub struct Constant {
    /// The declared type of the constant
    ty: PrimitiveType,
    /// The value of the constant
    ///
    /// The type of this value may not be equal to its declared type.
    value: Value,
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
        if is_reserved_keyword(name.name) {
            return Err(
                make_error(format!("Use of reserved keyword {}", name.name), name.span).into(),
            );
        }
        let value = evaluate_expression(cx, value)?;
        check_type_compatibility(&ty, &value, value_span)?;

        Ok(Constant { ty, value })
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

/// Checks a declared type and an expression value to determine if they are compatible
///
/// This is based on table 3.14 in the specification.
fn check_type_compatibility(
    declared: &PrimitiveType,
    value: &Value,
    value_span: Span<'_>,
) -> Result<(), canadensis_dsdl_parser::Error> {
    match (declared, value) {
        // bool = bool
        (PrimitiveType::Boolean, Value::Boolean(_)) => Ok(()),
        // [any cast mode] uint8 = integer-convertible-string
        (PrimitiveType::UInt { bits: 8, .. }, Value::String(string)) => {
            if string.implicit_int().is_some() {
                Ok(())
            } else {
                Err(make_error(
                    format!(
                        "String {:?} cannot be assigned to a constant of type {}",
                        string.deref(),
                        declared
                    ),
                    value_span,
                ))
            }
        }
        // int[any bits] = rational that is an integer and fits into the int type
        (PrimitiveType::Int { bits }, Value::Rational(value)) => {
            if value.is_integer() {
                let value = value.numer();
                if signed_int_bounds(*bits).contains(value) {
                    Ok(())
                } else {
                    Err(make_error(
                        format!(
                            "Integer {} cannot be assigned to a {} constant because it is too large",
                            value,declared
                        ),
                        value_span,
                    ))
                }
            } else {
                Err(make_error(
                    format!(
                        "Non-integer {} cannot be assigned to an integer constant",
                        value
                    ),
                    value_span,
                ))
            }
        }
        // uint[any bits] = rational that is an integer and fits into the int type
        (PrimitiveType::UInt { bits, .. }, Value::Rational(value)) => {
            if value.is_integer() {
                let value = value.numer();
                if unsigned_int_bounds(*bits).contains(value) {
                    Ok(())
                } else {
                    Err(make_error(
                        format!(
                            "Integer {} cannot be assigned to a {} constant because it is too large or negative",
                            value,declared
                        ),
                        value_span,
                    ))
                }
            } else {
                Err(make_error(
                    format!(
                        "Non-integer {} cannot be assigned to an integer constant",
                        value
                    ),
                    value_span,
                ))
            }
        }
        // float16 = rational that fits into a float16
        (PrimitiveType::Float16 { .. }, Value::Rational(value)) => {
            match value.to_f32().map(f16::from_f32) {
                Some(float_value) if float_value.is_finite() => Ok(()),
                _ =>  Err(make_error(
                    format!(
                        "Rational {} cannot be assigned to floating-point constant because it is too large",
                        value
                    ),
                    value_span,
                )),
            }
        }
        // float32 = rational that fits into a float32
        (PrimitiveType::Float32 { .. }, Value::Rational(value)) => {
            match value.to_f32() {
                Some(float_value) if float_value.is_finite() => Ok(()),
                _ =>  Err(make_error(
                    format!(
                        "Value {} cannot be assigned to a floating-point constant because it is too large",
                        value
                    ),
                    value_span,
                )),
            }
        }
        // float64 = rational that fits into a float64
        (PrimitiveType::Float64 { .. }, Value::Rational(value)) => {
            match value.to_f64() {
                Some(float_value) if float_value.is_finite() => Ok(()),
                _ =>  Err(make_error(
                    format!(
                        "Value {} cannot be assigned to a floating-point constant because it is too large",
                        value
                    ),
                    value_span,
                )),
            }
        }
        // Any other combination is not allowed
        (declared, actual) => {
             Err(make_error(
                format!(
                    "Declared type {} of constant is not compatible with value of type {}",
                    declared, actual
                ),
                value_span,
            ))
        }
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
