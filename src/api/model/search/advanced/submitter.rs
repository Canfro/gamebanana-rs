use serde::{Deserialize, Serialize};

use crate::api::model::search::advanced::subject_shaper::SubjectShaper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submitter {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_bIsOnline")]
    pub is_online: bool,

    #[serde(rename = "_bHasRipe")]
    pub has_ripe: bool,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_sAvatarUrl")]
    pub avatar_url: String,

    #[serde(rename = "_aClearanceLevels")]
    pub clearance_levels: Option<Vec<String>>,

    #[serde(rename = "_sHdAvatarUrl")]
    pub hd_avatar_url: Option<String>,

    #[serde(rename = "_sUpicUrl")]
    pub upic_url: Option<String>,

    #[serde(rename = "_sHovatarUrl")]
    pub hovatar_url: Option<String>,

    #[serde(rename = "_aSubjectShaper")]
    pub subject_shaper: Option<SubjectShaper>,

    #[serde(rename = "_sSubjectShaperCssCode")]
    pub subject_shaper_css_code: Option<String>,
}
