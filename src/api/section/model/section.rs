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
    Collection,
    Contest,
    Concept,
    Event,
    File,
    Game,
    Idea,
    Initiative,
    Issue,
    Jam,
    Mod,
    ModCategory,
    Model,
    Member,
    News,
    Poll,
    Post,
    Project,
    Question,
    Review,
    Request,
    Ripe,
    Script,
    Sound,
    Spray,
    Subscription,
    Studio,
    Thread,
    Todo,
    Tool,
    Tutorial,
    Update,
    Wiki,
    Wip,
}

macro_rules! define_sections {
    ($($name:ident,)*) => {
        paste::paste! {
            $(
                #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
                #[serde(deny_unknown_fields)]
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
    App,
    Article,
    Bug,
    Blog,
    Collection,
    Club,
    Contest,
    Concept,
    Event,
    File,
    Game,
    Idea,
    Initiative,
    Issue,
    Jam,
    Mod,
    ModCategory,
    Model,
    Member,
    News,
    Poll,
    Post,
    Project,
    Question,
    Review,
    Request,
    Script,
    Sound,
    Spray,
    Subscription,
    Studio,
    Thread,
    Todo,
    Tool,
    Tutorial,
    Update,
    Wiki,
    Wip,
);
