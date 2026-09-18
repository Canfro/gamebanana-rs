use serde::{Deserialize, Serialize};

use crate::api::model::search::records_metadata::RecordsMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse<T> {
    #[serde(rename = "_aMetadata")]
    pub metadata: RecordsMetadata,

    #[serde(rename = "_aRecords")]
    pub records: Vec<T>,
}
