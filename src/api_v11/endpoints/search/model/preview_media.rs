use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    image::Image, preview_media_metadata::PreviewMediaMetadata,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewMedia {
    #[serde(rename = "_aMetadata")]
    metadata: Option<PreviewMediaMetadata>,

    #[serde(rename = "_aImages")]
    images: Option<Vec<Image>>,
}
