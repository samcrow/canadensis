use crate::error::Error;
use crate::operators::calculate_rational_or_set_binary;
use crate::types::Value;
use canadensis_dsdl_parser::Span;
use num_rational::BigRational;

/// Evaluates the bitwise or operator `expr | expr`
pub(crate) fn evaluate(lhs: Value, rhs: Value, span: Span<'_>) -> Result<Value, Box<Error>> {
    // a | b: Bitwise or on integers, or union of sets of the same type
    calculate_rational_or_set_binary(
        lhs,
        rhs,
        span,
        "|",
        rational_bitwise_or,
        |lhs, rhs| lhs.union(&rhs).unwrap(),
        |lhs, rhs| Value::BitLengthSet(lhs.unite([rhs])),
    )
}

fn rational_bitwise_or(
    lhs: BigRational,
    rhs: BigRational,
    span: Span<'_>,
) -> Result<Value, Box<Error>> {
    if lhs.is_integer() && rhs.is_integer() {
        let result = lhs.numer() | rhs.numer();
        Ok(Value::Rational(BigRational::from_integer(result)))
    } else {
        Err(span_error!(
            span,
            "Can't calculate {} | {}: Both operands must be integers",
            lhs,
            rhs
        ))
    }
}
