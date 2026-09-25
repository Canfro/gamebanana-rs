use serde::{Deserialize, Serialize};

use crate::api::model::search::advanced::{category::Category, submitter::Submitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopRecord {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sModelName")]
    pub model_name: String,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_sImageUrl")]
    pub image_url: String,

    #[serde(rename = "_sThumbnailUrl")]
    pub thumbnail_url: String,

    #[serde(rename = "_sInitialVisibility")]
    pub initial_visibility: String,

    #[serde(rename = "_sPeriod")]
    pub period: String,

    #[serde(rename = "_aSubmitter")]
    pub submitter: Submitter,

    #[serde(rename = "_nLikeCount")]
    pub like_count: u64,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,
}
