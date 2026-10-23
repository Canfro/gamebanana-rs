use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetParams {
    #[serde(rename = "_sSort")]
    pub sort: Option<String>,

    #[serde(rename = "_aFilters[Bug_Resolution]")]
    pub filters_bug_resolution: Option<String>,

    #[serde(rename = "_aFilters[Generic_ReleaseType]")]
    pub filters_generic_release_type: Option<String>,
}
