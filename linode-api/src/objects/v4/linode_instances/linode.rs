use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_json::{Map, Value};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Linode {
    pub id: u64,
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
