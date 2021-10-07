use crate::error::Error;
use crate::operators::calculate_rational_or_set_binary;
use crate::types::Value;
use canadensis_dsdl_parser::Span;
use num_rational::BigRational;

/// Evaluates the bitwise and operator `expr & expr`
pub fn evaluate(lhs: Value, rhs: Value, span: Span<'_>) -> Result<Value, Error> {
    // a & b: Bitwise and on integers, or intersection of sets of the same type
    calculate_rational_or_set_binary(lhs, rhs, span, "&", rational_bitwise_and, |lhs, rhs| {
        lhs.intersection(&rhs).unwrap()
    })
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
        Err(span_error!(
            span,
            "Can't calculate {} & {}: Both operands must be integers",
            lhs,
            rhs
        ))
    }
}
