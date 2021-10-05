use crate::compile::CompileContext;
use crate::package::Error;
use crate::types::expression::evaluate_expression;
use crate::types::Value;
use canadensis_dsdl_parser::{make_error, Expression, Identifier};
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
                Err(make_error(
                    "union directive must not have an associated expression",
                    name.span,
                )
                .into())
            }
        }
        "extent" => match expression {
            Some(expression) => {
                let expression_span = expression.span.clone();
                match evaluate_expression(cx, expression)? {
                    Value::Rational(value) => {
                        if value.is_integer() && !value.is_negative() {
                            let value = value.denom();
                            // Try to fit into a u64
                            match value.to_u64() {
                                Some(value) => cx.handle_extent(name.span, value),
                                None => Err(make_error(
                                    format!(
                                        "Extent value {} is too large to fit into 64 bits",
                                        value
                                    ),
                                    expression_span,
                                )
                                .into()),
                            }
                        } else {
                            Err(make_error(
                                format!("Extent value {} is negative or not an integer", value),
                                expression_span,
                            )
                            .into())
                        }
                    }
                    other => Err(make_error(
                        format!("Extent value is a {} (expected a rational)", other.ty()),
                        expression_span,
                    )
                    .into()),
                }
            }
            None => Err(make_error("Extent directive must have a value", name.span).into()),
        },
        "sealed" => {
            if expression.is_none() {
                cx.handle_sealed(name.span)
            } else {
                Err(make_error(
                    "sealed directive must not have an associated expression",
                    name.span,
                )
                .into())
            }
        }
        "deprecated" => {
            if expression.is_none() {
                cx.handle_deprecated(name.span)
            } else {
                Err(make_error(
                    "deprecated directive must not have an associated expression",
                    name.span,
                )
                .into())
            }
        }
        "assert" => match expression {
            Some(expr) => {
                let expr_span = expr.span.clone();
                match evaluate_expression(cx, expr)? {
                    Value::Boolean(true) => Ok(()),
                    Value::Boolean(false) => Err(make_error("Assertion failed", expr_span).into()),
                    other => Err(make_error(
                        format!("Assert expression evaluated to non-boolean value {}", other),
                        expr_span,
                    )
                    .into()),
                }
            }
            None => Err(make_error("Assert directive has no expression", name.span).into()),
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
        _ => Err(make_error(format!("Unrecognized directive {}", name.name), name.span).into()),
    }
}
