use crate::error::Error;
use crate::operators::calculate_elementwise_binary;
use crate::types::Value;
use canadensis_dsdl_parser::Span;
use num_rational::BigRational;
use num_traits::{FromPrimitive, Pow, ToPrimitive};

/// Evaluates the exponentiation operator `expr ** expr`
pub(crate) fn evaluate(base: Value, exponent: Value, span: Span<'_>) -> Result<Value, Box<Error>> {
    calculate_elementwise_binary(base, exponent, span, "**", rational_exponent)
}

/// Calculates the result of raising a single rational to the power of another rational
fn rational_exponent(
    base: BigRational,
    exponent: BigRational,
    span: Span<'_>,
) -> Result<Value, Box<Error>> {
    if exponent.is_integer() {
        // Exact power
        let power = exponent.numer();
        Ok(Value::Rational(Pow::pow(base, power)))
    } else {
        // Approximate power as a u64
        approx_exponent(base, exponent, span)
    }
}

/// Calculates the approximate value of a base raised to an exponent
///
/// # Errors
///
/// This function returns an error if the conversion between `BigRational` and `f64` fails.
fn approx_exponent(
    base: BigRational,
    exponent: BigRational,
    span: Span<'_>,
) -> Result<Value, Box<Error>> {
    let base = base.to_f64().ok_or_else(|| {
        span_error!(
            span,
            "Can't convert base {} to a floating-point approximation",
            base
        )
    })?;
    let exponent = exponent.to_f64().ok_or_else(|| {
        span_error!(
            span,
            "Can't convert exponent {} to a floating-point approximation",
            exponent
        )
    })?;

    let result = base.powf(exponent);
    let result = BigRational::from_f64(result).ok_or_else(|| {
        span_error!(
            span,
            "Can't convert exponent result {} back to a rational",
            result
        )
    })?;
    Ok(Value::Rational(result))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exponent_rational_rational() {
        let span = Span::new("2 ** 3", 0, 6).unwrap();
        let result = evaluate(int_value(2), int_value(3), span).unwrap();
        assert_eq!(result, int_value(8));
    }

    #[test]
    fn exponent_rational_set() {
        let span = Span::new("2 ** 3", 0, 6).unwrap();
        let result = evaluate(int_value(3), int_set_value([0, 1, 2, 5]), span).unwrap();
        assert_eq!(result, int_set_value([1, 3, 9, 243]));
    }

    #[test]
    fn exponent_set_rational() {
        let span = Span::new("2 ** 3", 0, 6).unwrap();
        let result = evaluate(int_set_value([0, 1, 4, 9]), int_value(2), span).unwrap();
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
