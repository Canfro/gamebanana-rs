use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Category {
    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_sIconUrl")]
    pub icon_url: String,
}
