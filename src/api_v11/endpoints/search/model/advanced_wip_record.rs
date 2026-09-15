use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_common_record::AdvancedCommonRecord, category::Category,
    finished_works::FinishedWorks, game::Game,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedWipRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    tags: Vec<String>,

    #[serde(rename = "_aGame")]
    game: Game,

    #[serde(rename = "_aRootCategory")]
    root_category: Category,

    #[serde(rename = "_akDevelopmentState")]
    ak_development_state: String,

    #[serde(rename = "_sDevelopmentState")]
    development_state: String,

    #[serde(rename = "_iCompletionPercentage")]
    completion_percentage: i64,

    #[serde(rename = "_aFinishedWork")]
    finished_work: FinishedWorks,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    was_featured: bool,

    #[serde(rename = "_nViewCount")]
    view_count: u64,
}
