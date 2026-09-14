use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_record_common::AdvancedCommonRecord, game::Game, root_category::RootCategory,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedBlogRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    tags: Vec<String>,

    #[serde(rename = "_aGame")]
    game: Game,

    #[serde(rename = "_aRootCategory")]
    root_category: RootCategory,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    was_featured: bool,

    #[serde(rename = "_nViewCount")]
    view_count: u64,
}
