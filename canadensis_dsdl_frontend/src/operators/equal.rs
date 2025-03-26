use crate::error::Error;
use crate::operators::calculate_rational_or_set_comparison;
use crate::types::Value;
use canadensis_dsdl_parser::Span;

/// Evaluates the equality operator `expr == expr`
pub(crate) fn evaluate(lhs: Value, rhs: Value, span: Span<'_>) -> Result<Value, Box<Error>> {
    let result = match (lhs, rhs) {
        // string == string
        (Value::String(lhs), Value::String(rhs)) => {
            // StringValues are already NFC-normalized, so they can be compared directly.
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
