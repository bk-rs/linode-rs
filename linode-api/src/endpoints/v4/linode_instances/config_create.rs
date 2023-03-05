use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::objects::v4::linode_instances::{Config, DiskId};

//
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RequestBody {
    pub devices: RequestBodyDevices,
    pub label: String,
    pub kernel: Option<String>,
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub _extra: Map<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RequestBodyDevices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sda: Option<RequestBodyDevicesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdb: Option<RequestBodyDevicesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdc: Option<RequestBodyDevicesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdd: Option<RequestBodyDevicesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sde: Option<RequestBodyDevicesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdf: Option<RequestBodyDevicesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdg: Option<RequestBodyDevicesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdh: Option<RequestBodyDevicesItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RequestBodyDevicesItem {
    pub disk_id: Option<DiskId>,
    pub volume_id: Option<u64>,
}
impl RequestBodyDevicesItem {
    pub fn with_disk_id(disk_id: DiskId) -> Self {
        Self {
            disk_id: Some(disk_id),
            volume_id: None,
        }
    }
}

//
wrapping_macro::wrapping! {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct ResponseBody(pub Config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<ResponseBody>(include_str!(
            "../../../../tests/response_body_files/linode_instances/config_create.json"
        )) {
            Ok(json) => {
                assert_eq!(json.kernel, "linode/latest-64bit");
            }
            x => panic!("{x:?}"),
        }
    }
}
