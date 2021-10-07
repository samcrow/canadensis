//! Types that represent compiled data types

pub mod package;

use crate::types::constant::Constant;
use crate::types::ResolvedType;
use canadensis_bit_length_set::BitLengthSet;
use std::collections::BTreeMap;

/// A compiled DSDL type
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
    pub(crate) deprecated: bool,
    pub(crate) extent: Extent,
    pub(crate) kind: MessageKind,
    pub(crate) bit_length: BitLengthSet,
}

impl Message {
    /// Returns true if this type is deprecated
    pub fn deprecated(&self) -> bool {
        self.deprecated
    }
    /// Returns the extent of this type
    pub fn extent(&self) -> &Extent {
        &self.extent
    }
    /// Returns the kind of this message (struct or union)
    pub fn kind(&self) -> &MessageKind {
        &self.kind
    }
    /// Returns the set of possible lengths of this message
    pub fn bit_length(&self) -> &BitLengthSet {
        &self.bit_length
    }
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
pub struct Field {
    /// The kind of this field
    kind: FieldKind,
    /// True if this field is always aligned to a multiple of 8 bits
    always_aligned: bool,
}

#[derive(Debug, Clone)]
pub enum FieldKind {
    Padding(u8),
    Data { ty: ResolvedType, name: String },
}

impl Field {
    /// A convenience constructor that makes a data field
    pub(crate) fn data(ty: ResolvedType, name: String, always_aligned: bool) -> Self {
        Field {
            kind: FieldKind::Data { ty, name },
            always_aligned,
        }
    }

    /// A convenience constructor that makes a padding field
    pub(crate) fn padding(bits: u8, always_aligned: bool) -> Self {
        Field {
            kind: FieldKind::Padding(bits),
            always_aligned,
        }
    }

    /// Returns the name of this field, if it has one
    pub fn name(&self) -> Option<&str> {
        match &self.kind {
            FieldKind::Padding(_) => None,
            FieldKind::Data { name, .. } => Some(&*name),
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
    pub(crate) fn new(ty: ResolvedType, name: String) -> Self {
        Variant { ty, name }
    }
}
