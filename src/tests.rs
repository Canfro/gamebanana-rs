use strum::IntoEnumIterator;

use crate::{
    GamebananaApi,
    api::model::search::{
        advanced::{field::Field, order::Order},
        latest::generic_latest_sort::GenericLatestSort,
        section::Section,
    },
};

#[tokio::test]
async fn advanced_search() {
    let api = GamebananaApi::new().unwrap();

    for section in Section::iter() {
        for page in 1..6 {
            println!("Page: {}", page);
            println!("Section: {}", section.as_str());
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
    let api = GamebananaApi::new().unwrap();

    for section in Section::iter() {
        api.search().modificators(section).await.unwrap();
    }
}

#[tokio::test]
async fn suggestions() {
    let api = GamebananaApi::new().unwrap();

    for section in Section::iter() {
        api.search()
            .suggestions("the", section, None)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn games_by_name() {
    let api = GamebananaApi::new().unwrap();

    for page in 1..6 {
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
    let api = GamebananaApi::new().unwrap();

    for page in 1..6 {
        api.search()
            .latest_all(
                Some(page),
                Some(50),
                Some(GenericLatestSort::New),
                Some(false),
            )
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn latest_game() {
    let api = GamebananaApi::new().unwrap();

    for page in 1..11 {
        println!("Page: {}", page);
        api.search()
            .latest_game(
                8552,
                Some(page),
                Some(GenericLatestSort::New),
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn latest_section() {
    let api = GamebananaApi::new().unwrap();

    for section in Section::iter() {
        println!("{}", section.as_str());
        let sorts = api.search().modificators(section).await.unwrap().sorts;

        for sort in sorts {
            println!("{}", sort.alias);
            api.search()
                .latest_section(section, Some(1), Some(50), Some(sort.alias.as_str()), None)
                .await
                .unwrap();
        }
    }
}
