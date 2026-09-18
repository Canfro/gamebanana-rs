use serde::{Deserialize, Serialize};

use crate::api::model::search::{
    advanced::advanced_common_record::AdvancedCommonRecord, advanced::category::Category,
    game::Game,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedToolRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Game,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_sOfficialDownloadUrl")]
    pub official_download_url: String,

    #[serde(rename = "_sHomepageUrl")]
    pub homepage_url: String,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}
