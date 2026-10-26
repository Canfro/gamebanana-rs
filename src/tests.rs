use crate::{
    GamebananaApi,
    api::model::{field::Field, generic_latest_sort::GenericLatestSort, order::Order},
};

#[tokio::test]
async fn results() {
    let api = GamebananaApi::new().unwrap();

    for page in 1..101 {
        api.search()
            .results(
                "the",
                Some(page),
                Some(50),
                Some(Order::BestMatch),
                Some(&[
                    Field::Name,
                    Field::Description,
                    Field::Article,
                    Field::Attribs,
                    Field::Studio,
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
async fn game_name_match() {
    let api = GamebananaApi::new().unwrap();

    for page in 1..101 {
        api.search()
            .game_name_match("the", Some(page), Some(50))
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn tags() {
    let api = GamebananaApi::new().unwrap();

    api.search().tags("the", None).await.unwrap();
}

#[tokio::test]
async fn discussions() {
    let api = GamebananaApi::new().unwrap();

    for page in 1..101 {
        api.search().discussions(8552, Some(page)).await.unwrap();
    }
}

#[tokio::test]
async fn latest_submissions() {
    let api = GamebananaApi::new().unwrap();

    for page in 1..101 {
        println!("Page: {}", page);
        api.search()
            .latest_submissions(Some(page), Some(50), Some(GenericLatestSort::New), None)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn top_games() {
    let api = GamebananaApi::new().unwrap();

    api.search().top_games().await.unwrap();
}

#[tokio::test]
async fn community_spotlight() {
    todo!()
}

#[tokio::test]
async fn featured() {
    let api = GamebananaApi::new().unwrap();

    for page in 1..101 {
        api.search()
            .featured(Some(page), Some(50), None)
            .await
            .unwrap();
    }
}
