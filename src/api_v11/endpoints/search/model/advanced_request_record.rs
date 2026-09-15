use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_common_record::AdvancedCommonRecord, category::Category, game::Game,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedRequestRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    tags: Vec<String>,

    #[serde(rename = "_aGame")]
    game: Game,

    #[serde(rename = "_aRootCategory")]
    root_category: Category,

    #[serde(rename = "_sResolution")]
    resolution: String,

    #[serde(rename = "_akResolution")]
    ak_resolution: String,

    #[serde(rename = "_sResolutionMessage")]
    resolution_message: String,

    #[serde(rename = "_aAdditionalRewards")]
    additional_rewards: Option<Vec<String>>,

    #[serde(rename = "_nBounty")]
    bounty: u64,

    #[serde(rename = "_tsLastBoostDate")]
    last_boost_date: i64,

    #[serde(rename = "_nLikeCount")]
    like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    view_count: u64,
}
