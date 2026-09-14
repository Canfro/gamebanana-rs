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
