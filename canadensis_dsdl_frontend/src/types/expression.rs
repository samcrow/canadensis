//! Functions used to evaluate expressions

use std::collections::BTreeSet;
use std::convert::TryInto;
use std::ops::RangeInclusive;

use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive};

use canadensis_dsdl_parser::num_bigint::BigInt;
use canadensis_dsdl_parser::{
    Expression, ExpressionAtom, ExpressionType, Literal, LiteralType, Span,
};

use crate::compile::CompileContext;
use crate::error::Error;
use crate::operators::{
    add, attribute, bit_and, bit_or, calculate_elementwise_binary,
    calculate_rational_or_set_binary, calculate_rational_or_set_comparison, equal, exponent,
    make_set_error, not_equal, unary_minus, unary_not, unary_plus,
};
use crate::types::set::{Set, TypeError};
use crate::types::string::StringValue;
use crate::types::{ScalarType, Type, Value};

pub(crate) fn evaluate_expression(
    cx: &mut CompileContext<'_>,
    expression: Expression<'_>,
) -> Result<Value, Error> {
    let span = expression.span;
    match expression.expression {
        ExpressionType::Atom(atom) => evaluate_atom(cx, *atom, span),
        ExpressionType::UnaryPlus(inner) => {
            let inner = evaluate_expression(cx, *inner)?;
            unary_plus::evaluate(inner, span)
        }
        ExpressionType::UnaryMinus(inner) => {
            let inner = evaluate_expression(cx, *inner)?;
            unary_minus::evaluate(inner, span)
        }
        ExpressionType::UnaryNot(inner) => {
            let inner = evaluate_expression(cx, *inner)?;
            unary_not::evaluate(inner, span)
        }
        ExpressionType::Attribute(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            attribute::evaluate(cx, lhs, rhs, span)
        }
        ExpressionType::Exponent(base, exponent) => {
            let base = evaluate_expression(cx, *base)?;
            let exponent = evaluate_expression(cx, *exponent)?;
            exponent::evaluate(base, exponent, span)
        }
        ExpressionType::Multiply(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            calculate_elementwise_binary(lhs, rhs, span, "*", |lhs, rhs, _| {
                Ok(Value::Rational(lhs * rhs))
            })
        }
        ExpressionType::Divide(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            calculate_elementwise_binary(lhs, rhs, span, "/", |lhs, rhs, _| {
                Ok(Value::Rational(lhs / rhs))
            })
        }
        ExpressionType::Modulo(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;

            match (lhs, rhs) {
                // Special case for BitLengthSet % integer
                (Value::BitLengthSet(lhs), Value::Rational(rhs)) if is_u64_integer(&rhs) => {
                    let rhs: u64 = rhs
                        .numer()
                        .try_into()
                        .expect("rhs BigInt to u64 conversion checks incorrect");
                    let result = lhs % rhs;
                    Ok(Value::BitLengthSet(result))
                }
                // General case
                (lhs, rhs) => calculate_elementwise_binary(lhs, rhs, span, "%", |lhs, rhs, _| {
                    Ok(Value::Rational(lhs % rhs))
                }),
            }
        }
        ExpressionType::Add(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            add::evaluate(lhs, rhs, span)
        }
        ExpressionType::Subtract(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            calculate_elementwise_binary(lhs, rhs, span, "-", |lhs, rhs, _| {
                Ok(Value::Rational(lhs - rhs))
            })
        }
        ExpressionType::BitOr(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            bit_or::evaluate(lhs, rhs, span)
        }
        ExpressionType::BitXor(lhs, rhs) => {
            // a ^ b: Bitwise xor on integers, or disjunctive union of sets of the same type
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            calculate_rational_or_set_binary(
                lhs,
                rhs,
                span,
                "^",
                rational_bitwise_xor,
                |lhs, rhs| lhs.symmetric_difference(&rhs).unwrap(),
                |lhs, rhs| {
                    let lhs = lhs.expand();
                    let rhs = rhs.expand();
                    let symmetric_difference = lhs
                        .symmetric_difference(&rhs)
                        .copied()
                        .collect::<BTreeSet<u64>>();
                    Value::Set(Set::from(symmetric_difference))
                },
            )
        }
        ExpressionType::BitAnd(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            bit_and::evaluate(lhs, rhs, span)
        }
        ExpressionType::Equal(lhs, rhs) => {
            // a == b: Rationals exactly equal, strings NFC equal, sets equal
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            equal::evaluate(lhs, rhs, span)
        }
        ExpressionType::NotEqual(lhs, rhs) => {
            // Opposite of equal
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            not_equal::evaluate(lhs, rhs, span)
        }
        ExpressionType::LessOrEqual(lhs, rhs) => {
            // a <= b: Less or equal for rationals, subset for sets
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            let result = calculate_rational_or_set_comparison(
                lhs,
                rhs,
                span,
                "<=",
                |lhs, rhs| lhs <= rhs,
                |lhs, rhs| lhs.is_subset(&rhs),
            )?;
            Ok(Value::Boolean(result))
        }
        ExpressionType::GreaterOrEqual(lhs, rhs) => {
            // a >= b: Greater than or equal for rationals, superset for sets
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            let result = calculate_rational_or_set_comparison(
                lhs,
                rhs,
                span,
                ">=",
                |lhs, rhs| lhs >= rhs,
                |lhs, rhs| lhs.is_superset(&rhs),
            )?;
            Ok(Value::Boolean(result))
        }
        ExpressionType::Less(lhs, rhs) => {
            // a < b: Less for rationals, proper subset for sets
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            let result = calculate_rational_or_set_comparison(
                lhs,
                rhs,
                span,
                "<",
                |lhs, rhs| lhs < rhs,
                |lhs, rhs| lhs.is_subset(&rhs) && lhs != rhs,
            )?;
            Ok(Value::Boolean(result))
        }
        ExpressionType::Greater(lhs, rhs) => {
            // a > b: Greater for rationals, proper superset for sets
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            let result = calculate_rational_or_set_comparison(
                lhs,
                rhs,
                span,
                ">",
                |lhs, rhs| lhs > rhs,
                |lhs, rhs| lhs.is_superset(&rhs) && lhs != rhs,
            )?;
            Ok(Value::Boolean(result))
        }
        ExpressionType::LogicalOr(lhs, rhs) => {
            // a || b for booleans only
            // Don't short circuit (the specification doesn't specify this)
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            match (lhs, rhs) {
                (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(lhs || rhs)),
                (lhs, rhs) => Err(span_error!(
                    span,
                    "Can't calculate {} || {}",
                    lhs.ty(),
                    rhs.ty()
                )),
            }
        }
        ExpressionType::LogicalAnd(lhs, rhs) => {
            // a && b for booleans only
            // Don't short circuit (the specification doesn't specify this)
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            match (lhs, rhs) {
                (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(lhs && rhs)),
                (lhs, rhs) => Err(span_error!(
                    span,
                    "Can't calculate {} && {}",
                    lhs.ty(),
                    rhs.ty()
                )),
            }
        }
    }
}

