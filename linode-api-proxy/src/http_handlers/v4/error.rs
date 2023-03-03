use axum::{
    extract::Json,
    http::{
        header::{HeaderName, HeaderValue},
        StatusCode,
    },
    response::{IntoResponse, Response},
};
use linode_api::objects::v4::error::{Error, ErrorResponseBody, Reason};

//
#[derive(Debug)]
pub enum HandleError {
    AuthenticationRequired,
    ReqQueryMissing,
    ReqQueryDeFailed(serde_qs::Error),
    ReqUriBuildFailed(axum::http::uri::InvalidUri),
    BackendResponseStatusCodeMismatch(Response),
    BackendResponseBodyReadFailed(axum::Error),
    BackendResponseBodyDeFailed(serde_json::Error),
    Other(
        StatusCode,
        Reason,
        Option<String>,
        Option<Vec<(HeaderName, HeaderValue)>>,
    ),
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
            HandleError::ReqQueryDeFailed(err) => (
                StatusCode::BAD_REQUEST,
                Reason::Other(format!("request query de failed, err:{err}")),
                None,
            ),
            HandleError::ReqUriBuildFailed(err) => (
                StatusCode::BAD_REQUEST,
                Reason::Other(format!("request uri build failed, err:{err}")),
                None,
            ),
            HandleError::BackendResponseStatusCodeMismatch(resp) => return resp,
            HandleError::BackendResponseBodyReadFailed(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Reason::Other(format!("backend response body read failed, err:{err}")),
                None,
            ),
            HandleError::BackendResponseBodyDeFailed(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Reason::Other(format!("backend response body de failed, err:{err}")),
                None,
            ),
            HandleError::Other(status_code, reason, field, backend_resp_headers) => {
                let body = Json(ErrorResponseBody {
                    errors: vec![Error { field, reason }],
                });
                let mut resp = (status_code, body).into_response();
                if let Some(backend_resp_headers) = backend_resp_headers {
                    for (k, v) in backend_resp_headers {
                        resp.headers_mut().insert(k, v);
                    }
                }
                return resp;
            }
        };

        let body = Json(ErrorResponseBody {
            errors: vec![Error { field, reason }],
        });
        (status_code, body).into_response()
    }
}
