use crate::compile::CompileContext;
use crate::compiled::DsdlKind;
use crate::error::Error;
use crate::types::set::Set;
use crate::types::{ExprType, ScalarType, Type, Value};
use canadensis_bit_length_set::BitLengthSet;
use canadensis_dsdl_parser::Span;

/// Evaluates the attribute operator `expr.attribute`
pub(crate) fn evaluate(
    cx: &mut CompileContext<'_, '_>,
    lhs: Value,
    rhs: &str,
    span: Span<'_>,
) -> Result<Value, Error> {
    match lhs {
        Value::Set(lhs) => evaluate_set_attr(lhs, rhs, span),
        Value::BitLengthSet(lhs) => evaluate_bit_length_set_attr(lhs, rhs, span),
        Value::Type(ty) => evaluate_type_attr(cx, ty, rhs, span),
        Value::Rational(_) | Value::String(_) | Value::Boolean(_) => {
            Err(span_error!(span, "{} has no attribute {}", lhs.ty(), rhs))
        }
    }
}

/// Evaluates an attribute of a set
fn evaluate_set_attr(lhs: Set, rhs: &str, span: Span<'_>) -> Result<Value, Error> {
    // Sets have min, max, and count attributes
    match rhs {
        "min" => evaluate_set_min(lhs, span),
        "max" => evaluate_set_max(lhs, span),
        "count" => Ok(lhs.len().into()),
        _ => Err(span_error!(span, "Set does not have a {} attribute", rhs)),
    }
}

/// Evaluates an attribute of a bit length set
fn evaluate_bit_length_set_attr(
    lhs: BitLengthSet,
    rhs: &str,
    span: Span<'_>,
) -> Result<Value, Error> {
    // Sets have min, max, and count attributes
    match rhs {
        "min" => Ok(lhs.min_value().into()),
        "max" => Ok(lhs.max_value().into()),
        "count" => Ok(lhs.expand().len().into()),
        _ => Err(span_error!(span, "Set does not have a {} attribute", rhs)),
    }
}

fn evaluate_set_min(lhs: Set, span: Span<'_>) -> Result<Value, Error> {
    match lhs.min_value() {
        Some(value) => Ok(value),
        None => match lhs.ty() {
            None => Err(span_error!(
                span,
                "Set does not have a min attribute because it is empty",
            )),
            Some(element_ty) => Err(make_set_min_max_gt_undefined_error("min", element_ty, span)),
        },
    }
}

fn evaluate_set_max(lhs: Set, span: Span<'_>) -> Result<Value, Error> {
    match lhs.max_value() {
        Some(value) => Ok(value),
        None => match lhs.ty() {
            None => Err(span_error!(
                span,
                "Set does not have a min attribute because it is empty",
            )),
            Some(element_ty) => Err(make_set_min_max_gt_undefined_error("max", element_ty, span)),
        },
    }
}

fn make_set_min_max_gt_undefined_error(
    attribute: &str,
    element_ty: ExprType,
    span: Span<'_>,
) -> Error {
    span_error!(span,
            "Set does not have a {} attribute because the < operator is not defined for its element type ({})",
            attribute,
            element_ty)
}

fn evaluate_type_attr(
    cx: &mut CompileContext<'_, '_>,
    ty: Type,
    rhs: &str,
    span: Span<'_>,
) -> Result<Value, Error> {
    // The _bit_length_ special attribute is not part of the specification (v1.0-beta),
    // but pydsdl implements it and some of the public regulated data types use it.
    match rhs {
        "_bit_length_" => {
            let bit_length = ty.bit_length_set(cx, span)?;
            Ok(Value::BitLengthSet(bit_length))
        }
        _ => match ty {
            Type::Scalar(ty) => {
                match ty {
                    ScalarType::Versioned(ty) => {
                        // Recursion!
                        // Look up the type that this refers to and check its properties
                        let (ty, ty_compiled) = cx.type_by_key(ty)?;

                        match &ty_compiled.kind {
                            DsdlKind::Message(message) => {
                                // Look up the constant
                                match message.constants().get(rhs) {
                                    Some(constant) => Ok(constant.dsdl_value().clone()),
                                    None => Err(span_error!(
                                        span,
                                        "Type {} has no attribute {}",
                                        ty,
                                        rhs
                                    )),
                                }
                            }
                            DsdlKind::Service { .. } => {
                                // A service type can't be named and its constants are not accessible
                                Err(span_error!(
                                    span,
                                    "Type {} has no attributes because it is a service",
                                    ty
                                ))
                            }
                        }
                    }
                    _ => Err(span_error!(span, "Type {} has no attribute {}", ty, rhs)),
                }
            }
            _ => Err(span_error!(span, "Type {} has no attribute {}", ty, rhs)),
        },
    }
}
