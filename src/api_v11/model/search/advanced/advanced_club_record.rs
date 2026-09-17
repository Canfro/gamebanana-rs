use serde::{Deserialize, Serialize};

use crate::api_v11::model::search::advanced::advanced_common_record::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedClubRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_iMemberCount")]
    pub i_member_count: u64,

    #[serde(rename = "_nMemberCount")]
    pub member_count: u64,

    #[serde(rename = "_tsLastActivityDate")]
    pub last_activity_date: i64,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,
}
