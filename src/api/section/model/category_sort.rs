use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CategorySort {
    AtoZ,
    Count,
}

impl CategorySort {
    pub fn as_str(&self) -> &str {
        match self {
            CategorySort::AtoZ => "a_to_z",
            CategorySort::Count => "count",
        }
    }
}
