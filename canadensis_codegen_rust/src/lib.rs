extern crate canadensis_bit_length_set;
extern crate canadensis_dsdl_frontend;
extern crate heck;

mod impl_data_type;
mod impl_serialize;
mod module_tree;
mod size_bits;

use canadensis_bit_length_set::BitLengthSet;
use heck::{CamelCase, SnakeCase};
use std::iter;

use crate::module_tree::ModuleTree;
use canadensis_dsdl_frontend::compiled::package::CompiledPackage;
use canadensis_dsdl_frontend::compiled::{
    CompiledDsdl, DsdlKind, Extent, FieldKind, Message, MessageKind, Struct, Union,
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
                generated_types.push(generate_rust_type(
                    key,
                    message,
                    &rust_type,
                    message.extent().clone(),
                    MessageRole::Message,
                ));
            }
            DsdlKind::Service { request, response } => {
                let rust_type = ServiceTypeNames::for_service_type(key);
                // println!("{} request from {}", rust_type.request, key);
                // println!("{} response from {}", rust_type.response, key);
                generated_types.push(generate_rust_type(
                    key,
                    request,
                    &rust_type.request,
                    request.extent().clone(),
                    MessageRole::Request,
                ));
                generated_types.push(generate_rust_type(
                    key,
                    response,
                    &rust_type.response,
                    response.extent().clone(),
                    MessageRole::Response,
                ));
            }
        }
    }
    let tree: ModuleTree = generated_types.into_iter().collect();

    println!("#![allow_unused_variables\n{}", tree);
}

fn generate_rust_type(
    key: &TypeKey,
    message: &Message,
    rust_type: &RustTypeName,
    extent: Extent,
    role: MessageRole,
) -> GeneratedType {
    let length = message.bit_length().clone();
    match message.kind() {
        MessageKind::Struct(uavcan_struct) => {
            GeneratedType::new_struct(key, rust_type.clone(), length, extent, role, uavcan_struct)
        }
        MessageKind::Union(uavcan_union) => {
            GeneratedType::new_enum(key, rust_type.clone(), length, extent, role, uavcan_union)
        }
    }
}

struct GeneratedType {
    uavcan_name: String,
    name: RustTypeName,
    size: BitLengthSet,
    extent: Extent,
    role: MessageRole,
    kind: GeneratedTypeKind,
}

enum GeneratedTypeKind {
    Struct(GeneratedStruct),
    Enum(GeneratedEnum),
}

impl GeneratedType {
    pub fn new_struct(
        key: &TypeKey,
        name: RustTypeName,
        size: BitLengthSet,
        extent: Extent,
        role: MessageRole,
        uavcan_struct: &Struct,
    ) -> GeneratedType {
        let fields = uavcan_struct
            .fields
            .iter()
            .cloned()
            .filter_map(|field| match field.kind() {
                FieldKind::Padding(_) => None,
                FieldKind::Data { ty, name } => Some(GeneratedField::new(
                    ty.clone(),
                    name.clone(),
                    field.always_aligned(),
                )),
            })
            .collect();
        GeneratedType::new(
            key,
            name,
            size,
            extent,
            role,
            GeneratedTypeKind::Struct(GeneratedStruct { fields }),
        )
    }
    pub fn new_enum(
        key: &TypeKey,
        name: RustTypeName,
        size: BitLengthSet,
        extent: Extent,
        role: MessageRole,
        uavcan_union: &Union,
    ) -> GeneratedType {
        let variants = uavcan_union
            .variants
            .iter()
            .cloned()
            .map(|variant| GeneratedVariant::new(variant.ty.clone(), variant.name))
            .collect();
        GeneratedType::new(
            key,
            name,
            size,
            extent,
            role,
            GeneratedTypeKind::Enum(GeneratedEnum {
                discriminant_bits: uavcan_union.discriminant_bits,
                variants,
            }),
        )
    }

    fn new(
        key: &TypeKey,
        name: RustTypeName,
        size: BitLengthSet,
        extent: Extent,
        role: MessageRole,
        kind: GeneratedTypeKind,
    ) -> Self {
        GeneratedType {
            uavcan_name: key.to_string(),
            name,
            size,
            extent,
            role,
            kind,
        }
    }
}

struct GeneratedStruct {
    fields: Vec<GeneratedField>,
}

struct GeneratedField {
    name: String,
    ty: String,
    uavcan_ty: ResolvedType,
    always_aligned: bool,
}

impl GeneratedField {
    pub fn new(ty: ResolvedType, name: String, always_aligned: bool) -> Self {
        GeneratedField {
            name: make_rust_identifier(name),
            ty: to_rust_type(&ty),
            uavcan_ty: ty,
            always_aligned,
        }
    }
}

struct GeneratedEnum {
    /// The number of bits used for the discriminant, which identifies the active variant
    discriminant_bits: u8,
    /// The enum variants
    variants: Vec<GeneratedVariant>,
}

struct GeneratedVariant {
    name: String,
    ty: String,
    uavcan_ty: ResolvedType,
}

impl GeneratedVariant {
    pub fn new(ty: ResolvedType, name: String) -> Self {
        GeneratedVariant {
            name: make_rust_identifier(name).to_camel_case(),
            ty: to_rust_type(&ty),
            uavcan_ty: ty,
        }
    }
}

/// The role of a generated message type
enum MessageRole {
    /// A message (not service-related)
    Message,
    /// A service request
    Request,
    /// A service response
    Response,
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
        65..=u8::MAX => panic!("Integer too large"),
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
    use super::{GeneratedField, GeneratedType, RustTypeName};
    use crate::impl_data_type::ImplementDataType;
    use crate::impl_serialize::ImplementSerialize;
    use crate::{GeneratedTypeKind, GeneratedVariant};
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
            writeln!(f, "/// `{}`", self.uavcan_name)?;
            let min_size = self.size.min();
            let max_size = self.size.max();
            if min_size == max_size {
                writeln!(f, "/// Fixed size {} bytes", min_size / 8)?;
            } else {
                writeln!(
                    f,
                    "/// Size ranges from {} to {} bytes",
                    min_size / 8,
                    max_size / 8
                )?;
            }
            match &self.kind {
                GeneratedTypeKind::Struct(inner) => {
                    writeln!(f, "pub struct {} {{", self.name.type_name)?;
                    for field in &inner.fields {
                        field.fmt(f)?;
                    }
                    writeln!(f, "}}")?;
                }
                GeneratedTypeKind::Enum(inner) => {
                    writeln!(f, "pub enum {} {{", self.name.type_name)?;
                    for variant in &inner.variants {
                        variant.fmt(f)?;
                    }
                    writeln!(f, "}}")?;
                }
            }

            Display::fmt(&ImplementDataType(self), f)?;
            Display::fmt(&ImplementSerialize(self), f)?;

            Ok(())
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

    impl Display for GeneratedVariant {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            writeln!(f, "// {}", self.uavcan_ty)?;
            writeln!(f, "{}({}),", self.name, self.ty)
        }
    }
}
