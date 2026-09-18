use serde::{Deserialize, Serialize};

use crate::api::model::search::{
    advanced::advanced_common_record::AdvancedCommonRecord, advanced::category::Category,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedWikiRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: u64,
}
