use serde::{Deserialize, Serialize};

use crate::api_v11::model::search::advanced::advanced_common_record::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedBugRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_akResolution")]
    pub ak_resolution: String,

    #[serde(rename = "_sResolution")]
    pub resolution: String,

    #[serde(rename = "_akPriority")]
    pub ak_priority: String,

    #[serde(rename = "_sPriority")]
    pub priority: String,

    #[serde(rename = "_sSourceUrl")]
    pub source_url: String,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,
}
