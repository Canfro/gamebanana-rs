pub enum Section {
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

impl Section {
    pub fn as_str(&self) -> &str {
        match self {
            Section::App => "App",
            Section::Article => "Article",
            Section::Bug => "Bug",
            Section::Blog => "Blog",
            Section::Club => "Club",
            Section::Contest => "Contest",
            Section::Concept => "Concept",
            Section::Event => "Event",
            Section::Game => "Game",
            Section::Idea => "Idea",
            Section::Initiative => "Initiative",
            Section::Jam => "Jam",
            Section::Mod => "Mod",
            Section::Model => "Model",
            Section::Member => "Member",
            Section::News => "News",
            Section::Poll => "Poll",
            Section::Project => "Project",
            Section::Question => "Question",
            Section::Review => "Review",
            Section::Request => "Request",
            Section::Script => "Script",
            Section::Sound => "Sound",
            Section::Spray => "Spray",
            Section::Studio => "Studio",
            Section::Thread => "Thread",
            Section::Tool => "Tool",
            Section::Tutorial => "Tutorial",
            Section::Wiki => "Wiki",
            Section::Wip => "Wip",
        }
    }
}

pub enum Order {
    BestMatch,
    Popularity,
    Date,
    Update,
}

impl Order {
    pub fn as_str(&self) -> &str {
        match self {
            Order::BestMatch => "best_match",
            Order::Popularity => "popularity",
            Order::Date => "date",
            Order::Update => "Update",
        }
    }
}

pub enum Field {
    Name,
    Description,
    Article,
    Attribs,
    Studio,
    Owner,
    Credits,
}

impl Field {
    pub fn as_str(&self) -> &str {
        match self {
            Field::Name => "name",
            Field::Description => "description",
            Field::Article => "article",
            Field::Attribs => "attribs",
            Field::Studio => "studio",
            Field::Owner => "owner",
            Field::Credits => "credits",
        }
    }
}
