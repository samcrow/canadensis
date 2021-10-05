//! Parsing of serialization attributes

use proc_macro2::{Delimiter, TokenTree};
use syn::{Attribute, Path, PathSegment};

#[derive(Debug)]
pub enum SerializationAttribute {
    Sealed,
    Delimited(proc_macro2::TokenStream),
}

pub fn parse_serialization_attributes(attrs: Vec<Attribute>) -> Vec<SerializationAttribute> {
    attrs
        .into_iter()
        .filter(|attr| is_canadensis_path(&attr.path))
        .map(parse_one_serialization_attribute)
        .collect()
}

fn parse_one_serialization_attribute(attr: Attribute) -> SerializationAttribute {
    // All attributes are surrounded by parentheses
    let mut stream = attr.tokens.into_iter();
    let tree = stream.next().expect("No attribute tokens");
    if stream.next().is_some() {
        panic!("Extra token tree at end of attribute");
    }

    match tree {
        TokenTree::Group(group) => {
            if group.delimiter() != Delimiter::Parenthesis {
                panic!("Group in attribute not delimited by parentheses");
            }
            // Look inside the group for a supported format
            parse_serialization_attribute_token_stream(group.stream())
        }
        _ => panic!("Tree in attribute is not a group"),
    }
}

fn parse_serialization_attribute_token_stream(
    stream: proc_macro2::TokenStream,
) -> SerializationAttribute {
    let mut iter = stream.into_iter();
    match iter.next() {
        // canadensis(sealed)
        Some(TokenTree::Ident(ident)) if ident.to_string() == "sealed" => {
            if iter.next().is_none() {
                SerializationAttribute::Sealed
            } else {
                panic!("Extra token tree after \"sealed\"")
            }
        }
        // canadensis(extent = <one or more token trees>)
        Some(TokenTree::Ident(ident)) if ident.to_string() == "extent" => {
            match iter.next() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
                    // The remaining token trees in the stream are interpreted as an expression
                    SerializationAttribute::Delimited(iter.collect())
                }
                _ => panic!("Expected = after \"extent\" in attribute"),
            }
        }
        _ => panic!("Empty token stream"),
    }
}

fn is_canadensis_path(path: &Path) -> bool {
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && is_canadensis_path_segment(&path.segments[0])
}

fn is_canadensis_path_segment(segment: &PathSegment) -> bool {
    segment.ident.to_string() == "canadensis" && segment.arguments.is_empty()
}
