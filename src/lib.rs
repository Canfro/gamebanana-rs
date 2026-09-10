use crate::error::Error;
use reqwest::{Client, Url};
use std::str::FromStr;

pub mod api_v11;
pub(crate) mod error;

#[derive(Debug, Clone)]
pub struct GamebananaApiV11 {
    base_url: Url,
    client: Client,
}

impl GamebananaApiV11 {
    pub fn new() -> Result<GamebananaApiV11, Error> {
        Ok(GamebananaApiV11 {
            base_url: Url::from_str("https://gamebanana.com/apiv11")
                .map_err(|_| Error::UrlError)?,
            client: Client::new(),
        })
    }
}
