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
            .join(format!("Mod/{}/ProfilePage", self.submission_id).as_str())?;

        let builder = self.section.api.client.get(url);

        self.section.api.execute(builder).await
    }
}

#[cfg(test)]
mod test {
    use crate::{
        GamebananaApi,
        api::{model::response::Response, section::model::section::ArticleSection},
    };

    #[tokio::test]
    async fn profile_page() {
        let api = GamebananaApi::new().unwrap();

        for id in 0..100 {
            println!("{}", id);
            let r = api
                .section::<ArticleSection>()
                .get_submission(id)
                .await
                .profile_page()
                .await
                .unwrap();

            if let Response::Ok(r) = r {
                println!("{:?}", r);
            } else if let Response::Err {
                error_code,
                error_message,
                ..
            } = r
            {
                println!("{}", error_code);
                println!("{}", error_message);
            }
        }
    }
}
