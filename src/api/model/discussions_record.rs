use serde::{Deserialize, Serialize};

use crate::api::model::{post::Post, results_record::ResultsRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionsRecord {
    #[serde(rename = "_aSubmission")]
    pub submission: ResultsRecord,

    #[serde(rename = "_aPost")]
    pub post: Post,
}
