/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p linode-api-demo --bin linode_api_demo_v4_linode_instances_linodes_list -- 'YOUR_ACCESS_TOKEN'
*/

use std::env;

use futures_lite::future::block_on;
use http_api_isahc_client::IsahcClient;
use linode_api::V4Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = env::args().nth(1).ok_or("access_token missing")?;

    let client = V4Client::new(IsahcClient::new()?, Some(access_token.into()), None);

    //
    let resp_body = client.linode_instances_linodes_list(None, 100).await?;
    println!("{resp_body:?}");

    Ok(())
}
