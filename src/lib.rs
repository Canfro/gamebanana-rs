use std::str::FromStr;

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use crate::error::Error;

mod api_v12;
pub(crate) mod error;

#[derive(Debug, Clone)]
pub struct GamebananaApi {
    url_v12: Url,
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Response<'a, T> {
    status: &'a str,
    data: T,
}

impl<'a> GamebananaApi {
    pub fn new() -> Result<GamebananaApi, Error> {
        Ok(GamebananaApi {
            url_v12: Url::from_str("https://gamebanana.com/apiv12").map_err(|_| Error::UrlError)?,
            client: Client::new(),
        })
    }
}
