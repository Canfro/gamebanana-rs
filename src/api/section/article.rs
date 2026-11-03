use anyhow::Error;

use crate::api::{
    model::response::Response,
    section::{
        common::Submission,
        model::{profile_page::ArticleProfilePage, section::ArticleSection},
    },
};

impl<'a> Submission<'a, ArticleSection> {
    pub async fn profile_page(&self) -> Result<Response<ArticleProfilePage>, Error> {
        let url = self
            .section
            .api
            .base_url
            .join(format!("Article/{}/ProfilePage", self.submission_id).as_str())?;

        let builder = self.section.api.client.get(url);

        self.section.api.execute(builder).await
    }
}

#[cfg(test)]
mod test {
    use crate::{GamebananaApi, api::section::model::section::ArticleSection};

    #[tokio::test]
    async fn profile_page() {
        let api = GamebananaApi::new().unwrap();

        for id in 100..200 {
            println!("{}", id);
            api.section::<ArticleSection>()
                .get_submission(id)
                .await
                .profile_page()
                .await
                .unwrap();
        }
    }
}
