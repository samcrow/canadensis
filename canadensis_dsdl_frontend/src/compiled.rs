//! Types that represent compiled data types

pub mod package;

use crate::constants::Constants;
use crate::types::ResolvedType;
use canadensis_bit_length_set::BitLengthSet;
use canadensis_dsdl_parser::Span;

/// A compiled DSDL type
#[derive(Debug)]
pub struct CompiledDsdl {
    pub fixed_port_id: Option<u32>,
    pub kind: DsdlKind,
}

/// The two types of compiled DSDL files
#[derive(Debug)]
pub enum DsdlKind {
    /// A message type
    Message(Message),
    /// A service type, containing a request and a response
    ///
    /// Service types can't be named and their constants are not accessible from other DSDL files.
    Service { request: Message, response: Message },
}

/// A DSDL message, request, or response
#[derive(Debug, Clone)]
pub struct Message {
    pub(crate) deprecated: bool,
    pub(crate) extent: Extent,
    pub(crate) kind: MessageKind,
    pub(crate) bit_length: BitLengthSet,
    /// The constants that this message type makes available
    pub(crate) constants: Constants,
    /// Top-level documentation comments from the beginning of the DSDL file (if the file represents
    /// a message type or if this is the request of a service type) or between the request-response
    /// marker and the first field or constant if this is the response of a service type
    pub(crate) comments: String,
}

impl Message {
    /// Returns true if this type is deprecated
    #[inline]
    pub fn deprecated(&self) -> bool {
        self.deprecated
    }
    /// Returns the extent of this type
    #[inline]
    pub fn extent(&self) -> &Extent {
        &self.extent
    }
    /// Returns the kind of this message (struct or union)
    #[inline]
    pub fn kind(&self) -> &MessageKind {
        &self.kind
    }
    /// Returns the set of possible lengths of this message
    #[inline]
    pub fn bit_length(&self) -> &BitLengthSet {
        &self.bit_length
    }
    /// Returns the constants that this message type makes available
    #[inline]
    pub fn constants(&self) -> &Constants {
        &self.constants
    }
    /// Returns the documentation comments for this message
    #[inline]
    pub fn comments(&self) -> &str {
        &self.comments
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
    /// The number of bits used for the discriminant, which identifies the active variant
    pub discriminant_bits: u8,
    /// The possible variants of this union
    pub variants: Vec<Variant>,
}

/// A field of a struct
#[derive(Debug, Clone)]
pub struct Field {
    /// The kind of this field
    kind: FieldKind,
    /// True if this field is always aligned to a multiple of 8 bits
    always_aligned: bool,
    /// Documentation comments
    comments: String,
    /// The offset, in bytes into the file, of the end of this field definition
    end_offset: usize,
}

impl Field {
    pub fn kind(&self) -> &FieldKind {
        &self.kind
    }
    pub fn always_aligned(&self) -> bool {
        self.always_aligned
    }
    /// Returns the documentation comments for this variant
    pub fn comments(&self) -> &str {
        &self.comments
    }
    /// Appends a newline if self.comments is not empty,
    /// followed by the provided string, to the comments of this field
    pub(crate) fn append_comment(&mut self, comment: &str) {
        if !self.comments.is_empty() {
            self.comments.push('\n');
        }
        self.comments.push_str(comment);
    }
    /// Returns the offset in bytes from the beginning of the file of the end of the declaration
    /// of this field
    pub(crate) fn end_offset(&self) -> usize {
        self.end_offset
    }
}

#[derive(Debug, Clone)]
pub enum FieldKind {
    Padding(u8),
    Data { ty: ResolvedType, name: String },
}

impl Field {
    /// A convenience constructor that makes a data field
    pub(crate) fn data(
        ty: ResolvedType,
        name: String,
        always_aligned: bool,
        span: Span<'_>,
    ) -> Self {
        Field {
            kind: FieldKind::Data { ty, name },
            always_aligned,
            comments: String::new(),
            end_offset: span.end(),
        }
    }

    /// A convenience constructor that makes a padding field
    pub(crate) fn padding(bits: u8, always_aligned: bool, span: Span<'_>) -> Self {
        Field {
            kind: FieldKind::Padding(bits),
            always_aligned,
            comments: String::new(),
            end_offset: span.end(),
        }
    }

    /// Returns the name of this field, if it has one
    ///
    /// Padding fields do not have names. All other fields have names.
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
    /// The type of this variant
    ty: ResolvedType,
    /// The name of this variant
    name: String,
    /// Documentation comments
    comments: String,
    /// The offset, in bytes into the file, of the end of this variant definition
    end_offset: usize,
}

impl Variant {
    /// A convenience function that makes a `Variant`
    pub(crate) fn new(ty: ResolvedType, name: String, span: Span<'_>) -> Self {
        Variant {
            ty,
            name,
            comments: String::new(),
            end_offset: span.end(),
        }
    }
    /// Returns the type of this variant's value
    pub fn ty(&self) -> &ResolvedType {
        &self.ty
    }
    /// Returns the name of this variant
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the comments that describe this variant
    pub fn comments(&self) -> &str {
        &self.comments
    }
    /// Appends a newline if self.comments is not empty,
    /// followed by the provided string, to the comments of this variant
    pub(crate) fn append_comment(&mut self, comment: &str) {
        if !self.comments.is_empty() {
            self.comments.push('\n');
        }
        self.comments.push_str(comment);
    }
    /// Returns the offset in bytes from the beginning of the file of the end of the declaration
    /// of this variant
    pub(crate) fn end_offset(&self) -> usize {
        self.end_offset
    }
}
