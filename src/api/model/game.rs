use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_sIconUrl")]
    pub icon_url: String,

    #[serde(rename = "_sBannerUrl")]
    banner_url: Option<String>,

    #[serde(rename = "_aModCounts")]
    mod_counts: Option<HashMap<String, u64>>,
}
