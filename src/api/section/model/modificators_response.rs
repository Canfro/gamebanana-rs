use serde::{Deserialize, Serialize};

use crate::api::section::model::{filter::Filter, preset::Preset, sort::Sort};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModificatorsResponse {
    #[serde(rename = "_aSorts")]
    pub sorts: Vec<Sort>,

    #[serde(rename = "_aFilters")]
    pub filters: Vec<Filter>,

    #[serde(rename = "_aPresets")]
    pub presets: Vec<Preset>,
}
