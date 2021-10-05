use crate::types::constant::Constant;
use crate::types::Type;
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
#[derive(Debug)]
pub struct Message {
    pub deprecated: bool,
    pub extent: Extent,
    pub kind: MessageKind,
    pub bit_length: BitLengthSet,
}

/// The extent of a type
#[derive(Debug)]
pub enum Extent {
    /// Sealed type
    Sealed,
    /// Delimited type with a fixed extent
    Delimited(u64),
}

#[derive(Debug)]
pub enum MessageKind {
    Struct(Struct),
    Union(Union),
}

#[derive(Debug)]
pub struct Struct {
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Union {
    pub variants: Vec<Field>,
}

#[derive(Debug)]
pub enum Field {
    Padding(u8),
    Data { ty: Type, name: String },
}
