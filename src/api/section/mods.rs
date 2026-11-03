use anyhow::Error;

use crate::api::{
    model::response::Response,
    section::{
        common::Submission,
        model::{profile_page::ModProfilePage, section::ModSection},
    },
};

impl<'a> Submission<'a, ModSection> {
    pub async fn profile_page(&self) -> Result<Response<ModProfilePage>, Error> {
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
    use crate::{GamebananaApi, api::section::model::section::ModSection};

    #[tokio::test]
    async fn profile_page() {
        let api = GamebananaApi::new().unwrap();

        for id in 10000..11000 {
            println!("{}", id);
            api.section::<ModSection>()
                .get_submission(id)
                .await
                .profile_page()
                .await
                .unwrap();
        }
    }
}
