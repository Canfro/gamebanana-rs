use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::finished_work::FinishedWork;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishedWorks {
    #[serde(rename = "_aFinishedWorksOnGameBanana")]
    finished_works_on_gamebanana: Vec<FinishedWork>,

    #[serde(rename = "_aRemoteFinishedWorkUrls")]
    remote_finished_work_urls: Vec<String>,
}
