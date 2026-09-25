use serde::{Deserialize, Serialize};

use crate::api::model::search::records_metadata::RecordsMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchResponse<T> {
    Ok {
        #[serde(rename = "_aMetadata")]
        metadata: RecordsMetadata,

        #[serde(rename = "_aRecords")]
        records: Vec<T>,
    },
    Error {
        #[serde(rename = "_sErrorCode")]
        error_code: String,

        #[serde(rename = "_sErrorMessage")]
        error_message: String,
    },
}
