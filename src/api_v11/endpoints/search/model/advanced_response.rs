use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedResponse {
    #[serde(rename = "_aMetadata")]
    metadata: Metadata,

    #[serde(rename = "_aRecords")]
    records: Vec<AdvancedRecord>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "_nRecordCount")]
    record_count: u64,

    #[serde(rename = "_bIsComplete")]
    is_complete: bool,

    #[serde(rename = "_nPerpage")]
    per_page: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedRecord {
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
    is_safe: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewMedia {
    #[serde(rename = "_aImages")]
    images: Vec<Image>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "_sType")]
    ty: String,

    #[serde(rename = "_sUrl")]
    url: Option<String>,

    #[serde(rename = "_sBaseUrl")]
    base_url: Option<String>,

    #[serde(rename = "_sCaption")]
    caption: Option<String>,

    #[serde(rename = "_sFile")]
    file: Option<String>,

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
pub struct Submitter {
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

    #[serde(rename = "_aClearanceLevels")]
    clearance_levels: Option<Vec<String>>,

    #[serde(rename = "_sHdAvatarUrl")]
    hd_avatar_url: Option<String>,

    #[serde(rename = "_sUpicUrl")]
    upic_url: Option<String>,

    #[serde(rename = "_sHovatarUrl")]
    hovatar_url: Option<String>,

    #[serde(rename = "_aSubjectShaper")]
    subject_shaper: Option<SubjectShaper>,

    #[serde(rename = "_sSubjectShaperCssCode")]
    subject_shaper_css_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectShaper {
    #[serde(rename = "_sBorderStyle")]
    border_style: String,

    #[serde(rename = "_sFont")]
    font: String,

    #[serde(rename = "_sTextColor")]
    text_color: String,

    #[serde(rename = "_sTextHoverColor")]
    text_hover_color: String,

    #[serde(rename = "_sBorderColor")]
    border_color: Option<String>,

    #[serde(rename = "_sBorderHoverColor")]
    border_hover_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "_sProfileModuleUrl")]
    profile_module_url: String,

    #[serde(rename = "_sNavigatorTabUrl")]
    navigator_tab_url: String,

    #[serde(rename = "_sMainUrl")]
    main_url: String,

    #[serde(rename = "_sSettingsUrl")]
    settings_url: String,
}
