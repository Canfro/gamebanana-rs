use std::{collections::HashMap, marker::PhantomData};

use anyhow::Error;

use crate::{
    GamebananaApi,
    api::{
        model::{
            field::Field, order::Order, results_record::ResultsRecord,
            search_response::SearchResponse,
        },
        section::model::{modificators_response::ModificatorsResponse, section::SectionTrait},
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Section<'a, T: SectionTrait<'a>> {
    _phantom: PhantomData<T>,
    pub api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn section<T: SectionTrait<'a>>(&'a self) -> Section<'a, T> {
        Section {
            _phantom: PhantomData,
            api: self,
        }
    }
}

impl<'a, T: SectionTrait<'a>> Section<'a, T> {
    pub async fn results(
        &self,
        search_string: &str,
        page: Option<u64>,
        per_page: Option<u64>,
        order: Option<Order>,
        fields: Option<&[Field]>,
        game_row: Option<u64>,
    ) -> Result<SearchResponse<ResultsRecord>, Error> {
        let url = self.api.base_url.join("Util/Search/Results")?;

        let mut builder = self.api.client.get(url).query(&[
            ("_sSearchString", search_string),
            ("_sModelName", T::as_str()),
        ]);

        if let Some(page) = page {
            builder = builder.query(&[("_nPage", page)]);
        }
        if let Some(per_page) = per_page {
            builder = builder.query(&[("_nPerpage", per_page)]);
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

    pub async fn suggestions(
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

    pub async fn featured(
        &self,
        page: Option<u64>,
        per_page: Option<u64>,
        game_row: Option<u64>,
    ) -> Result<SearchResponse<ResultsRecord>, Error> {
        let url = self.api.base_url.join("Util/List/Featured")?;

        let mut builder = self
            .api
            .client
            .get(url)
            .query(&[("_sModelName", T::as_str())]);

        if let Some(page) = page {
            builder = builder.query(&[("_nPage", page)]);
        }
        if let Some(per_page) = per_page {
            builder = builder.query(&[("_nPerpage", per_page)]);
        }
        if let Some(game_row) = game_row {
            builder = builder.query(&[("_idGameRow", game_row)]);
        }

        self.api.execute(builder).await
    }

    pub async fn list_filter_config(&self) -> Result<ModificatorsResponse, Error> {
        let url = self
            .api
            .base_url
            .join(format!("{}/ListFilterConfig", T::as_str()).as_str())?;

        let builder = self.api.client.get(url);

        self.api.execute(builder).await
    }

    pub async fn index(
        &self,
        page: Option<u64>,
        per_page: Option<u64>,
        sort: Option<&str>,
        filters: Option<&[(&str, &str)]>,
    ) -> Result<SearchResponse<ResultsRecord>, Error> {
        let url = self
            .api
            .base_url
            .join(format!("{}/Index", T::as_str()).as_str())?;

        let mut builder = self.api.client.get(url);

        if let Some(page) = page {
            builder = builder.query(&[("_nPage", page)]);
        }
        if let Some(per_page) = per_page {
            builder = builder.query(&[("_nPerpage", per_page)]);
        }
        if let Some(sort) = sort {
            builder = builder.query(&[("_sSort", sort)]);
        }
        if let Some(filters) = filters {
            let filters = filters
                .iter()
                .map(|(k, v)| (format!("_aFilters[{}]", k), *v))
                .collect::<HashMap<String, &str>>();

            builder = builder.query(&filters);
        }

        self.api.execute(builder).await
    }

    pub async fn categories(
        &self,
        sort: CategorySort,
        show_empty: Option<bool>,
        game_row: Option<u64>,
        category_row: Option<u64>,
    ) -> Result<, Error> {
        todo!()
    }
}
