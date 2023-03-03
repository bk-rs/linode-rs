use core::{future::Future, pin::Pin};

use axum::{
    body::Body,
    handler::Handler,
    http::{
        uri::{Builder as UriBuilder, Parts as UriParts},
        Request, StatusCode,
    },
    response::{IntoResponse as _, Json, Response},
};
use linode_api::types::Version;

use crate::context::LinodeApiHttpClient;

//
#[derive(Debug, Clone)]
pub struct FallbackHandler {
    pub linode_api_http_client: LinodeApiHttpClient,
    pub version: Version,
}

impl<T, S> Handler<T, S, Body> for FallbackHandler {
    type Future = Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

    fn call(self, mut req: Request<Body>, _state: S) -> Self::Future {
        let req_uri_path = req.uri().path();
        let req_uri_query = req.uri().query();

        //
        let base_uri = match self.version {
            Version::V4 => {
                use linode_api::endpoints::v4::BASE_URI;

                BASE_URI.to_owned()
            }
        };

        let req_uri = {
            let UriParts {
                scheme,
                authority,
                path_and_query,
                ..
            } = base_uri.into_parts();

            let mut path_and_query_str = path_and_query
                .map(|x| x.path().to_owned())
                .unwrap_or_default();

            if !req_uri_path.starts_with('/') {
                path_and_query_str.push('/');
            }
            path_and_query_str.push_str(req_uri_path);

            if let Some(req_uri_query) = req_uri_query {
                path_and_query_str.push('?');
                path_and_query_str.push_str(req_uri_query)
            }

            let uri_builder = UriBuilder::new();
            let uri_builder = if let Some(scheme) = scheme {
                uri_builder.scheme(scheme)
            } else {
                uri_builder
            };
            let uri_builder = if let Some(authority) = authority {
                uri_builder.authority(authority)
            } else {
                uri_builder
            };
            let uri_builder = uri_builder.path_and_query(path_and_query_str);

            match uri_builder.build() {
                Ok(x) => x,
                Err(err) => {
                    return Box::pin(async move {
                        let mut resp = match self.version {
                            Version::V4 => {
                                use linode_api::objects::{
                                    error::Reason, Error, ErrorResponseBody,
                                };

                                Json(ErrorResponseBody {
                                    errors: vec![Error {
                                        field: None,
                                        reason: Reason::Other(format!(
                                            "change request uri failed, err:{err}"
                                        )),
                                    }],
                                })
                                .into_response()
                            }
                        };

                        *resp.status_mut() = StatusCode::from_u16(599).expect("Never");
                        resp
                    })
                }
            }
        };

        *req.uri_mut() = req_uri;

        Box::pin(async move {
            match internal::reqwest_execute(&self.linode_api_http_client, req).await {
                Ok(resp) => resp,
                Err(err) => match self.version {
                    Version::V4 => {
                        use linode_api::objects::{error::Reason, Error, ErrorResponseBody};

                        let mut resp = Json(ErrorResponseBody {
                            errors: vec![Error {
                                field: None,
                                reason: Reason::Other(format!("respond failed, err:{err}")),
                            }],
                        })
                        .into_response();

                        *resp.status_mut() = StatusCode::from_u16(599).expect("Never");
                        resp
                    }
                },
            }
        })
    }
}

//
pub mod internal {
    use axum::{
        body::{Body as HyperBody, StreamBody as AxumStreamBody},
        http::{response::Parts as HttpResponseParts, Request as HttpRequest},
        response::Response,
    };
    use reqwest::Request as ReqwestRequest;

    use crate::context::LinodeApiHttpClient;

    //
    pub async fn reqwest_execute(
        client: &LinodeApiHttpClient,
        http_req: HttpRequest<HyperBody>,
    ) -> Result<Response, reqwest::Error> {
        let reqwest_req = ReqwestRequest::try_from(http_req)?;
        let reqwest_resp = client.execute(reqwest_req).await?;
        let http_resp = {
            let (mut parts, _) = Response::new(()).into_parts();

            let HttpResponseParts {
                status,
                version,
                headers,
                extensions: _,
                ..
            } = &mut parts;
            *status = reqwest_resp.status();
            *version = reqwest_resp.version();
            *headers = reqwest_resp.headers().to_owned();

            let body = AxumStreamBody::new(reqwest_resp.bytes_stream());

            Response::from_parts(parts, axum::body::boxed(body))
        };
        Ok(http_resp)
    }
}
