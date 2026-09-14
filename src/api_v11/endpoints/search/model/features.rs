use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "_sProfileModuleUrl")]
    profile_module_url: String,

    #[serde(rename = "_sNavigatorTabUrl")]
    navigator_tab_url: String,

    #[serde(rename = "_sMainUrl")]
    main_url: String,

    #[serde(rename = "_sSettingsUrl")]
    settings_url: String,
}
