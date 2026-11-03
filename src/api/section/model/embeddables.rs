use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Embeddables {
    #[serde(rename = "_sEmbeddableImageBaseUrl")]
    pub embeddable_image_base_url: String,

    #[serde(rename = "_aVariants")]
    pub variants: Vec<String>,
}
