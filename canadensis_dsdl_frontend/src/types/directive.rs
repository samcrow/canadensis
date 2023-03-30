use crate::compile::CompileContext;
use crate::error::Error;
use crate::types::expression::evaluate_expression;
use crate::types::Value;
use canadensis_dsdl_parser::{Expression, Identifier};
use num_traits::{Signed, ToPrimitive};

pub(crate) fn evaluate_directive(
    cx: &mut CompileContext<'_>,
    name: Identifier<'_>,
    expression: Option<Expression<'_>>,
) -> Result<(), Error> {
    match name.name {
        "union" => {
            if expression.is_none() {
                cx.handle_union(name.span)
            } else {
                Err(span_error!(
                    name.span,
                    "union directive must not have an associated expression"
                ))
            }
        }
        "extent" => match expression {
            Some(expression) => {
                let expression_span = expression.span;
                match evaluate_expression(cx, expression)? {
                    Value::Rational(value) => {
                        if value.is_integer() && !value.is_negative() {
                            let value = value.numer();
                            // Try to fit into a u64
                            match value.to_u64() {
                                Some(value) => {
                                    if value % 8 == 0 {
                                        cx.handle_extent(name.span, value)
                                    } else {
                                        Err(span_error!(
                                            expression_span,
                                            "Extent value {} is not a multiple of 8 bits",
                                            value
                                        ))
                                    }
                                }
                                None => Err(span_error!(
                                    expression_span,
                                    "Extent value {} is too large to fit into 64 bits",
                                    value
                                )),
                            }
                        } else {
                            Err(span_error!(
                                expression_span,
                                "Extent value {} is negative or not an integer",
                                value
                            ))
                        }
                    }
                    other => Err(span_error!(
                        expression_span,
                        "Extent value is a {} (expected a rational)",
                        other.ty()
                    )),
                }
            }
            None => Err(span_error!(name.span, "Extent directive must have a value")),
        },
        "sealed" => {
            if expression.is_none() {
                cx.handle_sealed(name.span)
            } else {
                Err(span_error!(
                    name.span,
                    "sealed directive must not have an associated expression"
                ))
            }
        }
        "deprecated" => {
            if expression.is_none() {
                cx.handle_deprecated(name.span)
            } else {
                Err(span_error!(
                    name.span,
                    "deprecated directive must not have an associated expression"
                ))
            }
        }
        "assert" => match expression {
            Some(expr) => {
                let expr_span = expr.span;
                match evaluate_expression(cx, expr)? {
                    Value::Boolean(true) => Ok(()),
                    Value::Boolean(false) => Err(span_error!(expr_span, "Assertion failed")),
                    other => Err(span_error!(
                        expr_span,
                        "Assert expression evaluated to non-boolean value {}",
                        other
                    )),
                }
            }
            None => Err(span_error!(name.span, "Assert directive has no expression")),
        },
        "print" => {
            if let Some(expr) = expression {
                let value = evaluate_expression(cx, expr)?;
                // TODO: Should this be printed in some other way?
                println!("{}", value);
                Ok(())
            } else {
                // Print with no expression has no effect (perhaps it should print a new line)
                Ok(())
            }
        }
        _ => Err(span_error!(
            name.span,
            "Unrecognized directive {}",
            name.name
        )),
    }
}
