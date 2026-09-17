use serde::{Deserialize, Serialize};

use crate::api_v11::model::search::advanced::{
    advanced_app_record::AdvancedAppRecord, advanced_article_record::AdvancedArticleRecord,
    advanced_blog_record::AdvancedBlogRecord, advanced_bug_record::AdvancedBugRecord,
    advanced_club_record::AdvancedClubRecord, advanced_concept_record::AdvancedConceptRecord,
    advanced_contest_record::AdvancedContestRecord, advanced_event_record::AdvancedEventRecord,
    advanced_game_record::AdvancedGameRecord, advanced_idea_record::AdvancedIdeaRecord,
    advanced_initiative_record::AdvancedInitiativeRecord, advanced_jam_record::AdvancedJamRecord,
    advanced_mod_record::AdvancedModRecord, advanced_model_record::AdvancedModelRecord,
    advanced_news_record::AdvancedNewsRecord, advanced_poll_record::AdvancedPollRecord,
    advanced_project_record::AdvancedProjectRecord,
    advanced_question_record::AdvancedQuestionRecord,
    advanced_request_record::AdvancedRequestRecord, advanced_review_record::AdvancedReviewRecord,
    advanced_script_record::AdvancedScriptRecord, advanced_sound_record::AdvancedSoundRecord,
    advanced_spray_record::AdvancedSprayRecord, advanced_studio_record::AdvancedStudioRecord,
    advanced_thread_record::AdvancedThreadRecord, advanced_tool_record::AdvancedToolRecord,
    advanced_tutorial_record::AdvancedTutorialRecord, advanced_wiki_record::AdvancedWikiRecord,
    advanced_wip_record::AdvancedWipRecord,
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
