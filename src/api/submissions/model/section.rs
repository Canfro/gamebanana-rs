use serde::{Deserialize, Serialize};

pub trait Section {
    fn as_str() -> &'static str;
}

macro_rules! define_sections {
    ($($name:ident),* $(,)?) => {
        $(
            #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
            pub struct $name;

            impl Section for $name {
                fn as_str() -> &'static str {
                    stringify!($name)
                }
            }
        )*
    };
}

define_sections!(
    App, Article, Bug, Blog, Club, Contest, Concept, Event, Game, Idea, Initiative, Jam, Mod,
    Model, Member, News, Poll, Project, Question, Review, Request, Script, Sound, Spray, Studio,
    Thread, Tool, Tutorial, Wiki, Wip,
);
