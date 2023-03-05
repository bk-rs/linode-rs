use serde::{Deserialize, Serialize};

use crate::objects::v4::linode_instances::ConfigId;

//
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RequestBody {
    pub config_id: ConfigId,
}
