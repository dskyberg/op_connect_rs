use serde::{Deserialize, Serialize};

use super::CharacterSet;

/// Generator Recipe Object
/// The recipe is used in conjunction with the "generate" property to set
/// the character set used to generate a new secure value
/// ```json
/// {
///    "length": 55,
///    "characterSets": [
///        "LETTERS",
///        "DIGITS"
///    ]
///}
/// ```
///
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorRecipe {
    /// An array containing of the kinds of characters to include
    #[serde(default = "default_character_sets")]
    pub character_sets: Vec<CharacterSet>,
    /// The length of the password to generate.  Defaults to 32
    #[serde(default = "default_length")]
    pub length: u32,
    /// A list of all characters that should be excluded from generated passwords.
    pub exclude_characteres: Option<String>,
}

fn default_length() -> u32 {
    32
}

fn default_character_sets() -> Vec<CharacterSet> {
    vec![
        CharacterSet::Letters,
        CharacterSet::Digits,
        CharacterSet::Symbols,
    ]
}
