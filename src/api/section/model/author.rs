use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Author {
    #[serde(rename = "_sRole")]
    pub role: String,

    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_sAvatarUrl")]
    pub avatar_url: String,

    #[serde(rename = "_bIsOnline")]
    pub is_online: bool,
}
