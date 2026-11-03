use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Featuring {
    #[serde(rename = "_sFeatureGroup")]
    pub feature_group: String,

    #[serde(rename = "_sTitle")]
    pub title: String,

    #[serde(rename = "_sIconClasses")]
    pub icon_classes: String,

    #[serde(rename = "_tsDate")]
    pub date: i64,
}
