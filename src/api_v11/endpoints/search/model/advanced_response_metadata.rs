use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AdvancedResponseMetadata {
    #[serde(rename = "_nRecordCount")]
    record_count: u64,

    #[serde(rename = "_bIsComplete")]
    is_complete: bool,

    #[serde(rename = "_nPerpage")]
    per_page: u64,
}
