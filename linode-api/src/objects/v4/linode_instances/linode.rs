use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_json::{Map, Value};

//
wrapping_macro::wrapping_int! {
    #[derive(Deserialize, Serialize, Debug, Clone, Copy)]
    pub struct LinodeId(pub u64);
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Linode {
    pub id: LinodeId,
    pub label: String,
    pub status: LinodeStatus,
    #[serde(flatten)]
    pub _extra: Map<String, Value>,
}

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LinodeStatus {
    Running,
    Offline,
    Booting,
    Rebooting,
    ShuttingDown,
    Provisioning,
    Deleting,
    Migrating,
    Rebuilding,
    Cloning,
    Restoring,
    Stopped,
    #[serde(other)]
    Other(String),
}
