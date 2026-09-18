use serde::{Deserialize, Serialize};

use crate::api::model::search::{
    advanced::advanced_common_record::AdvancedCommonRecord, advanced::category::Category, advanced::game::Game,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedQuestionRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Game,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_bIsStuck")]
    pub is_stuck: bool,

    #[serde(rename = "_akState")]
    pub ak_state: String,

    #[serde(rename = "_sState")]
    pub state: String,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: u64,
}
