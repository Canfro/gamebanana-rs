use anyhow::Error;
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use std::str::FromStr;

pub mod api;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone)]
pub struct GamebananaApi {
    base_url: Url,
    client: Client,
}

impl GamebananaApi {
    pub fn new() -> Result<GamebananaApi, Error> {
        Ok(GamebananaApi {
            base_url: Url::from_str("https://gamebanana.com/apiv11/")?,
            client: Client::new(),
        })
    }

    async fn execute<T: DeserializeOwned>(&self, builder: RequestBuilder) -> Result<T, Error> {
        let request = builder.build()?;
        let response = self.client.execute(request).await?;
        let body = response.text().await?;

        let mut deserializer = serde_json::Deserializer::from_str(body.as_str());
        let result = serde_path_to_error::deserialize::<_, T>(&mut deserializer);

        match result {
            Ok(response) => Ok(response),
            Err(e) => {
                println!("Path: {}", e.path());
                println!("Error: {}", e.inner());
                Err(e.into())
            }
        }
    }
}
