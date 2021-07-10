extern crate proc_macro;
use proc_macro::TokenStream;

use proc_macro2::{Literal, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Implements `canadensis::register::RegisterBlock` for a struct that contains zero or more
/// register fields
///
/// Each field in the struct must implement `canadensis::register::Register`.
#[proc_macro_derive(RegisterBlock)]
pub fn derive_register_block(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let result = implement_register_block(input);
    result.into()
}

fn implement_register_block(input: DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = input.ident;
    let field_names = get_struct_field_names(input.data);
    let field_indices = 0..field_names.len();

    let im_field_names = field_names.clone();
    let im_field_indices = field_indices.clone();

    let name_field_names = field_names.clone();

    quote! {
        impl ::canadensis::register::RegisterBlock for #struct_name {
            fn register_by_index(&self, index: usize) -> Option<&dyn ::canadensis::register::Register> {
                match index {
                    #( #field_indices => Some(&self.#field_names), )*
                    _ => None,
                }
            }
            fn register_by_index_mut(&mut self, index: usize) -> Option<&mut dyn ::canadensis::register::Register> {
                match index {
                    #( #im_field_indices => Some(&mut self.#im_field_names), )*
                    _ => None,
                }
            }
            fn register_by_name_mut(&mut self, name: &str) -> Option<&mut dyn ::canadensis::register::Register> {
                match name {
                    #( name if name == canadensis::register::Register::name(&self.#name_field_names) => Some(&mut self.#name_field_names), )*
                    _ => None,
                }
            }
        }
    }
}

fn get_struct_field_names(data: Data) -> Vec<TokenTree> {
    match data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(named) => named
                    .named
                    .into_iter()
                    .map(|field| {
                        let ident = field.ident.expect("Field must have a name");
                        TokenTree::Ident(ident)
                    })
                    .collect(),
                Fields::Unnamed(unnamed) => {
                    // Field names are 0, 1, ...
                    let num_fields = unnamed.unnamed.len();
                    (0..num_fields)
                        .map(|i| TokenTree::Literal(Literal::usize_unsuffixed(i)))
                        .collect()
                }
                Fields::Unit => {
                    // No fields
                    vec![]
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => panic!("RegisterBlock can only be derived for structs"),
    }
}
