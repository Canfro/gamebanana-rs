use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum GenericLatestSort {
    Default,
    New,
    Updated,
}

impl GenericLatestSort {
    pub fn as_str(&self) -> &str {
        match self {
            GenericLatestSort::Default => "default",
            GenericLatestSort::New => "new",
            GenericLatestSort::Updated => "updated",
        }
    }
}
