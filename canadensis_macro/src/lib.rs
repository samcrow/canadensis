extern crate canadensis_codegen_rust;
extern crate canadensis_dsdl_frontend;
extern crate proc_macro;

mod input;

use crate::input::{Input, ParsedString, Statement};
use canadensis_dsdl_frontend::{Package, TypeKey};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenTree};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::env;
use syn::spanned::Spanned;

/// Parses one or more DSDL definitions and expands into the corresponding Rust code
#[proc_macro]
pub fn types_from_dsdl(input: TokenStream) -> TokenStream {
    match types_from_dsdl_inner(input.into()) {
        Ok(result) => result.into(),
        Err(err) => err.into(),
    }
}

fn types_from_dsdl_inner(
    input: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
    let input = Input::parse(input)?;

    // Prepare a package of DSDL to parse
    let mut package = Package::new();
    let mut external_packages = BTreeMap::new();
    let mut output = proc_macro2::TokenStream::default();

    let mut statements = input.statements.into_iter();
    // Zero or more package/inline type statements, followed by zero or one generate statement
    for statement in statements.by_ref() {
        match statement {
            Statement::Function { name, arguments } => match &*name.to_string() {
                "package" => eval_package_function(&mut package, arguments)?,
                "make_external" => eval_make_external_function(&mut external_packages, arguments)?,
                "generate" => {
                    // Generate and break
                    let compiled = package.compile().map_err(|e| {
                        make_error(
                            name.span(),
                            format!("Failed to compile DSDL: {}", ErrorChain(e)),
                        )
                    })?;
                    let code =
                        canadensis_codegen_rust::generate_code(&compiled, &external_packages);
                    let code_string = code.to_string();
                    let parsed_code: proc_macro2::TokenStream = code_string
                        .parse()
                        .expect("Internal error: Generated invalid code");
                    output.extend(parsed_code);

                    break;
                }
                _ => return Err(make_error(name.span(), "Unrecognized function")),
            },
            Statement::InlineType { name, dsdl } => eval_inline_type(&mut package, name, dsdl)?,
        }
    }

    // Expect nothing else
    match statements.next() {
        None => {}
        Some(Statement::InlineType { name, .. }) => {
            return Err(make_error(name.span, "Unexpected type after generate"))
        }
        Some(Statement::Function { name, .. }) => {
            return Err(make_error(
                name.span(),
                "Unexpected function after generate",
            ))
        }
    }

    Ok(output)
}

fn eval_package_function(
    package: &mut Package,
    arguments: proc_macro2::TokenStream,
) -> Result<(), proc_macro2::TokenStream> {
    let path = eval_path_arguments(arguments)?;

    package
        .add_files(&path.value)
        .map_err(|e| make_error(path.span, format!("Can't add package: {}", e)))
}

fn eval_make_external_function(
    external_packages: &mut BTreeMap<Vec<String>, Vec<String>>,
    arguments: proc_macro2::TokenStream,
) -> Result<(), proc_macro2::TokenStream> {
    // Expect two arguments: a Cyphal package and a Rust module path
    let arguments_span = arguments.span();
    let mut cyphal_package = Vec::new();
    let mut rust_module = Vec::new();

    let mut iter = arguments.into_iter();
    // Loop 1: get Cyphal package name
    while let Some(tree) = iter.next() {
        match tree {
            TokenTree::Ident(ident) => {
                cyphal_package.push(ident.to_string());
                // After a package name segment, expect . or ,
                match iter.next() {
                    Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {
                        // OK, check for the next identifier
                    }
                    Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {
                        // OK, end of Cyphal package
                        break;
                    }
                    Some(other) => {
                        return Err(make_error(
                            other.span(),
                            "Expected dot or comma after identifier",
                        ))
                    }
                    None => {
                        return Err(make_error(ident.span(), "Expected comma after identifier"))
                    }
                }
            }
            _ => return Err(make_error(tree.span(), "Expected an identifier")),
        }
    }
    // Check that we got a package
    if cyphal_package.is_empty() {
        return Err(make_error(
            arguments_span,
            "Expected at least one Cyphal package name segment before comma",
        ));
    }
    // Loop 2: get Rust module name
    while let Some(tree) = iter.next() {
        match tree {
            TokenTree::Ident(ident) => {
                rust_module.push(ident.to_string());
                // Should be followed by :: or the end of arguments
                match iter.next() {
                    Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => {
                        match iter.next() {
                            Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => {
                                // Path segment OK, go back and read the next identifier
                            }
                            _ => {
                                return Err(make_error(
                                    ident.span(),
                                    "Expected :: or end of arguments after identifier",
                                ))
                            }
                        }
                    }
                    None => {
                        // End of arguments
                        break;
                    }
                    _ => {
                        return Err(make_error(
                            ident.span(),
                            "Expected :: or end of arguments after identifier",
                        ))
                    }
                }
            }
            _ => return Err(make_error(tree.span(), "Expected an identifier")),
        }
    }
    // Check that we got a Rust module
    if rust_module.is_empty() {
        return Err(make_error(
            arguments_span,
            "Expected at least one Rust module segment before end of arguments",
        ));
    }

    match external_packages.entry(cyphal_package) {
        Entry::Vacant(entry) => {
            entry.insert(rust_module);
            Ok(())
        }
        Entry::Occupied(_) => Err(make_error(
            arguments_span,
            "This package has already been marked as external",
        )),
    }
}

