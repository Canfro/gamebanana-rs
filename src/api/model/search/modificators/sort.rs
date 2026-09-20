use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sort {
    #[serde(rename = "_sAlias")]
    pub alias: String,

    #[serde(rename = "_sTitle")]
    pub title: String,
}
