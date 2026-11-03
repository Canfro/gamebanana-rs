use serde::{Deserialize, Serialize};

use crate::api::section::model::validator::Validator;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DonationMethod {
    #[serde(rename = "_sTitle")]
    pub title: String,

    #[serde(rename = "_sCustomTitle")]
    pub custom_title: Option<String>,

    #[serde(rename = "_sValue")]
    pub value: String,

    #[serde(rename = "_sInputType")]
    pub input_type: Option<String>,

    #[serde(rename = "_sInputPlaceholder")]
    pub input_placeholder: Option<String>,

    #[serde(rename = "_sIconClasses")]
    pub icon_classes: Option<String>,

    #[serde(rename = "_aValidator")]
    pub validator: Option<Validator>,

    #[serde(rename = "_sValueTemplate")]
    pub value_template: Option<String>,

    #[serde(rename = "_bIsUrl")]
    pub is_url: bool,

    #[serde(rename = "_sFormattedValue")]
    pub formatted_value: Option<String>,
}
