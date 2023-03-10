use std::sync::Arc;

use axum::{routing::get, Router};
use linode_api::types::Version;

use crate::{http_handlers::fallback_handler::FallbackHandler, Context};

//
pub mod error;
pub use error::HandleError;

pub mod linode_instances;

//
pub fn router(ctx: Arc<Context>) -> Router {
    Router::new()
        .route(
            "/linode/instances/view_by_label",
            get(linode_instances::linode_view_by_label_handler::handle),
        )
        .fallback::<_, ()>(FallbackHandler::new(
            ctx.linode_api_http_client.clone(),
            Version::V4,
        ))
        .with_state(ctx)
}
