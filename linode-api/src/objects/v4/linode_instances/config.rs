use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

//
wrapping_macro::wrapping_int! {
    #[derive(Deserialize, Serialize, Debug, Clone, Copy)]
    pub struct ConfigId(pub u64);
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub id: ConfigId,
    pub label: String,
    pub kernel: String,
    #[serde(flatten)]
    pub _extra: Map<String, Value>,
}
