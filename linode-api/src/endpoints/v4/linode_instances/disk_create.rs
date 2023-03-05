use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::objects::v4::linode_instances::Disk;

//
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RequestBodyWithImage {
    pub size: usize,
    pub image: String,
    pub root_pass: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_keys: Option<Vec<String>>,
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub _extra: Map<String, Value>,
}

//
wrapping_macro::wrapping! {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct ResponseBody(pub Disk);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::v4::linode_instances::DiskStatus;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<ResponseBody>(include_str!(
            "../../../../tests/response_body_files/linode_instances/disk_create.json"
        )) {
            Ok(json) => {
                assert_eq!(json.status, DiskStatus::Ready);
            }
            x => panic!("{x:?}"),
        }
    }
}
