use serde::{Deserialize, Serialize};

use crate::objects::v4::linode_instances::Linode;

//
wrapping_macro::wrapping! {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct LinodeCreateResponseBody(pub Linode);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::v4::linode_instances::LinodeStatus;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<LinodeCreateResponseBody>(include_str!(
            "../../../../tests/response_body_files/linode_instances/linode_create.json"
        )) {
            Ok(json) => {
                assert_eq!(json.status, LinodeStatus::Running);
            }
            x => panic!("{x:?}"),
        }
    }
}
