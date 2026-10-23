use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "_sProfileModuleUrl")]
    pub profile_module_url: String,

    #[serde(rename = "_sNavigatorTabUrl")]
    pub navigator_tab_url: String,

    #[serde(rename = "_sMainUrl")]
    pub main_url: String,

    #[serde(rename = "_sSettingsUrl")]
    pub settings_url: String,
}
