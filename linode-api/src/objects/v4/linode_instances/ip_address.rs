use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_json::{Map, Value};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IpAddress {
    pub address: IpAddr,
    pub public: bool,
    pub r#type: IpAddressType,
    #[serde(flatten)]
    pub _extra: Map<String, Value>,
}

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpAddressType {
    Ipv4,
    Ipv6,
    #[serde(rename = "ipv6/pool")]
    Ipv6Pool,
    #[serde(rename = "ipv6/range")]
    Ipv6Range,
    #[serde(other)]
    Other(String),
}
