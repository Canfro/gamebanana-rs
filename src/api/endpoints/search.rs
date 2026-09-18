use anyhow::Error;

use crate::{
    GamebananaApi,
    api::model::search::{
        advanced::{advanced_record::AdvancedRecord, field::Field, order::Order},
        game::Game,
        modificators::modificators_response::ModificatorsResponse,
        search_response::SearchResponse,
        section::Section,
        tags_by_text::tag::Tag,
    },
};

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
    pub async fn advanced(
        &self,
        search_string: &str,
        page: Option<u64>,
        per_page: Option<u64>,
        model_name: Option<Section>,
        order: Option<Order>,
        fields: Option<&[Field]>,
        game_row: Option<i64>,
    ) -> Result<SearchResponse<AdvancedRecord>, Error> {
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

        self.api.execute(builder).await
    }

    pub async fn modificators(&self, model_name: Section) -> Result<ModificatorsResponse, Error> {
        let url = self
            .api
            .base_url
            .join(format!("{}/ListFilterConfig", model_name.as_str()).as_str())?;

        let builder = self.api.client.get(url);

        self.api.execute(builder).await
    }

    pub async fn suggestions(
        &self,
        search_string: &str,
        model_name: Section,
        game_row: Option<i64>,
    ) -> Result<Vec<String>, Error> {
        let url = self.api.base_url.join("Util/Search/Suggestions")?;

        let mut builder = self.api.client.get(url).query(&[
            ("_sSearchString", search_string),
            ("_sModelName", model_name.as_str()),
        ]);

        if let Some(game_row) = game_row {
            builder = builder.query(&[("_idGameRow", game_row)]);
        }

        self.api.execute(builder).await
    }

    pub async fn games_by_name(
        &self,
        name: &str,
        page: Option<u64>,
        per_page: Option<u64>,
    ) -> Result<SearchResponse<Game>, Error> {
        let url = self.api.base_url.join("Util/Game/NameMatch")?;

        let mut builder = self.api.client.get(url).query(&[("_sName", name)]);

        if let Some(page) = page {
            builder = builder.query(&[("_nPage", page)]);
        }
        if let Some(per_page) = per_page {
            builder = builder.query(&[("_nPerpage", per_page)]);
        }

        self.api.execute(builder).await
    }

    pub async fn tags_by_text(
        &self,
        tag: &str,
        game_row: Option<i64>,
    ) -> Result<SearchResponse<Tag>, Error> {
        let url = self.api.base_url.join("Util/Generic/Tags")?;

        let mut builder = self.api.client.get(url).query(&[("_sTag", tag)]);

        if let Some(game_row) = game_row {
            builder = builder.query(&[("_idGameRow", game_row)]);
        }

        self.api.execute(builder).await
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
