use std::ops::Deref;
use unicode_normalization::UnicodeNormalization;

/// A string value
///
/// This type has two differences from a normal String:
///
/// 1. String values containing a single ASCII character have implicit integer values
/// 2. String values are always in NFC form (Unicode Normalization Form C: canonical decomposition
///    followed by canonical composition)
///
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct StringValue(String);

impl StringValue {
    /// Creates an empty string value
    pub fn new() -> Self {
        StringValue(String::new())
    }

    /// Returns the implicit integer value of this string, if any
    ///
    /// A string has an implicit integer value if it contains exactly one character and that
    /// character is in the ASCII range (0..=127). The implicit integer value is the value of
    /// that character (see section 3.3.2 of the specification).
    pub fn implicit_int(&self) -> Option<u8> {
        match self.0.as_bytes() {
            [character] if character.is_ascii() => Some(*character),
            _ => None,
        }
    }

    /// Appends another string to this string
    pub fn push_str(&mut self, other: &str) {
        self.0.push_str(other);
        // Re-apply normalization
        self.0 = self.0.nfc().collect();
    }
}

impl From<String> for StringValue {
    fn from(s: String) -> Self {
        <Self as From<&'_ str>>::from(&s)
    }
}

impl From<&'_ str> for StringValue {
    fn from(s: &str) -> Self {
        // Apply normalization
        StringValue(s.nfc().collect())
    }
}

impl Deref for StringValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Default for StringValue {
    fn default() -> Self {
        StringValue::new()
    }
}
