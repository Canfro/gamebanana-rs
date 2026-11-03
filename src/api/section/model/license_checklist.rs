use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseChecklist {
    #[serde(rename = "yes")]
    pub yes: Option<Vec<String>>,

    #[serde(rename = "ask")]
    pub ask: Option<Vec<String>>,

    #[serde(rename = "no")]
    pub no: Option<Vec<String>>,

    #[serde(rename = "only_if_same")]
    pub only_if_same: Option<Vec<String>>,
}
