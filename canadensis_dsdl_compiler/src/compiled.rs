use crate::types::constant::Constant;
use crate::types::ResolvedType;
use canadensis_bit_length_set::BitLengthSet;
use std::collections::BTreeMap;

/// A compiled DSDL message
#[derive(Debug)]
pub struct CompiledDsdl {
    pub fixed_port_id: Option<u32>,
    pub kind: DsdlKind,
}

#[derive(Debug)]
pub enum DsdlKind {
    Message {
        message: Message,
        /// The constants that this message type makes available
        constants: BTreeMap<String, Constant>,
    },
    /// A service type, containing a request and a response
    ///
    /// Service types can't be named and don't have any available constants.
    Service { request: Message, response: Message },
}

/// A DSDL message, request, or response
#[derive(Debug, Clone)]
pub struct Message {
    pub deprecated: bool,
    pub extent: Extent,
    pub kind: MessageKind,
    pub bit_length: BitLengthSet,
}

/// The extent of a type
#[derive(Debug, Clone)]
pub enum Extent {
    /// Sealed type
    Sealed,
    /// Delimited type with a fixed extent
    Delimited(u64),
}

#[derive(Debug, Clone)]
pub enum MessageKind {
    Struct(Struct),
    Union(Union),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Union {
    pub variants: Vec<Variant>,
}

/// A field of a struct
#[derive(Debug, Clone)]
pub enum Field {
    Padding(u8),
    Data { ty: ResolvedType, name: String },
}

impl Field {
    /// A convenience constructor that makes a `Field::Data`
    pub fn data(ty: ResolvedType, name: String) -> Self {
        Field::Data { ty, name }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Field::Padding(_) => None,
            Field::Data { name, .. } => Some(&*name),
        }
    }
}

/// A variant of a union
#[derive(Debug, Clone)]
pub struct Variant {
    pub ty: ResolvedType,
    pub name: String,
}

impl Variant {
    /// A convenience function that makes a `Variant`
    pub fn new(ty: ResolvedType, name: String) -> Self {
        Variant { ty, name }
    }
}
