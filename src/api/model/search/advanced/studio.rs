use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Studio {
    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sBannerUrl")]
    pub banner_url: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_sFlagUrl")]
    pub flag_url: Option<String>,
}
