use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetParams {
    #[serde(rename = "_sSort")]
    sort: Option<String>,

    #[serde(rename = "_aFilters[Bug_Resolution]")]
    filters_bug_resolution: Option<String>,

    #[serde(rename = "_aFilters[Generic_ReleaseType]")]
    filters_generic_release_type: Option<String>,
}
