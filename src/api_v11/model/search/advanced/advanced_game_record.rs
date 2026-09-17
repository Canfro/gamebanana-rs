use serde::{Deserialize, Serialize};

use crate::api_v11::model::search::advanced::advanced_common_record::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedGameRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_sAbbreviation")]
    pub abbreviation: String,

    #[serde(rename = "_sHomepage")]
    pub homepage: String,

    #[serde(rename = "_dsReleaseDate")]
    pub release_date: String,
}
