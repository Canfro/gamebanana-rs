use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_record::AdvancedRecord, metadata::Metadata,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedResponse {
    #[serde(rename = "_aMetadata")]
    metadata: Metadata,

    #[serde(rename = "_aRecords")]
    records: Vec<AdvancedRecord>,
}
