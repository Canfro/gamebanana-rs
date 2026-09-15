use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_common_record::AdvancedCommonRecord, category::Category, game::Game,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedRequestRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Game,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_sResolution")]
    pub resolution: String,

    #[serde(rename = "_akResolution")]
    pub ak_resolution: String,

    #[serde(rename = "_sResolutionMessage")]
    pub resolution_message: String,

    #[serde(rename = "_aAdditionalRewards")]
    pub additional_rewards: Option<Vec<String>>,

    #[serde(rename = "_nBounty")]
    pub bounty: u64,

    #[serde(rename = "_tsLastBoostDate")]
    pub last_boost_date: i64,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: u64,
}
