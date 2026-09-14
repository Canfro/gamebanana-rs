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
