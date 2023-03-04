use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    pub reason: ErrorReason,
}

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorReason {
    #[serde(rename = "Invalid JSON")]
    InvalidJson,
    #[serde(rename = "Invalid Token")]
    InvalidToken,
    #[serde(rename = "Not found", alias = "Not Found")]
    NotFound,
    #[serde(other)]
    Other(String),
}
impl ErrorReason {
    pub fn http_status_code(&self) -> Option<StatusCode> {
        match self {
            ErrorReason::InvalidJson => Some(StatusCode::BAD_REQUEST),
            ErrorReason::InvalidToken => Some(StatusCode::NOT_FOUND),
            ErrorReason::NotFound => Some(StatusCode::UNAUTHORIZED),
            ErrorReason::Other(_) => None,
        }
    }
}
