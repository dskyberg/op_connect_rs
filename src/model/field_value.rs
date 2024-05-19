use std::ops::Deref;

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Zeroized value to ensure secure cleanup.  Note, if you clone the
/// field value inner, you are responsible for cleanup.
///
/// Deref is supported.  
/// ```rust
/// use op_connect_rs::FieldValue;
///
/// fn print_value() {
///     let field_value = FieldValue::new("secret");
///     println!("Value: {}", *field_value);
/// }
/// ```
///
#[derive(Debug, Clone, Deserialize, Serialize, Zeroize, ZeroizeOnDrop)]
pub struct FieldValue(String);

impl FieldValue {
    /// Create a FieldValue.  This is just for testing.
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    /// Returns a reference to the actual value.
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Deref for FieldValue {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deref() {
        let secret = "secret";
        let field_value = FieldValue::new(secret);
        assert_eq!(*field_value, secret);
    }
}
