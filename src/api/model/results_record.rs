use serde::{Deserialize, Serialize};

use crate::api::model::{
    category::Category, features::Features, finished_works::FinishedWorks, game::Game,
    preview_media::PreviewMedia, ratings_summary::RatingsSummary, string_or_bool::StringOrBool,
    string_or_uint::StringOrUint, studio::Studio, submitter::Submitter,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_sModelName")]
pub enum ResultsRecord {
    #[serde(rename = "App")]
    App(AppRecord),

    #[serde(rename = "Article")]
    Article(ArticleRecord),

    #[serde(rename = "Bug")]
    Bug(BugRecord),

    #[serde(rename = "Blog")]
    Blog(BlogRecord),

    #[serde(rename = "Club")]
    Club(ClubRecord),

    #[serde(rename = "Contest")]
    Contest(ContestRecord),

    #[serde(rename = "Concept")]
    Concept(ConceptRecord),

    #[serde(rename = "Event")]
    Event(EventRecord),

    #[serde(rename = "Game")]
    Game(GameRecord),

    #[serde(rename = "Idea")]
    Idea(IdeaRecord),

    #[serde(rename = "Initiative")]
    Initiative(InitiativeRecord),

    #[serde(rename = "Jam")]
    Jam(JamRecord),

    #[serde(rename = "Mod")]
    Mod(ModRecord),

    #[serde(rename = "Model")]
    Model(ModelRecord),

    #[serde(rename = "Member")]
    Member(MemberRecord),

    #[serde(rename = "News")]
    News(NewsRecord),

    #[serde(rename = "Poll")]
    Poll(PollRecord),

    #[serde(rename = "Project")]
    Project(ProjectRecord),

    #[serde(rename = "Question")]
    Question(QuestionRecord),

    #[serde(rename = "Review")]
    Review(ReviewRecord),

    #[serde(rename = "Request")]
    Request(RequestRecord),

    #[serde(rename = "Script")]
    Script(ScriptRecord),

    #[serde(rename = "Sound")]
    Sound(SoundRecord),

    #[serde(rename = "Spray")]
    Spray(SprayRecord),

    #[serde(rename = "Studio")]
    Studio(StudioRecord),

    #[serde(rename = "Thread")]
    Thread(ThreadRecord),

    #[serde(rename = "Tool")]
    Tool(ToolRecord),

    #[serde(rename = "Tutorial")]
    Tutorial(TutorialRecord),

    #[serde(rename = "Wiki")]
    Wiki(WikiRecord),

    #[serde(rename = "Wip")]
    Wip(WipRecord),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct AppRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct ArticleRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct BlogRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct ClubRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct CommonRecord {
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
pub struct ConceptRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct ContestRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct EventRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct GameRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

    #[serde(rename = "_sAbbreviation")]
    pub abbreviation: String,

    #[serde(rename = "_sHomepage")]
    pub homepage: String,

    #[serde(rename = "_dsReleaseDate")]
    pub release_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct InitiativeRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

    #[serde(rename = "_sMissionStatement")]
    pub mission_statement: String,

    #[serde(rename = "_aGoals")]
    pub goals: Vec<String>,

    #[serde(rename = "_dsCompletionDate")]
    pub completion_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JamRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct MemberRecord {
    #[serde(flatten)]
    common: CommonRecord,

    #[serde(rename = "_nPostCount")]
    post_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct ModelRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct NewsRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct PollRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct ProjectRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct QuestionRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct RequestRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct ReviewRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

    #[serde(rename = "_aTags")]
    pub tags: Vec<String>,

    #[serde(rename = "_bWasFeatured")]
    pub was_featured: bool,

    #[serde(rename = "_nViewCount")]
    pub view_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct SoundRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct SprayRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct StudioRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct ThreadRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct ToolRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct TutorialRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct WikiRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
pub struct WipRecord {
    #[serde(flatten)]
    pub common: CommonRecord,

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
