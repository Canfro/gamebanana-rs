use serde::{Deserialize, Serialize};

use crate::api_v11::model::metadata::Metadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AdvancedResponse {
    #[serde(rename = "_aMetadata")]
    metadata: Metadata,

    #[serde(rename = "_aRecords")]
    records: Vec<AdvancedRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AdvancedRecord {
    #[serde(rename = "_idRow")]
    row: u64,

    #[serde(rename = "_sModelName")]
    model_name: String,

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

    #[serde(rename = "_aGame")]
    game: Game,

    #[serde(rename = "_aRootCategory")]
    root_category: RootCategory,

    #[serde(rename = "_sVersion")]
    version: String,

    #[serde(rename = "_bIsObsolete")]
    is_obsolete: bool,

    #[serde(rename = "_sInitialVisibility")]
    initial_visibility: String,

    #[serde(rename = "_nLikeCount")]
    like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    was_featured: bool,

    #[serde(rename = "_nViewCount")]
    view_count: u64,

    #[serde(rename = "_bIsOwnedByAccessor")]
    is_owned_by_accessor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreviewMedia {
    #[serde(rename = "_aImages")]
    images: Vec<Image>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Image {
    #[serde(rename = "_sType")]
    image_type: String,

    #[serde(rename = "_sBaseUrl")]
    base_url: String,

    #[serde(rename = "_sFile")]
    file: String,

    #[serde(rename = "_sFile100")]
    file100: Option<String>,

    #[serde(rename = "_sFile220")]
    file220: Option<String>,

    #[serde(rename = "_sFile530")]
    file530: Option<String>,

    #[serde(rename = "_wFile100")]
    w_file100: Option<u64>,

    #[serde(rename = "_wFile220")]
    w_file220: Option<u64>,

    #[serde(rename = "_wFile530")]
    w_file530: Option<u64>,

    #[serde(rename = "_hFile100")]
    h_file100: Option<u64>,

    #[serde(rename = "_hFile220")]
    h_file220: Option<u64>,

    #[serde(rename = "_hFile530")]
    h_file530: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Submitter {
    #[serde(rename = "_idRow")]
    row: u64,

    #[serde(rename = "_sName")]
    name: String,

    #[serde(rename = "_bIsOnline")]
    is_online: bool,

    #[serde(rename = "_bHasRipe")]
    has_ripe: bool,

    #[serde(rename = "_sProfileUrl")]
    profile_url: String,

    #[serde(rename = "_sAvatarUrl")]
    avatar_url: String,

    #[serde(rename = "_sHdAvatarUrl")]
    hd_avatar_url: Option<String>,

    #[serde(rename = "_sUpicUrl")]
    upic_url: Option<String>,

    #[serde(rename = "_sSubjectShaperCssCode")]
    subject_shaper_css_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Game {
    #[serde(rename = "_idRow")]
    row: u64,

    #[serde(rename = "_sName")]
    name: String,

    #[serde(rename = "_sProfileUrl")]
    profile_url: String,

    #[serde(rename = "_sIconUrl")]
    icon_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RootCategory {
    #[serde(rename = "_sName")]
    name: String,

    #[serde(rename = "_sProfileUrl")]
    profile_url: String,

    #[serde(rename = "_sIconUrl")]
    icon_url: String,
}
