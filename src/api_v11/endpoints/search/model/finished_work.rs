use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::preview_media::PreviewMedia;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishedWork {
    #[serde(rename = "_sName")]
    name: String,

    #[serde(rename = "_sModelName")]
    model_name: String,

    #[serde(rename = "_sProfileUrl")]
    profile_url: String,

    #[serde(rename = "_aPreviewMedia")]
    preview_media: PreviewMedia,

    #[serde(rename = "_tsDateAdded")]
    date_added: i64,

    #[serde(rename = "_sInitialVisibility")]
    initial_visibility: String,
}
