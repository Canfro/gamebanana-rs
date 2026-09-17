use anyhow::Error;

use crate::{
    GamebananaApiV11,
    api_v11::model::search::{
        advanced::{advanced_response::AdvancedResponse, field::Field, order::Order},
        modificators::modificators_response::ModificatorsResponse,
        section::Section,
    },
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
    pub async fn advanced(
        &self,
        search_string: &str,
        page: Option<u64>,
        per_page: Option<u64>,
        model_name: Option<Section>,
        order: Option<Order>,
        fields: Option<&[Field]>,
        game_row: Option<i64>,
    ) -> Result<AdvancedResponse, Error> {
        let url = self.api.base_url.join("Util/Search/Results")?;

        let mut builder = self
            .api
            .client
            .get(url)
            .query(&[("_sSearchString", search_string)]);

        if let Some(page) = page {
            builder = builder.query(&[("_nPage", page)]);
        }
        if let Some(per_page) = per_page {
            builder = builder.query(&[("_nPerpage", per_page)]);
        }
        if let Some(model_name) = model_name {
            builder = builder.query(&[("_sModelName", model_name.as_str())]);
        }
        if let Some(order) = order {
            builder = builder.query(&[("_sOrder", order.as_str())]);
        }
        if let Some(fields) = fields {
            let fields = fields
                .iter()
                .map(|f| f.as_str())
                .collect::<Vec<&str>>()
                .join(",");

            builder = builder.query(&[("_csvFields", fields)]);
        }
        if let Some(game_row) = game_row {
            builder = builder.query(&[("_idGameRow", game_row)]);
        }

        let request = builder.build()?;
        let response = self.api.client.execute(request).await?;
        let body = response.text().await?;

        let mut deserializer = serde_json::Deserializer::from_str(body.as_str());
        let res = serde_path_to_error::deserialize::<_, AdvancedResponse>(&mut deserializer);

        /*
        println!(
           "{}",
           serde_json::to_string_pretty(&serde_json::from_str::<Value>(body.as_str())?)?
        );
        */

        match res {
            Ok(response) => Ok(response),
            Err(e) => {
                println!("Path: {}", e.path());
                println!("Error: {}", e.inner());
                Err(e.into())
            }
        }
    }

    pub async fn modificators(&self, model_name: Section) -> Result<ModificatorsResponse, Error> {
        let url = self
            .api
            .base_url
            .join(format!("{}/ListFilterConfig", model_name.as_str()).as_str())?;

        let builder = self.api.client.get(url);
        let request = builder.build()?;
        let response = self.api.client.execute(request).await?;
        let body = response.text().await?;

        let mut deserializer = serde_json::Deserializer::from_str(body.as_str());
        let res = serde_path_to_error::deserialize::<_, ModificatorsResponse>(&mut deserializer);

        match res {
            Ok(response) => Ok(response),
            Err(e) => {
                println!("Path: {}", e.path());
                println!("Error: {}", e.inner());
                Err(e.into())
            }
        }
    }

    pub async fn suggestions(&self) {
        todo!()
    }

    pub async fn games_by_name(&self) {
        todo!()
    }

    pub async fn tags_by_text(&self) {
        todo!()
    }

    pub async fn latest_all(&self) {
        todo!()
    }

    pub async fn latest_game(&self) {
        todo!()
    }

    pub async fn latest_section(&self) {
        todo!()
    }

    pub async fn latest_member(&self) {
        todo!()
    }

    pub async fn featured(&self) {
        todo!()
    }

    pub async fn top(&self) {
        todo!()
    }
}
