use std::{collections::HashMap, marker::PhantomData};

use anyhow::Error;

use crate::{
    GamebananaApi,
    api::{
        model::{record::AdvancedRecord, search_response::SearchResponse},
        submissions::model::section::Section,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Submission<'a, T: Section> {
    _phantom: PhantomData<T>,
    pub api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn submission<T: Section>(&'a self) -> Submission<'a, T> {
        Submission {
            _phantom: PhantomData,
            api: self,
        }
    }
}

impl<'a, T: Section> Submission<'a, T> {
    pub async fn index(
        &self,
        page: Option<u64>,
        per_page: Option<u64>,
        sort: Option<&str>,
        filters: Option<&[(&str, &str)]>,
    ) -> Result<SearchResponse<AdvancedRecord>, Error> {
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
}
