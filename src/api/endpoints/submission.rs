use crate::GamebananaApi;

#[derive(Debug, Clone, Copy)]
pub struct Submission<'a> {
    api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn submission(&'a self) -> Submission<'a> {
        Submission { api: self }
    }
}
