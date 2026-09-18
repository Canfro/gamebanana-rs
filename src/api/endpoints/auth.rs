use crate::GamebananaApi;

#[derive(Debug, Clone, Copy)]
pub struct Auth<'a> {
    api: &'a GamebananaApi,
}

impl<'a> GamebananaApi {
    pub fn auth(&'a self) -> Auth<'a> {
        Auth { api: self }
    }
}
