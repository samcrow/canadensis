use crate::error::Error;
use crate::operators::calculate_rational_or_set_binary;
use crate::types::set::Set;
use crate::types::Value;
use canadensis_dsdl_parser::Span;
use num_rational::BigRational;
use std::collections::BTreeSet;

/// Evaluates the bitwise and operator `expr & expr`
pub(crate) fn evaluate(lhs: Value, rhs: Value, span: Span<'_>) -> Result<Value, Box<Error>> {
    // a & b: Bitwise and on integers, or intersection of sets of the same type
    calculate_rational_or_set_binary(
        lhs,
        rhs,
        span,
        "&",
        rational_bitwise_and,
        |lhs, rhs| lhs.intersection(&rhs).unwrap(),
        |lhs, rhs| {
            // Expand bit length sets into normal sets and intersect them
            let lhs = lhs.expand();
            let rhs = rhs.expand();
            Value::Set(Set::from(
                lhs.intersection(&rhs).copied().collect::<BTreeSet<u64>>(),
            ))
        },
    )
}

fn rational_bitwise_and(
    lhs: BigRational,
    rhs: BigRational,
    span: Span<'_>,
) -> Result<Value, Box<Error>> {
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
