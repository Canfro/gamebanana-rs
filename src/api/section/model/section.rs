use serde::{Deserialize, Serialize};

pub trait SectionTrait<'a> {
    fn as_str() -> &'a str;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnySection {
    App,
    Article,
    Bug,
    Blog,
    Club,
    Contest,
    Concept,
    Event,
    Game,
    Idea,
    Initiative,
    Jam,
    Mod,
    Model,
    Member,
    News,
    Poll,
    Project,
    Question,
    Review,
    Request,
    Script,
    Sound,
    Spray,
    Studio,
    Thread,
    Tool,
    Tutorial,
    Wiki,
    Wip,
}

macro_rules! define_sections {
    ($($name:ident,)*) => {
        paste::paste! {
            $(
                #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
                pub struct [<$name Section>];

                impl<'a> SectionTrait<'a> for [<$name Section>] {
                    fn as_str() -> &'a str {
                        stringify!($name)
                    }
                }
            )*
        }
    };
}

define_sections!(
    App, Article, Bug, Blog, Club, Contest, Concept, Event, Game, Idea, Initiative, Jam, Mod,
    Model, Member, News, Poll, Project, Question, Review, Request, Script, Sound, Spray, Studio,
    Thread, Tool, Tutorial, Wiki, Wip,
);
