extern crate canadensis_dsdl_frontend;
extern crate heck;

mod module_tree;

use heck::{CamelCase, SnakeCase};
use std::iter;

use crate::module_tree::ModuleTree;
use canadensis_dsdl_frontend::compiled::package::CompiledPackage;
use canadensis_dsdl_frontend::compiled::{
    CompiledDsdl, DsdlKind, FieldKind, Message, MessageKind, Struct, Union,
};
use canadensis_dsdl_frontend::types::{PrimitiveType, ResolvedScalarType, ResolvedType};
use canadensis_dsdl_frontend::TypeKey;

pub fn generate_code(package: &CompiledPackage) {
    let mut generated_types = Vec::new();

    for (key, dsdl) in package {
        match &dsdl.kind {
            DsdlKind::Message { message, .. } => {
                let rust_type = RustTypeName::for_message_type(key);
                // println!("{} from {}", rust_type, key);
                generated_types.push(generate_rust_type(key, message, &rust_type));
            }
            DsdlKind::Service { request, response } => {
                let rust_type = ServiceTypeNames::for_service_type(key);
                // println!("{} request from {}", rust_type.request, key);
                // println!("{} response from {}", rust_type.response, key);
                generated_types.push(generate_rust_type(key, request, &rust_type.request));
                generated_types.push(generate_rust_type(key, response, &rust_type.response));
            }
        }
    }
    let tree: ModuleTree = generated_types.into_iter().collect();

    println!("{}", tree);
}

fn generate_rust_type(key: &TypeKey, message: &Message, rust_type: &RustTypeName) -> GeneratedType {
    match message.kind() {
        MessageKind::Struct(uavcan_struct) => {
            GeneratedType::Struct(GeneratedStruct::new(key, rust_type.clone(), uavcan_struct))
        }
        MessageKind::Union(uavcan_union) => {
            GeneratedType::Enum(GeneratedEnum::new(key, rust_type.clone(), uavcan_union))
        }
    }
}

enum GeneratedType {
    Struct(GeneratedStruct),
    Enum(GeneratedEnum),
}

impl GeneratedType {
    pub fn name(&self) -> &RustTypeName {
        match self {
            GeneratedType::Struct(generated_struct) => &generated_struct.name,
            GeneratedType::Enum(generated_enum) => &generated_enum.name,
        }
    }
}

struct GeneratedStruct {
    uavcan_name: String,
    name: RustTypeName,
    fields: Vec<GeneratedField>,
}

impl GeneratedStruct {
    pub fn new(key: &TypeKey, name: RustTypeName, uavcan_struct: &Struct) -> Self {
        let fields = uavcan_struct
            .fields
            .iter()
            .cloned()
            .filter_map(|field| match field.kind() {
                FieldKind::Padding(_) => None,
                FieldKind::Data { ty, name } => Some(GeneratedField::new(
                    ty,
                    name.clone(),
                    field.always_aligned(),
                )),
            })
            .collect();
        GeneratedStruct {
            uavcan_name: key.to_string(),
            name,
            fields,
        }
    }
}

struct GeneratedField {
    name: String,
    ty: String,
    uavcan_ty: String,
    always_aligned: bool,
}

impl GeneratedField {
    pub fn new(ty: &ResolvedType, name: String, always_aligned: bool) -> Self {
        GeneratedField {
            name: make_rust_identifier(name),
            ty: to_rust_type(ty),
            uavcan_ty: ty.to_string(),
            always_aligned,
        }
    }
}

struct GeneratedEnum {
    uavcan_name: String,
    name: RustTypeName,
    variants: Vec<GeneratedVariant>,
}

impl GeneratedEnum {
    pub fn new(key: &TypeKey, name: RustTypeName, uavcan_enum: &Union) -> Self {
        let variants = uavcan_enum
            .variants
            .iter()
            .cloned()
            .map(|variant| GeneratedVariant::new(&variant.ty, variant.name))
            .collect();
        GeneratedEnum {
            uavcan_name: key.to_string(),
            name,
            variants,
        }
    }
}

struct GeneratedVariant {
    name: String,
    ty: String,
    uavcan_ty: String,
}

impl GeneratedVariant {
    pub fn new(ty: &ResolvedType, name: String) -> Self {
        GeneratedVariant {
            name: make_rust_identifier(name).to_camel_case(),
            ty: to_rust_type(ty),
            uavcan_ty: ty.to_string(),
        }
    }
}

