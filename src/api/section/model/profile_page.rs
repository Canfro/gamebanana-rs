use serde::{Deserialize, Serialize};

use crate::api::{
    model::{category::Category, features::Features, preview_media::PreviewMedia, studio::Studio},
    section::model::{
        credit::Credit, embeddables::Embeddables, file::File, game_profile::GameProfile,
        license_checklist::LicenseChecklist, submitter_profile::SubmitterProfile,
        tag_profile::TagProfile,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppProfilePage {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_nStatus")]
    pub status: String,

    #[serde(rename = "_bIsPrivate")]
    pub is_private: bool,

    #[serde(rename = "_bAccessorIsSubmitter")]
    pub accessor_is_submitter: bool,

    #[serde(rename = "_bIsTrashed")]
    pub is_trashed: bool,

    #[serde(rename = "_bIsWithheld")]
    pub is_withheld: bool,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_tsDateModified")]
    pub date_modified: Option<i64>,

    #[serde(rename = "_tsDateAdded")]
    pub date_added: Option<i64>,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: Option<String>,

    #[serde(rename = "_aPreviewMedia")]
    pub preview_media: Option<PreviewMedia>,

    #[serde(rename = "_nUpdatesCount")]
    pub updates_count: Option<u64>,

    #[serde(rename = "_bHasUpdates")]
    pub has_updates: Option<bool>,

    #[serde(rename = "_tsDateUpdated")]
    pub date_updated: Option<i64>,

    #[serde(rename = "_nAllTodosCount")]
    pub all_todos_count: Option<u64>,

    #[serde(rename = "_bHasTodos")]
    pub has_todos: Option<bool>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_aTags")]
    pub tags: Option<Vec<TagProfile>>,

    #[serde(rename = "_nThanksCount")]
    pub thanks_count: Option<u64>,

    #[serde(rename = "_sInitialVisibility")]
    pub initial_visibility: Option<String>,

    #[serde(rename = "_bHasFiles")]
    pub has_files: Option<bool>,

    #[serde(rename = "_nSubscriberCount")]
    pub subscriber_count: Option<u64>,

    #[serde(rename = "_sDescription")]
    pub description: Option<String>,

    #[serde(rename = "_bGenerateTableOfContents")]
    pub generate_table_contents: Option<bool>,

    #[serde(rename = "_sText")]
    pub text: Option<String>,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,

    #[serde(rename = "_sIncludeVariableName")]
    pub include_variable_name: Option<String>,

    #[serde(rename = "_sVersion")]
    pub version: Option<String>,

    #[serde(rename = "_akState")]
    pub ak_state: Option<String>,

    #[serde(rename = "_sState")]
    pub state: Option<String>,

    #[serde(rename = "_sType")]
    pub ty: Option<String>,

    #[serde(rename = "_nUserCount")]
    pub user_count: Option<u64>,

    #[serde(rename = "_sbIsSafe")]
    pub is_safe: Option<String>,

    #[serde(rename = "_aFeatures")]
    pub features: Option<Features>,

    #[serde(rename = "_bAcceptsDonations")]
    pub accepts_donations: Option<bool>,

    #[serde(rename = "_bShowRipePromo")]
    pub show_ripe_promo: Option<bool>,

    #[serde(rename = "_aEmbeddables")]
    pub embeddables: Option<Embeddables>,

    #[serde(rename = "_aSubmitter")]
    pub submitter: Option<SubmitterProfile>,

    #[serde(rename = "_bFollowLinks")]
    pub follow_links: Option<bool>,

    #[serde(rename = "_aCredits")]
    pub credits: Option<Vec<Credit>>,

    #[serde(rename = "_sStickerUrl")]
    pub sticker_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArticleProfilePage {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_nStatus")]
    pub status: String,

    #[serde(rename = "_bIsPrivate")]
    pub is_private: bool,

    #[serde(rename = "_bAccessorIsSubmitter")]
    pub accessor_is_submitter: bool,

    #[serde(rename = "_bIsTrashed")]
    pub is_trashed: bool,

    #[serde(rename = "_bIsWithheld")]
    pub is_withheld: bool,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sCommentsMode")]
    pub comments_mode: String,

    #[serde(rename = "_tsDateModified")]
    pub date_modified: Option<i64>,

    #[serde(rename = "_tsDateAdded")]
    pub date_added: Option<i64>,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: Option<String>,

    #[serde(rename = "_aPreviewMedia")]
    pub preview_media: Option<PreviewMedia>,

    #[serde(rename = "_nUpdatesCount")]
    pub updates_count: Option<u64>,

    #[serde(rename = "_bHasUpdates")]
    pub has_updates: Option<bool>,

    #[serde(rename = "_nAllTodosCount")]
    pub all_todos_count: Option<u64>,

    #[serde(rename = "_bHasTodos")]
    pub has_todos: Option<bool>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_aTags")]
    pub tags: Option<Vec<TagProfile>>,

    #[serde(rename = "_nThanksCount")]
    pub thanks_count: Option<u64>,

    #[serde(rename = "_sInitialVisibility")]
    pub initial_visibility: Option<String>,

    #[serde(rename = "_bHasFiles")]
    pub has_files: Option<bool>,

    #[serde(rename = "_nSubscriberCount")]
    pub subscriber_count: Option<u64>,

    #[serde(rename = "_bGenerateTableOfContents")]
    pub generate_table_contents: Option<bool>,

    #[serde(rename = "_sText")]
    pub text: Option<String>,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,

    #[serde(rename = "_bShowRipePromo")]
    pub show_ripe_promo: Option<bool>,

    #[serde(rename = "_aEmbeddables")]
    pub embeddables: Option<Embeddables>,

    #[serde(rename = "_aSubmitter")]
    pub submitter: Option<SubmitterProfile>,

    #[serde(rename = "_bFollowLinks")]
    pub follow_links: Option<bool>,

    #[serde(rename = "_aGame")]
    pub game: GameProfile,

    #[serde(rename = "_aCategory")]
    pub category: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModProfilePage {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_nStatus")]
    pub status: String,

    #[serde(rename = "_bIsPrivate")]
    pub is_private: bool,

    #[serde(rename = "_bAccessorIsSubmitter")]
    pub accessor_is_submitter: bool,

    #[serde(rename = "_bIsTrashed")]
    pub is_trashed: bool,

    #[serde(rename = "_bIsWithheld")]
    pub is_withheld: bool,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sCommentsMode")]
    pub comments_mode: Option<String>,

    #[serde(rename = "_tsDateModified")]
    pub date_modified: Option<i64>,

    #[serde(rename = "_tsDateAdded")]
    pub date_added: Option<i64>,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: Option<String>,

    #[serde(rename = "_aPreviewMedia")]
    pub preview_media: Option<PreviewMedia>,

    #[serde(rename = "_nUpdatesCount")]
    pub updates_count: Option<u64>,

    #[serde(rename = "_bHasUpdates")]
    pub has_updates: Option<bool>,

    #[serde(rename = "_tsDateUpdated")]
    pub date_updated: Option<i64>,

    #[serde(rename = "_nAllTodosCount")]
    pub all_todos_count: Option<u64>,

    #[serde(rename = "_bHasTodos")]
    pub has_todos: Option<bool>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_aTags")]
    pub tags: Option<Vec<TagProfile>>,

    #[serde(rename = "_bCreatedBySubmitter")]
    pub created_by_submitter: Option<bool>,

    #[serde(rename = "_bIsPorted")]
    pub is_ported: Option<bool>,

    #[serde(rename = "_nThanksCount")]
    pub thanks_count: Option<u64>,

    #[serde(rename = "_sInitialVisibility")]
    pub initial_visibility: Option<String>,

    #[serde(rename = "_sDownloadUrl")]
    pub download_url: Option<String>,

    #[serde(rename = "_nDownloadCount")]
    pub download_count: Option<u64>,

    #[serde(rename = "_aFiles")]
    pub files: Option<Vec<File>>,

    #[serde(rename = "_nSubscriberCount")]
    pub subscriber_count: Option<u64>,

    #[serde(rename = "_aContributingStudios")]
    pub contributing_studios: Option<Vec<Studio>>,

    #[serde(rename = "_sLicense")]
    pub license: Option<String>,

    #[serde(rename = "_aLicenseChecklist")]
    pub license_checklist: Option<LicenseChecklist>,

    #[serde(rename = "_bGenerateTableOfContents")]
    pub generate_table_contents: Option<bool>,

    #[serde(rename = "_sText")]
    pub text: Option<String>,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: Option<bool>,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,

    #[serde(rename = "_sVersion")]
    pub version: Option<String>,

    #[serde(rename = "_bAcceptsDonations")]
    pub accepts_donations: Option<bool>,

    #[serde(rename = "_bShowRipePromo")]
    pub show_ripe_promo: Option<bool>,

    #[serde(rename = "_aEmbeddables")]
    pub embeddables: Option<Embeddables>,

    #[serde(rename = "_aSubmitter")]
    pub submitter: Option<SubmitterProfile>,

    #[serde(rename = "_bFollowLinks")]
    pub follow_links: Option<bool>,

    #[serde(rename = "_aGame")]
    pub game: Option<GameProfile>,

    #[serde(rename = "_aCategory")]
    pub category: Option<Category>,

    #[serde(rename = "_aSuperCategory")]
    pub super_category: Option<Category>,

    #[serde(rename = "_aCredits")]
    pub credits: Option<Vec<Credit>>,
}
