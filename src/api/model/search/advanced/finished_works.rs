use serde::{Deserialize, Serialize};

use crate::api::model::search::advanced::finished_work::FinishedWork;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishedWorks {
    #[serde(rename = "_aFinishedWorksOnGameBanana")]
    pub finished_works_on_gamebanana: Vec<FinishedWork>,

    #[serde(rename = "_aRemoteFinishedWorkUrls")]
    pub remote_finished_work_urls: Vec<String>,
}
