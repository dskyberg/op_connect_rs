use serde::{Deserialize, Serialize};

/// Item Category
/// You can't create items using the "CUSTOM" or "DOCUMENT" categories.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category {
    Login,
    Password,
    ApiCredential,
    Server,
    Database,
    CreditCard,
    Membership,
    Passport,
    SoftwareLicense,
    OutdoorLicense,
    SecureNote,
    WirelessRouter,
    BankAccount,
    DriverLicense,
    Identity,
    RewardProgram,
    Document,
    EmailAccount,
    SocialSecurityNumber,
    MedicalRecord,
    SshKey,
    Custom,
}
