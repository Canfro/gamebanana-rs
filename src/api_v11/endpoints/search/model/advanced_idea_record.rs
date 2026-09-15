use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_common_record::AdvancedCommonRecord, ratings_summary::RatingsSummary,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedIdeaRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    tags: Vec<String>,

    #[serde(rename = "_sResolution")]
    resolution: String,

    #[serde(rename = "_akResolution")]
    ak_resolution: String,

    #[serde(rename = "_aRatingsSummary")]
    ratings_summary: RatingsSummary,

    #[serde(rename = "_sResolutionMessage")]
    resolution_message: String,

    #[serde(rename = "_bIsObsolete")]
    is_obsolete: bool,

    #[serde(rename = "_nPostCount")]
    post_count: u64,

    #[serde(rename = "_bWasFeatured")]
    was_featured: bool,

    #[serde(rename = "_nViewCount")]
    view_count: u64,
}
