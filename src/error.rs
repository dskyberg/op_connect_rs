use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("OP_API_TOKEN was not set in the env")]
    TokenNotSet,
    #[error("Url parse error")]
    UrlParse,
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Not Found")]
    NotFound,
}
