use crate::{
    GamebananaApiV11, api_v11::endpoints::search::model::advanced::AdvancedResponse, error::Error,
};

#[derive(Debug, Clone, Copy)]
pub struct Search<'a> {
    api: &'a GamebananaApiV11,
}

impl<'a> GamebananaApiV11 {
    pub fn search(&'a self) -> Search<'a> {
        Search { api: self }
    }
}

impl<'a> Search<'a> {
    pub async fn advanced(&self) -> Result<AdvancedResponse, Error> {
        todo!()
    }

    pub async fn modificators(&self) -> Result<Response, Error> {
        todo!()
    }

    pub async fn suggestions(&self) -> Result<Response, Error> {
        todo!()
    }

    pub async fn games_by_name(&self) -> Result<Response, Error> {
        todo!()
    }

    pub async fn tags_by_text(&self) -> Result<Response, Error> {
        todo!()
    }

    pub async fn latest_all(&self) -> Result<Response, Error> {
        todo!()
    }

    pub async fn latest_game(&self) -> Result<Response, Error> {
        todo!()
    }

    pub async fn latest_section(&self) -> Result<Response, Error> {
        todo!()
    }

    pub async fn latest_member(&self) -> Result<Response, Error> {
        todo!()
    }

    pub async fn featured(&self) -> Result<Response, Error> {
        todo!()
    }

    pub async fn top(&self) -> Result<Response, Error> {
        todo!()
    }
}
