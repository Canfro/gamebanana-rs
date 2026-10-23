use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "_idRow")]
    pub row: i64,

    #[serde(rename = "_sRawTag")]
    pub raw_tag: String,
}
