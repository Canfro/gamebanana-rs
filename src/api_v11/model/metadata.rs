use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "_nRecordCount")]
    record_count: u64,

    #[serde(rename = "_bIsComplete")]
    is_complete: bool,

    #[serde(rename = "_nPerpage")]
    per_page: Option<u64>,
}
