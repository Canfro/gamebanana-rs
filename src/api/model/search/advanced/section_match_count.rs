use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionMatchCount {
    #[serde(rename = "_sModelName")]
    pub model_name: String,

    #[serde(rename = "_sPluralTitle")]
    pub plural_title: String,

    #[serde(rename = "_sDescription")]
    pub description: String,

    #[serde(rename = "_sIconClasses")]
    pub icon_classes: String,

    #[serde(rename = "_nMatchCount")]
    pub match_count: u64,
}
