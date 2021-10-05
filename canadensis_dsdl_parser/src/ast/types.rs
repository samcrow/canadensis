//! Abstract syntax tree types

use num_rational::BigRational;
use pest::Span;

/// An expression and its associated span
#[derive(Debug)]
pub struct Expression<'i> {
    /// The expression
    pub expression: ExpressionType<'i>,
    /// The span in the source that represents this expression
    pub span: Span<'i>,
}

/// An expression
#[derive(Debug)]
pub enum ExpressionType<'i> {
    /// An atom (single value)
    Atom(Box<ExpressionAtom<'i>>),
    /// `+expr`
    UnaryPlus(Box<Expression<'i>>),
    /// `-expr`
    UnaryMinus(Box<Expression<'i>>),
    /// `!expr`
    UnaryNot(Box<Expression<'i>>),
    /// `expr.ident`
    Attribute(Box<Expression<'i>>, &'i str),
    /// `expr ** expr`
    Exponent(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr * expr`
    Multiply(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr / expr`
    Divide(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr % expr`
    Modulo(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr + expr`
    Add(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr - expr`
    Subtract(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr | expr`
    BitOr(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr ^ expr`
    BitXor(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr & expr`
    BitAnd(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr == expr`
    Equal(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr != expr`
    NotEqual(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr <= expr`
    LessOrEqual(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr >= expr`
    GreaterOrEqual(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr < expr`
    Less(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr > expr`
    Greater(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr || expr`
    LogicalOr(Box<Expression<'i>>, Box<Expression<'i>>),
    /// `expr && expr`
    LogicalAnd(Box<Expression<'i>>, Box<Expression<'i>>),
}

/// An atom (single value in an expression)
#[derive(Debug)]
pub enum ExpressionAtom<'i> {
    /// Expression in parentheses
    Parenthesized(Expression<'i>),
    /// Data type
    Type(Type<'i>),
    /// Literal value
    Literal(Literal<'i>),
    /// Identifier
    Identifier(&'i str),
}

/// A literal value
#[derive(Debug)]
pub struct Literal<'i> {
    /// The value
    pub literal: LiteralType<'i>,
    /// The span in the source that contains this literal
    pub span: Span<'i>,
}

/// An identifier
#[derive(Debug)]
pub struct Identifier<'i> {
    /// The name of the identifier
    pub name: &'i str,
    /// The span in the source where this identifier was found
    pub span: Span<'i>,
}

/// A literal value
#[derive(Debug)]
pub enum LiteralType<'i> {
    /// A set of expressions
    Set(Vec<Expression<'i>>),
    /// A number (integer or real)
    Number(BigRational),
    /// A string
    String(String),
    /// A boolean value
    Boolean(bool),
}

/// A type
#[derive(Debug)]
pub enum Type<'i> {
    /// A scalar type
    Scalar(ScalarType<'i>),
    /// An array type
    Array(ArrayType<'i>),
}

/// Scalar types (single values)
#[derive(Debug)]
pub enum ScalarType<'i> {
    /// A composite type with a name and version
    Versioned(VersionedType<'i>),
    /// A primitive type
    Primitive(PrimitiveType),
    /// A void type
    Void { bits: u8 },
}

/// A primitive type
#[derive(Debug)]
pub enum PrimitiveType {
    /// Boolean, always saturated
    Boolean,
    /// Signed integer, always saturated
    Int { bits: u8 },
    /// Unsigned integer
    UInt { bits: u8, mode: CastMode },
    /// 16-bit float
    Float16 { mode: CastMode },
    /// 32-bit float
    Float32 { mode: CastMode },
    /// 64-bit float
    Float64 { mode: CastMode },
}

/// Cast modes for numeric values
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CastMode {
    Truncated,
    Saturated,
}

/// A composite type with a name and version
#[derive(Debug)]
pub struct VersionedType<'i> {
    /// The path segments before the type name, if any
    pub path: Vec<&'i str>,
    /// The name of the type
    pub name: &'i str,
    /// The version of the type
    pub version: TypeVersion,
}

/// The version of a versioned type
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct TypeVersion {
    // These can be 8 bits each: "Valid data type version numbers range from 0 to 255, inclusively"
    pub major: u8,
    pub minor: u8,
}

mod fmt_impl {
    use super::TypeVersion;
    use std::fmt::*;

    impl Display for TypeVersion {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}.{}", self.major, self.minor)
        }
    }
}

/// An array type, with an element type and length
#[derive(Debug)]
pub struct ArrayType<'i> {
    /// The type of each array element
    ///
    /// This can't be an array or void type.
    pub element: ScalarType<'i>,
    /// The allowed range of array lengths
    pub length: ArrayLength<'i>,
}

/// A range of allowed array lengths
#[derive(Debug)]
pub enum ArrayLength<'i> {
    /// `type[expr]`
    Fixed(Expression<'i>),
    /// `type[<=expr]`
    Inclusive(Expression<'i>),
    /// `name[<expr]`
    Exclusive(Expression<'i>),
}

/// A statement in a DSDL file
#[derive(Debug)]
pub enum Statement<'i> {
    /// A directive, like `@assert`
    Directive {
        name: Identifier<'i>,
        value: Option<Expression<'i>>,
    },
    /// A constant, like `uint8 THINGY = 9`
    Constant {
        ty: PrimitiveType,
        name: Identifier<'i>,
        value: Expression<'i>,
    },
    /// A field, like `uint16 value`
    Field {
        ty: Type<'i>,
        name: Identifier<'i>,
        span: Span<'i>,
    },
    /// A void padding field, like `void6`
    PaddingField { bits: u8 },
    /// The `---` marker that separates the request and response fields in a service type
    ServiceResponseMarker,
}

/// The top-level abstract syntax tree of a DSDL file
#[derive(Debug)]
pub struct Definition<'i> {
    /// The statements in the file
    pub statements: Vec<Statement<'i>>,
}
