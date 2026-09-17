use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api_v11::model::search::advanced::section_match_count::SectionMatchCount;

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
    PreviewMediaPoll {
        #[serde(rename = "_aResults")]
        results: HashMap<String, u64>,

        #[serde(rename = "_nTotalVotes")]
        total_votes: u64,
    },
    PreviewMediaQuestion {
        #[serde(rename = "_sState")]
        state: String,

        #[serde(rename = "_sSnippet")]
        snippet: String,

        #[serde(rename = "_nPostCount")]
        post_count: u64,
    },
    PreviewMediaRequest {
        #[serde(rename = "_nBounty")]
        bounty: u64,

        #[serde(rename = "_sSnippet")]
        snippet: String,
    },
    PreviewMediaAudio {
        #[serde(rename = "_sAudioUrl")]
        audio_url: String,
    },
    PreviewMediaStudio {
        #[serde(rename = "_nMemberCount")]
        member_count: u64,

        #[serde(rename = "_sGamesDeveloped")]
        games_developed: String,
    },
    PreviewMediaThread {
        #[serde(rename = "_sSnippet")]
        snippet: String,

        #[serde(rename = "_nPostCount")]
        post_count: u64,
    },
}
