use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorResponseBody {
    pub errors: Vec<Error>,
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub field: Option<String>,
    pub reason: Reason,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_error_response_body() {
        match serde_json::from_str::<ErrorResponseBody>(include_str!(
            "../../tests/response_body_files/error__with_invalid_token.json"
        )) {
            Ok(json) => {
                assert_eq!(json.errors.len(), 1);
                assert!(json.errors[0].field.is_none());
                assert_eq!(json.errors[0].reason, Reason::InvalidToken);
            }
            x => panic!("{x:?}"),
        }

        match serde_json::from_str::<ErrorResponseBody>(include_str!(
            "../../tests/response_body_files/error__with_not_found.json"
        )) {
            Ok(json) => {
                assert_eq!(json.errors.len(), 1);
                assert!(json.errors[0].field.is_none());
                assert_eq!(json.errors[0].reason, Reason::NotFound);
            }
            x => panic!("{x:?}"),
        }

        match serde_json::from_str::<ErrorResponseBody>(include_str!(
            "../../tests/response_body_files/error__with_page_size_invalid.json"
        )) {
            Ok(json) => {
                assert_eq!(json.errors.len(), 1);
                assert_eq!(json.errors[0].field, Some("page_size".into()));
                assert_eq!(
                    json.errors[0].reason,
                    Reason::Other("Must be 25-500".into())
                );
            }
            x => panic!("{x:?}"),
        }
    }
}
