/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p linode-api-demo --bin linode_api_demo_v4_linode_types_types_list --
*/

use futures_lite::future::block_on;
use http_api_isahc_client::IsahcClient;
use linode_api::V4Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = V4Client::new(IsahcClient::new()?, None, None);

    //
    let resp_body = client.linode_types_types_list().await?;
    println!("{resp_body:?}");

    Ok(())
}
