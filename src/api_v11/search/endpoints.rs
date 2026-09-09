use crate::GamebananaApi;

#[derive(Debug, Clone, Copy)]
pub struct Search<'a> {
    api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn search(&'a self) -> Search<'a> {
        Search { api: self }
    }
}

impl<'a> Search<'a> {
    pub async fn advanced(&self) -> () {}

    pub async fn modificators(&self) -> () {}

    pub async fn suggestions(&self) -> () {}

    pub async fn games_by_name(&self) -> () {}

    pub async fn tags_by_text(&self) -> () {}

    pub async fn latest_all(&self) -> () {}

    pub async fn latest_game(&self) -> () {}

    pub async fn latest_section(&self) -> () {}

    pub async fn latest_member(&self) -> () {}

    pub async fn featured(&self) -> () {}

    pub async fn top(&self) -> () {}
}
