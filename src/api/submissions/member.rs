use anyhow::Error;

use crate::api::{
    model::{record::AdvancedRecord, search_response::SearchResponse},
    submissions::{common::Submission, model::section::Member},
};

impl<'a> Submission<'a, Member> {
    pub async fn subfeed(
        &self,
        id: u64,
        page: Option<u64>,
        per_page: Option<u64>,
    ) -> Result<SearchResponse<AdvancedRecord>, Error> {
        let url = self
            .api
            .base_url
            .join(format!("Member/{}/SubFeed", id).as_str())?;

        let mut builder = self.api.client.get(url);

        if let Some(page) = page {
            builder = builder.query(&[("_nPage", page)]);
        }
        if let Some(per_page) = per_page {
            builder = builder.query(&[("_nPerpage", per_page)]);
        }

        self.api.execute(builder).await
    }
}
