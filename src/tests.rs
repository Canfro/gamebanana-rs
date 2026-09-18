use strum::IntoEnumIterator;

use crate::{
    GamebananaApi,
    api::model::search::{
        advanced::{field::Field, order::Order},
        section::Section,
    },
};

#[tokio::test]
async fn advanced_search() {
    for section in Section::iter() {
        let api = GamebananaApi::new().unwrap();
        let res = api
            .search()
            .advanced(
                "the",
                Some(1),
                Some(50),
                Some(section),
                Some(Order::BestMatch),
                Some(&[
                    Field::Studio,
                    Field::Name,
                    Field::Description,
                    Field::Article,
                    Field::Attribs,
                    Field::Owner,
                    Field::Credits,
                ]),
                None,
            )
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn modificators() {
    for section in Section::iter() {
        let api = GamebananaApi::new().unwrap();
        let res = api.search().modificators(section).await.unwrap();
    }
}

#[tokio::test]
async fn suggestions() {
    for section in Section::iter() {
        let api = GamebananaApi::new().unwrap();
        let res = api
            .search()
            .suggestions("the", section, None)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn games_by_name() {
    let api = GamebananaApi::new().unwrap();
    let res = api
        .search()
        .games_by_name("the", Some(1), Some(50))
        .await
        .unwrap();
}
