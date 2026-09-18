use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum LatestSort {
    Default,
    New,
    Updated,
}

impl LatestSort {
    pub fn as_str(&self) -> &str {
        match self {
            LatestSort::Default => "default",
            LatestSort::New => "new",
            LatestSort::Updated => "updated",
        }
    }
}
