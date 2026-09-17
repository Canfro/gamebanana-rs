use serde::{Deserialize, Serialize};

use crate::api_v11::model::search::modificators::option;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "_sType")]
    ty: String,

    #[serde(rename = "_sTitle")]
    title: Option<String>,

    #[serde(rename = "_aOptions")]
    options: Option<Vec<option::Option>>,

    #[serde(rename = "_sAlias")]
    alias: String,
}
