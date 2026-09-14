use serde::{Deserialize, Serialize};

use crate::api_v11::endpoints::search::model::{
    advanced_app_record::AdvancedAppRecord, advanced_article_record::AdvancedArticleRecord,
    advanced_blog_record::AdvancedBlogRecord, advanced_bug_record::AdvancedBugRecord,
    advanced_club_record::AdvancedClubRecord, advanced_concept_record::AdvancedConceptRecord,
    advanced_contest_record::AdvancedContestRecord, advanced_event_record::AdvancedEventRecord,
    advanced_game_record::AdvancedGameRecord, advanced_idea_record::AdvancedIdeaRecord,
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
}
