use crate::error::Error;
use reqwest::{Client, Url};
use std::str::FromStr;

pub mod api_v11;
pub mod error;

#[derive(Debug, Clone)]
pub struct GamebananaApi {
    base_url_v11: Url,
    client: Client,
}

impl GamebananaApi {
    pub fn new() -> Result<GamebananaApi, Error> {
        Ok(GamebananaApi {
            base_url_v11: Url::from_str("https://gamebanana.com/apiv11")
                .map_err(|_| Error::UrlError)?,
            client: Client::new(),
        })
    }
}
