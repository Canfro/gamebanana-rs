use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sort {
    #[serde(rename = "_sAlias")]
    alias: String,

    #[serde(rename = "_sTitle")]
    title: String,
}
