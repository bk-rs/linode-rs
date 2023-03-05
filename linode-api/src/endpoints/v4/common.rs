use http_api_client::http::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    HeaderMap, Method, Request, Response, StatusCode,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use url::Url;

use crate::endpoints::v4::{AccessToken, ErrorResponseBody, BASE_URL};

//
type Body = Vec<u8>;

//
pub fn render_request<ReqQuery, ReqBody>(
    method: Method,
    path: &str,
    query: Option<&ReqQuery>,
    body: Option<&ReqBody>,
    access_token: Option<&AccessToken>,
    base_url: Option<&Url>,
) -> Result<Request<Body>, RenderRequestError>
where
    ReqQuery: Serialize,
    ReqBody: Serialize,
{
    let path = path.strip_prefix('/').unwrap_or(path);
    let path = path.strip_prefix("v4/").unwrap_or(path);
    let base_url = base_url.unwrap_or(&BASE_URL);

    let request = Request::builder().method(method);

    let mut url = base_url.join(path).map_err(RenderRequestError::UrlJoin)?;

    if let Some(query) = query {
        let query = serde_qs::to_string(query).map_err(RenderRequestError::QuerySer)?;
        url.set_query(Some(&query));
    }
    let request = request.uri(url.as_str());

    let request = if let Some(access_token) = access_token {
        request.header(AUTHORIZATION, format!("Bearer {}", access_token))
    } else {
        request
    };

    let request = request
        .header(ACCEPT, "application/json")
        .header(USER_AGENT, "linode-api");

    let request = if let Some(body) = body {
        let body = serde_json::to_vec(body).map_err(RenderRequestError::BodySer)?;
        request.header(CONTENT_TYPE, "application/json").body(body)
    } else {
        request.body(vec![])
    };
    let request = request.map_err(RenderRequestError::RequestBuild)?;

    Ok(request)
}

//
#[derive(Debug)]
pub enum RenderRequestError {
    UrlJoin(url::ParseError),
    QuerySer(serde_qs::Error),
    BodySer(serde_json::Error),
    RequestBuild(http::Error),
}
impl core::fmt::Display for RenderRequestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for RenderRequestError {}

//
pub fn parse_response<RespBody>(
    response: Response<Body>,
) -> Result<(StatusCode, Result<RespBody, ErrorResponseBody>, HeaderMap), ParseResponseError>
where
    RespBody: DeserializeOwned,
{
    let status = response.status();
    let headers = response.headers().to_owned();

    match status {
        StatusCode::NO_CONTENT => {
            let json =
                serde_json::from_slice::<RespBody>(&[]).map_err(ParseResponseError::BodyDe)?;
            Ok((status, Ok(json), headers))
        }
        status if status.is_success() => {
            let json = serde_json::from_slice::<RespBody>(response.body())
                .map_err(ParseResponseError::BodyDe)?;
            Ok((status, Ok(json), headers))
        }
        status => {
            let json = serde_json::from_slice::<ErrorResponseBody>(response.body())
                .map_err(ParseResponseError::BodyDe)?;
            Ok((status, Err(json), headers))
        }
    }
}

//
#[derive(Debug)]
pub enum ParseResponseError {
    BodyDe(serde_json::Error),
}
impl core::fmt::Display for ParseResponseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ParseResponseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request() {
        for (path, uri) in &[
            ("foo", "https://api.linode.com/v4/foo"),
            ("/foo", "https://api.linode.com/v4/foo"),
            ("foo/bar", "https://api.linode.com/v4/foo/bar"),
            ("/foo/bar", "https://api.linode.com/v4/foo/bar"),
            ("v4/foo", "https://api.linode.com/v4/foo"),
            ("/v4/foo", "https://api.linode.com/v4/foo"),
            ("v4/foo/bar", "https://api.linode.com/v4/foo/bar"),
            ("/v4/foo/bar", "https://api.linode.com/v4/foo/bar"),
        ] {
            match render_request::<(), ()>(Method::GET, path, None, None, None, None) {
                Ok(req) => {
                    assert_eq!(req.uri().to_string(), *uri);
                }
                Err(err) => panic!("{err}"),
            }
        }
    }
}
