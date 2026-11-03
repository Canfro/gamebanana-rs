use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct File {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sFile")]
    pub file: String,

    #[serde(rename = "_nFilesize")]
    pub file_size: u64,

    #[serde(rename = "_tsDateAdded")]
    pub date_added: i64,

    #[serde(rename = "_nDownloadCount")]
    pub download_count: u64,

    #[serde(rename = "_sDownloadUrl")]
    pub download_url: String,

    #[serde(rename = "_sMd5Checksum")]
    pub md5_checksum: String,

    #[serde(rename = "_sAnalysisState")]
    pub analysis_state: String,

    #[serde(rename = "_sAnalysisResult")]
    pub analysis_result: String,

    #[serde(rename = "_sAnalysisResultVerbose")]
    pub analysis_result_verbose: String,

    #[serde(rename = "_sAvState")]
    pub av_state: String,

    #[serde(rename = "_sAvResult")]
    pub av_result: String,

    #[serde(rename = "_bIsArchived")]
    pub is_archived: bool,

    #[serde(rename = "_bHasContents")]
    pub has_contents: bool,
}
