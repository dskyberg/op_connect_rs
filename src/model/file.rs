use serde::{Deserialize, Serialize};

use super::Section;

/// Field Object
///
/// ```json
/// {
///    "id": "6r65pjq33banznomn7q22sj44e",
///    "name": "testfile.txt",
///    "size": 35,
///    "content_path": "v1/vaults/ftz4pm2xxwmwrsd7rjqn7grzfz/items/2fcbqwe9ndg175zg2dzwftvkpa/files/6r65pjq33banznomn7q22sj44e/content",
///    "content": "VGhlIGZ1dHVyZSBiZWxvbmdzIHRvIHRoZSBjdXJpb3VzLgo=",
///    "section": {
///        "id": "95cdbc3b-7742-47ec-9056-44d6af82d562"
///    },
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    /// The UUID of the file.
    pub id: String,
    /// The name of the file.
    pub name: String,
    /// The size of the file in bytes.
    pub size: u64,
    /// The path to download the contents of the file.
    pub content_path: String,
    /// The Base64-encoded contents of the file, if inline_files is set to true.
    pub content: String,
    /// An object containing the UUID of a section in the item.
    pub section: Option<Section>,
}
