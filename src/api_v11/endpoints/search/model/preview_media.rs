use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::image::Image;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewMedia {
    #[serde(rename = "_aImages")]
    images: Vec<Image>,
}
