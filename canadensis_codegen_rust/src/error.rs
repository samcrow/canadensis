use canadensis_dsdl_frontend::types::PrimitiveType;
use canadensis_dsdl_frontend::TypeKey;
use num_bigint::BigInt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum EnumError {
    #[error("Multiple fields in enum type, expected one discriminant field")]
    EnumMultipleFields,
    #[error("Field {0} must be an unsigned integer")]
    FieldType(String),
    #[error("Constant {name} has type {actual}, but should be the same as the field ({expected})")]
    ConstantType {
        name: String,
        actual: PrimitiveType,
        expected: PrimitiveType,
    },
    #[error("Constant {name} has value {value}, which is already used by {already_used_by}")]
    ConstantValue {
        name: String,
        value: BigInt,
        already_used_by: String,
    },
    #[error("#[canadensis(enum)] is not allowed on DSDL union types")]
    NotStruct,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to generate code for {key}")]
    Dsdl {
        key: TypeKey,
        #[source]
        inner: Box<dyn std::error::Error>,
    },
}
