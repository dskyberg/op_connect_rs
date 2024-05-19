use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FieldType {
    String,
    Email,
    Concealed,
    Url,
    Otp,       // format: otpauth://
    Date,      // format: YYYY-MM-DD
    MonthYear, // format: YYYYMM or YYYY/MM
    Menu,
}
