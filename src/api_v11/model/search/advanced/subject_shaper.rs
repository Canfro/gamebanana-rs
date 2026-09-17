use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectShaper {
    #[serde(rename = "_sBorderStyle")]
    pub border_style: String,

    #[serde(rename = "_sFont")]
    pub font: String,

    #[serde(rename = "_sTextColor")]
    pub text_color: String,

    #[serde(rename = "_sTextHoverColor")]
    pub text_hover_color: String,

    #[serde(rename = "_sBorderColor")]
    pub border_color: Option<String>,

    #[serde(rename = "_sBorderHoverColor")]
    pub border_hover_color: Option<String>,
}
