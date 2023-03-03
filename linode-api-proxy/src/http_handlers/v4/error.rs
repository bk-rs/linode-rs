use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use linode_api::objects::{error::Reason, Error, ErrorResponseBody};

//
#[derive(Debug)]
pub enum HandleError {
    AuthenticationRequired,
    ReqQueryMissing,
    DeReqQueryFailed(serde_qs::Error),
    Other(StatusCode, Reason, Option<String>),
}

//
impl IntoResponse for HandleError {
    fn into_response(self) -> Response {
        let (status_code, reason, field) = match self {
            HandleError::AuthenticationRequired => {
                (StatusCode::UNAUTHORIZED, Reason::InvalidToken, None)
            }
            HandleError::ReqQueryMissing => {
                (StatusCode::BAD_REQUEST, Reason::Other("".into()), None)
            }
            HandleError::DeReqQueryFailed(err) => (
                StatusCode::BAD_REQUEST,
                Reason::Other(format!("de request query failed, err:{err}")),
                None,
            ),
            HandleError::Other(status_code, reason, field) => (status_code, reason, field),
        };

        let body = Json(ErrorResponseBody {
            errors: vec![Error { field, reason }],
        });
        (status_code, body).into_response()
    }
}
