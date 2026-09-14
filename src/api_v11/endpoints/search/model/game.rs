use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_idRow")]
    row: u64,

    #[serde(rename = "_sName")]
    name: String,

    #[serde(rename = "_sProfileUrl")]
    profile_url: String,

    #[serde(rename = "_sIconUrl")]
    icon_url: String,
}
