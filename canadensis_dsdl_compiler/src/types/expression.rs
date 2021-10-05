//! Functions used to evaluate expressions

use crate::compile::CompileContext;
use crate::compiled::DsdlKind;
use crate::package::Error;
use crate::types::set::{Set, SetTypeError};
use crate::types::string::StringValue;
use crate::types::{ExprType, ScalarType, Type, Value};
use canadensis_dsdl_parser::num_bigint::BigInt;
use canadensis_dsdl_parser::{
    make_error, Expression, ExpressionAtom, ExpressionType, Literal, LiteralType, Span,
};
use num_rational::BigRational;
use num_traits::{FromPrimitive, Pow, Signed, ToPrimitive};

pub(crate) fn evaluate_expression(
    cx: &mut CompileContext,
    expression: Expression<'_>,
) -> Result<Value, Error> {
    let span = expression.span;
    match expression.expression {
        ExpressionType::Atom(atom) => evaluate_atom(cx, *atom, span),
        ExpressionType::UnaryPlus(inner) => {
            let inner = evaluate_expression(cx, *inner)?;
            match inner {
                // Unary + on a number has no effect
                Value::Rational(n) => Ok(Value::Rational(n)),
                // On all other types, it's an error.
                Value::String(_) => {
                    Err(make_error("Can't apply unary + operator to a string", span).into())
                }
                Value::Set { .. } => {
                    Err(make_error("Can't apply unary + operator to a set", span).into())
                }
                Value::Boolean(_) => {
                    Err(make_error("Can't apply unary + operator to a boolean", span).into())
                }
                Value::Type(_) => {
                    Err(make_error("Can't apply unary + operator to a type", span).into())
                }
                Value::Identifier(_) => {
                    Err(make_error("Can't apply unary + operator to an identifier", span).into())
                }
            }
        }
        ExpressionType::UnaryMinus(inner) => {
            let inner = evaluate_expression(cx, *inner)?;
            match inner {
                // Unary - on a number negates it
                Value::Rational(n) => Ok(Value::Rational(-n)),
                // On all other types, it's an error.
                Value::String(_) => {
                    Err(make_error("Can't apply unary - operator to a string", span).into())
                }
                Value::Set { .. } => {
                    Err(make_error("Can't apply unary - operator to a set", span).into())
                }
                Value::Boolean(_) => {
                    Err(make_error("Can't apply unary - operator to a boolean", span).into())
                }
                Value::Type(_) => {
                    Err(make_error("Can't apply unary - operator to a type", span).into())
                }
                Value::Identifier(_) => {
                    Err(make_error("Can't apply unary - operator to an identifier", span).into())
                }
            }
        }
        ExpressionType::UnaryNot(inner) => {
            let inner = evaluate_expression(cx, *inner)?;
            match inner {
                Value::Rational(_) => {
                    Err(make_error("Can't apply unary ! to a rational", span).into())
                }
                Value::String(_) => Err(make_error("Can't apply unary ! to a string", span).into()),
                Value::Set { .. } => Err(make_error("Can't apply unary ! to a set", span).into()),
                Value::Boolean(value) => Ok(Value::Boolean(!value)),
                Value::Type(_) => Err(make_error("Can't apply unary ! to a type", span).into()),
                Value::Identifier(_) => {
                    Err(make_error("Can't apply unary ! to an identifier", span).into())
                }
            }
        }
        ExpressionType::Attribute(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            println!("Evaluating attribute {} of {:?}", rhs, lhs);
            match lhs {
                Value::Set(lhs) => evaluate_set_attr(lhs, rhs, span),
                Value::Type(ty) => evaluate_type_attr(cx, ty, rhs, span),
                Value::Identifier(id) => match &*id {
                    "_offset_" => todo!("Offset not yet implemented"),
                    _ => Err(make_error(format!("Unrecognized identifier {}", id), span).into()),
                },
                _ => Err(make_error(format!("{} has no attribute {}", lhs.ty(), rhs), span).into()),
            }
        }
        ExpressionType::Exponent(base, exponent) => {
            let base = evaluate_expression(cx, *base)?;
            let exponent = evaluate_expression(cx, *exponent)?;
            calculate_exponent(base, exponent, span)
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
            calculate_elementwise_binary(lhs, rhs, span, "%", |lhs, rhs, _| {
                Ok(Value::Rational(lhs % rhs))
            })
        }
        ExpressionType::Add(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            // + concatenates strings, and also works on rationals and sets like several other operators.
            match (lhs, rhs) {
                (Value::String(mut lhs), Value::String(rhs)) => {
                    lhs.push_str(&rhs);
                    Ok(Value::String(lhs))
                }
                (lhs, rhs) => calculate_elementwise_binary(lhs, rhs, span, "+", |lhs, rhs, _| {
                    Ok(Value::Rational(lhs + rhs))
                }),
            }
        }
        ExpressionType::Subtract(lhs, rhs) => {
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            calculate_elementwise_binary(lhs, rhs, span, "-", |lhs, rhs, _| {
                Ok(Value::Rational(lhs - rhs))
            })
        }
        ExpressionType::BitOr(lhs, rhs) => {
            // a | b: Bitwise or on integers, or union of sets of the same type
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            calculate_rational_or_set_binary(
                lhs,
                rhs,
                span,
                "|",
                rational_bitwise_or,
                |lhs, rhs| lhs.union(&rhs).unwrap(),
            )
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
            )
        }
        ExpressionType::BitAnd(lhs, rhs) => {
            // a & b: Bitwise and on integers, or intersection of sets of the same type
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            calculate_rational_or_set_binary(
                lhs,
                rhs,
                span,
                "&",
                rational_bitwise_and,
                |lhs, rhs| lhs.intersection(&rhs).unwrap(),
            )
        }
        ExpressionType::Equal(lhs, rhs) => {
            // a == b: Rationals exactly equal, strings NFC equal, sets equal
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            let result = match (lhs, rhs) {
                // string == string
                (Value::String(lhs), Value::String(rhs)) => {
                    // StringValues are alrady NFC-normalized, so they can be compared directly.
                    Ok(lhs == rhs)
                }
                // boolean == boolean
                (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(lhs == rhs),
                // Fall back and check set/rational/string-as-int possibilities
                (lhs, rhs) => calculate_rational_or_set_comparison(
                    lhs,
                    rhs,
                    span,
                    "==",
                    |lhs, rhs| lhs == rhs,
                    |lhs, rhs| lhs == rhs,
                ),
            }?;
            Ok(Value::Boolean(result))
        }
        ExpressionType::NotEqual(lhs, rhs) => {
            // Opposite of equal
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            let result = match (lhs, rhs) {
                // string == string
                (Value::String(lhs), Value::String(rhs)) => {
                    // StringValues are already NFC-normalized, so they can be compared directly.
                    Ok(lhs != rhs)
                }
                // boolean != boolean
                (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(lhs != rhs),
                // Fall back and check set/rational/string-as-int possibilities
                (lhs, rhs) => calculate_rational_or_set_comparison(
                    lhs,
                    rhs,
                    span,
                    "!=",
                    |lhs, rhs| lhs != rhs,
                    |lhs, rhs| lhs != rhs,
                ),
            }?;
            Ok(Value::Boolean(result))
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
                (lhs, rhs) => Err(make_error(
                    format!("Can't calculate {} || {}", lhs.ty(), rhs.ty()),
                    span,
                )
                .into()),
            }
        }
        ExpressionType::LogicalAnd(lhs, rhs) => {
            // a && b for booleans only
            // Don't short circuit (the specification doesn't specify this)
            let lhs = evaluate_expression(cx, *lhs)?;
            let rhs = evaluate_expression(cx, *rhs)?;
            match (lhs, rhs) {
                (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(lhs && rhs)),
                (lhs, rhs) => Err(make_error(
                    format!("Can't calculate {} && {}", lhs.ty(), rhs.ty()),
                    span,
                )
                .into()),
            }
        }
    }
}

