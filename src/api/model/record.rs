use serde::{Deserialize, Serialize};

use crate::api::model::{
    category::Category, features::Features, finished_works::FinishedWorks, game::Game,
    preview_media::PreviewMedia, ratings_summary::RatingsSummary, string_or_bool::StringOrBool,
    string_or_uint::StringOrUint, studio::Studio, submitter::Submitter,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_sModelName")]
pub enum AdvancedRecord {
    #[serde(rename = "App")]
    App(AdvancedAppRecord),

    #[serde(rename = "Article")]
    Article(AdvancedArticleRecord),

    #[serde(rename = "Bug")]
    Bug(AdvancedBugRecord),

    #[serde(rename = "Blog")]
    Blog(AdvancedBlogRecord),

    #[serde(rename = "Club")]
    Club(AdvancedClubRecord),

    #[serde(rename = "Contest")]
    Contest(AdvancedContestRecord),

    #[serde(rename = "Concept")]
    Concept(AdvancedConceptRecord),

    #[serde(rename = "Event")]
    Event(AdvancedEventRecord),

    #[serde(rename = "Game")]
    Game(AdvancedGameRecord),

    #[serde(rename = "Idea")]
    Idea(AdvancedIdeaRecord),

    #[serde(rename = "Initiative")]
    Initiative(AdvancedInitiativeRecord),

    #[serde(rename = "Jam")]
    Jam(AdvancedJamRecord),

    #[serde(rename = "Mod")]
    Mod(AdvancedModRecord),

    #[serde(rename = "Model")]
    Model(AdvancedModelRecord),

    #[serde(rename = "Member")]
    Member(AdvancedMemberRecord),

    #[serde(rename = "News")]
    News(AdvancedNewsRecord),

    #[serde(rename = "Poll")]
    Poll(AdvancedPollRecord),

    #[serde(rename = "Project")]
    Project(AdvancedProjectRecord),

    #[serde(rename = "Question")]
    Question(AdvancedQuestionRecord),

    #[serde(rename = "Review")]
    Review(AdvancedReviewRecord),

    #[serde(rename = "Request")]
    Request(AdvancedRequestRecord),

    #[serde(rename = "Script")]
    Script(AdvancedScriptRecord),

    #[serde(rename = "Sound")]
    Sound(AdvancedSoundRecord),

    #[serde(rename = "Spray")]
    Spray(AdvancedSprayRecord),

    #[serde(rename = "Studio")]
    Studio(AdvancedStudioRecord),

    #[serde(rename = "Thread")]
    Thread(AdvancedThreadRecord),

    #[serde(rename = "Tool")]
    Tool(AdvancedToolRecord),

    #[serde(rename = "Tutorial")]
    Tutorial(AdvancedTutorialRecord),

    #[serde(rename = "Wiki")]
    Wiki(AdvancedWikiRecord),

    #[serde(rename = "Wip")]
    Wip(AdvancedWipRecord),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedBugRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_akResolution")]
    pub ak_resolution: String,

    #[serde(rename = "_sResolution")]
    pub resolution: String,

    #[serde(rename = "_akPriority")]
    pub ak_priority: String,

    #[serde(rename = "_sPriority")]
    pub priority: String,

    #[serde(rename = "_sSourceUrl")]
    pub source_url: String,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedAppRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_sIncludeVariableName")]
    pub include_variable_name: String,

    #[serde(rename = "_sVersion")]
    pub version: String,

    #[serde(rename = "_akState")]
    pub ak_state: String,

    #[serde(rename = "_sState")]
    pub state: String,

    #[serde(rename = "_sType")]
    pub ty: String,

    #[serde(rename = "_nUserCount")]
    pub user_count: u64,

    #[serde(rename = "_sbIsSafe")]
    pub is_safe: String,

    #[serde(rename = "_aFeatures")]
    pub features: Features,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedArticleRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedBlogRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedClubRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_iMemberCount")]
    pub i_member_count: u64,

    #[serde(rename = "_nMemberCount")]
    pub member_count: u64,

    #[serde(rename = "_tsLastActivityDate")]
    pub last_activity_date: i64,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedCommonRecord {
    #[serde(rename = "_idRow")]
    pub row: u64,

    #[serde(rename = "_sSingularTitle")]
    pub singular_title: String,

    #[serde(rename = "_sIconClasses")]
    pub icon_classes: String,

    #[serde(rename = "_sName")]
    pub name: String,

    #[serde(rename = "_sProfileUrl")]
    pub profile_url: String,

    #[serde(rename = "_tsDateAdded")]
    pub date_added: i64,

    #[serde(rename = "_tsDateModified")]
    pub date_modified: i64,

    #[serde(rename = "_bHasFiles")]
    pub has_files: bool,

    #[serde(rename = "_aPreviewMedia")]
    pub preview_media: PreviewMedia,

    #[serde(rename = "_aSubmitter")]
    pub submitter: Submitter,

    #[serde(rename = "_sInitialVisibility")]
    pub initial_visibility: String,

    #[serde(rename = "_bHasContentRatings")]
    pub has_content_ratings: bool,

    #[serde(rename = "_bIsOwnedByAccessor")]
    pub is_owned_by_accessor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConceptRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedContestRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedEventRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedGameRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_sAbbreviation")]
    pub abbreviation: String,

    #[serde(rename = "_sHomepage")]
    pub homepage: String,

    #[serde(rename = "_dsReleaseDate")]
    pub release_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedIdeaRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_sResolution")]
    pub resolution: String,

    #[serde(rename = "_akResolution")]
    pub ak_resolution: String,

    #[serde(rename = "_aRatingsSummary")]
    pub ratings_summary: RatingsSummary,

    #[serde(rename = "_sResolutionMessage")]
    pub resolution_message: String,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedInitiativeRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_sMissionStatement")]
    pub mission_statement: String,

    #[serde(rename = "_aGoals")]
    pub goals: Vec<String>,

    #[serde(rename = "_dsCompletionDate")]
    pub completion_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedJamRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_sState")]
    pub state: String,

    #[serde(rename = "_sStateMessage")]
    pub state_message: String,

    #[serde(rename = "_aAdditionalRewards")]
    pub additional_rewards: Option<Vec<String>>,

    #[serde(rename = "_nBounty")]
    pub bounty: StringOrUint,

    #[serde(rename = "_tsLastBoostDate")]
    pub last_boost_date: i64,

    #[serde(rename = "_nEntryCount")]
    pub entry_count: u64,

    #[serde(rename = "_tsDeadline")]
    pub deadline: i64,

    #[serde(rename = "_tsDateUpdated")]
    pub date_updated: Option<i64>,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedMemberRecord {
    #[serde(flatten)]
    common: AdvancedCommonRecord,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedModRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_sVersion")]
    pub version: String,

    #[serde(rename = "_tsDateUpdated")]
    pub date_updated: Option<i64>,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedModelRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_bIsMapped")]
    pub is_mapped: bool,

    #[serde(rename = "_bIsTextured")]
    pub is_textured: bool,

    #[serde(rename = "_xIsAnimated")]
    pub is_animated: StringOrBool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_aStudio")]
    pub studio: Option<Studio>,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedNewsRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedPollRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_dsLastDayOfVoting")]
    pub last_day_of_voting: String,

    #[serde(rename = "_akState")]
    pub ak_state: String,

    #[serde(rename = "_sState")]
    pub state: String,

    #[serde(rename = "_bIsOfficial")]
    pub is_official: bool,

    #[serde(rename = "_nVoteCount")]
    pub vote_count: u64,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedProjectRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_iCompletionPercentage")]
    pub completion_percentage: i64,

    #[serde(rename = "_sFinishedSubmissionUrl")]
    pub finished_submission_url: String,

    #[serde(rename = "_akDevelopmentState")]
    pub ak_development_state: String,

    #[serde(rename = "_sDevelopmentState")]
    pub development_state: String,

    #[serde(rename = "_nWipCount")]
    pub wip_count: u64,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedQuestionRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_bIsStuck")]
    pub is_stuck: bool,

    #[serde(rename = "_akState")]
    pub ak_state: String,

    #[serde(rename = "_sState")]
    pub state: String,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedRequestRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_sResolution")]
    pub resolution: String,

    #[serde(rename = "_akResolution")]
    pub ak_resolution: String,

    #[serde(rename = "_sResolutionMessage")]
    pub resolution_message: String,

    #[serde(rename = "_aAdditionalRewards")]
    pub additional_rewards: Option<Vec<String>>,

    #[serde(rename = "_nBounty")]
    pub bounty: StringOrUint,

    #[serde(rename = "_tsLastBoostDate")]
    pub last_boost_date: i64,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedReviewRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedScriptRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_sRawCode")]
    pub raw_code: String,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSoundRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_aSubCategory")]
    pub sub_category: Option<Category>,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSprayRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedStudioRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_iMemberCount")]
    pub i_member_count: i64,

    #[serde(rename = "_nMemberCount")]
    pub member_count: u64,

    #[serde(rename = "_tsLastActivityDate")]
    pub last_activity_date: i64,

    #[serde(rename = "_nRank")]
    pub rank: u64,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedThreadRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_bIsStuck")]
    pub is_stuck: bool,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedToolRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_sOfficialDownloadUrl")]
    pub official_download_url: String,

    #[serde(rename = "_sHomepageUrl")]
    pub homepage_url: String,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedTutorialRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_akDifficultyLevel")]
    pub ak_difficulty_level: String,

    #[serde(rename = "_sDifficultyLevel")]
    pub difficulty_level: String,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedWikiRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_bIsObsolete")]
    pub is_obsolete: bool,

    #[serde(rename = "_nLikeCount")]
    pub like_count: Option<u64>,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedWipRecord {
    #[serde(flatten)]
    pub common: AdvancedCommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_aGame")]
    pub game: Option<Game>,

    #[serde(rename = "_aRootCategory")]
    pub root_category: Category,

    #[serde(rename = "_akDevelopmentState")]
    pub ak_development_state: String,

    #[serde(rename = "_sDevelopmentState")]
    pub development_state: Option<String>,

    #[serde(rename = "_iCompletionPercentage")]
    pub completion_percentage: i64,

    #[serde(rename = "_aFinishedWork")]
    pub finished_work: FinishedWorks,

    #[serde(rename = "_nPostCount")]
    pub post_count: Option<u64>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}