fn evaluate_atom(
    cx: &mut CompileContext<'_>,
    atom: ExpressionAtom,
    span: Span<'_>,
) -> Result<Value, Error> {
    match atom {
        ExpressionAtom::Parenthesized(inner) => evaluate_expression(cx, inner),
        ExpressionAtom::Type(ty) => Ok(Value::Type(convert_type(cx, ty)?)),
        ExpressionAtom::Literal(Literal { literal, span }) => match literal {
            LiteralType::Set(expressions) => {
                let set: Result<Result<Set, TypeError>, Error> = expressions
                    .into_iter()
                    .map(|expr| evaluate_expression(cx, expr))
                    .collect();
                match set {
                    Ok(Ok(set)) => Ok(Value::Set(set)),
                    Err(expr_error) => Err(expr_error),
                    Ok(Err(set_error)) => Err(make_set_error(set_error, span)),
                }
            }
            LiteralType::Number(value) => Ok(Value::Rational(value)),
            LiteralType::String(value) => Ok(Value::String(StringValue::from(value))),
            LiteralType::Boolean(value) => Ok(Value::Boolean(value)),
        },
        // This is where we try to replace an identifier with is value
        ExpressionAtom::Identifier(identifier) => {
            match identifier {
                // Magic variables
                "_offset_" => {
                    let bit_length = cx.bit_length_set().clone();
                    Ok(Value::BitLengthSet(bit_length))
                }
                _ => {
                    // Try constants
                    match cx.constants().get(identifier) {
                        Some(constant) => Ok(constant.dsdl_value().clone()),
                        None => Err(span_error!(span, "Identifier {} not found", identifier)),
                    }
                }
            }
        }
    }
}

