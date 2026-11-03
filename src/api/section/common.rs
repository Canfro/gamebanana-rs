use std::{collections::HashMap, marker::PhantomData};

use anyhow::Error;

use crate::{
    GamebananaApi,
    api::{
        model::{
            field::Field, response::Response, results_record::ResultsRecord,
            results_sort::ResultsSort, search_response::SearchResponse,
        },
        section::model::{
            category_count::CategoryCount, category_sort::CategorySort,
            modificators_response::ModificatorsResponse, section::SectionTrait,
        },
    },
};

impl<'a> GamebananaApi {
    pub fn section<T: SectionTrait<'a>>(&'a self) -> Section<'a, T> {
        Section {
            _phantom: PhantomData,
            api: self,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Section<'a, T: SectionTrait<'a>> {
    _phantom: PhantomData<T>,
    pub api: &'a GamebananaApi,
}

impl<'a, T: SectionTrait<'a>> Section<'a, T> {
    pub async fn results(
        &self,
        search_string: &str,
        page: Option<u64>,
        per_page: Option<u64>,
        order: Option<ResultsSort>,
        fields: Option<&[Field]>,
        game_row: Option<u64>,
    ) -> Result<Response<SearchResponse<ResultsRecord>>, Error> {
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
    ) -> Result<Response<Vec<String>>, Error> {
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
    ) -> Result<Response<SearchResponse<ResultsRecord>>, Error> {
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
    ) -> Result<Response<SearchResponse<ResultsRecord>>, Error> {
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
    ) -> Result<Response<Vec<CategoryCount>>, Error> {
        let url = self
            .api
            .base_url
            .join(format!("{}/Categories", T::as_str()).as_str())?;

        let mut builder = self.api.client.get(url).query(&[("_sSort", sort.as_str())]);

        if let Some(show_empty) = show_empty {
            builder = builder.query(&[("_bShowEmpty", show_empty)]);
        }
        if let Some(game_row) = game_row {
            builder = builder.query(&[("_idGameRow", game_row)]);
        }
        if let Some(category_row) = category_row {
            builder = builder.query(&[("_idCategoryRow", category_row)]);
        }

        self.api.execute(builder).await
    }

    pub async fn get_submission(&'a self, submission_id: u64) -> Submission<'a, T> {
        Submission {
            section: self,
            submission_id,
        }
    }
}

pub struct Submission<'a, T: SectionTrait<'a>> {
    pub submission_id: u64,
    pub section: &'a Section<'a, T>,
}

impl<'a, T: SectionTrait<'a>> Submission<'a, T> {
    pub async fn download_page(&self) {
        todo!()
    }

    pub async fn collections(&self) {
        todo!()
    }

    pub async fn accessor_collections(&self) {
        todo!()
    }

    pub async fn add_to_collection(&self) {
        todo!()
    }

    pub async fn remove_from_collection(&self) {
        todo!()
    }

    pub async fn embeddables_page(&self) {
        todo!()
    }

    pub async fn issues(&self) {
        todo!()
    }

    pub async fn add_issue(&self) {
        todo!()
    }

    pub async fn likes(&self) {
        todo!()
    }

    pub async fn like(&self) {
        todo!()
    }

    pub async fn posts(&self) {
        todo!()
    }

    pub async fn add_post(&self) {
        todo!()
    }

    pub async fn subscribers(&self) {
        todo!()
    }

    pub async fn add_subscription(&self) {
        todo!()
    }

    pub async fn thanks(&self) {
        todo!()
    }

    pub async fn add_thanks(&self) {
        todo!()
    }

    pub async fn todos(&self) {
        todo!()
    }

    pub async fn add_todo(&self) {
        todo!()
    }

    pub async fn updates(&self) {
        todo!()
    }

    pub async fn files(&self) {
        todo!()
    }

    pub async fn update(&self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        GamebananaApi,
        api::{
            model::{field::Field, results_sort::ResultsSort},
            section::model::{category_sort::CategorySort, section::ModSection},
        },
    };

    #[tokio::test]
    async fn results() {
        let api = GamebananaApi::new().unwrap();

        for page in 1..101 {
            api.section::<ModSection>()
                .results(
                    "the",
                    Some(page),
                    Some(50),
                    Some(ResultsSort::BestMatch),
                    Some(&[
                        Field::Name,
                        Field::Description,
                        Field::Article,
                        Field::Attribs,
                        Field::Studio,
                        Field::Owner,
                        Field::Credits,
                    ]),
                    None,
                )
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    async fn suggestions() {
        let api = GamebananaApi::new().unwrap();

        api.section::<ModSection>()
            .suggestions("the", None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn featured() {
        let api = GamebananaApi::new().unwrap();

        for page in 1..101 {
            api.section::<ModSection>()
                .featured(Some(page), Some(50), None)
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    async fn list_filter_config() {
        let api = GamebananaApi::new().unwrap();

        api.section::<ModSection>()
            .list_filter_config()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn index() {
        let api = GamebananaApi::new().unwrap();

        for page in 1..101 {
            api.section::<ModSection>()
                .index(Some(page), Some(50), None, None)
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    async fn categories() {
        let api = GamebananaApi::new().unwrap();

        api.section::<ModSection>()
            .categories(CategorySort::AtoZ, None, None, None)
            .await
            .unwrap();
    }
}
