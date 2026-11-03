use anyhow::Error;

use crate::api::{
    model::response::Response,
    section::{
        common::Submission,
        model::{profile_page::AppProfilePage, section::AppSection},
    },
};

impl<'a> Submission<'a, AppSection> {
    pub async fn profile_page(&self) -> Result<Response<AppProfilePage>, Error> {
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
    use crate::{GamebananaApi, api::section::model::section::AppSection};

    #[tokio::test]
    async fn profile_page() {
        let api = GamebananaApi::new().unwrap();

        for id in 2..1000 {
            println!("{}", id);
            api.section::<AppSection>()
                .get_submission(id)
                .await
                .profile_page()
                .await
                .unwrap();
        }
    }
}
