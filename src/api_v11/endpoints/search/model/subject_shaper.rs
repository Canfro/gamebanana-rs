use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectShaper {
    #[serde(rename = "_sBorderStyle")]
    border_style: String,

    #[serde(rename = "_sFont")]
    font: String,

    #[serde(rename = "_sTextColor")]
    text_color: String,

    #[serde(rename = "_sTextHoverColor")]
    text_hover_color: String,

    #[serde(rename = "_sBorderColor")]
    border_color: Option<String>,

    #[serde(rename = "_sBorderHoverColor")]
    border_hover_color: Option<String>,
}