/// Replaces $CARGO_MANIFEST_DIR with the value of the environment variable CARGO_MANIFEST_DIR,
/// and concatenates all comma-separated arguments
fn eval_path_arguments(
    arguments: proc_macro2::TokenStream,
) -> Result<ParsedString, proc_macro2::TokenStream> {
    let args_span = arguments.span();
    let mut path = String::new();
    let mut iter = arguments.into_iter();
    while let Some(tree) = iter.next() {
        match tree {
            TokenTree::Punct(punct) if punct.as_char() == '$' => {
                // Start of an environment variable reference
                match iter.next() {
                    Some(TokenTree::Ident(ident)) => {
                        let value = eval_env_variable(ident)?;
                        path.push_str(&value);
                    }
                    Some(_) | None => {
                        return Err(make_error(
                            punct.span(),
                            "Expected name of environment variable after $",
                        ))
                    }
                }
            }
            TokenTree::Literal(literal) => {
                // String literal
                let segment = ParsedString::from_literal(literal)?;
                path.push_str(&segment.value);
            }
            other => return Err(make_error(other.span(), "Unexpected argument")),
        }

        // Expect either a comma or the end
        match iter.next() {
            None => break,
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => { /* OK, continue */ }
            Some(other) => {
                return Err(make_error(
                    other.span(),
                    "Expected comma before next argument",
                ))
            }
        }
    }
    Ok(ParsedString {
        span: args_span,
        value: path,
    })
}

fn eval_env_variable(name: Ident) -> Result<String, proc_macro2::TokenStream> {
    let var_name = name.to_string();
    let name_allowed = var_name == "CARGO_MANIFEST_DIR";
    if !name_allowed {
        return Err(make_error(
            name.span(),
            format!("Access to variable {} is not allowed", var_name),
        ));
    }
    env::var(&var_name)
        .map_err(|e| make_error(name.span(), format!("Can't read {}: {}", var_name, e)))
}

fn eval_inline_type(
    package: &mut Package,
    name: ParsedString,
    dsdl: ParsedString,
) -> Result<(), proc_macro2::TokenStream> {
    let key: TypeKey = name
        .value
        .parse()
        .map_err(|e| make_error(name.span, format!("Invalid type key: {}", e)))?;
    package
        .add_string(None, key, dsdl.value)
        .map_err(|e| make_error(name.span, format!("Can't add inline DSDL: {}", e)))?;
    Ok(())
}

fn make_error<S>(span: Span, message: S) -> proc_macro2::TokenStream
where
    S: AsRef<str>,
{
    let message = message.as_ref();
    quote::quote_spanned! { span => compile_error!(#message); }
}

/// Displays an error and its sources
struct ErrorChain<E>(pub E);

impl<E> std::fmt::Display for ErrorChain<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.0)?;
        if let Some(source) = self.0.source() {
            write!(f, "Caused by: ")?;
            std::fmt::Display::fmt(&ErrorChain(source), f)
        } else {
            Ok(())
        }
    }
}
