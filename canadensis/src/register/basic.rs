//! Basic register types

use crate::encoding::f16_zerocopy::ZeroCopyF16;
use crate::register::{Access, Register, WriteError};
use canadensis_data_types::uavcan::primitive::array::integer16_1_0::Integer16;
use canadensis_data_types::uavcan::primitive::array::integer32_1_0::Integer32;
use canadensis_data_types::uavcan::primitive::array::integer64_1_0::Integer64;
use canadensis_data_types::uavcan::primitive::array::integer8_1_0::Integer8;
use canadensis_data_types::uavcan::primitive::array::natural16_1_0::Natural16;
use canadensis_data_types::uavcan::primitive::array::natural32_1_0::Natural32;
use canadensis_data_types::uavcan::primitive::array::natural64_1_0::Natural64;
use canadensis_data_types::uavcan::primitive::array::natural8_1_0::Natural8;
use canadensis_data_types::uavcan::primitive::array::real16_1_0::Real16;
use canadensis_data_types::uavcan::primitive::array::real32_1_0::Real32;
use canadensis_data_types::uavcan::primitive::array::real64_1_0::Real64;
use canadensis_data_types::uavcan::primitive::string_1_0;
use canadensis_data_types::uavcan::primitive::unstructured_1_0;
use canadensis_data_types::uavcan::register::value_1_0::Value;
use core::convert::TryFrom;
use half::f16;

/// A register containing its name, value, and mutable/persistent flags
#[derive(Debug, Clone)]
pub struct SimpleRegister<T> {
    name: &'static str,
    access: Access,
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
            access: Access {
                mutable,
                persistent,
            },
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

    fn access(&self) -> Access {
        self.access.clone()
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

/// A register with a validation function that can reject invalid write operations
///
/// Each write operation is checked with the validator. If the validator returns false, the value
/// of the register does not change.
///
/// # Examples
///
/// ```
/// # use canadensis_data_types::uavcan::register::value_1_0::Value;
/// # use canadensis_data_types::uavcan::primitive::array::real32_1_0::Real32;
/// # use canadensis::register::basic::ValidatedRegister;
/// # use canadensis::register::Register;
/// fn is_finite_float(value: &f32) -> bool {
///     !(value.is_infinite() || value.is_nan())
/// }
/// let mut finite_float_register =
///     ValidatedRegister::new("test.float", true, true, is_finite_float);
/// assert!(finite_float_register
///     .write(&Value::Real32(Real32 { value: heapless::Vec::from_slice(&[37.0]).unwrap() }))
///     .is_ok());
/// assert!(finite_float_register
///     .write(&Value::Real32(Real32 {
///         value: heapless::Vec::from_slice(&[f32::INFINITY]).unwrap()}
///     ))
///     .is_err());
/// assert!(finite_float_register
///     .write(&Value::Real32(Real32 {
///         value:heapless::Vec::from_slice(&[f32::NEG_INFINITY]).unwrap()}
///     ))
///     .is_err());
/// assert!(finite_float_register
///     .write(&Value::Real32(Real32 {
///         value: heapless::Vec::from_slice(&[f32::NAN]).unwrap()}
///     ))
///     .is_err());
/// ```
pub struct ValidatedRegister<T, V = fn(&T) -> bool> {
    name: &'static str,
    access: Access,
    value: T,
    validator: V,
}

impl<T, V> core::fmt::Debug for ValidatedRegister<T, V>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ValidatedRegister")
            .field("name", &self.name)
            .field("access", &self.access)
            .field("value", &self.value)
            .finish()
    }
}

impl<T, V> ValidatedRegister<T, V>
where
    T: Default,
{
    /// Creates a register containing the default value of type T
    ///
    /// The validator should consider `T::default()` to be valid.
    pub fn new(name: &'static str, mutable: bool, persistent: bool, validator: V) -> Self {
        Self::with_value(name, mutable, persistent, T::default(), validator)
    }
}

