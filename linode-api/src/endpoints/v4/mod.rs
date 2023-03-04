use http::Uri;
use once_cell::sync::Lazy;
use url::Url;

//
pub static BASE_URL: Lazy<Url> =
    Lazy::new(|| "https://api.linode.com/v4".parse::<Url>().expect("Never"));

pub static BASE_URI: Lazy<Uri> =
    Lazy::new(|| "https://api.linode.com/v4".parse::<Uri>().expect("Never"));

//
pub mod linode_instances;
pub mod linode_types;

pub mod error;
pub use error::*;
