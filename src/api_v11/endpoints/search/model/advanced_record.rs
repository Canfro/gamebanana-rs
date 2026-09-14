use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_app_record::AdvancedAppRecord, advanced_article_record::AdvancedArticleRecord,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_sModelName")]
pub enum AdvancedRecord {
    #[serde(rename = "App")]
    App(AdvancedAppRecord),

    #[serde(rename = "Article")]
    Article(AdvancedArticleRecord),
}
