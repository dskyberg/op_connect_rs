use serde::{Deserialize, Serialize};

/// Section Object
///
/// ```json
/// {
///    "id": "95cdbc3b-7742-47ec-9056-44d6af82d562"
///    "label": "Security Questions",
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    /// A unique identifier for the section.
    pub id: String,
    /// The label for the section.
    pub label: Option<String>,
}

impl Section {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            label: None,
        }
    }
    pub fn with_label(self, label: &str) -> Self {
        Self {
            id: self.id,
            label: Some(label.to_owned()),
        }
    }
}
