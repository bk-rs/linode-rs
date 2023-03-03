use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LinodesListResponseBody {
    pub data: Vec<LinodesListResponseBodyDataItem>,
    pub page: usize,
    pub pages: usize,
    pub results: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LinodesListResponseBodyDataItem {
    pub id: u64,
    pub label: String,
    #[serde(flatten)]
    pub _extra: Map<String, Value>,
}
