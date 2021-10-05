extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;

use canadensis_bit_length_set::BitLengthSet;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields, Lit,
    Type,
};

mod attributes;

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Serialize, attributes(canadensis))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    let serialization_attributes = attributes::parse_serialization_attributes(derive_input.attrs);
    println!("Serialization attributes {:#?}", serialization_attributes);

    let output2: proc_macro2::TokenStream = match derive_input.data {
        Data::Struct(data) => derive_serialize_struct(data, derive_input.ident),
        Data::Enum(data) => derive_serialize_enum(data),
        Data::Union(_) => panic!("Serialize cannot be derived for unions"),
    };
    output2.into()
}

#[proc_macro_derive(DataType)]
pub fn derive_data_type(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let struct_name = derive_input.ident;

    // TODO: Real things
    let output2 = quote::quote! {
        impl ::canadensis_encoding::DataType for #struct_name {
            // TODO: This is wrong
            const EXTENT_BYTES: Option<u32> = None;
        }
    };
    output2.into()
}

fn derive_serialize_struct(data: DataStruct, struct_name: Ident) -> proc_macro2::TokenStream {
    let fields = parse_struct_fields(data.fields);
    println!("Fields {:#?}", fields);

    let output = quote::quote! {
        impl ::canadensis_encoding::Serialize for #struct_name {
            fn size_bits(&self) -> usize { todo!() }
            fn serialize(&self, _cursor: &mut ::canadensis_encoding::WriteCursor<'_>) { todo!() }
        }
    };
    output
}

fn derive_serialize_enum(data: DataEnum) -> proc_macro2::TokenStream {
    todo!()
}

fn parse_struct_fields(fields: Fields) -> Vec<UavcanField> {
    match fields {
        Fields::Named(named_fields) => named_fields
            .named
            .into_iter()
            .map(UavcanField::from)
            .collect(),
        Fields::Unnamed(_) => panic!("Non-named fields are not supported"),
        Fields::Unit => Vec::new(),
    }
}

/// A composite type or union that may be a top-level object
#[derive(Debug)]
enum UavcanObject {
    Composite(Vec<UavcanType>),
    Union(Vec<UavcanType>),
}

#[derive(Debug)]
struct UavcanField {
    name: Ident,
    ty: UavcanType,
}

impl From<Field> for UavcanField {
    fn from(field: Field) -> Self {
        UavcanField {
            name: field.ident.expect("Field has no name"),
            ty: field.ty.into(),
        }
    }
}

/// UAVCAN data types
#[derive(Debug)]
enum UavcanType {
    /// A void (padding) with the specified number of bits
    Void(u8),
    /// A signed integer with the specified number of bits
    Int(u8),
    /// An unsigned integer with the specified number of bits
    UInt(u8),
    /// A 1-bit boolean value
    Bool,
    /// A 16-bit floating-point number
    Float16,
    /// A 32-bit floating-point number
    Float32,
    /// A 64-bit floating-point number
    Float64,
    /// An array with a fixed length
    FixedArray {
        /// The element type of the array
        inner: Box<UavcanType>,
        /// The number of elements
        length: usize,
    },
    /// An array with a variable length
    VariableArray {
        /// The element type of the array
        inner: Box<UavcanType>,
        /// The maximum number of elements
        capacity: usize,
    },
    /// A composite object
    Composite {
        /// The type of the composite
        inner: Type,
    },
}

impl From<Type> for UavcanType {
    fn from(ty: Type) -> Self {
        match &*ty.to_token_stream().to_string() {
            "u8" => UavcanType::UInt(8),
            "u16" => UavcanType::UInt(16),
            "u32" => UavcanType::UInt(32),
            "u64" => UavcanType::UInt(64),
            "i8" => UavcanType::Int(8),
            "i16" => UavcanType::Int(16),
            "i32" => UavcanType::Int(32),
            "i64" => UavcanType::Int(64),
            "bool" => UavcanType::Bool,
            "f16" => UavcanType::Float16,
            "f32" => UavcanType::Float32,
            "f64" => UavcanType::Float64,
            _ => {
                // Look deeper into the type
                match ty {
                    Type::Array(ty) => {
                        // Fixed array
                        // Investigate the element type and length
                        let element_type: UavcanType = (*ty.elem).into();
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Int(length_literal),
                            ..
                        }) = ty.len
                        {
                            let length = length_literal
                                .base10_parse()
                                .expect("Couldn't parse length");
                            UavcanType::FixedArray {
                                inner: Box::new(element_type),
                                length,
                            }
                        } else {
                            panic!("Array length is not an integer literal")
                        }
                    }
                    _ => panic!("Unsupported type {:#?}", ty),
                }
            }
        }
    }
}
