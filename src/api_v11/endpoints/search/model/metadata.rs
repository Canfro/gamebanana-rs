use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Metadata {
    AdvancedResponse {
        #[serde(rename = "_nRecordCount")]
        record_count: u64,

        #[serde(rename = "_bIsComplete")]
        is_complete: bool,

        #[serde(rename = "_nPerpage")]
        per_page: u64,
    },
    PreviewMediaBlog {
        #[serde(rename = "_sSnippet")]
        snippet: String,
    },
    PreviewMediaBug {
        #[serde(rename = "_sSnippet")]
        snippet: String,

        #[serde(rename = "_akResolution")]
        ak_resolution: String,

        #[serde(rename = "_sResolution")]
        resolution: String,

        #[serde(rename = "_akPriority")]
        ak_priority: String,

        #[serde(rename = "_sPriority")]
        priority: String,
    },
    PreviewMediaClub {
        #[serde(rename = "_nMemberCount")]
        member_count: u64,
    },
    PreviewMediaContest {
        #[serde(rename = "_sSnippet")]
        snippet: String,

        #[serde(rename = "_tsDeadline")]
        deadline: i64,

        #[serde(rename = "_secTimeLeft")]
        sec_time_left: i64,
    },
    PreviewMediaJam {
        #[serde(rename = "_nBounty")]
        bounty: u64,

        #[serde(rename = "_sSnippet")]
        snippet: String,

        #[serde(rename = "_secTimeLeft")]
        sec_time_left: i64,
    },
}
