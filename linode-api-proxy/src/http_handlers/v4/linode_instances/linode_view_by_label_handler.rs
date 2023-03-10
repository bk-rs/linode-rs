/*
Label must be unique among your linodes
*/

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{RawQuery, State},
    handler::Handler,
    http::{
        header::{AUTHORIZATION, CONTENT_LENGTH},
        HeaderMap, Method, Request, StatusCode,
    },
    response::{IntoResponse as _, Json, Response},
};
use linode_api::{
    endpoints::v4::linode_instances::linodes_list, objects::v4::ErrorReason, types::Version,
};
use serde::{Deserialize, Serialize};

use crate::{
    http_handlers::{v4::HandleError, FallbackHandler},
    Context,
};

//
pub async fn handle(
    RawQuery(req_query_str): RawQuery,
    headers: HeaderMap,
    State(ctx): State<Arc<Context>>,
) -> Result<Response, HandleError> {
    let req_header_authorization = headers
        .get(AUTHORIZATION)
        .ok_or(HandleError::RequestHeaderAuthorizationRequired)?;
    if !req_header_authorization
        .as_bytes()
        .to_ascii_lowercase()
        .starts_with(b"bearer ")
    {
        return Err(HandleError::RequestHeaderAuthorizationRequired);
    }

    let req_query_str = req_query_str.ok_or(HandleError::RequestQueryMissing)?;
    let req_query: ReqQuery =
        serde_qs::from_str(&req_query_str).map_err(HandleError::RequestQueryDeFailed)?;

    //
    let fallback_handler = FallbackHandler::new(ctx.linode_api_http_client.clone(), Version::V4);

    let mut page = 0;
    loop {
        page += 1;

        let backend_req_uri = format!("/linode/instances?page={page}");

        let mut backend_req = Request::new(Body::empty());
        *backend_req.method_mut() = Method::GET;
        *backend_req.uri_mut() = backend_req_uri
            .parse()
            .map_err(HandleError::BackendRequestUriBuildFailed)?;
        backend_req
            .headers_mut()
            .insert(AUTHORIZATION, req_header_authorization.into());
        let backend_resp =
            <FallbackHandler as Handler<(), _, _>>::call(fallback_handler.clone(), backend_req, ())
                .await;

        let backend_resp_status = backend_resp.status();
        if backend_resp_status != StatusCode::OK {
            return Err(HandleError::BackendResponseStatusCodeMismatch(backend_resp));
        }
        let backend_resp_headers = backend_resp
            .headers()
            .iter()
            .filter_map(|(k, v)| match k {
                &CONTENT_LENGTH => None,
                _ => Some((k.to_owned(), v.to_owned())),
            })
            .collect::<Vec<_>>();

        let backend_resp_body = backend_resp.into_body();
        let backend_resp_body_bytes = http_body_to_bytes::http_body_to_bytes_with_max_length(
            backend_resp_body,
            ctx.args.backend_max_response_body_size(),
        )
        .await
        .map_err(HandleError::BackendResponseBodyReadFailed)?;

        let backend_resp_body_json: linodes_list::ResponseBody =
            serde_json::from_slice(&backend_resp_body_bytes)
                .map_err(HandleError::BackendResponseBodyDeFailed)?;

        if backend_resp_body_json.data.is_empty() {
            return Err(HandleError::Other(
                StatusCode::NOT_FOUND,
                ErrorReason::NotFound,
                None,
                Some(backend_resp_headers),
            ));
        }

        if let Some(item) = backend_resp_body_json
            .data
            .iter()
            .find(|x| x.label == req_query.label)
            .cloned()
        {
            let body = Json(item);
            let mut resp = (backend_resp_status, body).into_response();
            for (k, v) in backend_resp_headers {
                resp.headers_mut().insert(k, v);
            }
            return Ok(resp);
        }
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReqQuery {
    pub label: String,
}