fn evaluate_array_length(
    cx: &mut CompileContext<'_>,
    length: Expression<'_>,
) -> Result<u64, Error> {
    let length_span = length.span;
    match evaluate_expression(cx, length)? {
        Value::Rational(rational) => {
            if rational.is_integer() {
                let length = rational.numer().clone();
                if length.is_negative() {
                    Err(span_error!(
                        length_span,
                        "Array length evaluated to negative value {}",
                        length,
                    ))
                } else {
                    // Convert to usize
                    match length.to_u64() {
                        Some(length) => Ok(length),
                        None => Err(span_error!(
                            length_span,
                            "Compiler limitation: Array length {} is too large",
                            length,
                        )),
                    }
                }
            } else {
                Err(span_error!(
                    length_span,
                    "Array length evaluated to non-integer value {}",
                    rational
                ))
            }
        }
        other => Err(span_error!(
            length_span,
            "Array length evaluated to non-rational type {}",
            other.ty()
        )),
    }
}

fn rational_bitwise_xor(
    lhs: BigRational,
    rhs: BigRational,
    span: Span<'_>,
) -> Result<Value, Error> {
    if lhs.is_integer() && rhs.is_integer() {
        let result = lhs.numer() ^ rhs.numer();
        Ok(Value::Rational(BigRational::from_integer(result)))
    } else {
        Err(span_error!(
            span,
            "Can't calculate {} ^ {}: Both operands must be integers",
            lhs,
            rhs
        ))
    }
}

/// Converts an AST type into a compiler type
pub(crate) fn convert_type(
    cx: &mut CompileContext<'_>,
    ty: canadensis_dsdl_parser::Type<'_>,
) -> Result<Type, Error> {
    match ty {
        canadensis_dsdl_parser::Type::Scalar(scalar) => Ok(Type::Scalar(scalar.into())),
        canadensis_dsdl_parser::Type::Array(array) => {
            let element: ScalarType = array.element.into();
            match array.length {
                canadensis_dsdl_parser::ArrayLength::Fixed(length) => {
                    let length = evaluate_array_length(cx, length)?;
                    Ok(Type::FixedArray {
                        inner: element,
                        len: length,
                    })
                }
                canadensis_dsdl_parser::ArrayLength::Inclusive(length) => {
                    let length = evaluate_array_length(cx, length)?;
                    Ok(Type::VariableArray {
                        inner: element,
                        max_len: length,
                    })
                }
                canadensis_dsdl_parser::ArrayLength::Exclusive(length) => {
                    let length_span = length.span;
                    let length = evaluate_array_length(cx, length)?;
                    if length > 0 {
                        // Convert to inclusive length by subtracting 1
                        Ok(Type::VariableArray {
                            inner: element,
                            max_len: length - 1,
                        })
                    } else {
                        Err(span_error!(
                            length_span,
                            "Non-inclusive array length evaluated to non-positive value {}",
                            length
                        ))
                    }
                }
            }
        }
    }
}

/// Returns true if the provided rational number is an integer that can be converted into a u64
fn is_u64_integer(value: &BigRational) -> bool {
    let u64_range: RangeInclusive<BigInt> = BigInt::from(0u64)..=BigInt::from(u64::MAX);
    value.is_integer() && u64_range.contains(value.numer())
}
