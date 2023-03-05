use serde::{Deserialize, Serialize};

use crate::objects::v4::linode_types::Type;

//
wrapping_macro::wrapping! {
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct ResponseBody(pub Type);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::v4::linode_types::TypeClass;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<ResponseBody>(include_str!(
            "../../../../tests/response_body_files/linode_types/type_view.json"
        )) {
            Ok(json) => {
                assert_eq!(json.class, TypeClass::Standard);
            }
            x => panic!("{x:?}"),
        }
    }
}
