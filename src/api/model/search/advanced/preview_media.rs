use serde::{Deserialize, Serialize};

use crate::api::model::search::advanced::{
    image::Image, preview_media_metadata::PreviewMediaMetadata,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewMedia {
    #[serde(rename = "_aMetadata")]
    pub metadata: Option<PreviewMediaMetadata>,

    #[serde(rename = "_aImages")]
    pub images: Option<Vec<Image>>,
}
