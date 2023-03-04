use serde::{Deserialize, Serialize};

use crate::objects::v4::linode_instances::Config;

//
wrapping_macro::wrapping! {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct ConfigCreateResponseBody(pub Config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<ConfigCreateResponseBody>(include_str!(
            "../../../../tests/response_body_files/linode_instances/config_create.json"
        )) {
            Ok(json) => {
                assert_eq!(json.kernel, "linode/latest-64bit");
            }
            x => panic!("{x:?}"),
        }
    }
}
