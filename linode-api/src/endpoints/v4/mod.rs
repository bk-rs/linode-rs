use http_api_client::http::Uri;
use once_cell::sync::Lazy;
use url::Url;

//
pub static BASE_URL: Lazy<Url> =
    Lazy::new(|| "https://api.linode.com/v4/".parse::<Url>().expect("Never"));

pub static BASE_URI: Lazy<Uri> =
    Lazy::new(|| "https://api.linode.com/v4/".parse::<Uri>().expect("Never"));

//
pub mod linode_instances;
pub mod linode_types;

pub mod error;
pub use error::ErrorResponseBody;

pub mod x_list;
pub use x_list::{XListRequestQuery, XListResponseBody};

//
pub mod common;
pub use common::{parse_response, render_request, ParseResponseError, RenderRequestError};

//
wrapping_macro::wrapping_string! {
    #[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
    pub struct AccessToken(pub String);
}

//
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct EmptyMapResponseBody(pub serde_json::Map<String, serde_json::Value>);
