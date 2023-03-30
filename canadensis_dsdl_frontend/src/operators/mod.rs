//! All DSDL operators

pub(crate) mod add;
pub(crate) mod attribute;
pub(crate) mod bit_and;
pub(crate) mod bit_or;
pub(crate) mod equal;
pub(crate) mod exponent;
pub(crate) mod not_equal;
pub(crate) mod unary_minus;
pub(crate) mod unary_not;
pub(crate) mod unary_plus;

use crate::error::Error;
use crate::types::set::{Set, TypeError};
use crate::types::{ExprType, Value};
use canadensis_bit_length_set::BitLengthSet;
use canadensis_dsdl_parser::Span;
use num_rational::BigRational;

/// Calculates the result of a binary operation that can take the form
/// `rational op rational -> rational`, `set<rational> op rational -> set<rational>`, or
/// `rational op set<rational> -> set<rational>`
///
/// This also accounts for strings that can implicitly convert to integers.
///
/// When one input is a set, the operation will be applied to each element of the set. The result
/// will have the operation applied between each element and the other (rational) input.
///
/// `symbol` should be a short text representation of the operator, which will be used in error messages.
///
/// `rational_op` should be a function that applies the operator to two rational values and returns the
/// result or an error.
pub(crate) fn calculate_elementwise_binary<F>(
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
            let new_elements: Result<Set, TypeError> = set
                .into_iter()
                .map(|element| match element {
                    Value::Rational(rhs) => rational_op(lhs.clone(), rhs, span),
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
            let new_elements: Result<Set, TypeError> = set
                .into_iter()
                .map(|element| match element {
                    Value::Rational(lhs) => rational_op(lhs, rhs.clone(), span),
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
        // If either side is a BitLengthSet, expand it into a normal set and try again
        (Value::BitLengthSet(lhs), rhs) => calculate_elementwise_binary(
            Value::Set(Set::from(lhs.expand())),
            rhs,
            span,
            symbol,
            rational_op,
        ),
        (lhs, Value::BitLengthSet(rhs)) => calculate_elementwise_binary(
            lhs,
            Value::Set(Set::from(rhs.expand())),
            span,
            symbol,
            rational_op,
        ),
        // Fall back and check rational/string possibilities
        (lhs, rhs) => calculate_rational_binary(lhs, rhs, span, symbol, rational_op),
    }
}

/// Calculates the result of a binary operation that can take the form
/// `rational op rational -> rational` or `set<T> op set<T> -> set<T>` for any `T`
///
/// This also accounts for strings that can implicitly convert to integers.
///
/// `symbol` should be a short text representation of the operator, which will be used in error messages.
///
/// `rational_op` should be a function that applies the operator to two rational values and returns the
/// result or an error.
///
/// `set_op` should be a function that applies the operator to two sets and returns the result.
///
/// `bit_length_set_op` should be a function that applies the operator to two bit length sets and
/// returns the result. The result may be a BitLengthSet or a Set.
pub(crate) fn calculate_rational_or_set_binary<F, G, H>(
    lhs: Value,
    rhs: Value,
    span: Span<'_>,
    symbol: &str,
    rational_op: F,
    mut set_op: G,
    mut bit_length_set_op: H,
) -> Result<Value, Error>
where
    F: FnMut(BigRational, BigRational, Span<'_>) -> Result<Value, Error>,
    G: FnMut(Set, Set) -> Set,
    H: FnMut(BitLengthSet, BitLengthSet) -> Value,
{
    match (lhs, rhs) {
        // set<T> op set<T> -> set<T>
        (Value::Set(lhs), Value::Set(rhs)) if lhs.is_compatible(&rhs) => {
            let result = set_op(lhs, rhs);
            Ok(Value::Set(result))
        }
        (Value::BitLengthSet(lhs), Value::BitLengthSet(rhs)) => Ok(bit_length_set_op(lhs, rhs)),
        // Fall back and check rational/string possibilities
        (lhs, rhs) => calculate_rational_binary(lhs, rhs, span, symbol, rational_op),
    }
}

/// Calculates the result of a comparison operator that evaluates to a boolean
///
/// The operator takes the form `rational op rational -> bool` or `set<T> op set<T> -> bool`,
/// also accounting for strings that implicitly convert to integers.
///
/// `symbol` should be a short text representation of the operator, which will be used in error messages.
///
/// `rational_op` should be a function that applies the operator to two rational values and returns
/// the result.
///
/// `set_op` should be a function that applies the operator to two sets and returns
/// the result.
pub(crate) fn calculate_rational_or_set_comparison<F, G>(
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
        // If either side is a BitLengthSet, expand it into a normal set and try again
        (Value::BitLengthSet(lhs), rhs) => {
            let lhs = Value::Set(Set::from(lhs.expand()));
            calculate_rational_or_set_comparison(lhs, rhs, span, symbol, rational_op, set_op)
        }
        (lhs, Value::BitLengthSet(rhs)) => {
            let rhs = Value::Set(Set::from(rhs.expand()));
            calculate_rational_or_set_comparison(lhs, rhs, span, symbol, rational_op, set_op)
        }
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
/// `symbol` should be a short text representation of the operator, which will be used in error messages.
///
/// `rational_op` should be a function that applies the operator to two rational values and returns the
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
        (lhs, rhs) => Err(span_error!(
            span,
            "Can't calculate {} {} {}",
            lhs.ty(),
            symbol,
            rhs.ty()
        )),
    }
}

pub(crate) fn make_set_error(e: TypeError, span: Span<'_>) -> Error {
    span_error!(span, "Invalid type in set: {}", e)
}
