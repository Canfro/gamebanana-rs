use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Option {
    #[serde(rename = "_sAlias")]
    pub alias: String,

    #[serde(rename = "_sTitle")]
    pub title: String,
}
