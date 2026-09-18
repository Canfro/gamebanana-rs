use crate::GamebananaApi;

#[derive(Debug, Clone, Copy)]
pub struct Dict<'a> {
    api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn dict(&'a self) -> Dict<'a> {
        Dict { api: self }
    }
}
