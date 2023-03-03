use std::sync::Arc;

use axum::{routing::get, Router};

use crate::Context;

//
pub mod error;
pub use error::HandleError;

pub mod linode_instances;

//
pub fn router(ctx: Arc<Context>) -> Router {
    Router::new()
        .route(
            "/linode/instances/show_by_label",
            get(linode_instances::linode_show_by_label_handler::handle),
        )
        .with_state(ctx)
}
