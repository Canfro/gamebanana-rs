use crate::{
    GamebananaApiV11,
    api_v11::endpoints::search::model::{order::Order, section::Section},
};

#[tokio::test]
async fn test_advanced_search() {
    let api = GamebananaApiV11::new();

    assert!(api.is_ok());
    let api = api.unwrap();

    let response = api
        .search()
        .advanced(
            "the",
            Some(1),
            Some(50),
            Some(Section::Idea),
            Some(Order::BestMatch),
            None,
            None,
        )
        .await;

    assert!(response.is_ok());
}
