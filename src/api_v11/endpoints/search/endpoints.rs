use crate::{
    GamebananaApiV11,
    api_v11::endpoints::search::model::{
        advanced_params::{Field, Order, Section},
        advanced_response::AdvancedResponse,
    },
    error::Error,
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
        let url = self
            .api
            .base_url
            .join("Util/Search/Results")
            .map_err(|_| Error::UrlError)?;

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
            builder = builder.query(&[(
                "_csvFields",
                fields.iter().map(|f| f.as_str()).collect::<Vec<&str>>(),
            )]);
        }
        if let Some(game_row) = game_row {
            builder = builder.query(&[("_idGameRow", game_row)]);
        }

        let response = builder.send().await.map_err(|_| Error::FailedToSend)?;
        let body = response.text().await.unwrap();

        println!(
            "{}",
            serde_json::to_string_pretty(
                &serde_json::from_str::<serde_json::Value>(body.as_str()).unwrap()
            )
            .unwrap()
        );

        Ok(serde_json::from_str(body.as_str()).unwrap())
    }

    pub async fn modificators(&self) {
        todo!()
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
