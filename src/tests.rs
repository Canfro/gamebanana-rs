use strum::IntoEnumIterator;

use crate::{
    GamebananaApi,
    api::model::search::{
        advanced::{field::Field, order::Order},
        latest_all::latest_sort::LatestSort,
        section::Section,
    },
};

#[tokio::test]
async fn advanced_search() {
    for section in Section::iter() {
        for page in 1..6 {
            println!("Page: {}", page);
            println!("Section: {}", section.as_str());
            let api = GamebananaApi::new().unwrap();
            api.search()
                .advanced(
                    "the",
                    Some(page),
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
}

#[tokio::test]
async fn modificators() {
    for section in Section::iter() {
        let api = GamebananaApi::new().unwrap();
        api.search().modificators(section).await.unwrap();
    }
}

#[tokio::test]
async fn suggestions() {
    for section in Section::iter() {
        let api = GamebananaApi::new().unwrap();
        api.search()
            .suggestions("the", section, None)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn games_by_name() {
    for page in 1..6 {
        let api = GamebananaApi::new().unwrap();
        api.search()
            .games_by_name("the", Some(page), Some(50))
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn tags_by_text() {
    let api = GamebananaApi::new().unwrap();
    api.search().tags_by_text("the", None).await.unwrap();
}

#[tokio::test]
async fn latest_all() {
    for page in 1..6 {
        let api = GamebananaApi::new().unwrap();
        api.search()
            .latest_all(Some(page), Some(50), Some(LatestSort::New), Some(false))
            .await
            .unwrap();
    }
}
