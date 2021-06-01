//! Basic register types

use crate::register::{Register, WriteError};
use canadensis_data_types::uavcan::register::value::Value;
use core::convert::TryFrom;
use half::f16;

/// A register containing its name, value, and mutable/persistent flags
#[derive(Debug, Clone)]
pub struct SimpleRegister<T> {
    name: &'static str,
    mutable: bool,
    persistent: bool,
    value: T,
}

impl<T> SimpleRegister<T> {
    /// Creates a register initialized to the default value of type T
    ///
    /// The name should not be more than 256 bytes long.
    pub fn new(name: &'static str, mutable: bool, persistent: bool) -> Self
    where
        T: Default,
    {
        Self::with_value(name, mutable, persistent, T::default())
    }

    /// Creates a register initialized to the default value of type T
    ///
    /// The name should not be more than 256 bytes long.
    pub fn with_value(name: &'static str, mutable: bool, persistent: bool, value: T) -> Self {
        SimpleRegister {
            name,
            mutable,
            persistent,
            value,
        }
    }

    /// Returns a reference to the value of this register
    pub fn value(&self) -> &T {
        &self.value
    }
    /// Returns a mutable reference to the value of this register
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
    /// Sets the value of this register
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

impl<T> Register for SimpleRegister<T>
where
    T: RegisterType,
{
    fn name(&self) -> &str {
        self.name
    }

    fn is_mutable(&self) -> bool {
        self.mutable
    }

    fn is_persistent(&self) -> bool {
        self.persistent
    }

    fn read(&self) -> Value {
        self.value.read()
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        // Ignore the mutable flag, because this may be used for initialization from persistent
        // storage
        self.value.write(value)
    }
}

/// A type that can be stored in a register
pub trait RegisterType {
    /// Reads this register and returns its value
    ///
    /// This function must not return `Value::Empty`.
    fn read(&self) -> Value;
    /// Writes the value of this register
    ///
    /// This function returns an error if the provided value does not have an appropriate type
    /// for this register. This function will not be called on a non-mutable register.
    ///
    /// If this function returns an error, the value of this register must be the same as before
    /// the call to write().
    fn write(&mut self, value: &Value) -> Result<(), WriteError>;
}

/// Implements RegisterType for an integer
macro_rules! register_integer {
    ($type:ty, $variant:ident) => {
        impl RegisterType for $type {
            fn read(&self) -> Value {
                Value::$variant(heapless::Vec::from_slice(&[*self]).unwrap())
            }

            fn write(&mut self, value: &Value) -> Result<(), WriteError> {
                if let Value::$variant(values) = value {
                    if values.len() == 1 {
                        *self = values[0];
                        Ok(())
                    } else {
                        Err(WriteError::Type)
                    }
                } else {
                    Err(WriteError::Type)
                }
            }
        }
    };
}

register_integer!(u8, Natural8);
register_integer!(u16, Natural16);
register_integer!(u32, Natural32);
register_integer!(u64, Natural64);
register_integer!(i8, Integer8);
register_integer!(i16, Integer16);
register_integer!(i32, Integer32);
register_integer!(i64, Integer64);

impl RegisterType for f16 {
    fn read(&self) -> Value {
        Value::Real16(*self)
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        match value {
            Value::Real16(inner) => {
                *self = *inner;
                Ok(())
            }
            _ => Err(WriteError::Type),
        }
    }
}

impl RegisterType for f32 {
    fn read(&self) -> Value {
        Value::Real32(*self)
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        match value {
            Value::Real32(inner) => {
                *self = *inner;
                Ok(())
            }
            _ => Err(WriteError::Type),
        }
    }
}

impl RegisterType for f64 {
    fn read(&self) -> Value {
        Value::Real64(*self)
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        match value {
            Value::Real64(inner) => {
                *self = *inner;
                Ok(())
            }
            _ => Err(WriteError::Type),
        }
    }
}

/// A string value for a register
#[derive(Debug, Clone, Default)]
pub struct RegisterString(pub heapless::Vec<u8, 256>);

impl RegisterType for RegisterString {
    fn read(&self) -> Value {
        Value::String(self.0.clone())
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        match value {
            Value::String(bytes) => {
                self.0.clone_from(bytes);
                Ok(())
            }
            _ => Err(WriteError::Type),
        }
    }
}

impl TryFrom<&[u8]> for RegisterString {
    type Error = LengthError;

    /// Creates a register string from a slice of bytes, or returns an error if the length of bytes
    /// is greater than 256
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let byte_vec = heapless::Vec::from_slice(value).map_err(|_| LengthError(()))?;
        Ok(RegisterString(byte_vec))
    }
}

impl TryFrom<&str> for RegisterString {
    type Error = LengthError;

    /// Creates a register string from a string slice, or returns an error if the number of bytes
    /// in the string is greater than 256
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        RegisterString::try_from(value.as_bytes())
    }
}

/// An unstructured byte array value for a register
#[derive(Debug, Clone, Default)]
pub struct Unstructured(pub heapless::Vec<u8, 256>);

impl RegisterType for Unstructured {
    fn read(&self) -> Value {
        Value::String(self.0.clone())
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        match value {
            Value::String(bytes) => {
                self.0.clone_from(bytes);
                Ok(())
            }
            _ => Err(WriteError::Type),
        }
    }
}

impl TryFrom<&[u8]> for Unstructured {
    type Error = LengthError;

    /// Creates an unstructured value from a slice of bytes, or returns an error if the length of
    /// bytes is greater than 256
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let byte_vec = heapless::Vec::from_slice(value).map_err(|_| LengthError(()))?;
        Ok(Unstructured(byte_vec))
    }
}

/// An error indicating that a provided value was too long
#[derive(Debug)]
pub struct LengthError(());
