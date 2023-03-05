use serde::{Deserialize, Serialize};

use crate::objects::v4::linode_instances::Disk;

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
            "../../../../tests/response_body_files/linode_instances/disk_view.json"
        )) {
            Ok(json) => {
                assert_eq!(json.status, DiskStatus::Ready);
            }
            x => panic!("{x:?}"),
        }
    }
}
