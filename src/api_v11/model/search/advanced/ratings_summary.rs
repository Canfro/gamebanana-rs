use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api_v11::model::search::advanced::rating_breakdown::RatingBreakdown;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingsSummary {
    #[serde(rename = "_nRatingsCount")]
    pub ratings_count: u64,

    #[serde(rename = "_iCumulativeRating")]
    pub cumulative_rating: i64,

    #[serde(rename = "_iCumulativePositivity")]
    pub cumulative_positivity: i64,

    #[serde(rename = "_iCumulativeNegativity")]
    pub cumulative_negativity: i64,

    #[serde(rename = "_aRatingsBreakdown")]
    pub ratings_breakdown: HashMap<String, RatingBreakdown>,
}
