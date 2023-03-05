use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::objects::v4::linode_instances::Linode;

//
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RequestBody {
    pub region: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub _extra: Map<String, Value>,
}

//
wrapping_macro::wrapping! {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct ResponseBody(pub Linode);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::v4::linode_instances::LinodeStatus;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<ResponseBody>(include_str!(
            "../../../../tests/response_body_files/linode_instances/linode_create.json"
        )) {
            Ok(json) => {
                assert_eq!(json.status, LinodeStatus::Running);
            }
            x => panic!("{x:?}"),
        }
    }
}
