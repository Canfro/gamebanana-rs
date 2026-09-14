use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::advanced_record_common::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedBugRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_akResolution")]
    ak_resolution: String,

    #[serde(rename = "_sResolution")]
    resolution: String,

    #[serde(rename = "_akPriority")]
    ak_priority: String,

    #[serde(rename = "_sPriority")]
    priority: String,

    #[serde(rename = "_sSourceUrl")]
    source_url: String,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,
}
