use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_common_record::AdvancedCommonRecord, category::Category, game::Game,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedProjectRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Game,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_iCompletionPercentage")]
    pub completion_percentage: i64,

    #[serde(rename = "_sFinishedSubmissionUrl")]
    pub finished_submission_url: String,

    #[serde(rename = "_akDevelopmentState")]
    pub ak_development_state: String,

    #[serde(rename = "_sDevelopmentState")]
    pub development_state: String,

    #[serde(rename = "_nWipCount")]
    pub wip_count: u64,

    #[serde(rename = "_nViewCount")]
    pub view_count: u64,
}
