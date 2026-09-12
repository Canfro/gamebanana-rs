use crate::{
    GamebananaApiV11,
    api_v11::endpoints::search::model::advanced_params::{Order, Section},
};

#[tokio::test]
async fn test_advanced_search() {
    let api = GamebananaApiV11::new();

    assert!(api.is_ok());
    let api = api.unwrap();

    let response = api
        .search()
        .advanced(
            "fix",
            None,
            Some(15),
            Some(Section::Mod),
            Some(Order::BestMatch),
            None,
            None,
        )
        .await;

    assert!(response.is_ok());
}
