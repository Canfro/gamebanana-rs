use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cooltip {
    #[serde(rename = "_sBorderStyle")]
    pub border_style: String,

    #[serde(rename = "_sFont")]
    pub font: String,

    #[serde(rename = "_sBoxBorderStyle")]
    pub box_border_style: String,

    #[serde(rename = "_sTextColor")]
    pub text_color: Option<String>,

    #[serde(rename = "_sBackgroundImageFile")]
    pub background_image_file: Option<String>,

    #[serde(rename = "_sBackgroundImageBaseUrl")]
    pub background_image_base_url: Option<String>,

    #[serde(rename = "_sBackgroundImageUrl")]
    pub background_image_url: Option<String>,

    #[serde(rename = "_sBackgroundColor")]
    pub background_color: Option<String>,

    #[serde(rename = "_sBoxBorderSize")]
    pub box_border_size: u64,

    #[serde(rename = "_sLinkColor")]
    pub link_color: Option<String>,

    #[serde(rename = "_sLinkHoverColor")]
    pub link_hover_color: Option<String>,

    #[serde(rename = "_sBoxBorderColor")]
    pub box_border_color: Option<String>,
}
