use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "_idRow")]
    row: i64,

    #[serde(rename = "_sRawTag")]
    raw_tag: String,
}
