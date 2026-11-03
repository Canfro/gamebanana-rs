use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameProfile {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sAbbreviation")]
    pub abbreviation: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_sIconUrl")]
    pub icon_url: String,

    #[serde(rename = "_sBannerUrl")]
    pub banner_url: String,

    #[serde(rename = "_nSubscriberCount")]
    pub subscriber_count: u64,

    #[serde(rename = "_bHasSubmissionQueue")]
    pub has_submission_queue: bool,

    #[serde(rename = "_bAccessorIsSubscribed")]
    pub accessor_is_subscribed: bool,
}
