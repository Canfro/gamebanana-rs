use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCount {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_nItemCount")]
    pub item_count: u64,

    #[serde(rename = "_nCategoryCount")]
    pub category_count: u64,

    #[serde(rename = "_sUrl")]
    pub url: String,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_sIconUrl")]
    pub icon_url: Option<String>,
}
