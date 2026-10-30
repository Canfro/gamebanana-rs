use serde::{Deserialize, Serialize};

use crate::api::model::{
    club_or_studio::ClubOrStudio, community_spotlight_record::CommunitySpotlightRecord,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunitySpotlight {
    #[serde(rename = "_aContent")]
    pub content: Vec<CommunitySpotlightRecord>,

    #[serde(rename = "_aStudios")]
    pub studio: Vec<ClubOrStudio>,

    #[serde(rename = "_aClubs")]
    pub clubs: Vec<ClubOrStudio>,
}