impl<T, V> ValidatedRegister<T, V> {
    /// Creates a register with the provided initial value
    ///
    /// The validator should consider the provided value to be valid
    pub fn with_value(
        name: &'static str,
        mutable: bool,
        persistent: bool,
        value: T,
        validator: V,
    ) -> Self {
        ValidatedRegister {
            name,
            access: Access {
                mutable,
                persistent,
            },
            value,
            validator,
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

impl<T, V> Register for ValidatedRegister<T, V>
where
    T: RegisterType + Clone,
    V: Validator<T>,
{
    fn name(&self) -> &str {
        self.name
    }

    fn access(&self) -> Access {
        self.access.clone()
    }

    fn read(&self) -> Value {
        self.value.read()
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        let mut new_value = self.value.clone();
        new_value.write(value)?;
        if self.validator.accept(&new_value) {
            self.value = new_value;
            Ok(())
        } else {
            Err(WriteError::Type)
        }
    }
}

/// A validator that can accept or reject a new register value
///
/// This trait is implemented for all function pointers and `FnMut`/`Fn` closures that accept
/// an `&T` and return a `bool`.
pub trait Validator<T> {
    /// Returns true if the value is valid and should be stored, or false to reject the value
    fn accept(&mut self, value: &T) -> bool;
}

impl<F, T> Validator<T> for F
where
    F: FnMut(&T) -> bool,
{
    fn accept(&mut self, value: &T) -> bool {
        self(value)
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
macro_rules! register_primitive {
    ($type:ty, $variant:ident) => {
        impl RegisterType for $type {
            fn read(&self) -> Value {
                Value::$variant($variant {
                    value: heapless::Vec::from_slice(&[*self]).unwrap(),
                })
            }

            fn write(&mut self, value: &Value) -> Result<(), WriteError> {
                if let Value::$variant($variant { value: values }) = value {
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

register_primitive!(u8, Natural8);
register_primitive!(u16, Natural16);
register_primitive!(u32, Natural32);
register_primitive!(u64, Natural64);
register_primitive!(i8, Integer8);
register_primitive!(i16, Integer16);
register_primitive!(i32, Integer32);
register_primitive!(i64, Integer64);
// register_primitive!(f16, Real16);
register_primitive!(f32, Real32);
register_primitive!(f64, Real64);

macro_rules! register_primitive_array {
    ($type:ty, $variant:ident) => {
        impl<const N: usize> RegisterType for [$type; N] {
            /// Reads the value of an array register
            ///
            /// If this array is longer than the maximum capacity of the corresponding `Value` variant,
            /// the returned value will be truncated.
            fn read(&self) -> Value {
                let mut value_vec = heapless::Vec::new();
                if N <= value_vec.capacity() {
                    value_vec
                        .extend_from_slice(&*self)
                        .expect("Incorrect length calculation");
                } else {
                    // Truncate to the maximum allowed size
                    value_vec
                        .extend_from_slice(&self[..value_vec.capacity()])
                        .expect("Incorrect length calculation");
                }
                Value::$variant($variant { value: value_vec })
            }

            /// Writes an array register
            ///
            /// This function returns an error if the length of the provided `Value` is not equal to
            /// the length of this array.
            fn write(&mut self, value: &Value) -> Result<(), WriteError> {
                match value {
                    Value::$variant($variant { value: values }) => {
                        if values.len() == N {
                            self.copy_from_slice(&values);
                            Ok(())
                        } else {
                            Err(WriteError::Type)
                        }
                    }
                    _ => Err(WriteError::Type),
                }
            }
        }
    };
}

register_primitive_array!(u8, Natural8);
register_primitive_array!(u16, Natural16);
register_primitive_array!(u32, Natural32);
register_primitive_array!(u64, Natural64);
register_primitive_array!(i8, Integer8);
register_primitive_array!(i16, Integer16);
register_primitive_array!(i32, Integer32);
register_primitive_array!(i64, Integer64);
// register_primitive_array!(f16, Real16);
register_primitive_array!(f32, Real32);
register_primitive_array!(f64, Real64);

// Need to implement f16 manually because of the ZeroCopyF16 type,
// until this is merged and released: https://github.com/starkat99/half-rs/pull/45

impl RegisterType for f16 {
    fn read(&self) -> Value {
        Value::Real16(Real16 {
            value: heapless::Vec::from_slice(&[ZeroCopyF16::from(*self)]).unwrap(),
        })
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        if let Value::Real16(Real16 { value: values }) = value {
            if values.len() == 1 {
                *self = values[0].into();
                Ok(())
            } else {
                Err(WriteError::Type)
            }
        } else {
            Err(WriteError::Type)
        }
    }
}
impl<const N: usize> RegisterType for [f16; N] {
    /// Reads the value of an array register
    ///
    /// If this array is longer than the maximum capacity of the corresponding `Value` variant,
    /// the returned value will be truncated.
    fn read(&self) -> Value {
        let mut value_vec = heapless::Vec::new();
        if N <= value_vec.capacity() {
            value_vec.extend(self.iter().map(|value| ZeroCopyF16::from(*value)));
        } else {
            // Truncate to the maximum allowed size
            value_vec.extend(
                self[..value_vec.capacity()]
                    .iter()
                    .map(|value| ZeroCopyF16::from(*value)),
            );
        }
        Value::Real16(Real16 { value: value_vec })
    }

    /// Writes an array register
    ///
    /// This function returns an error if the length of the provided `Value` is not equal to
    /// the length of this array.
    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        match value {
            Value::Real16(Real16 { value: values }) => {
                if values.len() == N {
                    for (entry, value) in self.iter_mut().zip(values.iter()) {
                        *entry = f16::from(*value);
                    }
                    Ok(())
                } else {
                    Err(WriteError::Type)
                }
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
        Value::String(string_1_0::String {
            value: self.0.clone(),
        })
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        match value {
            Value::String(string_1_0::String { value: bytes }) => {
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
        Value::Unstructured(unstructured_1_0::Unstructured {
            value: self.0.clone(),
        })
    }

    fn write(&mut self, value: &Value) -> Result<(), WriteError> {
        match value {
            Value::Unstructured(unstructured_1_0::Unstructured { value: bytes }) => {
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

/// A non-mutable, persistent register that holds a fixed string value
///
/// This is useful for registers that provide information and cannot be changed.
#[derive(Debug)]
pub struct FixedStringRegister {
    /// Register name, not empty, 256 bytes or shorter
    name: &'static str,
    /// Register value, 256 bytes or shorter
    value: &'static str,
}

impl FixedStringRegister {
    /// Creates a register
    ///
    /// This function returns None if name is empty or if either parameter is longer than 256 bytes.
    pub fn new(name: &'static str, value: &'static str) -> Option<Self> {
        if !name.is_empty() && name.len() <= 256 && value.len() <= 256 {
            Some(FixedStringRegister { name, value })
        } else {
            None
        }
    }
}

impl Register for FixedStringRegister {
    fn name(&self) -> &str {
        self.name
    }

    fn access(&self) -> Access {
        Access {
            mutable: false,
            persistent: true,
        }
    }

    fn read(&self) -> Value {
        Value::String(string_1_0::String {
            value: heapless::Vec::from_slice(self.value.as_bytes())
                .expect("Register value too long"),
        })
    }

    fn write(&mut self, _value: &Value) -> Result<(), WriteError> {
        unimplemented!("A FixedStringRegister cannot be written")
    }
}
