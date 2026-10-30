use serde::{Deserialize, Serialize};

use crate::api::model::{game::Game, preview_media::PreviewMedia, submitter::Submitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunitySpotlightRecord {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sModelName")]
    pub model_name: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_aPreviewMedia")]
    pub preview_media: PreviewMedia,

    #[serde(rename = "_sInitialVisibility")]
    pub initial_visibility: Option<String>,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_aSubmitter")]
    pub submitter: Submitter,

    #[serde(rename = "_aGame")]
    pub game: Game,
}
