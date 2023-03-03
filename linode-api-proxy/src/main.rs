use linode_api_proxy::*;

use std::sync::Arc;

use axum::Router;

//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    pretty_env_logger::init();

    //
    let args = Args::parse();

    //
    let ctx = Context::new(&args).await?;
    let ctx = Arc::new(ctx);

    //
    let app = Router::new().nest("/v4", http_handlers::v4::router(ctx));

    axum::Server::bind(&args.http_listen_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
