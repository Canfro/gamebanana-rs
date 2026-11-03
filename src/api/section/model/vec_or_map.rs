use std::collections::HashMap;

use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, DeserializeUntaggedVerboseError)]
#[serde(deny_unknown_fields)]
pub enum VecOrMap {
    Vec(Vec<String>),
    Map(HashMap<String, String>),
}
