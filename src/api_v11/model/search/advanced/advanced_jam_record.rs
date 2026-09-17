use serde::{Deserialize, Serialize};

use crate::api_v11::model::search::advanced::advanced_common_record::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedJamRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_sState")]
    pub state: String,

    #[serde(rename = "_sStateMessage")]
    pub state_message: String,

    #[serde(rename = "_aAdditionalRewards")]
    pub additional_rewards: Option<Vec<String>>,

    #[serde(rename = "_nBounty")]
    pub bounty: u64,

    #[serde(rename = "_tsLastBoostDate")]
    pub last_boost_date: i64,

    #[serde(rename = "_nEntryCount")]
    pub entry_count: u64,

    #[serde(rename = "_tsDeadline")]
    pub deadline: i64,

    #[serde(rename = "_tsDateUpdated")]
    pub date_updated: Option<i64>,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: u64,
}
