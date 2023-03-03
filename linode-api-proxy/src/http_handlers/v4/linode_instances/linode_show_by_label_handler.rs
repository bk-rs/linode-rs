/*
Label must be unique among your linodes
*/

use std::sync::Arc;

use axum::{
    extract::{RawQuery, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{http_handlers::v4::HandleError, Context};

//
pub async fn handle(
    RawQuery(req_query_str): RawQuery,
    State(_ctx): State<Arc<Context>>,
) -> Result<Json<Map<String, Value>>, HandleError> {
    let req_query_str = req_query_str.ok_or(HandleError::ReqQueryMissing)?;
    let req_query: ReqQuery =
        serde_qs::from_str(&req_query_str).map_err(HandleError::DeReqQueryFailed)?;

    todo!()
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReqQuery {
    pub label: String,
}
