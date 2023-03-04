use serde::{Deserialize, Serialize};

use crate::objects::v4::error::Error;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorResponseBody {
    pub errors: Vec<Error>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::v4::error::ErrorReason;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<ErrorResponseBody>(include_str!(
            "../../../tests/response_body_files/error/error__with_invalid_json.json"
        )) {
            Ok(json) => {
                assert_eq!(json.errors.len(), 1);
                assert!(json.errors[0].field.is_none());
                assert_eq!(json.errors[0].reason, ErrorReason::InvalidJson);
            }
            x => panic!("{x:?}"),
        }

        match serde_json::from_str::<ErrorResponseBody>(include_str!(
            "../../../tests/response_body_files/error/error__with_invalid_token.json"
        )) {
            Ok(json) => {
                assert_eq!(json.errors.len(), 1);
                assert!(json.errors[0].field.is_none());
                assert_eq!(json.errors[0].reason, ErrorReason::InvalidToken);
            }
            x => panic!("{x:?}"),
        }

        match serde_json::from_str::<ErrorResponseBody>(include_str!(
            "../../../tests/response_body_files/error/error__with_not_found.json"
        )) {
            Ok(json) => {
                assert_eq!(json.errors.len(), 1);
                assert!(json.errors[0].field.is_none());
                assert_eq!(json.errors[0].reason, ErrorReason::NotFound);
            }
            x => panic!("{x:?}"),
        }

        match serde_json::from_str::<ErrorResponseBody>(include_str!(
            "../../../tests/response_body_files/error/error__with_page_size_invalid.json"
        )) {
            Ok(json) => {
                assert_eq!(json.errors.len(), 1);
                assert_eq!(json.errors[0].field, Some("page_size".into()));
                assert_eq!(
                    json.errors[0].reason,
                    ErrorReason::Other("Must be 25-500".into())
                );
            }
            x => panic!("{x:?}"),
        }
    }
}
