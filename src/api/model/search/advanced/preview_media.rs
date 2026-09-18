use serde::{Deserialize, Serialize};

use crate::api::model::search::advanced::{image::Image, metadata::Metadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewMedia {
    #[serde(rename = "_aMetadata")]
    pub metadata: Option<Metadata>,

    #[serde(rename = "_aImages")]
    pub images: Option<Vec<Image>>,
}
