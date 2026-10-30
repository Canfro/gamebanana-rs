use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum SubmissionsSort {
    Default,
    New,
    Updated,
}

impl SubmissionsSort {
    pub fn as_str(&self) -> &str {
        match self {
            SubmissionsSort::Default => "default",
            SubmissionsSort::New => "new",
            SubmissionsSort::Updated => "updated",
        }
    }
}
