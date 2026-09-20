use serde::{Deserialize, Serialize};

use crate::api::model::search::modificators::preset_params::PresetParams;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    #[serde(rename = "_sTitle")]
    pub title: String,

    #[serde(rename = "_sIconClasses")]
    pub icon_classes: String,

    #[serde(rename = "_aParams")]
    pub params: PresetParams,
}
