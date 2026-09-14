use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_record::AdvancedRecord, advanced_response_metadata::AdvancedResponseMetadata,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedResponse {
    #[serde(rename = "_aMetadata")]
    metadata: AdvancedResponseMetadata,

    #[serde(rename = "_aRecords")]
    records: Vec<AdvancedRecord>,
}
