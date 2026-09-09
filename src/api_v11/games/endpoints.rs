use crate::GamebananaApi;

#[derive(Debug, Clone, Copy)]
pub struct Games<'a> {
    api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn games(&'a self) -> Games<'a> {
        Games { api: self }
    }
}
