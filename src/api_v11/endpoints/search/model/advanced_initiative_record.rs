use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::advanced_record_common::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedInitiativeRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_sMissionStatement")]
    mission_statement: String,

    #[serde(rename = "_aGoals")]
    goals: Vec<String>,

    #[serde(rename = "_dsCompletionDate")]
    completion_date: String,
}
