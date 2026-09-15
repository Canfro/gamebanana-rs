use crate::{
    GamebananaApiV11,
    api_v11::endpoints::search::model::{
        advanced_record::AdvancedRecord, field::Field, image::Image, order::Order, section::Section,
    },
};

#[tokio::test]
async fn test_advanced_search() {
    let api = GamebananaApiV11::new().unwrap();

    let response = api
        .search()
        .advanced(
            "the",
            Some(1),
            Some(50),
            None,
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