fn evaluate_atom(
    cx: &mut CompileContext,
    atom: ExpressionAtom,
    span: Span<'_>,
) -> Result<Value, Error> {
    match atom {
        ExpressionAtom::Parenthesized(inner) => evaluate_expression(cx, inner),
        ExpressionAtom::Type(ty) => Ok(Value::Type(convert_type(cx, ty)?)),
        ExpressionAtom::Literal(Literal { literal, span }) => match literal {
            LiteralType::Set(expressions) => {
                let set: Result<Result<Set, SetTypeError>, Error> = expressions
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
            println!("Evaluating identifier atom {}", identifier);
            // This can only be a constant
            match cx.constants().get(identifier) {
                Some(constant) => Ok(constant.value().clone()),
                None => Err(make_error(format!("Constant {} not found", identifier), span).into()),
            }
        }
    }
}

/// Evaluates an attribute of a set
fn evaluate_set_attr(lhs: Set, rhs: &str, span: Span<'_>) -> Result<Value, Error> {
    // Sets have min, max, and count attributes
    match rhs {
        "min" => evaluate_set_min(lhs, span),
        "max" => evaluate_set_max(lhs, span),
        "count" => Ok(Value::Rational(BigRational::from_integer(lhs.len().into()))),
        _ => Err(make_error(format!("Set does not have a {} attribute", rhs), span).into()),
    }
}

fn evaluate_set_min(lhs: Set, span: Span<'_>) -> Result<Value, Error> {
    match lhs.min_value() {
        Some(value) => Ok(value),
        None => match lhs.ty() {
            None => Err(make_error(
                "Set does not have a min attribute because it is empty",
                span,
            )
            .into()),
            Some(element_ty) => Err(make_set_min_max_gt_undefined_error("min", element_ty, span)),
        },
    }
}

fn evaluate_set_max(lhs: Set, span: Span<'_>) -> Result<Value, Error> {
    match lhs.max_value() {
        Some(value) => Ok(value),
        None => match lhs.ty() {
            None => Err(make_error(
                "Set does not have a min attribute because it is empty",
                span,
            )
            .into()),
            Some(element_ty) => Err(make_set_min_max_gt_undefined_error("max", element_ty, span)),
        },
    }
}

fn make_set_min_max_gt_undefined_error(
    attribute: &str,
    element_ty: ExprType,
    span: Span<'_>,
) -> Error {
    make_error(
        format!(
            "Set does not have a {} attribute because the < operator is not defined for its element type ({})",
            attribute,
            element_ty
        ),
        span,
    ).into()
}

fn evaluate_type_attr(
    cx: &mut CompileContext,
    ty: Type,
    rhs: &str,
    span: Span<'_>,
) -> Result<Value, Error> {
    match ty {
        Type::Scalar(ty) => {
            match ty {
                ScalarType::Versioned(ty) => {
                    // Recursion!
                    // Look up the type that this refers to and check its properties
                    let ty_compiled = cx.get_by_key(&ty)?;

                    match &ty_compiled.kind {
                        DsdlKind::Message { constants, .. } => {
                            // Look up the constant
                            match constants.get(rhs) {
                                Some(constant) => Ok(constant.value().clone()),
                                None => Err(make_error(
                                    format!("Type {} has no attribute {}", ty, rhs),
                                    span,
                                )
                                .into()),
                            }
                        }
                        DsdlKind::Service { .. } => {
                            // A service type can't be named
                            Err(make_error(
                                format!("Type {} has no attributes because it is a service", ty),
                                span,
                            )
                            .into())
                        }
                    }
                }
                _ => Err(make_error(format!("Type {} has no attribute {}", ty, rhs), span).into()),
            }
        }
        _ => Err(make_error(format!("Type {} has no attribute {}", ty, rhs), span).into()),
    }
}

fn evaluate_array_length(cx: &mut CompileContext, length: Expression<'_>) -> Result<BigInt, Error> {
    let length_span = length.span.clone();
    match evaluate_expression(cx, length)? {
        Value::Rational(rational) => {
            if rational.is_integer() {
                let length = rational.numer().clone();
                if length.is_negative() {
                    return Err(make_error(
                        format!("Array length evaluated to negative value {}", length),
                        length_span,
                    )
                    .into());
                } else {
                    Ok(length)
                }
            } else {
                return Err(make_error(
                    format!("Array length evaluated to non-integer value {}", rational),
                    length_span,
                )
                .into());
            }
        }
        other => {
            return Err(make_error(
                format!("Array length evaluated to non-rational type {}", other.ty()),
                length_span,
            )
            .into())
        }
    }
}

fn calculate_exponent(base: Value, exponent: Value, span: Span<'_>) -> Result<Value, Error> {
    calculate_elementwise_binary(base, exponent, span, "**", rational_pow)
}

/// Calculates the result of a binary operation that can take the form
/// `rational op rational -> rational`, `set<rational> op rational -> set<rational>`, or
/// `rational op set<rational> -> set<rational>`
///
/// This also accounts for strings that can implicitly convert to integers.
///
/// When one input is a set, the operation will be applied to each element of the set. The result
/// will have the operation applied between each element and the other (rational) input.
///
/// symbol should be a short text representation of the operator, which will be used in error messages.
///
/// rational_op should be a function that applies the operator to two rational values and returns the
/// result or an error.
fn calculate_elementwise_binary<F>(
    lhs: Value,
    rhs: Value,
    span: Span,
    symbol: &str,
    mut rational_op: F,
) -> Result<Value, Error>
where
    F: FnMut(BigRational, BigRational, Span<'_>) -> Result<Value, Error>,
{
    match (lhs, rhs) {
        // rational op set<rational>
        (Value::Rational(lhs), Value::Set(set)) if set.can_hold(ExprType::Rational) => {
            // Apply elementwise
            let new_elements: Result<Set, SetTypeError> = set
                .into_iter()
                .map(|element| match element {
                    Value::Rational(rhs) => rational_op(lhs.clone(), rhs, span.clone()),
                    _ => panic!(
                        "Element {:?} does not match set type {}",
                        element,
                        ExprType::Rational
                    ),
                })
                .collect::<Result<_, _>>()?;
            let new_elements = new_elements.map_err(|e| make_set_error(e, span))?;
            Ok(Value::Set(new_elements))
        }
        // set<rational> op rational
        (Value::Set(set), Value::Rational(rhs)) if set.can_hold(ExprType::Rational) => {
            let new_elements: Result<Set, SetTypeError> = set
                .into_iter()
                .map(|element| match element {
                    Value::Rational(lhs) => rational_op(lhs, rhs.clone(), span.clone()),
                    _ => panic!(
                        "Element {:?} does not match set type {}",
                        element,
                        ExprType::Rational
                    ),
                })
                .collect::<Result<_, _>>()?;
            let new_elements = new_elements.map_err(|e| make_set_error(e, span))?;
            Ok(Value::Set(new_elements))
        }
        // Fall back and check rational/string possibilities
        (lhs, rhs) => calculate_rational_binary(lhs, rhs, span, symbol, rational_op),
    }
}

/// Calculates the result of a binary operation that can take the form
/// `rational op rational -> rational` or `set<T> op set<T> -> set<T>` for any `T`
///
/// This also accounts for strings that can implicitly convert to integers.
///
/// symbol should be a short text representation of the operator, which will be used in error messages.
///
/// rational_op should be a function that applies the operator to two rational values and returns the
/// result or an error.
///
/// set_op shold be a function that applies the operator to two sets and returns the result.
fn calculate_rational_or_set_binary<F, G>(
    lhs: Value,
    rhs: Value,
    span: Span<'_>,
    symbol: &str,
    rational_op: F,
    mut set_op: G,
) -> Result<Value, Error>
where
    F: FnMut(BigRational, BigRational, Span<'_>) -> Result<Value, Error>,
    G: FnMut(Set, Set) -> Set,
{
    match (lhs, rhs) {
        // set<T> op set<T> -> set<T>
        (Value::Set(lhs), Value::Set(rhs)) if lhs.is_compatible(&rhs) => {
            let result = set_op(lhs, rhs);
            Ok(Value::Set(result))
        }
        // Fall back and check rational/string possibilities
        (lhs, rhs) => calculate_rational_binary(lhs, rhs, span, symbol, rational_op),
    }
}

/// Calculates the result of a comparison operator that evaluates to a boolean
///
/// The operator takes the form `rational op rational -> bool` or `set<T> op set<T> -> bool`,
/// also accounting for strings that implicitly convert to integers.
///
/// symbol should be a short text representation of the operator, which will be used in error messages.
///
/// rational_op should be a function that applies the operator to two rational values and returns
/// the result.
///
/// set_op should be a function that applies the operator to two sets and returns
/// the result.
fn calculate_rational_or_set_comparison<F, G>(
    lhs: Value,
    rhs: Value,
    span: Span<'_>,
    symbol: &str,
    mut rational_op: F,
    mut set_op: G,
) -> Result<bool, Error>
where
    F: FnMut(BigRational, BigRational) -> bool,
    G: FnMut(Set, Set) -> bool,
{
    match (lhs, rhs) {
        // set<T> op set<T> -> bool
        (Value::Set(lhs), Value::Set(rhs)) if lhs.is_compatible(&rhs) => Ok(set_op(lhs, rhs)),
        // Fall back and try rational/string-as-integer possibilities
        (lhs, rhs) => {
            let result_value = calculate_rational_binary(lhs, rhs, span, symbol, |lhs, rhs, _| {
                Ok(Value::Boolean(rational_op(lhs, rhs)))
            })?;
            match result_value {
                Value::Boolean(value) => Ok(value),
                _ => unreachable!("Value is not boolean"),
            }
        }
    }
}

/// Calculates the result of a binary operation that can take the form
/// `rational op rational -> rational`
///
/// This also accounts for strings that can implicitly convert to integers.
///
/// symbol should be a short text representation of the operator, which will be used in error messages.
///
/// rational_op should be a function that applies the operator to two rational values and returns the
/// result or an error.
fn calculate_rational_binary<F>(
    lhs: Value,
    rhs: Value,
    span: Span<'_>,
    symbol: &str,
    mut rational_op: F,
) -> Result<Value, Error>
where
    F: FnMut(BigRational, BigRational, Span<'_>) -> Result<Value, Error>,
{
    match (lhs, rhs) {
        // rational op rational -> rational
        (Value::Rational(lhs), Value::Rational(rhs)) => {
            let result = rational_op(lhs, rhs, span)?;
            Ok(result)
        }
        // integer-convertible-string op rational -> rational
        (Value::String(lhs), Value::Rational(rhs)) if lhs.implicit_int().is_some() => {
            let lhs = BigRational::from_integer(lhs.implicit_int().unwrap().into());
            let result = rational_op(lhs, rhs, span)?;
            Ok(result)
        }
        // rational op integer-convertible-string -> rational
        (Value::Rational(lhs), Value::String(rhs)) if rhs.implicit_int().is_some() => {
            let rhs = BigRational::from_integer(rhs.implicit_int().unwrap().into());
            let result = rational_op(lhs, rhs, span)?;
            Ok(result)
        }
        // integer-convertible-string op integer-convertible-string -> rational
        (Value::String(lhs), Value::String(rhs))
            if lhs.implicit_int().is_some() && rhs.implicit_int().is_some() =>
        {
            let lhs = BigRational::from_integer(lhs.implicit_int().unwrap().into());
            let rhs = BigRational::from_integer(rhs.implicit_int().unwrap().into());
            let result = rational_op(lhs, rhs, span)?;
            Ok(result)
        }
        (lhs, rhs) => Err(make_error(
            format!("Can't calculate {} {} {}", lhs.ty(), symbol, rhs.ty()),
            span,
        )
        .into()),
    }
}

/// Calculates the result of raising a single rational to the power ofo another rational
fn rational_pow(base: BigRational, exponent: BigRational, span: Span<'_>) -> Result<Value, Error> {
    if exponent.is_integer() {
        // Exact power
        let power = exponent.numer();
        Ok(Value::Rational(Pow::pow(base, power)))
    } else {
        // Approximate power as a u64
        approx_pow(base, exponent, span)
    }
}

/// Calculates the approximate value of a base raised to an exponent
///
/// This function returns an error if the conversion between BigRational and f64 fails.
fn approx_pow(base: BigRational, exponent: BigRational, span: Span<'_>) -> Result<Value, Error> {
    let base = base.to_f64().ok_or_else(|| {
        make_error(
            format!(
                "Can't convert base {} to a floating-point approximation",
                base
            ),
            span.clone(),
        )
    })?;
    let exponent = exponent.to_f64().ok_or_else(|| {
        make_error(
            format!(
                "Can't convert exponent {} to a floating-point approximation",
                exponent
            ),
            span.clone(),
        )
    })?;

    let result = base.powf(exponent);
    let result = BigRational::from_f64(result).ok_or_else(|| {
        make_error(
            format!(
                "Can't convert exponent result {} back to a rational",
                result
            ),
            span,
        )
    })?;
    Ok(Value::Rational(result))
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
        Err(make_error(
            format!(
                "Can't calculate {} ^ {}: Both operands must be integers",
                lhs, rhs
            ),
            span,
        )
        .into())
    }
}

fn rational_bitwise_and(
    lhs: BigRational,
    rhs: BigRational,
    span: Span<'_>,
) -> Result<Value, Error> {
    if lhs.is_integer() && rhs.is_integer() {
        let result = lhs.numer() & rhs.numer();
        Ok(Value::Rational(BigRational::from_integer(result)))
    } else {
        Err(make_error(
            format!(
                "Can't calculate {} & {}: Both operands must be integers",
                lhs, rhs
            ),
            span,
        )
        .into())
    }
}

fn rational_bitwise_or(lhs: BigRational, rhs: BigRational, span: Span<'_>) -> Result<Value, Error> {
    if lhs.is_integer() && rhs.is_integer() {
        let result = lhs.numer() | rhs.numer();
        Ok(Value::Rational(BigRational::from_integer(result)))
    } else {
        Err(make_error(
            format!(
                "Can't calculate {} | {}: Both operands must be integers",
                lhs, rhs
            ),
            span,
        )
        .into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exponent_rational_rational() {
        let span = Span::new("2 ** 3", 0, 6).unwrap();
        let result = calculate_exponent(int_value(2), int_value(3), span).unwrap();
        assert_eq!(result, int_value(8));
    }

    #[test]
    fn exponent_rational_set() {
        let span = Span::new("2 ** 3", 0, 6).unwrap();
        let result = calculate_exponent(int_value(3), int_set_value([0, 1, 2, 5]), span).unwrap();
        assert_eq!(result, int_set_value([1, 3, 9, 243]));
    }

    #[test]
    fn exponent_set_rational() {
        let span = Span::new("2 ** 3", 0, 6).unwrap();
        let result = calculate_exponent(int_set_value([0, 1, 4, 9]), int_value(2), span).unwrap();
        assert_eq!(result, int_set_value([0, 1, 16, 81]));
    }

    /// Creates a value of type set<rational> containing the provided integers
    fn int_set_value<I>(values: I) -> Value
    where
        I: IntoIterator<Item = i32>,
    {
        let set = values
            .into_iter()
            .map(|value| Value::Rational(BigRational::from_integer(value.into())))
            .collect::<Result<_, _>>()
            .unwrap();
        Value::Set(set)
    }

    /// Creates a value of type rational representing an integer
    fn int_value(value: i32) -> Value {
        Value::Rational(BigRational::from_integer(value.into()))
    }
}

fn make_set_error(e: SetTypeError, span: Span<'_>) -> Error {
    make_error(format!("Invalid type in set: {}", e), span).into()
}

/// Converts an AST type into a compiler type
fn convert_type(
    cx: &mut CompileContext,
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
                    let length_span = length.span.clone();
                    let length = evaluate_array_length(cx, length)?;
                    if length.is_positive() {
                        // Convert to inclusive length by subtracting 1
                        Ok(Type::VariableArray {
                            inner: element,
                            max_len: length - 1,
                        })
                    } else {
                        Err(make_error(
                            format!(
                                "Non-inclusive array length evaluated to non-positive value {}",
                                length
                            ),
                            length_span,
                        )
                        .into())
                    }
                }
            }
        }
    }
}
