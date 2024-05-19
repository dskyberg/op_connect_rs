use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Category, Field, FieldValue, File, Section, Url, Vault};

///
/// ```json
///{
///    "id": "2fcbqwe9ndg175zg2dzwftvkpa",
///    "title": "Secrets Automation Item",
///    "tags": [
///        "connect",
///        "\ud83d\udc27"
///    ],
///    "vault": {
///        "id": "ftz4pm2xxwmwrsd7rjqn7grzfz"
///    },
///    "category": "LOGIN",
///    "sections": [
///        {
///            "id": "95cdbc3b-7742-47ec-9056-44d6af82d562",
///            "label": "Security Questions"
///        }
///    ],
///    "fields": [
///        {
///            "id": "username",
///            "type": "STRING",
///            "purpose": "USERNAME",
///            "label": "username",
///            "value": "wendy"
///        },
///        {
///            "id": "password",
///            "type": "CONCEALED",
///            "purpose": "PASSWORD",
///            "label": "password",
///            "value": "hLDegPkuMQqyQiyDZqRdWGoojiN5KYQtXuA0wBDe9z3Caj6FQGHpbGu",
///            "entropy": 189.78359985351562
///        },
///        {
///            "id": "notesPlain",
///            "type": "STRING",
///            "purpose": "NOTES",
///            "label": "notesPlain"
///        },
///        {
///            "id": "a6cvmeqakbxoflkgmor4haji7y",
///            "type": "URL",
///            "label": "Example",
///            "value": "https://example.com"
///        },
///        {
///            "id": "boot3vsxwhuht6g7cmcx4m6rcm",
///            "section": {
///                "id": "95cdbc3b-7742-47ec-9056-44d6af82d562"
///            },
///            "type": "CONCEALED",
///            "label": "Recovery Key",
///            "value": "s=^J@GhHP_isYP>LCq?vv8u7T:*wBP.c"
///        },
///        {
///            "id": "axwtgyjrvwfp5ij7mtkw2zvijy",
///            "section": {
///                "id": "95cdbc3b-7742-47ec-9056-44d6af82d562"
///            },
///            "type": "STRING",
///            "label": "Random Text",
///            "value": "R)D~KZdV!8?51QoCibDUse7=n@wKR_}]"
///        }
///    ],
///    "files": [
///    {
///        "id": "6r65pjq33banznomn7q22sj44e",
///        "name": "testfile.txt",
///        "size": 35,
///        "content_path": "v1/vaults/ftz4pm2xxwmwrsd7rjqn7grzfz/items/2fcbqwe9ndg175zg2dzwftvkpa/files/6r65pjq33banznomn7q22sj44e/content",
///    },
///    {
///        "id": "oyez5gf6xjfptlhc3o4n6o6hvm",
///        "name": "samplefile.png",
///        "size": 296639,
///        "content_path": "v1/vaults/ftz4pm2xxwmwrsd7rjqn7grzfz/items/2fcbqwe9ndg175zg2dzwftvkpa/files/oyez5gf6xjfptlhc3o4n6o6hvm/content",
///    }
///    ],
///    "createdAt": "2021-04-10T17:20:05.98944527Z",
///    "updatedAt": "2021-04-13T17:20:05.989445411Z"
///}
/// ```
///
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    /// The UUID of the item.
    pub id: String,
    /// The title of the item.
    pub title: String,
    /// An object containing an id property whose value is the UUID of the vault the item is in.
    pub vault: Vault,
    /// The category of the item.
    pub category: Category,
    /// Array of [ItemUrl] objects containing URLs for the item
    pub urls: Option<Vec<Url>>,
    /// Whether the item is marked as a favorite.
    pub favorite: Option<bool>,
    /// An array of strings of the tags assigned to the item.
    pub tags: Option<Vec<String>>,
    /// The version of the item.
    pub version: u32,
    /// Date and time when the item was created.
    pub created_at: DateTime<Utc>,
    /// Date and time when the vault or its contents were last changed.
    pub updated_at: DateTime<Utc>,
    /// UUID of the account that last changed the item.
    pub last_edited_by: String,
    /// An object containing the UUID of a section in the item.
    pub additional_information: Option<String>,
    pub fields: Option<Vec<Field>>,
    pub files: Option<Vec<File>>,
    pub sections: Option<Vec<Section>>,
}

impl Item {
    pub fn get_field_by_id(&self, id: &str) -> Option<FieldValue> {
        let Some(fields) = &self.fields else {
            return None;
        };

        if let Some(field) = fields.iter().filter(|field| field.id == id).last() {
            return field.value.clone();
        }
        None
    }
}

impl Item {
    pub fn get_field_by_label(&self, label: &str) -> Option<FieldValue> {
        let Some(fields) = &self.fields else {
            return None;
        };

        if let Some(field) = fields.iter().filter(|field| field.label == label).last() {
            return field.value.clone();
        }
        None
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Item");
        dbg.field("id", &self.id);
        dbg.field("title", &self.title);
        dbg.field("vault", &self.vault);
        dbg.field("category", &self.category);
        if self.urls.is_some() {
            dbg.field("urls", &self.urls);
        }
        if self.favorite.is_some() {
            dbg.field("favorite", &self.favorite);
        }

        if self.tags.is_some() {
            dbg.field("tags", &self.tags);
        }
        dbg.field("version", &self.version);
        dbg.field("created_at", &self.created_at);
        dbg.field("updated_at", &self.updated_at);
        dbg.field("last_edited_by", &self.last_edited_by);

        if self.additional_information.is_some() {
            dbg.field("additional_information", &self.additional_information);
        }
        if self.fields.is_some() {
            dbg.field("fields", &self.fields);
        }
        if self.files.is_some() {
            dbg.field("files", &self.files);
        }
        if self.sections.is_some() {
            dbg.field("sections", &self.sections);
        }

        dbg.finish()
    }
}
