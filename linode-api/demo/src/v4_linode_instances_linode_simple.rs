/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p linode-api-demo --bin linode_api_demo_v4_linode_instances_linode_simple -- 'YOUR_ACCESS_TOKEN'
*/

use core::time::Duration;
use std::env;

use async_io::Timer;
use futures_lite::future::block_on;
use http_api_isahc_client::IsahcClient;
use linode_api::{
    endpoints::v4::linode_instances::config_create::RequestBodyDevicesItem,
    objects::v4::linode_instances::{DiskStatus, LinodeStatus},
    V4Client, V4ClientRespondError,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = env::args().nth(1).ok_or("access_token missing")?;

    let client = V4Client::new(IsahcClient::new()?, Some(access_token.into()), None);

    //
    let linode_label = {
        use rand::{distributions::Alphanumeric, thread_rng, Rng as _};

        let suffix = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect::<String>();

        format!("test_linode_api_{suffix}")
    };

    //
    let resp_body = client
        .linode_instances_linode_create("us-west", "g6-nanode-1", |req_body| {
            req_body.label = Some(linode_label.to_owned());
        })
        .await?;
    println!("linode_instances_linode_create: {resp_body:?}");

    let linode_id = resp_body.id;

    loop {
        Timer::after(Duration::from_secs(5)).await;

        let resp_body = client.linode_instances_linode_view(linode_id).await?;
        println!("linode_instances_linode_view: {resp_body:?}");
        if resp_body.status != LinodeStatus::Provisioning {
            break;
        }
    }

    //
    let root_pass = {
        use rand::{distributions::Alphanumeric, thread_rng, Rng as _};

        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>()
    };

    let resp_body = client
        .linode_instances_disk_create_with_image(
            linode_id,
            10240,
            "linode/ubuntu22.04",
            &root_pass,
            |_req_body| {
                // req_body.authorized_keys = Some(vec!["xxx".into()]);
            },
        )
        .await?;
    println!("linode_instances_disk_create_with_image: {resp_body:?}");

    let disk_id = resp_body.id;

    loop {
        Timer::after(Duration::from_secs(5)).await;

        match client.linode_instances_disk_view(linode_id, disk_id).await {
            Ok(resp_body) => {
                println!("linode_instances_disk_view: {resp_body:?}");
                if resp_body.status == DiskStatus::Ready {
                    break;
                }
            }
            Err(V4ClientRespondError::ResponseStatusCodeNoSuccess(status, _, _))
                if status.as_u16() == 404 =>
            {
                println!("linode_instances_disk_view: 404");
                continue;
            }
            Err(err) => panic!("{err}"),
        }
    }

    //
    let resp_body = client
        .linode_instances_config_create(
            linode_id,
            format!("{linode_label}_grub2").as_str(),
            |req_body| {
                req_body.devices.sda = Some(RequestBodyDevicesItem::with_disk_id(disk_id));
                req_body.kernel = Some("linode/grub2".into());
            },
        )
        .await?;
    println!("linode_instances_config_create: {resp_body:?}");

    let config_id = resp_body.id;

    //
    client
        .linode_instances_linode_boot(linode_id, config_id)
        .await?;

    loop {
        Timer::after(Duration::from_secs(10)).await;

        let resp_body = client.linode_instances_linode_view(linode_id).await?;
        println!("linode_instances_linode_view: {resp_body:?}");
        if resp_body.status != LinodeStatus::Booting {
            break;
        }
    }

    //
    client
        .linode_instances_linode_reboot(linode_id, None)
        .await?;

    loop {
        Timer::after(Duration::from_secs(10)).await;

        let resp_body = client.linode_instances_linode_view(linode_id).await?;
        println!("linode_instances_linode_view: {resp_body:?}");
        if resp_body.status == LinodeStatus::Running {
            break;
        }
    }

    //
    client.linode_instances_linode_delete(linode_id).await?;

    Ok(())
}
