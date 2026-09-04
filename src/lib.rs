use std::str::FromStr;

use reqwest::Url;

use crate::error::Error;

mod api_v12;
pub(crate) mod error;

pub struct GamebananaApi {
    main_url: Url,
}

impl GamebananaApi {
    pub fn new() -> Result<GamebananaApi, Error> {
        Ok(GamebananaApi {
            main_url: Url::from_str("https://gamebanana.com/apiv12")
                .map_err(|_| Error::UrlError)?,
        })
    }
}
