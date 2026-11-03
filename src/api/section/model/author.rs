use serde::{Deserialize, Serialize};

use crate::api::model::{string_or_uint::StringOrUint, studio::Studio};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Author {
    #[serde(rename = "_sRole")]
    pub role: Option<String>,

    #[serde(rename = "_idRow")]
    pub row: Option<StringOrUint>,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sUrl")]
    pub url: Option<String>,

    #[serde(rename = "_sUpicUrl")]
    pub upic_url: Option<String>,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: Option<String>,

    #[serde(rename = "_sAvatarUrl")]
    pub avatar_url: Option<String>,

    #[serde(rename = "_bIsOnline")]
    pub is_online: Option<bool>,

    #[serde(rename = "_aAffiliatedStudio")]
    pub affiliated_studio: Option<Studio>,
}
