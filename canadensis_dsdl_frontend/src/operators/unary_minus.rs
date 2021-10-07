use crate::error::Error;
use crate::types::Value;
use canadensis_dsdl_parser::Span;

/// Evaluates the unary minus operator `-expr`
pub fn evaluate(inner: Value, span: Span<'_>) -> Result<Value, Error> {
    match inner {
        // Unary - on a number negates it
        Value::Rational(n) => Ok(Value::Rational(-n)),
        // On all other types, it's an error.
        Value::String(_) => Err(span_error!(
            span,
            "Can't apply unary - operator to a string"
        )),
        Value::Set { .. } => Err(span_error!(span, "Can't apply unary - operator to a set")),
        Value::Boolean(_) => Err(span_error!(
            span,
            "Can't apply unary - operator to a boolean"
        )),
        Value::Type(_) => Err(span_error!(span, "Can't apply unary - operator to a type")),
        Value::Identifier(_) => Err(span_error!(
            span,
            "Can't apply unary - operator to an identifier"
        )),
    }
}
