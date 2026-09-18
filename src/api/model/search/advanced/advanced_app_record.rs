use crate::api::model::search::advanced::{
    advanced_common_record::AdvancedCommonRecord, features::Features,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedAppRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_sIncludeVariableName")]
    pub include_variable_name: String,

    #[serde(rename = "_sVersion")]
    pub version: String,

    #[serde(rename = "_akState")]
    pub ak_state: String,

    #[serde(rename = "_sState")]
    pub state: String,

    #[serde(rename = "_sType")]
    pub ty: String,

    #[serde(rename = "_nUserCount")]
    pub user_count: u64,

    #[serde(rename = "_sbIsSafe")]
    pub is_safe: String,

    #[serde(rename = "_aFeatures")]
    pub features: Features,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}
