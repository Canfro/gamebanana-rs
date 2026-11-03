use serde::{Deserialize, Serialize};

use crate::api::section::model::{bio::Bio, donation_method::DonationMethod, vec_or_map::VecOrMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubmitterProfile {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sUserTitle")]
    pub user_title: String,

    #[serde(rename = "_sHonoraryTitle")]
    pub honorary_title: String,

    #[serde(rename = "_tsJoinDate")]
    pub join_date: i64,

    #[serde(rename = "_sAvatarUrl")]
    pub avatar_url: String,

    #[serde(rename = "_sSigUrl")]
    pub sig_url: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_sPointsUrl")]
    pub points_url: String,

    #[serde(rename = "_sMedalsUrl")]
    pub medals_url: String,

    #[serde(rename = "_bIsOnline")]
    pub is_online: bool,

    #[serde(rename = "_sLocation")]
    pub location: String,

    #[serde(rename = "_sOnlineTitle")]
    pub online_title: String,

    #[serde(rename = "_sOfflineTitle")]
    pub offline_title: String,

    #[serde(rename = "_nPoints")]
    pub points: i64,

    #[serde(rename = "_nPointsRank")]
    pub points_rank: u64,

    #[serde(rename = "_aNormalMedals")]
    pub normal_medals: Vec<(String, String, String, u64)>,

    #[serde(rename = "_aRareMedals")]
    pub rare_medals: Vec<(String, String, String, u64)>,

    #[serde(rename = "_aLegendaryMedals")]
    pub legendary_medals: Vec<(String, String, String, u64)>,

    #[serde(rename = "_bHasRipe")]
    pub has_ripe: bool,

    #[serde(rename = "_nBuddyCount")]
    pub buddy_count: u64,

    #[serde(rename = "_nSubscriberCount")]
    pub subscriber_count: u64,

    #[serde(rename = "_aDonationMethods")]
    pub donation_methods: Option<Vec<DonationMethod>>,

    #[serde(rename = "_bAccessorIsBuddy")]
    pub accessor_is_buddy: bool,

    #[serde(rename = "_bBuddyRequestExistsWithAccessor")]
    pub buddy_request_exists_with_accessor: bool,

    #[serde(rename = "_bAccessorIsSubscribed")]
    pub accessor_is_subscribed: bool,

    #[serde(rename = "_aBio")]
    pub bio: Option<Vec<Bio>>,

    #[serde(rename = "_aDefaultLicenseChecklist")]
    pub default_license_checklist: VecOrMap,

    #[serde(rename = "_sDefaultLicense")]
    pub default_license: String,
}
