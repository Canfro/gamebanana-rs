use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::advanced_record_common::AdvancedCommonRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedGameRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_sAbbreviation")]
    abbreviation: String,

    #[serde(rename = "_sHomepage")]
    homepage: String,

    #[serde(rename = "_dsReleaseDate")]
    release_date: String,
}
