use serde::{Deserialize, Serialize};

use crate::api::model::submitter::Submitter;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Post {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_tsDateAdded")]
    pub date_added: i64,

    #[serde(rename = "_sText")]
    pub text: String,

    #[serde(rename = "_aPoster")]
    pub poster: Submitter,
}
