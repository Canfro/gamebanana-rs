use anyhow::Error;

use crate::api::{
    model::{feed_sort::FeedSort, results_record::ResultsRecord, search_response::SearchResponse},
    section::{
        common::Section,
        model::{
            section::{AnySection, GameSection},
            top_record::TopRecord,
        },
    },
};

impl<'a> Section<'a, GameSection> {
    pub async fn subfeed(
        &self,
        id: u64,
        page: Option<u64>,
        sort: Option<FeedSort>,
        name: Option<&str>,
        model_inclusions: Option<&[AnySection]>,
        model_exclusions: Option<&[AnySection]>,
        tag_inclusions: Option<&[&str]>,
        tag_exclusions: Option<&[&str]>,
    ) -> Result<SearchResponse<ResultsRecord>, Error> {
        let url = self
            .api
            .base_url
            .join(format!("Game/{}/Subfeed", id).as_str())?;

        let mut builder = self.api.client.get(url);

        if let Some(page) = page {
            builder = builder.query(&[("_nPage", page)]);
        }
        if let Some(sort) = sort {
            builder = builder.query(&[("_sSort", sort.as_str())]);
        }
        if let Some(name) = name {
            builder = builder.query(&[("_sName", name)]);
        }
        if let Some(model_inclusions) = model_inclusions {
            let model_inclusions = model_inclusions
                .iter()
                .map(|s| format!("{:?}", s))
                .collect::<Vec<_>>()
                .join(",");

            builder = builder.query(&[("_csvModelInclusions", model_inclusions)]);
        }
        if let Some(model_exclusions) = model_exclusions {
            let model_exclusions = model_exclusions
                .iter()
                .map(|s| format!("{:?}", s))
                .collect::<Vec<_>>()
                .join(",");

            builder = builder.query(&[("_csvModelExclusions", model_exclusions)]);
        }
        if let Some(tag_inclusions) = tag_inclusions {
            let tag_inclusions = tag_inclusions.join(",");
            builder = builder.query(&[("_csvTagInclusions", tag_inclusions)]);
        }
        if let Some(tag_exclusions) = tag_exclusions {
            let tag_exclusions = tag_exclusions.join(",");
            builder = builder.query(&[("_csvTagExclusions", tag_exclusions)]);
        }

        self.api.execute(builder).await
    }

    pub async fn top_subs(&self, id: u64) -> Result<Vec<TopRecord>, Error> {
        let url = self
            .api
            .base_url
            .join(format!("Game/{}/TopSubs", id).as_str())?;

        let builder = self.api.client.get(url);

        self.api.execute(builder).await
    }
}
