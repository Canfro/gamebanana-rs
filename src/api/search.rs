use anyhow::Error;

use crate::{
    GamebananaApi,
    api::{
        model::{
            field::Field, game::Game, generic_latest_sort::GenericLatestSort,
            modificators_response::ModificatorsResponse, order::Order, record::AdvancedRecord,
            search_response::SearchResponse, tag::Tag,
        },
        submissions::model::section::Section,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Search<'a> {
    pub api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn search(&'a self) -> Search<'a> {
        Search { api: self }
    }
}

impl<'a> Search<'a> {
    pub async fn results<T: Section>(
        &self,
        search_string: &str,
        page: Option<u64>,
        per_page: Option<u64>,
        model_name: Option<T>,
        order: Option<Order>,
        fields: Option<&[Field]>,
        game_row: Option<u64>,
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
        if model_name.is_some() {
            builder = builder.query(&[("_sModelName", T::as_str())]);
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

    pub async fn list_filter_config<T: Section>(&self) -> Result<ModificatorsResponse, Error> {
        let url = self
            .api
            .base_url
            .join(format!("{}/ListFilterConfig", T::as_str()).as_str())?;

        let builder = self.api.client.get(url);

        self.api.execute(builder).await
    }

    pub async fn suggestions<T: Section>(
        &self,
        search_string: &str,
        game_row: Option<u64>,
    ) -> Result<Vec<String>, Error> {
        let url = self.api.base_url.join("Util/Search/Suggestions")?;

        let mut builder = self.api.client.get(url).query(&[
            ("_sSearchString", search_string),
            ("_sModelName", T::as_str()),
        ]);

        if let Some(game_row) = game_row {
            builder = builder.query(&[("_idGameRow", game_row)]);
        }

        self.api.execute(builder).await
    }

    pub async fn game_name_match(
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

    pub async fn tags(
        &self,
        tag: &str,
        game_row: Option<u64>,
    ) -> Result<SearchResponse<Tag>, Error> {
        let url = self.api.base_url.join("Util/Generic/Tags")?;

        let mut builder = self.api.client.get(url).query(&[("_sTag", tag)]);

        if let Some(game_row) = game_row {
            builder = builder.query(&[("_idGameRow", game_row)]);
        }

        self.api.execute(builder).await
    }

    pub async fn latest_submissions(
        &self,
        page: Option<u64>,
        per_page: Option<u64>,
        sort: Option<GenericLatestSort>,
        filter_by_subscribed_games: Option<bool>,
    ) -> Result<SearchResponse<AdvancedRecord>, Error> {
        let url = self.api.base_url.join("Util/Homepage/Submissions")?;

        let mut builder = self.api.client.get(url);

        if let Some(page) = page {
            builder = builder.query(&[("_nPage", page)]);
        }
        if let Some(per_page) = per_page {
            builder = builder.query(&[("_nPerpage", per_page)]);
        }
        if let Some(sort) = sort {
            builder = builder.query(&[("_sSort", sort.as_str())]);
        }
        if let Some(filter_by_subscribed_games) = filter_by_subscribed_games {
            builder = builder.query(&[("_bFilterBySubscribedGames", filter_by_subscribed_games)]);
        }

        self.api.execute(builder).await
    }

    pub async fn featured<T: Section>(
        &self,
        page: Option<u64>,
        per_page: Option<u64>,
        model_name: Option<T>,
        game_row: Option<u64>,
    ) -> Result<SearchResponse<AdvancedRecord>, Error> {
        let url = self.api.base_url.join("Util/List/Featured")?;

        let mut builder = self.api.client.get(url);

        if let Some(page) = page {
            builder = builder.query(&[("_nPage", page)]);
        }
        if let Some(per_page) = per_page {
            builder = builder.query(&[("_nPerpage", per_page)]);
        }
        if model_name.is_some() {
            builder = builder.query(&[("_sModelName", T::as_str())]);
        }
        if let Some(game_row) = game_row {
            builder = builder.query(&[("_idGameRow", game_row)]);
        }

        self.api.execute(builder).await
    }
}
