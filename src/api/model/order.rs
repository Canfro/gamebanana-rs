use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ResultsSort {
    BestMatch,
    Popularity,
    Date,
    Update,
}

impl ResultsSort {
    pub fn as_str(&self) -> &str {
        match self {
            ResultsSort::BestMatch => "best_match",
            ResultsSort::Popularity => "popularity",
            ResultsSort::Date => "date",
            ResultsSort::Update => "Update",
        }
    }
}
