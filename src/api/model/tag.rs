use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sRawTag")]
    pub raw_tag: String,
}
