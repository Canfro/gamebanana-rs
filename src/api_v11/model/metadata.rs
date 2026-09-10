use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct Metadata {
    record_count: u64,
    is_complete: bool,
    per_page: Option<u64>,
}
