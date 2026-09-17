use strum::IntoEnumIterator;

use crate::{
    GamebananaApiV11,
    api_v11::model::search::{
        advanced::{field::Field, order::Order},
        section::Section,
    },
};

#[tokio::test]
async fn advanced_search() {
    for section in Section::iter() {
        let api = GamebananaApiV11::new().unwrap();
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
        let api = GamebananaApiV11::new().unwrap();
        let res = api.search().modificators(section).await.unwrap();
    }
}
