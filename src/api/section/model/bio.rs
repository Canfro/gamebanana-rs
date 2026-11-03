use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bio {
    #[serde(rename = "_sTitle")]
    pub title: String,

    #[serde(rename = "_sCustomTitle")]
    pub custom_title: Option<String>,

    #[serde(rename = "_sValue")]
    pub value: String,
}
