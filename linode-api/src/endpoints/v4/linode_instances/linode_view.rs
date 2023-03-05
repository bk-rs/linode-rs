use serde::{Deserialize, Serialize};

use crate::objects::v4::linode_instances::Linode;

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
            "../../../../tests/response_body_files/linode_instances/linode_view.json"
        )) {
            Ok(json) => {
                assert_eq!(json.status, LinodeStatus::Running);
            }
            x => panic!("{x:?}"),
        }
    }
}
