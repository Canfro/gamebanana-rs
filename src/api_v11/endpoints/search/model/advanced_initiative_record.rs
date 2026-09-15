use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::advanced_common_record::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedInitiativeRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_sMissionStatement")]
    pub mission_statement: String,

    #[serde(rename = "_aGoals")]
    pub goals: Vec<String>,

    #[serde(rename = "_dsCompletionDate")]
    pub completion_date: String,
}
