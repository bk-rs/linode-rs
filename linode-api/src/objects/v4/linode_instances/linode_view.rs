use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LinodesViewResponseBody {
    pub id: u64,
    pub label: String,
    #[serde(flatten)]
    pub _extra: Map<String, Value>,
}
