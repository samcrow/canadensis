use crate::error::Error;
use crate::operators::calculate_elementwise_binary;
use crate::types::Value;
use canadensis_dsdl_parser::Span;

/// Evaluates the addition operator `expr + expr`
pub(crate) fn evaluate(lhs: Value, rhs: Value, span: Span<'_>) -> Result<Value, Error> {
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
