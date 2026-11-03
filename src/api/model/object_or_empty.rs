use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, DeserializeUntaggedVerboseError)]
#[serde(deny_unknown_fields)]
pub enum ObjectOrEmpty<T> {
    Object(T),
    Empty(Vec<Value>),
}
