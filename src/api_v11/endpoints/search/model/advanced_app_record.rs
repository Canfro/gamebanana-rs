use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    features::Features, preview_media::PreviewMedia, submitter::Submitter,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedAppRecord {
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

    #[serde(rename = "_aTags")]
    tags: Vec<String>,

    #[serde(rename = "_aPreviewMedia")]
    preview_media: PreviewMedia,

    #[serde(rename = "_aSubmitter")]
    submitter: Submitter,

    #[serde(rename = "_sIncludeVariableName")]
    include_variable_name: String,

    #[serde(rename = "_sVersion")]
    version: String,

    #[serde(rename = "_akState")]
    ak_state: String,

    #[serde(rename = "_sState")]
    state: String,

    #[serde(rename = "_sType")]
    ty: String,

    #[serde(rename = "_nUserCount")]
    user_count: u64,

    #[serde(rename = "_sbIsSafe")]
    is_safe: String,

    #[serde(rename = "_aFeatures")]
    features: Features,

    #[serde(rename = "_sInitialVisibility")]
    initial_visibility: String,

    #[serde(rename = "_bHasContentRatings")]
    has_content_ratings: bool,

    #[serde(rename = "_bWasFeatured")]
    was_featured: bool,

    #[serde(rename = "_nViewCount")]
    view_count: u64,

    #[serde(rename = "_bIsOwnedByAccessor")]
    is_owned_by_accessor: bool,
}
