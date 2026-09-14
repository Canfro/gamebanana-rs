use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{image::Image, metadata::Metadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewMedia {
    #[serde(rename = "_aMetadata")]
    metadata: Option<Metadata>,

    #[serde(rename = "_aImages")]
    images: Option<Vec<Image>>,
}
