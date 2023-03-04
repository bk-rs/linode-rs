use serde::{Deserialize, Serialize};

use crate::objects::v4::linode_instances::Linode;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LinodesListResponseBody {
    pub data: Vec<LinodesListResponseBodyDataItem>,
    pub page: usize,
    pub pages: usize,
    pub results: usize,
}

wrapping_macro::wrapping! {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct LinodesListResponseBodyDataItem(pub Linode);
}
