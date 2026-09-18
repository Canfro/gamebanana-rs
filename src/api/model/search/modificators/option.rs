use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Option {
    #[serde(rename = "_sAlias")]
    alias: String,

    #[serde(rename = "_sTitle")]
    title: String,
}
