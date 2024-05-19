use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::VaultType;

/// Vault Object
///
/// ```json
/// {
///    "id": "ytrfte14kw1uex5txaore1emkz",
///    "name": "Demo",
///   "attributeVersion": 1,
///    "contentVersion": 72,
///    "items": 7,
///    "type": "USER_CREATED",
///    "createdAt": "2021-04-10T17:34:26Z",
///    "updatedAt": "2021-04-13T14:33:50Z"
///}
/// ```
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vault {
    /// The UUID of the vault.
    pub id: String,
    /// The name of the vault.
    pub name: String,
    /// The description for the vault.
    pub description: Option<String>,
    /// The version of the vault metadata.
    pub attribute_version: Option<u32>,
    /// The version of the vault contents.
    pub content_version: Option<u32>,
    /// Number of active items in the vault.
    pub items: Option<u32>,
    /// The type of vault.
    #[serde(rename = "type")]
    pub _type: Option<VaultType>,
    /// Date and time when the vault was created.
    pub created_at: Option<DateTime<Utc>>,
    /// Date and time when the vault or its contents were last changed.
    pub updated_at: Option<DateTime<Utc>>,
}

impl std::fmt::Debug for Vault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Vault");

        debug.field("id", &self.id);
        debug.field("name", &self.name);

        if self.description.is_some() {
            debug.field("description", &self.description);
        }
        if self.attribute_version.is_some() {
            debug.field("attribute_version", &self.attribute_version);
        }
        if self.content_version.is_some() {
            debug.field("content_version", &self.content_version);
        }
        if self.items.is_some() {
            debug.field("items", &self.items);
        }
        if self._type.is_some() {
            debug.field("_type", &self._type);
        }
        if self.created_at.is_some() {
            debug.field("created_at", &self.created_at);
        }
        if self.updated_at.is_some() {
            debug.field("updated_at", &self.updated_at);
        }
        debug.finish()
    }
}
