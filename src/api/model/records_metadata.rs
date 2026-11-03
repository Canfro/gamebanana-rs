use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;
use serde::Serialize;

use crate::api::model::section_match_count::SectionMatchCount;

#[derive(Debug, Clone, Serialize, DeserializeUntaggedVerboseError)]
#[serde(deny_unknown_fields)]
pub enum RecordsMetadata {
    AdvancedResponse {
        #[serde(rename = "_nRecordCount")]
        record_count: u64,

        #[serde(rename = "_bIsComplete")]
        is_complete: bool,

        #[serde(rename = "_nPerpage")]
        per_page: u64,
    },
    AdvancedResponseGeneral {
        #[serde(rename = "_nRecordCount")]
        record_count: u64,

        #[serde(rename = "_bIsComplete")]
        is_complete: bool,

        #[serde(rename = "_nPerpage")]
        per_page: u64,

        #[serde(rename = "_aSectionMatchCounts")]
        section_match_counts: Vec<SectionMatchCount>,
    },
}
