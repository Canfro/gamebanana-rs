use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectOrEmpty<T> {
    Object(T),
    Empty(Vec<Value>),
}
