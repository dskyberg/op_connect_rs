use serde::{Deserialize, Serialize};

use super::{FieldPurpose, FieldType, FieldValue};

// Item field object
/// ```json
/// {
///    "section": {
///        "id": "95cdbc3b-7742-47ec-9056-44d6af82d562"
///    },
///    "type": "STRING",
///    "generate": true,
///    "label": "Random Text"
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Field {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub _type: FieldType,
    /// Use purpose for the username, password, and notes fields
    pub purpose: Option<FieldPurpose>,
    /// The value to save for the field.
    /// You can specify a generate field instead of value to create a password or other
    /// random information for the value.
    pub value: Option<FieldValue>,
    /// Generate a password and save in the value for the field. By default,
    /// the password is a 32-characters long, made up of letters, numbers, and symbols.
    /// To customize the password, include a recipe field.
    pub generate: Option<bool>,
}
