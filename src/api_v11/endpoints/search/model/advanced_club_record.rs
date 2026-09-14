use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::advanced_record_common::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedClubRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_iMemberCount")]
    i_member_count: u64,

    #[serde(rename = "_nMemberCount")]
    member_count: u64,

    #[serde(rename = "_tsLastActivityDate")]
    last_activity_date: i64,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,
}
