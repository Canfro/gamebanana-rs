use serde::{Deserialize, Serialize};

use crate::api::model::records_metadata::RecordsMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchResponse<T> {
    #[serde(rename = "_aMetadata")]
    metadata: RecordsMetadata,

    #[serde(rename = "_aRecords")]
    records: Vec<T>,
}
