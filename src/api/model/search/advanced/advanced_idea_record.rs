use serde::{Deserialize, Serialize};

use crate::api::model::search::{
    advanced::advanced_common_record::AdvancedCommonRecord,
    advanced::ratings_summary::RatingsSummary,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedIdeaRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_sResolution")]
    pub resolution: String,

    #[serde(rename = "_akResolution")]
    pub ak_resolution: String,

    #[serde(rename = "_aRatingsSummary")]
    pub ratings_summary: RatingsSummary,

    #[serde(rename = "_sResolutionMessage")]
    pub resolution_message: String,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nPostCount")]
    pub post_count: u64,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: u64,
}
