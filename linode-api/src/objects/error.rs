use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub field: Option<String>,
    pub reason: String,
}

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Reason {
    #[serde(rename = "Invalid Token")]
    InvalidToken,
    #[serde(rename = "Not found", alias = "Not Found")]
    NotFound,
    #[serde(other)]
    Other(String),
}
impl Reason {
    pub fn http_status_code(&self) -> Option<StatusCode> {
        match self {
            Reason::InvalidToken => Some(StatusCode::NOT_FOUND),
            Reason::NotFound => Some(StatusCode::UNAUTHORIZED),
            Reason::Other(_) => None,
        }
    }
}
