use serde::{Deserialize, Serialize};

use crate::objects::v4::linode_instances::IpAddress;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseBody {
    pub ipv4: ResponseBodyIpv4,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseBodyIpv4 {
    pub private: Vec<IpAddress>,
    pub public: Vec<IpAddress>,
    pub reserved: Vec<IpAddress>,
    pub shared: Vec<IpAddress>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::v4::linode_instances::IpAddressType;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<ResponseBody>(include_str!(
            "../../../../tests/response_body_files/linode_instances/ip_addresses_info.json"
        )) {
            Ok(json) => {
                assert_eq!(json.ipv4.public[0].r#type, IpAddressType::Ipv4);
            }
            x => panic!("{x:?}"),
        }
    }
}
