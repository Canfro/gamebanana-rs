use serde::{Deserialize, Serialize};

use crate::api_v11::{
    endpoints::search::model::{
        advanced_common_record::AdvancedCommonRecord, category::Category, studio::Studio,
    },
    model::string_or_bool::StringOrBool,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedModelRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_bIsMapped")]
    pub is_mapped: bool,

    #[serde(rename = "_bIsTextured")]
    pub is_textured: bool,

    #[serde(rename = "_xIsAnimated")]
    pub is_animated: StringOrBool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_aStudio")]
    pub studio: Option<Studio>,

    #[serde(rename = "_nViewCount")]
    pub view_count: u64,
}
