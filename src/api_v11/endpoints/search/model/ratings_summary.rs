use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::rating_breakdown::RatingBreakdown;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingsSummary {
    #[serde(rename = "_nRatingsCount")]
    ratings_count: u64,

    #[serde(rename = "_iCumulativeRating")]
    cumulative_rating: i64,

    #[serde(rename = "_iCumulativePositivity")]
    cumulative_positivity: i64,

    #[serde(rename = "_iCumulativeNegativity")]
    cumulative_negativity: i64,

    #[serde(rename = "_aRatingsBreakdown")]
    ratings_breakdown: HashMap<String, RatingBreakdown>,
}
