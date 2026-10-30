use serde::{Deserialize, Serialize};

use crate::api::model::game::Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopGamesResponse {
    #[serde(rename = "Trending")]
    pub trending: Vec<Game>,

    #[serde(rename = "Classics")]
    pub classics: Vec<Game>,

    #[serde(rename = "New")]
    pub new: Vec<Game>,
}
