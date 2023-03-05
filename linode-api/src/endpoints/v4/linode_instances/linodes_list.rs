use serde::{Deserialize, Serialize};

use crate::{endpoints::v4::XListResponseBody, objects::v4::linode_instances::Linode};

//
wrapping_macro::wrapping! {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct ResponseBody(pub XListResponseBody<Linode>);
}
