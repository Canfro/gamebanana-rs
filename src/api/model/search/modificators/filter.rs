use serde::{Deserialize, Serialize};

use crate::api::model::search::modificators::option;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "_sType")]
    pub ty: String,

    #[serde(rename = "_sTitle")]
    pub title: Option<String>,

    #[serde(rename = "_aOptions")]
    pub options: Option<Vec<option::Option>>,

    #[serde(rename = "_sAlias")]
    pub alias: String,
}
