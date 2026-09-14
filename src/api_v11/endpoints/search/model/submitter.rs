use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::subject_shaper::SubjectShaper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submitter {
    #[serde(rename = "_idRow")]
    row: u64,

    #[serde(rename = "_sName")]
    name: String,

    #[serde(rename = "_bIsOnline")]
    is_online: bool,

    #[serde(rename = "_bHasRipe")]
    has_ripe: bool,

    #[serde(rename = "_sProfileUrl")]
    profile_url: String,

    #[serde(rename = "_sAvatarUrl")]
    avatar_url: String,

    #[serde(rename = "_aClearanceLevels")]
    clearance_levels: Option<Vec<String>>,

    #[serde(rename = "_sHdAvatarUrl")]
    hd_avatar_url: Option<String>,

    #[serde(rename = "_sUpicUrl")]
    upic_url: Option<String>,

    #[serde(rename = "_sHovatarUrl")]
    hovatar_url: Option<String>,

    #[serde(rename = "_aSubjectShaper")]
    subject_shaper: Option<SubjectShaper>,

    #[serde(rename = "_sSubjectShaperCssCode")]
    subject_shaper_css_code: Option<String>,
}
