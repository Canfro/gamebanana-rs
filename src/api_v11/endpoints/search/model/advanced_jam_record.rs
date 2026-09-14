use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::advanced_record_common::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedJamRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    tags: Vec<String>,

    #[serde(rename = "_sState")]
    state: String,

    #[serde(rename = "_sStateMessage")]
    state_message: String,

    #[serde(rename = "_aAdditionalRewards")]
    additional_rewards: Option<Vec<String>>,

    #[serde(rename = "_nBounty")]
    bounty: u64,

    #[serde(rename = "_tsLastBoostDate")]
    last_boost_date: i64,

    #[serde(rename = "_nEntryCount")]
    entry_count: u64,

    #[serde(rename = "_tsDeadline")]
    deadline: i64,

    #[serde(rename = "_tsDateUpdated")]
    date_updated: Option<i64>,

    #[serde(rename = "_nLikeCount")]
    like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    view_count: u64,
}
