use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_json::{Map, Value};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Type {
    pub id: String,
    pub label: String,
    pub class: TypeClass,
    pub vcpus: usize,
    #[serde(flatten)]
    pub _extra: Map<String, Value>,
}

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TypeClass {
    Nanode,
    Standard,
    Dedicated,
    Gpu,
    Highmem,
    #[serde(other)]
    Other(String),
}
