use serde::{Deserialize, Serialize};

use crate::api::model::game::Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopGamesResponse {
    #[serde(rename = "Trending")]
    trending: Vec<Game>,

    #[serde(rename = "Classics")]
    classics: Vec<Game>,

    #[serde(rename = "New")]
    new: Vec<Game>,
}
