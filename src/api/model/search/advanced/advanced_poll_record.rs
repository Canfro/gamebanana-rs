use serde::{Deserialize, Serialize};

use crate::api::model::search::{
    advanced::advanced_common_record::AdvancedCommonRecord, advanced::category::Category, game::Game,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedPollRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Game,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_dsLastDayOfVoting")]
    pub last_day_of_voting: String,

    #[serde(rename = "_akState")]
    pub ak_state: String,

    #[serde(rename = "_sState")]
    pub state: String,

    #[serde(rename = "_bIsOfficial")]
    pub is_official: bool,

    #[serde(rename = "_nVoteCount")]
    pub vote_count: u64,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: u64,
}
