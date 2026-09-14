use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewMediaMetadata {
    #[serde(rename = "_sSnippet")]
    snippet: String,

    #[serde(rename = "_akResolution")]
    ak_resolution: String,

    #[serde(rename = "_sResolution")]
    resolution: String,

    #[serde(rename = "_akPriority")]
    ak_priority: String,

    #[serde(rename = "_sPriority")]
    priority: String,
}
