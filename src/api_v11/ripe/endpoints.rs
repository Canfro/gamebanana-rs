use crate::GamebananaApi;

#[derive(Debug, Clone, Copy)]
pub struct Ripe<'a> {
    api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn ripe(&'a self) -> Ripe<'a> {
        Ripe { api: self }
    }
}
