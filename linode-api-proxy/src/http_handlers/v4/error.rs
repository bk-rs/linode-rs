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
    RequestHeaderAuthorizationRequired,
    RequestQueryMissing,
    RequestQueryDeFailed(serde_qs::Error),
    BackendRequestUriBuildFailed(axum::http::uri::InvalidUri),
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
            HandleError::RequestHeaderAuthorizationRequired => {
                (StatusCode::UNAUTHORIZED, Reason::InvalidToken, None)
            }
            HandleError::RequestQueryMissing => (
                StatusCode::BAD_REQUEST,
                Reason::Other("request query required".into()),
                None,
            ),
            HandleError::RequestQueryDeFailed(err) => (
                StatusCode::BAD_REQUEST,
                Reason::Other(format!("request query de failed, err:{err}")),
                None,
            ),
            HandleError::BackendRequestUriBuildFailed(err) => (
                StatusCode::from_u16(599).expect("Never"),
                Reason::Other(format!("backend request uri build failed, err:{err}")),
                None,
            ),
            HandleError::BackendResponseStatusCodeMismatch(resp) => return resp,
            HandleError::BackendResponseBodyReadFailed(err) => (
                StatusCode::from_u16(599).expect("Never"),
                Reason::Other(format!("backend response body read failed, err:{err}")),
                None,
            ),
            HandleError::BackendResponseBodyDeFailed(err) => (
                StatusCode::from_u16(599).expect("Never"),
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
