//! Parses input to the types_from_dsdl macro

use crate::make_error;
use canadensis_dsdl_frontend::Config;
use proc_macro2::{Delimiter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use syn::Lit;

/// A single statement applied to the function
#[derive(Debug)]
pub enum Statement {
    /// A function, with arguments separated by commas
    Function { name: Ident, arguments: TokenStream },
    /// An inline DSDL type
    InlineType {
        name: ParsedString,
        dsdl: ParsedString,
    },
}

/// Input to the macro
#[derive(Debug)]
pub struct Input {
    pub statements: Vec<Statement>,
}

impl Input {
    /// Parses the input to the types_from_dsdl macro
    ///
    /// If an error occurs, this function returns a TokenStream that represents the error.
    pub fn parse(stream: TokenStream) -> Result<Input, TokenStream> {
        let mut statements = Vec::new();
        let mut iter = stream.into_iter();
        while let Some(tree) = iter.next() {
            match tree {
                TokenTree::Ident(ident) => {
                    // Look ahead for the next ( to indicate a function or { for an inline type
                    // (this is represented by a TokenTree::Group)
                    let mut prev_tree_span = ident.span();
                    let mut trees_before_group = vec![TokenTree::Ident(ident)];
                    let statement: Statement = loop {
                        match iter.next() {
                            Some(TokenTree::Group(group)) => {
                                break parse_statement(trees_before_group, group)?;
                            }
                            Some(other_tree) => {
                                prev_tree_span = other_tree.span();
                                trees_before_group.push(other_tree);
                            }
                            None => {
                                return Err(make_error(
                                    prev_tree_span,
                                    "Expected ( or { before end of input",
                                ))
                            }
                        }
                    };
                    statements.push(statement);
                }
                other => {
                    return Err(make_error(
                        other.span(),
                        format!("Unexpected token tree {}, expected identifier", other),
                    ))
                }
            }
        }

        Ok(Input { statements })
    }
}

fn parse_statement(
    trees_before_group: Vec<TokenTree>,
    group: Group,
) -> Result<Statement, TokenStream> {
    match group.delimiter() {
        Delimiter::Parenthesis => parse_function(trees_before_group, group),
        Delimiter::Brace => parse_inline_type(trees_before_group, group),
        Delimiter::Bracket | Delimiter::None => Err(make_error(
            group.span(),
            "Expected a function call with () or an inline type with {}",
        )),
    }
}

fn parse_function(
    trees_before_group: Vec<TokenTree>,
    group: Group,
) -> Result<Statement, TokenStream> {
    // trees_before_group must be a single identifier
    match &*trees_before_group {
        [TokenTree::Ident(ident)] => Ok(Statement::Function {
            name: ident.clone(),
            arguments: group.stream(),
        }),
        [other, ..] => Err(make_error(
            other.span(),
            "Expected one identifier before parenthesis",
        )),
        [] => panic!("No token trees before ("),
    }
}
fn parse_inline_type(
    trees_before_group: Vec<TokenTree>,
    group: Group,
) -> Result<Statement, TokenStream> {
    match &*trees_before_group {
        [TokenTree::Ident(type_ident), TokenTree::Literal(literal)] if type_ident == "type" => {
            Ok(Statement::InlineType {
                name: ParsedString::from_literal(literal.clone())?,
                dsdl: parse_string_in_group(group)?,
            })
        }
        [other, ..] => Err(make_error(
            other.span(),
            "Expected `type` and a string literal before {",
        )),
        [] => panic!("{}", "No trees before {"),
    }
}

fn parse_string_in_group(group: Group) -> Result<ParsedString, TokenStream> {
    let mut iter = group.stream().into_iter();
    let tree = iter
        .next()
        .ok_or_else(|| make_error(group.span(), "Expected a string literal"))?;

    if iter.next().is_some() {
        Err(make_error(
            group.span(),
            "Unexpected token tree after string literal",
        ))
    } else {
        match tree {
            TokenTree::Literal(literal) => ParsedString::from_literal(literal),
            _ => Err(make_error(tree.span(), "Expected string literal")),
        }
    }
}

/// A string parsed from a literal
#[derive(Debug)]
pub struct ParsedString {
    /// The span of the literal
    pub span: Span,
    /// The content
    pub value: String,
}
impl ParsedString {
    /// Extracts a string from a literal
    pub fn from_literal(literal: Literal) -> Result<ParsedString, TokenStream> {
        let literal = Lit::new(literal);
        match literal {
            Lit::Str(literal) => Ok(ParsedString {
                span: literal.span(),
                value: literal.value(),
            }),
            _ => Err(make_error(literal.span(), "Expected a string literal")),
        }
    }
}

pub fn parse_generate_config(arguments: TokenStream) -> Result<Config, TokenStream> {
    let mut trees = arguments.into_iter();
    let config = match trees.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
            parse_generate_config_inner(group)?
        }
        Some(other) => return Err(make_error(other.span(), "Expected options inside {}")),
        None => Config::default(),
    };
    if let Some(next) = trees.next() {
        return Err(make_error(next.span(), "Unexpected value after options"));
    }
    Ok(config)
}

fn parse_generate_config_inner(group: Group) -> Result<Config, TokenStream> {
    // Expected sequence: one or more of
    // Ident Punct(:) Ident(true|false) Punct(,)
    //
    // This currently requires trailing commas.
    let mut trees = group.stream().into_iter();
    let mut config = Config::default();
    while let Some((option_name, option_value)) = parse_one_option(&mut trees)? {
        let option_value = match option_value.to_string().as_str() {
            "true" => true,
            "false" => false,
            _ => {
                return Err(make_error(
                    option_value.span(),
                    "Unexpected value, expected true or false",
                ))
            }
        };
        match option_name.to_string().as_str() {
            "allow_utf8_and_byte" => config.allow_utf8_and_byte = option_value,
            "allow_saturated_bool" => config.allow_saturated_bool = option_value,
            _ => {
                return Err(make_error(
                    option_name.span(),
                    format!("Unrecognized option {}", option_name),
                ))
            }
        }
    }
    Ok(config)
}

fn parse_one_option(
    trees: &mut proc_macro2::token_stream::IntoIter,
) -> Result<Option<(Ident, Ident)>, TokenStream> {
    // Start of an option, expect identifier
    let option_name = match trees.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(other) => return Err(make_error(other.span(), "Expected identifier")),
        None => return Ok(None),
    };
    // Expect :
    let colon = match trees.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ':' => p,
        Some(other) => return Err(make_error(other.span(), "Expected :")),
        None => {
            return Err(make_error(
                option_name.span(),
                "Expected : after option name",
            ))
        }
    };
    // Value, expect identifier true or false
    let option_value = match trees.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(other) => return Err(make_error(other.span(), "Expected true or false")),
        None => return Err(make_error(colon.span(), "Expected true or false after :")),
    };
    // Expect comma after value
    match trees.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => {}
        Some(other) => return Err(make_error(other.span(), "Expected , after value")),
        None => return Err(make_error(option_value.span(), "Expected , after value")),
    }
    Ok(Some((option_name, option_value)))
}
