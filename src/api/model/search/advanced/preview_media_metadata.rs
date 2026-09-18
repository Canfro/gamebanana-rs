use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::model::search::advanced::{
    section_match_count::SectionMatchCount, string_or_uint::StringOrUint,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PreviewMediaMetadata {
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
        bounty: StringOrUint,

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
        post_count: Option<u64>,
    },
    PreviewMediaRequest {
        #[serde(rename = "_nBounty")]
        bounty: StringOrUint,

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
        post_count: Option<u64>,
    },
}
