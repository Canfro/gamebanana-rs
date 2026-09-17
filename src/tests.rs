use crate::{GamebananaApiV11, api_v11::model::search::advanced::section::Section};

#[tokio::test]
async fn modificators() {
    let api = GamebananaApiV11::new().unwrap();
    let res = api.search().modificators(Section::Wip).await.unwrap();
}
