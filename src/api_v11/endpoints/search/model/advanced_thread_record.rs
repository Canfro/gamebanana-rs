use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_common_record::AdvancedCommonRecord, category::Category, game::Game,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedThreadRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    tags: Vec<String>,

    #[serde(rename = "_aGame")]
    game: Game,

    #[serde(rename = "_aRootCategory")]
    root_category: Category,

    #[serde(rename = "_bIsStuck")]
    is_stuck: bool,

    #[serde(rename = "_bIsObsolete")]
    is_obsolete: bool,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    was_featured: bool,

    #[serde(rename = "_nViewCount")]
    view_count: u64,
}
