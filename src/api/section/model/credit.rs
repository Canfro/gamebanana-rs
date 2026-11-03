use serde::{Deserialize, Serialize};

use crate::api::section::model::author::Author;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Credit {
    #[serde(rename = "_sGroupName")]
    pub group_name: String,

    #[serde(rename = "_aAuthors")]
    pub authors: Option<Vec<Author>>,
}
