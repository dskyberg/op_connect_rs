use serde::{Deserialize, Serialize};

/// URL Object
///
/// ```json
/// {
///    "label": "website",
///    "primary": true,
///   "href": "https://example.com"
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Url {
    /// The label for the URL.
    pub label: String,
    /// Whether this is the primary URL for the item.
    pub primary: bool,
    /// The address.
    pub href: String,
}
