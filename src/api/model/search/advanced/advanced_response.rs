use serde::{Deserialize, Serialize};

use crate::api::model::search::{
    advanced::advanced_record::AdvancedRecord, advanced::metadata::Metadata,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedResponse {
    #[serde(rename = "_aMetadata")]
    pub metadata: Metadata,

    #[serde(rename = "_aRecords")]
    pub records: Vec<AdvancedRecord>,
}
