use serde::{Deserialize, Serialize};

use crate::api::model::search::advanced::preview_media::PreviewMedia;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishedWork {
    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sModelName")]
    pub model_name: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_aPreviewMedia")]
    pub preview_media: PreviewMedia,

    #[serde(rename = "_tsDateAdded")]
    pub date_added: i64,

    #[serde(rename = "_sInitialVisibility")]
    pub initial_visibility: String,
}
