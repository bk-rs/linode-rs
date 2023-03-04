use core::time::Duration;

use crate::Args;

//
wrapping_macro::wrapping! {
    #[derive(Debug, Clone)]
    pub struct LinodeApiHttpClient(pub reqwest::Client);
}

//
pub struct Context {
    pub args: Args,
    pub linode_api_http_client: LinodeApiHttpClient,
}

impl Context {
    pub async fn new(args: &Args) -> Result<Self, Box<dyn std::error::Error>> {
        let linode_api_http_client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .connection_verbose(args.verbose)
            .timeout(Duration::from_secs(30))
            .build()?
            .into();

        Ok(Self {
            args: args.to_owned(),
            linode_api_http_client,
        })
    }
}
