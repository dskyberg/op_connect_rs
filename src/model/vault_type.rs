use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VaultType {
    /// The team Shared vault.
    Everyone,
    /// The Private vault for the Connect server.
    Personal,
    /// A vault created by a user.
    UserCreated,
}
