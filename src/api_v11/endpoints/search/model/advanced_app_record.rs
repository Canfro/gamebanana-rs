use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_record_common::AdvancedCommonRecord, features::Features,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedAppRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    tags: Vec<String>,

    #[serde(rename = "_sIncludeVariableName")]
    include_variable_name: String,

    #[serde(rename = "_sVersion")]
    version: String,

    #[serde(rename = "_akState")]
    ak_state: String,

    #[serde(rename = "_sState")]
    state: String,

    #[serde(rename = "_sType")]
    ty: String,

    #[serde(rename = "_nUserCount")]
    user_count: u64,

    #[serde(rename = "_sbIsSafe")]
    is_safe: String,

    #[serde(rename = "_aFeatures")]
    features: Features,

    #[serde(rename = "_bWasFeatured")]
    was_featured: bool,

    #[serde(rename = "_nViewCount")]
    view_count: u64,
}
