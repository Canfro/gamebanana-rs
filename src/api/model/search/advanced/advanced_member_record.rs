use serde::{Deserialize, Serialize};

use crate::api::model::search::advanced::advanced_common_record::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedMemberRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,
}
