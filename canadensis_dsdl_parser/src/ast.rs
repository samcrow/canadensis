//! Stage 2 of the parser, which converts a Pairs tree into something with more meaning

use crate::ast::types::{
    ArrayLength, ArrayType, CastMode, Definition, Expression, ExpressionAtom, ExpressionType,
    Literal, LiteralType, PrimitiveType, ScalarType, Statement, Type, TypeVersion, VersionedType,
};
use crate::{make_error, Error, Identifier, Rule};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::identities::Zero;
use num_traits::pow::Pow;
use pest::iterators::{Pair, Pairs};
use pest::Span;

pub(crate) mod types;
mod unescape;

/// Converts a Pest parse tree into an abstract syntax tree
pub(crate) fn parse_to_ast(statements: Pairs<'_, Rule>) -> Result<Definition<'_>, Error> {
    let mut ast_statements: Vec<Statement> = Vec::new();
    let mut eof_span = None;

    for statement in statements {
        match statement.as_rule() {
            Rule::statement_directive => {
                ast_statements.push(parse_directive(statement)?);
            }
            Rule::statement_service_response_marker => {
                ast_statements.push(Statement::ServiceResponseMarker(statement.as_span()));
            }
            Rule::statement_constant => {
                ast_statements.push(parse_constant(statement)?);
            }
            Rule::statement_field => {
                ast_statements.push(parse_field(statement)?);
            }
            Rule::statement_padding_field => {
                ast_statements.push(parse_padding_field(statement)?);
            }
            Rule::comment_content => ast_statements.push(Statement::Comment(statement.as_span())),
            Rule::EOI => {
                eof_span = Some(statement.as_span());
            }
            other => unreachable!("Unexpected statement rule {:?}", other),
        }
    }
    Ok(Definition {
        statements: ast_statements,
        eof_span: eof_span.expect("Didn't get an EOI at the end"),
    })
}

fn parse_directive(directive: Pair<'_, Rule>) -> Result<Statement<'_>, Error> {
    debug_assert_eq!(directive.as_rule(), Rule::statement_directive);
    let inner = directive.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::statement_directive_with_expression
        | Rule::statement_directive_without_expression => {
            let mut id_and_expr = inner.into_inner();
            let identifier = id_and_expr.next().expect("No identifier");
            let expr = match id_and_expr.next() {
                Some(expr) => Some(parse_expression(expr)?),
                None => None,
            };
            Ok(Statement::Directive {
                name: Identifier {
                    name: identifier.as_str(),
                    span: identifier.as_span(),
                },
                value: expr,
            })
        }
        _ => unreachable!("Unexpected rule in statement_directive"),
    }
}

fn parse_constant(constant: Pair<'_, Rule>) -> Result<Statement<'_>, Error> {
    debug_assert_eq!(constant.as_rule(), Rule::statement_constant);
    let mut parts = constant.into_inner();

    let dtype = parts.next().expect("No data type");
    let identifier = parts.next().expect("No identifier");
    let value = parts.next().expect("No expression");

    Ok(Statement::Constant {
        // Note: Constants must have primitive types
        ty: parse_primitive_type(dtype)?,
        name: Identifier {
            name: identifier.as_str(),
            span: identifier.as_span(),
        },
        value: parse_expression(value)?,
    })
}

fn parse_field(field: Pair<'_, Rule>) -> Result<Statement<'_>, Error> {
    debug_assert_eq!(field.as_rule(), Rule::statement_field);
    let span = field.as_span();
    let mut children = field.into_inner();
    let dtype = children.next().expect("No dtype");
    let identifier = children.next().expect("No identifier");

    Ok(Statement::Field {
        ty: parse_data_type(dtype)?,
        name: Identifier {
            name: identifier.as_str(),
            span: identifier.as_span(),
        },
        span,
    })
}

fn parse_padding_field(field: Pair<'_, Rule>) -> Result<Statement<'_>, Error> {
    debug_assert_eq!(field.as_rule(), Rule::statement_padding_field);
    let field_span = field.as_span();
    let void = field.into_inner().next().unwrap();
    let bit_length_suffix = void.into_inner().next().unwrap();
    let bits = parse_bit_length_suffix(&bit_length_suffix)?;
    if (1..=64).contains(&bits) {
        Ok(Statement::PaddingField {
            bits,
            span: field_span,
        })
    } else {
        Err(make_error(
            "Padding length must be between 1 and 64 bits inclusive",
            field_span,
        ))
    }
}

fn parse_expression(pair: Pair<'_, Rule>) -> Result<Expression<'_>, Error> {
    let rule = pair.as_rule();
    let pair_span = pair.as_span();
    let mut children = pair.into_inner();
    match rule {
        Rule::expression => {
            let result = parse_expression(children.next().expect("No child"));
            assert!(children.next().is_none());
            result
        }
        Rule::expression_atom => Ok(Expression {
            expression: ExpressionType::Atom(Box::new(parse_expression_atom(
                children.next().expect("No child"),
            )?)),
            span: pair_span,
        }),
        Rule::ex_logical => parse_binary_op(pair_span, children, |rule, lhs, rhs| match rule {
            Rule::op2_log_or => ExpressionType::LogicalOr(lhs, rhs),
            Rule::op2_log_and => ExpressionType::LogicalAnd(lhs, rhs),
            _ => unreachable!("Unexpected rule in op2_log"),
        }),
        Rule::ex_comparison => parse_binary_op(pair_span, children, |rule, lhs, rhs| match rule {
            Rule::op2_cmp_equ => ExpressionType::Equal(lhs, rhs),
            Rule::op2_cmp_neq => ExpressionType::NotEqual(lhs, rhs),
            Rule::op2_cmp_leq => ExpressionType::LessOrEqual(lhs, rhs),
            Rule::op2_cmp_geq => ExpressionType::GreaterOrEqual(lhs, rhs),
            Rule::op2_cmp_lss => ExpressionType::Less(lhs, rhs),
            Rule::op2_cmp_grt => ExpressionType::Greater(lhs, rhs),
            _ => unreachable!("Unexpected rule in op2_cmp"),
        }),
        Rule::ex_bitwise => parse_binary_op(pair_span, children, |rule, lhs, rhs| match rule {
            Rule::op2_bit_or => ExpressionType::BitOr(lhs, rhs),
            Rule::op2_bit_xor => ExpressionType::BitXor(lhs, rhs),
            Rule::op2_bit_and => ExpressionType::BitAnd(lhs, rhs),
            _ => unreachable!("Unexpected rule in op2_bit"),
        }),
        Rule::ex_additive => parse_binary_op(pair_span, children, |rule, lhs, rhs| match rule {
            Rule::op2_add_add => ExpressionType::Add(lhs, rhs),
            Rule::op2_add_sub => ExpressionType::Subtract(lhs, rhs),
            _ => unreachable!("Unexpected rule in op2_add"),
        }),
        Rule::ex_multiplicative => {
            parse_binary_op(pair_span, children, |rule, lhs, rhs| match rule {
                Rule::op2_mul_mul => ExpressionType::Multiply(lhs, rhs),
                Rule::op2_mul_div => ExpressionType::Divide(lhs, rhs),
                Rule::op2_mul_mod => ExpressionType::Modulo(lhs, rhs),
                _ => unreachable!("Unexpected rule in op2_mul"),
            })
        }
        Rule::ex_exponential => parse_binary_op(pair_span, children, |rule, lhs, rhs| match rule {
            Rule::op2_exp_pow => ExpressionType::Exponent(lhs, rhs),
            _ => unreachable!("Unexpected rule in op2_exp"),
        }),
        Rule::ex_attribute => {
            // This doesn't use parse_binary_op because it's a little different: the right hand side
            // can only be an identifier.
            let mut expression = parse_expression(children.next().expect("No child"))?;

            while let Some(op2_attrib) = children.next() {
                assert_eq!(op2_attrib.as_rule(), Rule::op2_attrib);
                let rhs = children
                    .next()
                    .expect("Non-even number of tokens after expression_atom in ex_attribute");
                assert_eq!(rhs.as_rule(), Rule::identifier);

                expression = Expression {
                    expression: ExpressionType::Attribute(Box::new(expression), rhs.as_str()),
                    span: pair_span.clone(),
                };
            }

            Ok(expression)
        }
        // parse_binary_op(pair_span, children, |rule, lhs, rhs| match rule {
        //     Rule::op2_attrib => ExpressionType::Attribute(lhs, rhs),
        //     _ => unreachable!("Unexpected rule in op2_attrib"),
        // }),
        Rule::op1_form_log_not => Ok(Expression {
            expression: ExpressionType::UnaryNot(Box::new(parse_expression(
                children.next().expect("No child"),
            )?)),
            span: pair_span,
        }),
        Rule::op1_form_inv_pos => Ok(Expression {
            expression: ExpressionType::UnaryPlus(Box::new(parse_expression(
                children.next().expect("No child"),
            )?)),
            span: pair_span,
        }),
        Rule::op1_form_inv_neg => Ok(Expression {
            expression: ExpressionType::UnaryMinus(Box::new(parse_expression(
                children.next().expect("No child"),
            )?)),
            span: pair_span,
        }),
        _ => unreachable!("Unexpected rule {:?}", rule),
    }
}

fn parse_binary_op<'i, I, F>(
    span: Span<'i>,
    children: I,
    mut op_handler: F,
) -> Result<Expression<'i>, Error>
where
    I: IntoIterator<Item = Pair<'i, Rule>>,
    F: FnMut(Rule, Box<Expression<'i>>, Box<Expression<'i>>) -> ExpressionType<'i>,
{
    let mut children = children.into_iter();
    let mut expression = parse_expression(children.next().expect("No child"))?;

    while let Some(operator) = children.next() {
        let rhs = children
            .next()
            .expect("Non-even number of tokens after first sub-expression");
        let rhs = parse_expression(rhs)?;

        let operator_rule = get_deepest_rule(operator);
        let new_expr_type = op_handler(operator_rule, Box::new(expression), Box::new(rhs));
        expression = Expression {
            expression: new_expr_type,
            span: span.clone(),
        };
    }

    Ok(expression)
}

/// Searches depth-first until a pair with no children is found, and then
/// returns the matching rule
fn get_deepest_rule(mut pair: Pair<'_, Rule>) -> Rule {
    let mut rule = pair.as_rule();
    while let Some(inner) = pair.into_inner().next() {
        pair = inner;
        rule = pair.as_rule();
    }
    rule
}

fn parse_expression_atom(atom: Pair<'_, Rule>) -> Result<ExpressionAtom<'_>, Error> {
    let rule = atom.as_rule();
    match rule {
        Rule::expression_parenthesized => {
            let child = atom.into_inner().next().unwrap();
            Ok(ExpressionAtom::Parenthesized(parse_expression(child)?))
        }
        Rule::dtype => Ok(ExpressionAtom::Type(parse_data_type(atom)?)),
        Rule::literal => {
            let child = atom.into_inner().next().unwrap();
            Ok(ExpressionAtom::Literal(parse_literal(child)?))
        }
        Rule::identifier => Ok(ExpressionAtom::Identifier(atom.as_str())),
        _ => unreachable!("Unexpected rule in expression_atom"),
    }
}

fn parse_data_type(dtype: Pair<'_, Rule>) -> Result<Type<'_>, Error> {
    let array_or_scalar = dtype.into_inner().next().expect("No child");
    match array_or_scalar.as_rule() {
        Rule::type_array => Ok(Type::Array(parse_array_type(array_or_scalar)?)),
        Rule::type_scalar => Ok(Type::Scalar(parse_scalar_type(array_or_scalar)?)),
        _ => unreachable!("Unexpected rule in dtype"),
    }
}

fn parse_scalar_type(dtype: Pair<'_, Rule>) -> Result<ScalarType, Error> {
    debug_assert_eq!(dtype.as_rule(), Rule::type_scalar);
    let type_span = dtype.as_span();
    let child = dtype.into_inner().next().expect("No child");
    match child.as_rule() {
        Rule::type_versioned => Ok(ScalarType::Versioned(parse_versioned_type(child)?)),
        Rule::type_primitive => Ok(ScalarType::Primitive(parse_primitive_type(child)?)),
        Rule::type_void => {
            let suffix = child.into_inner().next().unwrap();
            let bits = parse_bit_length_suffix(&suffix)?;
            if (1..=64).contains(&bits) {
                Ok(ScalarType::Void { bits })
            } else {
                Err(make_error(
                    "Void type length must be between 1 and 64 bits inclusive",
                    type_span,
                ))
            }
        }
        _ => unreachable!("Unexpected rule in type_scalar"),
    }
}

fn parse_versioned_type(versioned: Pair<'_, Rule>) -> Result<VersionedType<'_>, Error> {
    debug_assert_eq!(versioned.as_rule(), Rule::type_versioned);
    let mut path_and_name = Vec::new();
    let mut version = None;
    for child in versioned.into_inner() {
        match child.as_rule() {
            Rule::identifier => path_and_name.push(child.as_str()),
            Rule::type_version_specifier => {
                let mut version_parts = child.into_inner();
                let version_major_lit = version_parts.next().unwrap();
                let version_minor_lit = version_parts.next().unwrap();
                version = Some(TypeVersion {
                    major: parse_version_digits(version_major_lit)?,
                    minor: parse_version_digits(version_minor_lit)?,
                });
                break;
            }
            _ => unreachable!("Unexpected rule in type_versioned"),
        }
    }
    let (name, path) = path_and_name.split_last().expect("No type name");
    let version = version.expect("No version");
    Ok(VersionedType {
        path: path.to_vec(),
        name,
        version,
    })
}

fn parse_version_digits(pair: Pair<'_, Rule>) -> Result<u8, Error> {
    pair.as_str()
        .parse()
        .map_err(|e| make_error(format!("Invalid version number: {}", e), pair.as_span()))
}

fn parse_array_type(dtype: Pair<'_, Rule>) -> Result<ArrayType, Error> {
    debug_assert_eq!(dtype.as_rule(), Rule::type_array);
    let variant = dtype.into_inner().next().unwrap();
    let variant_rule = variant.as_rule();

    let mut variant_children = variant.into_inner();
    let member_type = variant_children.next().expect("No member type");
    let length = variant_children.next().expect("No length");

    let member_type = parse_scalar_type(member_type)?;
    let length = parse_expression(length)?;

    let length = match variant_rule {
        Rule::type_array_variable_inclusive => ArrayLength::Inclusive(length),
        Rule::type_array_variable_exclusive => ArrayLength::Exclusive(length),
        Rule::type_array_fixed => ArrayLength::Fixed(length),
        _ => unreachable!("Unexpected rule in type_array"),
    };
    Ok(ArrayType {
        element: member_type,
        length,
    })
}

fn parse_literal(literal: Pair<'_, Rule>) -> Result<Literal<'_>, Error> {
    let rule = literal.as_rule();
    let span = literal.as_span();
    let lit_type = match rule {
        Rule::literal_set => {
            let expression_list = literal.into_inner().next().unwrap();
            let expressions = expression_list
                .into_inner()
                .map(parse_expression)
                .collect::<Result<_, _>>()?;
            LiteralType::Set(expressions)
        }
        Rule::literal_real => LiteralType::Number(parse_real_literal(literal)?),
        Rule::literal_integer => {
            LiteralType::Number(BigRational::from_integer(parse_integer_literal(literal)))
        }
        Rule::literal_string => LiteralType::String(parse_string_literal(literal)?),
        Rule::literal_boolean => {
            let bool_value = literal.into_inner().next().unwrap();
            let value = match bool_value.as_rule() {
                Rule::literal_boolean_true => true,
                Rule::literal_boolean_false => false,
                _ => unreachable!("Unexpected rule in literal_boolean"),
            };
            LiteralType::Boolean(value)
        }
        _ => unreachable!("Unexpected rule {:?} in literal", rule),
    };
    Ok(Literal {
        literal: lit_type,
        span,
    })
}

fn parse_real_literal(literal: Pair<'_, Rule>) -> Result<BigRational, Error> {
    let variant = literal.into_inner().next().unwrap();
    match variant.as_rule() {
        Rule::literal_real_exponent_notation => parse_real_exponent_notation(variant),
        Rule::literal_real_point_notation => parse_real_point_notation(variant),
        _ => unreachable!("Unexpected rule in literal_real"),
    }
}

fn parse_real_exponent_notation(literal: Pair<'_, Rule>) -> Result<BigRational, Error> {
    // This contains a literal_real_point_notation or literal_real_digits,
    // followed by a literal_real_exponent.
    let mut children = literal.into_inner();
    let before_exp = children.next().unwrap();
    let exp = children.next().unwrap();

    let before_exp = match before_exp.as_rule() {
        Rule::literal_real_point_notation => parse_real_point_notation(before_exp)?,
        Rule::literal_real_digits => BigRational::from_integer(parse_decimal_digits(before_exp)),
        _ => unreachable!("Unexpected rule in first child of literal_real_exponent_notation"),
    };

    let exp_sign = exp.as_str().chars().nth(1).expect("No sign");
    let exp_value = parse_decimal_digits(exp.into_inner().next().unwrap());
    let exp_value = if exp_sign == '-' {
        -exp_value
    } else {
        // The second character may be a +, or a digit if there is no sign
        exp_value
    };
    let ten = BigRational::from_integer(10.into());
    let scaling = Pow::pow(ten, &exp_value);

    Ok(before_exp * scaling)
}

fn parse_real_point_notation(literal: Pair<'_, Rule>) -> Result<BigRational, Error> {
    debug_assert_eq!(literal.as_rule(), Rule::literal_real_point_notation);
    let span = literal.as_span();
    let mut children = literal.into_inner();
    let whole_number_digits = children.next().unwrap();
    // There may be fractional digits after the decimal point
    // Go two layers down to skip the . and get to literal_real_digits
    let fractional_digits = children.next().map(|pair| {
        pair.into_inner()
            .next()
            .expect("No child of literal_real_fraction")
    });

    // Parse
    let whole_number_digits: BigInt = whole_number_digits
        .as_str()
        .parse()
        .map_err(|e| make_error(format!("Invalid real literal: {}", e), span.clone()))?;

    if let Some(fractional_digits) = fractional_digits {
        debug_assert_eq!(fractional_digits.as_rule(), Rule::literal_real_digits);
        let num_fractional_digits = fractional_digits.as_str().len();
        let fractional_digits: BigInt = fractional_digits.as_str().parse().map_err(|e| {
            make_error(
                format!("Invalid fractional part of real literal: {}", e),
                span,
            )
        })?;
        // Divide the fractional digits by a power of ten to get the correct scale
        let scale_factor = Pow::pow(BigInt::from(10_u32), num_fractional_digits);

        let scaled_fractional = BigRational::new(fractional_digits, scale_factor);
        // Add whole and fractional parts
        Ok(BigRational::from_integer(whole_number_digits) + scaled_fractional)
    } else {
        Ok(BigRational::from_integer(whole_number_digits))
    }
}

fn parse_string_literal(literal: Pair<'_, Rule>) -> Result<String, Error> {
    // Zoom in to the quoted variant
    let literal = literal.into_inner().next().unwrap();
    let between_quotes = literal
        .as_str()
        .strip_prefix(is_quote)
        .expect("String literal does not start with a quotation mark")
        .strip_suffix(is_quote)
        .expect("String literal does not end with a quotation mark");
    unescape::unescape_string(between_quotes, literal.as_span())
}

fn is_quote(c: char) -> bool {
    c == '"' || c == '\''
}

fn parse_integer_literal(literal: Pair<'_, Rule>) -> BigInt {
    let variant = literal.into_inner().next().expect("No child");
    match variant.as_rule() {
        Rule::literal_integer_binary => parse_binary_literal(variant),
        Rule::literal_integer_octal => parse_octal_literal(variant),
        Rule::literal_integer_hexadecimal => parse_hex_literal(variant),
        Rule::literal_integer_decimal => parse_decimal_digits(variant),
        _ => unreachable!("Unexpected rule in literal_integer"),
    }
}

fn parse_binary_literal(variant: Pair<'_, Rule>) -> BigInt {
    debug_assert_eq!(variant.as_rule(), Rule::literal_integer_binary);
    let mut value = BigInt::zero();
    // Skip the first two characters (the 0b or 0B prefix)
    for c in variant.as_str().chars().skip(2) {
        let digit_value = match c {
            '0' => 0,
            '1' => 1,
            // Ignore _
            '_' => continue,
            _ => unreachable!("Not a digit"),
        };
        value = value * 2 + digit_value;
    }
    value
}
fn parse_octal_literal(variant: Pair<'_, Rule>) -> BigInt {
    debug_assert_eq!(variant.as_rule(), Rule::literal_integer_octal);
    let mut value = BigInt::zero();
    // Skip the first two characters (the 0o or 0O prefix)
    for c in variant.as_str().chars().skip(2) {
        let digit_value = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            // Ignore _
            '_' => continue,
            _ => unreachable!("Not a digit"),
        };
        value = value * 8 + digit_value;
    }
    value
}
fn parse_hex_literal(variant: Pair<'_, Rule>) -> BigInt {
    debug_assert_eq!(variant.as_rule(), Rule::literal_integer_hexadecimal);
    let mut value = BigInt::zero();
    // Skip the first two characters (the 0x or 0X prefix)
    for c in variant.as_str().chars().skip(2) {
        let digit_value = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' | 'A' => 10,
            'b' | 'B' => 11,
            'c' | 'C' => 12,
            'd' | 'D' => 13,
            'e' | 'E' => 14,
            'f' | 'F' => 15,
            // Ignore _
            '_' => continue,
            _ => unreachable!("Not a digit"),
        };
        value = value * 16 + digit_value;
    }
    value
}

fn parse_decimal_digits(variant: Pair<'_, Rule>) -> BigInt {
    let mut value = BigInt::zero();
    // Process each character, ignoring _
    for character in variant.as_str().chars() {
        let digit_value = match character {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            // Ignore _
            '_' => continue,
            _ => unreachable!("Not a digit"),
        };
        value = value * 10 + digit_value;
    }
    value
}

fn parse_primitive_type(dtype: Pair<'_, Rule>) -> Result<PrimitiveType, Error> {
    debug_assert_eq!(dtype.as_rule(), Rule::type_primitive);
    let inner = dtype.into_inner().next().expect("No inner type");
    let inner_rule = inner.as_rule();
    let type_name = inner.into_inner().last().expect("No type name");
    match inner_rule {
        Rule::type_primitive_truncated => parse_primitive_type_name(type_name, CastMode::Truncated),
        Rule::type_primitive_saturated => parse_primitive_type_name(type_name, CastMode::Saturated),
        _ => unreachable!("Unexpected rule in type_primitive"),
    }
}

fn parse_primitive_type_name(
    name: Pair<'_, Rule>,
    cast_mode: CastMode,
) -> Result<PrimitiveType, Error> {
    debug_assert_eq!(name.as_rule(), Rule::type_primitive_name);
    let name_span = name.as_span();
    let name_kind = name.into_inner().next().expect("No name kind");
    let name_kind_rule = name_kind.as_rule();
    match name_kind_rule {
        Rule::type_primitive_name_boolean => match cast_mode {
            CastMode::Truncated => Err(make_error(
                "A boolean type with truncated cast mode is not allowed",
                name_span,
            )),
            CastMode::Saturated => Ok(PrimitiveType::Boolean),
        },
        Rule::type_primitive_name_unsigned_integer
        | Rule::type_primitive_name_signed_integer
        | Rule::type_primitive_name_floating_point => {
            let suffix = name_kind.into_inner().next().expect("No suffix");
            let bits = parse_bit_length_suffix(&suffix)?;
            match name_kind_rule {
                Rule::type_primitive_name_unsigned_integer
                | Rule::type_primitive_name_signed_integer => {
                    parse_integer_type(name_span, name_kind_rule, bits, cast_mode)
                }
                Rule::type_primitive_name_floating_point => match bits {
                    16 => Ok(PrimitiveType::Float16 { mode: cast_mode }),
                    32 => Ok(PrimitiveType::Float32 { mode: cast_mode }),
                    64 => Ok(PrimitiveType::Float64 { mode: cast_mode }),
                    _ => Err(make_error(
                        "Invalid length for floating-point type",
                        suffix.as_span(),
                    )),
                },
                _ => unreachable!("Unexpected rule in type_primitive_name"),
            }
        }
        _ => unreachable!("Unexpected rule in type_primitive_name"),
    }
}

fn parse_integer_type(
    name_span: Span<'_>,
    name_kind_rule: Rule,
    bits: u8,
    cast_mode: CastMode,
) -> Result<PrimitiveType, Error> {
    if valid_integer_length(bits) {
        match name_kind_rule {
            Rule::type_primitive_name_unsigned_integer => Ok(PrimitiveType::UInt {
                bits,
                mode: cast_mode,
            }),
            Rule::type_primitive_name_signed_integer => match cast_mode {
                CastMode::Saturated => Ok(PrimitiveType::Int { bits }),
                CastMode::Truncated => Err(make_error(
                    "Signed integers cannot use truncated mode",
                    name_span,
                )),
            },
            _ => unreachable!("Unexpected rule for integer type"),
        }
    } else {
        Err(make_error("Invalid length for integer type", name_span))
    }
}

fn parse_bit_length_suffix(suffix: &Pair<'_, Rule>) -> Result<u8, Error> {
    debug_assert_eq!(suffix.as_rule(), Rule::type_bit_length_suffix);
    suffix
        .as_str()
        .parse()
        .map_err(|e| make_error(format!("Invalid type length: {}", e), suffix.as_span()))
}

fn valid_integer_length(bits: u8) -> bool {
    (1..=64).contains(&bits)
}

#[cfg(test)]
mod test {
    use super::parse_to_ast;
    use crate::{DsdlParser, Rule};
    use pest::Parser;

    #[test]
    fn basic1() {
        let definition = DsdlParser::parse(
            Rule::definition,
            r"# Comment
uint32 TOVE = 19
uint8 a
int3 b
void5
@sealed
---
uint12 thingy
@extent 4 * 8
",
        )
        .unwrap();
        let _ast = parse_to_ast(definition).unwrap();
    }
    #[test]
    fn versioned() {
        let definition = DsdlParser::parse(
            Rule::definition,
            r"# Comment
SamePackage.3.2 frobnicator
uavcan.something.OtherPackage.0.7 wabe
",
        )
        .unwrap();
        let _ast = parse_to_ast(definition).unwrap();
    }
    #[test]
    fn set_literal() {
        let definition = DsdlParser::parse(
            Rule::definition,
            r"# Comment
uint8[<=2] sprinkles
@assert __offset__ == {8, 16, 24}
",
        )
        .unwrap();
        let _ast = parse_to_ast(definition).unwrap();
    }
    #[test]
    fn print_string() {
        let text = r#"# Comment
@print "Hello world!"
@print 'Hello world!'
@assert "oh,\u0020hi\U0000000aMark" == 'oh, hi\nMark'
"#;
        let definition = match DsdlParser::parse(Rule::definition, text) {
            Ok(def) => def,
            Err(e) => panic!("{}", e),
        };
        let _ast = parse_to_ast(definition).unwrap();
    }
}
