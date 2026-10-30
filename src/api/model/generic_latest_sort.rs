use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum FeedSort {
    Default,
    New,
    Updated,
}

impl FeedSort {
    pub fn as_str(&self) -> &str {
        match self {
            FeedSort::Default => "default",
            FeedSort::New => "new",
            FeedSort::Updated => "updated",
        }
    }
}
