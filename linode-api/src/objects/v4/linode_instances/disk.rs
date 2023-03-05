use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_json::{Map, Value};

//
wrapping_macro::wrapping_int! {
    #[derive(Deserialize, Serialize, Debug, Clone, Copy)]
    pub struct DiskId(pub u64);
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Disk {
    pub id: DiskId,
    pub label: String,
    pub status: DiskStatus,
    #[serde(flatten)]
    pub _extra: Map<String, Value>,
}

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiskStatus {
    Ready,
    #[serde(rename = "not ready")]
    NotReady,
    Deleting,
    #[serde(other)]
    Other(String),
}