fn to_rust_type(ty: &ResolvedType) -> String {
    match ty {
        ResolvedType::Scalar(scalar) => scalar_to_rust_type(scalar),
        ResolvedType::FixedArray { inner, len } => {
            format!("[{}; {}]", scalar_to_rust_type(inner), len)
        }
        ResolvedType::VariableArray { inner, max_len } => {
            format!(
                "::heapless::Vec<{}, {}>",
                scalar_to_rust_type(inner),
                max_len
            )
        }
    }
}

fn scalar_to_rust_type(scalar: &ResolvedScalarType) -> String {
    match scalar {
        ResolvedScalarType::Composite { key, .. } => {
            RustTypeName::for_message_type(key).to_string()
        }
        ResolvedScalarType::Primitive(primitive) => match primitive {
            PrimitiveType::Boolean => "bool".to_owned(),
            PrimitiveType::Int { bits, .. } => format!("i{}", round_up_integer_size(*bits)),
            PrimitiveType::UInt { bits, .. } => format!("u{}", round_up_integer_size(*bits)),
            PrimitiveType::Float16 { .. } => "::half::f16".to_owned(),
            PrimitiveType::Float32 { .. } => "f32".to_owned(),
            PrimitiveType::Float64 { .. } => "f64".to_owned(),
        },
        ResolvedScalarType::Void { .. } => "()".to_owned(),
    }
}

fn round_up_integer_size(bits: u8) -> u8 {
    match bits {
        0..=8 => 8,
        9..=16 => 16,
        17..=32 => 32,
        33..=64 => 64,
        65..=128 => 128,
        _ => panic!("Integer too large"),
    }
}

struct Module {
    name: String,
    types: Vec<CompiledDsdl>,
    children: Vec<Module>,
}

/// The path and name of a Rust type
#[derive(Debug, Clone)]
struct RustTypeName {
    path: Vec<String>,
    type_name: String,
}

impl RustTypeName {
    pub fn for_message_type(key: &TypeKey) -> Self {
        // For message types:
        // [UAVCAN package path]::[snake case type name][version]::[type name]

        let path = key
            .name()
            .path()
            .iter()
            .cloned()
            .map(make_rust_identifier)
            .chain(iter::once(format!(
                "{}_{}_{}",
                key.name().name().to_snake_case(),
                key.version().major,
                key.version().minor
            )))
            .collect();
        RustTypeName {
            path,
            type_name: make_rust_identifier(key.name().name().to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
struct ServiceTypeNames {
    request: RustTypeName,
    response: RustTypeName,
}

impl ServiceTypeNames {
    pub fn for_service_type(key: &TypeKey) -> Self {
        // For service types:
        // [UAVCAN package path]::[snake case type name][version]::[type name][Request/Response]

        let base = RustTypeName::for_message_type(key);
        let mut request = base.clone();
        request.type_name.push_str("Request");
        let mut response = base;
        response.type_name.push_str("Response");

        ServiceTypeNames { request, response }
    }
}

fn make_rust_identifier(mut identifier: String) -> String {
    if identifier == "_" {
        // Becomes _0
        identifier.push('0');
        identifier
    } else {
        identifier
    }
}

mod fmt_impl {
    use super::{GeneratedEnum, GeneratedField, GeneratedStruct, GeneratedType, RustTypeName};
    use crate::GeneratedVariant;
    use std::fmt::{Display, Formatter, Result};

    impl Display for RustTypeName {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str("crate::")?;
            for segment in &self.path {
                write!(f, "{}::", segment)?;
            }
            write!(f, "{}", self.type_name)
        }
    }

    impl Display for GeneratedType {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                GeneratedType::Struct(inner) => Display::fmt(inner, f),
                GeneratedType::Enum(inner) => Display::fmt(inner, f),
            }
        }
    }

    impl Display for GeneratedStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            writeln!(f, "/// `{}`", self.uavcan_name)?;
            writeln!(f, "pub struct {} {{", self.name.type_name)?;
            for field in &self.fields {
                field.fmt(f)?;
            }
            writeln!(f, "}}")
        }
    }

    impl Display for GeneratedField {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            writeln!(f, "// {}", self.uavcan_ty)?;
            if self.always_aligned {
                writeln!(f, "// Always aligned")?;
            } else {
                writeln!(f, "// Not always aligned")?;
            }
            writeln!(f, "pub {}: {},", self.name, self.ty)
        }
    }

    impl Display for GeneratedEnum {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            writeln!(f, "/// `{}`", self.uavcan_name)?;
            writeln!(f, "pub enum {} {{", self.name.type_name)?;
            for variant in &self.variants {
                variant.fmt(f)?;
            }
            writeln!(f, "}}")
        }
    }

    impl Display for GeneratedVariant {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            writeln!(f, "// {}", self.uavcan_ty)?;
            writeln!(f, "{}({}),", self.name, self.ty)
        }
    }
}
