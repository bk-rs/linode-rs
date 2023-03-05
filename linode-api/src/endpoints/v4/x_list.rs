use serde::{Deserialize, Serialize};

//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub struct XListRequestQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<usize>,
}
impl XListRequestQuery {
    pub fn new(page: impl Into<Option<usize>>, page_size: impl Into<Option<usize>>) -> Self {
        Self {
            page: page.into(),
            page_size: page_size.into(),
        }
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct XListResponseBody<T> {
    pub data: Vec<T>,
    pub page: usize,
    pub pages: usize,
    pub results: usize,
}
