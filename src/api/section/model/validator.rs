use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Validator {
    #[serde(rename = "_regexValidPattern")]
    pub regex_valid_pattern: String,

    #[serde(rename = "_sWarningMessage")]
    pub warning_message: String,
}
