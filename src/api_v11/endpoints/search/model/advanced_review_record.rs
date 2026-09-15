use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::advanced_common_record::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedReviewRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    tags: Vec<String>,

    #[serde(rename = "_bWasFeatured")]
    was_featured: bool,

    #[serde(rename = "_nViewCount")]
    view_count: u64,
}
