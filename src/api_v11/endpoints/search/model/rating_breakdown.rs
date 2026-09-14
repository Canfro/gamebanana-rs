use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingBreakdown {
    #[serde(rename = "_nCount")]
    count: u64,

    #[serde(rename = "_sTitle")]
    title: String,

    #[serde(rename = "_sVerb")]
    verb: String,

    #[serde(rename = "_sIconUrl")]
    icon_url: String,

    #[serde(rename = "_sIconClasses")]
    icon_classes: String,
}
