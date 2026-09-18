use serde::{Deserialize, Serialize};

use crate::api::model::search::{
    advanced::advanced_common_record::AdvancedCommonRecord, advanced::category::Category,
    advanced::finished_works::FinishedWorks, game::Game,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedWipRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_akDevelopmentState")]
    pub ak_development_state: String,

    #[serde(rename = "_sDevelopmentState")]
    pub development_state: String,

    #[serde(rename = "_iCompletionPercentage")]
    pub completion_percentage: i64,

    #[serde(rename = "_aFinishedWork")]
    pub finished_work: FinishedWorks,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}
