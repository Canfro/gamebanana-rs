use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Studio {
    #[serde(rename = "_sName")]
    name: String,

    #[serde(rename = "_sBannerUrl")]
    banner_url: String,

    #[serde(rename = "_sProfileUrl")]
    profile_url: String,

    #[serde(rename = "_sFlagUrl")]
    flag_url: String,
}
