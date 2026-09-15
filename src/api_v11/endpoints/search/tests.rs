use crate::{
    GamebananaApiV11,
    api_v11::endpoints::search::model::{field::Field, order::Order, section::Section},
};

#[tokio::test]
async fn test_advanced_search() {
    let api = GamebananaApiV11::new().unwrap();

    let response = api
        .search()
        .advanced(
            "the",
            Some(4),
            Some(50),
            Some(Section::Wip),
            Some(Order::Date),
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
