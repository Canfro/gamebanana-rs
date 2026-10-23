use anyhow::Error;

use crate::api::{
    model::{
        generic_latest_sort::GenericLatestSort, record::AdvancedRecord,
        search_response::SearchResponse,
    },
    submissions::{
        common::Submission,
        model::{
            section::{Game, Section},
            top_record::TopRecord,
        },
    },
};

impl<'a> Submission<'a, Game> {
    pub async fn subfeed<I: Section, E: Section>(
        &self,
        id: u64,
        page: Option<u64>,
        sort: Option<GenericLatestSort>,
        name: Option<&str>,
        model_inclusions: Option<&[I]>,
        model_exclusions: Option<&[E]>,
        tag_inclusions: Option<&[&str]>,
        tag_exclusions: Option<&[&str]>,
    ) -> Result<SearchResponse<AdvancedRecord>, Error> {
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
                .map(|_| I::as_str())
                .collect::<Vec<&str>>()
                .join(",");

            builder = builder.query(&[("_csvModelInclusions", model_inclusions)]);
        }
        if let Some(model_exclusions) = model_exclusions {
            let model_exclusions = model_exclusions
                .iter()
                .map(|_| E::as_str())
                .collect::<Vec<&str>>()
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
