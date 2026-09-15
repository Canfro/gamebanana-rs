use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{preview_media::PreviewMedia, submitter::Submitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedCommonRecord {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sSingularTitle")]
    pub singular_title: String,

    #[serde(rename = "_sIconClasses")]
    pub icon_classes: String,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_tsDateAdded")]
    pub date_added: i64,

    #[serde(rename = "_tsDateModified")]
    pub date_modified: i64,

    #[serde(rename = "_bHasFiles")]
    pub has_files: bool,

    #[serde(rename = "_aPreviewMedia")]
    pub preview_media: PreviewMedia,

    #[serde(rename = "_aSubmitter")]
    pub submitter: Submitter,

    #[serde(rename = "_sInitialVisibility")]
    pub initial_visibility: String,

    #[serde(rename = "_bHasContentRatings")]
    pub has_content_ratings: bool,

    #[serde(rename = "_bIsOwnedByAccessor")]
    pub is_owned_by_accessor: bool,
}
