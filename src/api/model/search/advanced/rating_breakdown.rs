use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingBreakdown {
    #[serde(rename = "_nCount")]
    pub count: u64,

    #[serde(rename = "_sTitle")]
    pub title: String,

    #[serde(rename = "_sVerb")]
    pub verb: String,

    #[serde(rename = "_sIconUrl")]
    pub icon_url: String,

    #[serde(rename = "_sIconClasses")]
    pub icon_classes: String,
}
