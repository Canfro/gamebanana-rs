use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{preview_media::PreviewMedia, submitter::Submitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedCommonRecord {
    #[serde(rename = "_idRow")]
    row: u64,

    #[serde(rename = "_sSingularTitle")]
    singular_title: String,

    #[serde(rename = "_sIconClasses")]
    icon_classes: String,

    #[serde(rename = "_sName")]
    name: String,

    #[serde(rename = "_sProfileUrl")]
    profile_url: String,

    #[serde(rename = "_tsDateAdded")]
    date_added: i64,

    #[serde(rename = "_tsDateModified")]
    date_modified: i64,

    #[serde(rename = "_bHasFiles")]
    has_files: bool,

    #[serde(rename = "_aPreviewMedia")]
    preview_media: PreviewMedia,

    #[serde(rename = "_aSubmitter")]
    submitter: Submitter,

    #[serde(rename = "_sInitialVisibility")]
    initial_visibility: String,

    #[serde(rename = "_bHasContentRatings")]
    has_content_ratings: bool,

    #[serde(rename = "_bIsOwnedByAccessor")]
    is_owned_by_accessor: bool,
}
