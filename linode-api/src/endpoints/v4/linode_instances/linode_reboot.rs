use serde::{Deserialize, Serialize};

use crate::objects::v4::linode_instances::ConfigId;

//
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<ConfigId>,
}
