use serde::{Deserialize, Serialize};

use crate::api::model::search::modificators::{filter::Filter, preset::Preset, sort::Sort};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModificatorsResponse {
    #[serde(rename = "_aSorts")]
    pub sorts: Vec<Sort>,

    #[serde(rename = "_aFilters")]
    pub filters: Vec<Filter>,

    #[serde(rename = "_aPresets")]
    pub presets: Vec<Preset>,
}
